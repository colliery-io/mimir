//! Todo List Tool for managing complex task workflows
//!
//! Helps the LLM track multi-step tasks and avoid context rot during long conversations.

use async_trait::async_trait;
use mimir_dm_llm::ToolTrait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tracing::{debug, error, info};

/// Represents a single todo item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoItem {
    pub content: String,
    pub status: String, // "pending", "in_progress", "completed"
    #[serde(rename = "activeForm")]
    pub active_form: String, // Present tense form for display during execution
}

/// Event for task state changes
#[derive(Clone, Serialize, Deserialize)]
pub struct TaskStateChangeEvent {
    pub task_content: String,
    pub old_status: String,
    pub new_status: String,
    pub session_id: String,
}

/// Tool for managing todo lists with JSON file persistence
pub struct TodoListTool {
    todos_dir: PathBuf,
    app_handle: Option<AppHandle>,
}

impl TodoListTool {
    pub fn new(app_data_dir: PathBuf) -> Self {
        let todos_dir = app_data_dir.join("todos");
        
        // Ensure the todos directory exists
        if let Err(e) = fs::create_dir_all(&todos_dir) {
            error!("Failed to create todos directory: {}", e);
        } else {
            debug!("Todo directory initialized: {}", todos_dir.display());
        }
        
        Self { 
            todos_dir,
            app_handle: None,
        }
    }
    
    /// Set app handle for event emission
    pub fn with_app_handle(mut self, app_handle: AppHandle) -> Self {
        self.app_handle = Some(app_handle);
        self
    }
    
    /// Get the file path for a session's todos
    fn get_session_file_path(&self, session_id: &str) -> PathBuf {
        self.todos_dir.join(format!("{}.json", session_id))
    }
    
    /// Load todos from file for a session
    fn load_todos(&self, session_id: &str) -> Result<Vec<TodoItem>, Box<dyn Error + Send + Sync>> {
        let file_path = self.get_session_file_path(session_id);
        
        if !file_path.exists() {
            debug!("Todo file does not exist for session {}, returning empty list", session_id);
            return Ok(Vec::new());
        }
        
        let content = fs::read_to_string(&file_path)
            .map_err(|e| format!("Failed to read todo file: {}", e))?;
        
        let todos: Vec<TodoItem> = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse todo JSON: {}", e))?;
        
        debug!("Loaded {} todos for session {}", todos.len(), session_id);
        Ok(todos)
    }
    
    /// Save todos to file for a session
    fn save_todos(&self, session_id: &str, todos: &[TodoItem]) -> Result<(), Box<dyn Error + Send + Sync>> {
        let file_path = self.get_session_file_path(session_id);
        
        let json_content = serde_json::to_string_pretty(todos)
            .map_err(|e| format!("Failed to serialize todos: {}", e))?;
        
        fs::write(&file_path, json_content)
            .map_err(|e| format!("Failed to write todo file: {}", e))?;
        
        info!("Saved {} todos for session {}", todos.len(), session_id);
        Ok(())
    }
    
    /// Validate that only one task can be in_progress at a time
    fn validate_single_in_progress(todos: &[TodoItem]) -> Result<(), Box<dyn Error + Send + Sync>> {
        let in_progress_count = todos.iter()
            .filter(|todo| todo.status == "in_progress")
            .count();
        
        if in_progress_count > 1 {
            return Err("Only one task can be in_progress at a time".into());
        }
        
        Ok(())
    }
}

#[async_trait]
impl ToolTrait for TodoListTool {
    fn name(&self) -> &str {
        "todo_write"
    }
    
    fn description(&self) -> &str {
        "Use this tool to create and manage a structured task list for your current coding session. This helps you track progress, organize complex tasks, and demonstrate thoroughness to the user.
It also helps the user understand the progress of the task and overall progress of their requests.

## When to Use This Tool
Use this tool proactively in these scenarios:

1. Complex multi-step tasks - When a task requires 3 or more distinct steps or actions
2. Non-trivial and complex tasks - Tasks that require careful planning or multiple operations
3. User explicitly requests todo list - When the user directly asks you to use the todo list
4. User provides multiple tasks - When users provide a list of things to be done (numbered or comma-separated)
5. After receiving new instructions - Immediately capture user requirements as todos
6. When you start working on a task - Mark it as in_progress BEFORE beginning work. Ideally you should only have one todo as in_progress at a time
7. After completing a task - Mark it as completed and add any new follow-up tasks discovered during implementation

## When NOT to Use This Tool

Skip using this tool when:
1. There is only a single, straightforward task
2. The task is trivial and tracking it provides no organizational benefit
3. The task can be completed in less than 3 trivial steps
4. The task is purely conversational or informational

NOTE that you should not use this tool if there is only one trivial task to do. In this case you are better off just doing the task directly.

## Task States and Management

1. **Task States**: Use these states to track progress:
   - pending: Task not yet started
   - in_progress: Currently working on (limit to ONE task at a time)
   - completed: Task finished successfully

   **IMPORTANT**: Task descriptions must have two forms:
   - content: The imperative form describing what needs to be done (e.g., \"Run tests\", \"Build the project\")
   - activeForm: The present continuous form shown during execution (e.g., \"Running tests\", \"Building the project\")

2. **Task Management**:
   - Update task status in real-time as you work
   - Mark tasks complete IMMEDIATELY after finishing (don't batch completions)
   - Exactly ONE task must be in_progress at any time (not less, not more)
   - Complete current tasks before starting new ones
   - Remove tasks that are no longer relevant from the list entirely

3. **Task Completion Requirements**:
   - ONLY mark a task as completed when you have FULLY accomplished it
   - If you encounter errors, blockers, or cannot finish, keep the task as in_progress
   - When blocked, create a new task describing what needs to be resolved
   - Never mark a task as completed if:
     - Tests are failing
     - Implementation is partial
     - You encountered unresolved errors
     - You couldn't find necessary files or dependencies

4. **Task Breakdown**:
   - Create specific, actionable items
   - Break complex tasks into smaller, manageable steps
   - Use clear, descriptive task names
   - Always provide both forms:
     - content: \"Fix authentication bug\"
     - activeForm: \"Fixing authentication bug\"

When in doubt, use this tool. Being proactive with task management demonstrates attentiveness and ensures you complete all requirements successfully.

## Example JSON Structure
When calling this tool, use EXACTLY this format:
{
  \"todos\": [
    {
      \"content\": \"Implement login feature\",
      \"status\": \"pending\",
      \"activeForm\": \"Implementing login feature\"
    },
    {
      \"content\": \"Write unit tests\", 
      \"status\": \"in_progress\",
      \"activeForm\": \"Writing unit tests\"
    }
  ]
}
CRITICAL: The field name is \"status\" NOT \"state\". Different LLM models may try to use \"state\" - this will cause errors."
    }
    
    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "todos": {
                    "type": "array",
                    "description": "The updated todo list",
                    "items": {
                        "type": "object",
                        "properties": {
                            "content": {
                                "type": "string",
                                "description": "Task description in imperative form (e.g., 'Fix bug', 'Run tests')",
                                "minLength": 1
                            },
                            "status": {
                                "type": "string",
                                "description": "Task status - MUST be 'status' field name, NOT 'state'",
                                "enum": ["pending", "in_progress", "completed"]
                            },
                            "activeForm": {
                                "type": "string",
                                "description": "Task description in present continuous form (e.g., 'Fixing bug', 'Running tests')",
                                "minLength": 1
                            }
                        },
                        "required": ["content", "status", "activeForm"],
                        "additionalProperties": false
                    }
                },
                "session_id": {
                    "type": "string",
                    "description": "Chat session ID for todo persistence (optional, defaults to 'default')"
                }
            },
            "required": ["todos"],
            "additionalProperties": false
        })
    }
    
    async fn execute(&self, arguments: Value) -> Result<String, Box<dyn Error + Send + Sync>> {
        // Accept both "todos" and "tasks" parameter names for backward compatibility
        let todos_array = arguments
            .get("todos")
            .or_else(|| arguments.get("tasks"))
            .and_then(|v| v.as_array())
            .ok_or("Missing or invalid 'todos' parameter")?;
        
        let todos: Vec<TodoItem> = todos_array
            .iter()
            .map(|item| -> Result<TodoItem, Box<dyn Error + Send + Sync>> {
                let content = item.get("content")
                    .and_then(|v| v.as_str())
                    .ok_or("Missing 'content' field")?;
                
                let status = item.get("status")
                    .and_then(|v| v.as_str())
                    .ok_or("Missing 'status' field")?;
                
                let active_form = item.get("activeForm")
                    .and_then(|v| v.as_str())
                    .ok_or("Missing 'activeForm' field")?;
                
                // Validate status
                if !["pending", "in_progress", "completed"].contains(&status) {
                    return Err(format!("Invalid status: {}", status).into());
                }
                
                Ok(TodoItem {
                    content: content.to_string(),
                    status: status.to_string(),
                    active_form: active_form.to_string(),
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        
        // Validate only one in_progress task
        Self::validate_single_in_progress(&todos)?;
        
        // Get session ID from arguments or use default
        let session_id = arguments
            .get("session_id")
            .and_then(|v| v.as_str())
            .unwrap_or("default");
        
        // Load previous todos to detect state changes
        let previous_todos = self.load_todos(session_id).unwrap_or_default();
        
        // Emit events for all state changes
        if let Some(ref app) = self.app_handle {
            for todo in &todos {
                // Find the previous state of this task
                let old_status = previous_todos
                    .iter()
                    .find(|prev| prev.content == todo.content)
                    .map(|prev| prev.status.as_str())
                    .unwrap_or("new"); // "new" for tasks that didn't exist before
                
                // Emit event if status changed
                if old_status != todo.status {
                    let state_change_event = TaskStateChangeEvent {
                        task_content: todo.content.clone(),
                        old_status: old_status.to_string(),
                        new_status: todo.status.clone(),
                        session_id: session_id.to_string(),
                    };
                    
                    if let Err(e) = app.emit("task-state-changed", &state_change_event) {
                        debug!("Failed to emit task state change event: {}", e);
                    } else {
                        debug!("Emitted state change event for task '{}': {} → {}", 
                               todo.content, old_status, todo.status);
                    }
                }
            }
        }
        
        // Save the todos
        self.save_todos(session_id, &todos)?;
        
        // Return a summary
        let pending_count = todos.iter().filter(|t| t.status == "pending").count();
        let in_progress_count = todos.iter().filter(|t| t.status == "in_progress").count();
        let completed_count = todos.iter().filter(|t| t.status == "completed").count();
        
        let summary = if in_progress_count > 0 {
            let current_task = todos.iter()
                .find(|t| t.status == "in_progress")
                .map(|t| t.active_form.as_str())
                .unwrap_or("Unknown task");
            
            format!(
                "Todos have been modified successfully. Ensure that you continue to use the todo list to track your progress. Please proceed with the current tasks if applicable\n\nCurrent status: {} ({} pending, {} in progress, {} completed)",
                current_task, pending_count, in_progress_count, completed_count
            )
        } else {
            format!(
                "Todos have been modified successfully. Ensure that you continue to use the todo list to track your progress. Please proceed with the current tasks if applicable\n\nStatus: {} pending, {} completed",
                pending_count, completed_count
            )
        };
        
        info!("Updated todo list: {} items total", todos.len());
        Ok(summary)
    }
}