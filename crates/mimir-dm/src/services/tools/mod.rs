//! Tool system for LLM function calling
//! 
//! This module provides tools that can be called by the LLM to fetch data
//! and perform actions within the application.

use anyhow::Result;
use mimir_dm_llm::{Tool as LlmTool, ToolTrait};
use mimir_dm_llm::traits::ToolCallContext as ToolCall;
use serde_json::Value;
use std::collections::{HashMap, VecDeque};
use crate::APP_PATHS;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tracing::{info, warn};


/// Registry of available tools
pub struct ToolRegistry {
    tools: HashMap<String, Arc<dyn ToolTrait>>,
    recent_calls: Arc<Mutex<VecDeque<ToolCall>>>,
}

impl ToolRegistry {
    /// Create a new empty tool registry
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
            recent_calls: Arc::new(Mutex::new(VecDeque::with_capacity(10))),
        }
    }
    
    /// Register a tool
    pub fn register(&mut self, tool: Arc<dyn ToolTrait>) {
        let name = tool.name().to_string();
        info!("Registering tool: {}", name);
        self.tools.insert(name, tool);
    }
    
    /// Get all tool definitions for the LLM
    pub fn get_tool_definitions(&self) -> Vec<LlmTool> {
        self.tools
            .values()
            .map(|tool| tool.to_llm_tool())
            .collect()
    }
    
    /// Execute a tool by name with the given arguments
    pub async fn execute_tool(&self, name: &str, arguments: Value) -> Result<String> {
        // Record the tool call before execution
        self.record_tool_call(name, &arguments);
        
        match self.tools.get(name) {
            Some(tool) => {
                let result = tool.execute_with_context(arguments, self.recent_calls.clone()).await
                    .map_err(|e| anyhow::anyhow!("Tool execution failed: {}", e))?;
                Ok(result)
            }
            None => {
                warn!("Tool not found: {}", name);
                Err(anyhow::anyhow!("Tool not found: {}", name))
            }
        }
    }
    
    /// Record a tool call in the recent calls history
    fn record_tool_call(&self, name: &str, arguments: &Value) {
        let mut calls = self.recent_calls.lock().unwrap();
        
        // Extract file_path from arguments if present
        let file_path = arguments.get("file_path")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        calls.push_back(ToolCall {
            name: name.to_string(),
            timestamp: Instant::now(),
            file_path,
        });
        
        // Keep only the last 10 calls
        while calls.len() > 10 {
            calls.pop_front();
        }
    }
    
    /// Check if a tool exists
    pub fn has_tool(&self, name: &str) -> bool {
        self.tools.contains_key(name)
    }
    
    
    /// Check if a tool requires confirmation
    pub fn requires_confirmation(&self, name: &str) -> bool {
        self.tools.get(name)
            .map(|tool| tool.requires_confirmation())
            .unwrap_or(false)
    }
    
    /// Get action description for a tool
    pub fn get_action_description(&self, name: &str, arguments: &Value) -> Option<mimir_dm_llm::traits::ActionDescription> {
        self.tools.get(name)
            .and_then(|tool| tool.describe_action(arguments))
    }
    
    /// Generate system prompt rules based on registered tools
    /// 
    /// This method examines the available tools and generates guidance rules
    /// that help the LLM understand tool dependencies and proper usage patterns.
    pub fn generate_system_rules(&self, session_id: Option<&str>) -> Vec<String> {
        let mut rules = Vec::new();
        
        // Debug: Check if APP_PATHS is available
        if let Some(app_paths) = APP_PATHS.get() {
            info!("APP_PATHS is available: data_dir = {}", app_paths.data_dir.display());
        } else {
            warn!("APP_PATHS is None in generate_system_rules!");
        }
        
        // Add general context information
        rules.push(self.generate_context_information(session_id));
        
        // Check if both get_document and update_document are available
        if self.has_tool("get_document") && self.has_tool("update_document") {
            rules.push(
                "TOOL USAGE RULE: When asked to update, create, or work on a document:\n\
                1. ALWAYS call get_document first to read the current content\n\
                2. THEN call update_document with your changes\n\
                3. NEVER just show content without saving it - if you generate content, you MUST use update_document to save it\n\
                4. Don't just explain what you would do - actually make the tool calls to complete the user's request\n\
                5. If you create new content for a document, use update_document to save it immediately".to_string()
            );
        }
        
        // Add file tool usage rules if file tools are available
        if self.has_tool("read_file") && self.has_tool("write_file") && self.has_tool("list_files") {
            if let Some(app_paths) = APP_PATHS.get() {
                rules.push(format!(
                    "## FILE PATH REQUIREMENTS\n\
                    \n\
                    **REQUIRED DIRECTORY**: {}\n\
                    \n\
                    ### Correct Usage\n\
                    - **CORRECT**: `{}/your_filename.txt`\n\
                    - **WRONG**: `/app/data/your_filename.txt` (will fail)\n\
                    - **WRONG**: `/private/tmp/your_filename.txt` (will fail)\n\
                    - **WRONG**: `/tmp/your_filename.txt` (will fail)\n\
                    \n\
                    ### Critical Rules\n\
                    1. **Use the complete full path** shown above - never abbreviate\n\
                    2. **Do not shorten** to /app/data/ or any other form\n\
                    3. **Copy the exact path**: {}\n\
                    4. **Any other path will fail** with 'Path not within allowed directories' error\n\
                    \n\
                    ### When Taking Action\n\
                    - **IMMEDIATELY use the correct path** - do not hesitate or ask for clarification\n\
                    - **If uncertain about structure**, run list_files first\n\
                    - **Take direct action** when given clear file operation instructions",
                    app_paths.data_dir.display(),
                    app_paths.data_dir.display(),
                    app_paths.data_dir.display()
                ));
            } else {
                rules.push(
                    "## FILE OPERATIONS - DISCOVERY REQUIRED\n\
                    \n\
                    ### Immediate Action Required\n\
                    1. **FIRST STEP**: Run `list_files` with no arguments to discover the allowed directory\n\
                    2. **Expected result**: Path like `/Users/username/Library/Application Support/app/data/`\n\
                    \n\
                    ### Forbidden Paths\n\
                    - **Never use**: /tmp, /private/tmp, /var/tmp, ~/ or any system directories\n\
                    - **Never guess**: Paths like `/private/tmp/test_file.txt` will always fail\n\
                    \n\
                    ### Required Workflow\n\
                    1. Run: `list_files()` (no arguments)\n\
                    2. Observe the directory path returned\n\
                    3. Use that exact path for all file operations\n\
                    \n\
                    ### Direct Action\n\
                    - **Take immediate action** - run list_files now if you need to perform file operations\n\
                    - **Do not explain or hesitate** - execute the required workflow".to_string()
                );
                warn!("APP_PATHS is None when generating file tool rules");
            }
        }
        
        // Add more tool-specific rules here as needed in the future
        
        // Debug: Log what rules we generated
        info!("Generated {} system rules total", rules.len());
        for (i, rule) in rules.iter().enumerate() {
            if rule.contains("FILE TOOL USAGE RULES") {
                info!("Rule {}: FILE TOOL USAGE RULES (first 200 chars): {}", i, &rule.chars().take(200).collect::<String>());
            }
        }
        
        rules
    }
    
    /// Generate context information for the LLM session
    fn generate_context_information(&self, session_id: Option<&str>) -> String {
        let mut context = String::from("## Session Context\n");
        
        // Add file directory first - this is critical for LLM to know
        if let Some(app_paths) = APP_PATHS.get() {
            context.push_str(&format!(
                "**CRITICAL: FILE OPERATIONS DIRECTORY**\n\
                - **REQUIRED PATH**: {}\n\
                - **DO NOT USE**: /app/data/, /private/tmp/, or any abbreviated paths\n\
                - **ACTION REQUIRED**: Use the exact full path shown above for ALL file operations\n\n",
                app_paths.data_dir.display()
            ));
        }
        
        // Add session ID if available
        if let Some(session_id) = session_id {
            context.push_str(&format!("- Session ID: {}\n", session_id));
        }
        
        // Add application directory information
        if let Some(app_paths) = APP_PATHS.get() {
            context.push_str(&format!(
                "- **File Directory**: {}\n\
                - **Database Location**: {}\n\
                - **Configuration Directory**: {}\n\
                - **Available subdirectories**: campaigns/, modules/, documents/, templates/\n\
                - **Path Requirement**: ALL file operations must use paths starting with: {}\n",
                app_paths.data_dir.display(),
                if app_paths.is_memory_db {
                    "In-memory (temporary)".to_string()
                } else {
                    app_paths.database_path.display().to_string()
                },
                app_paths.config_dir.display(),
                app_paths.data_dir.display()
            ));
        } else {
            // Fallback: try to get directory info from file tools configuration
            let mut found_directories = false;
            if let Some(read_tool) = self.tools.get("read_file") {
                // Try to extract directory information from tool configuration
                let empty_args = serde_json::json!({});
                if let Some(action_desc) = read_tool.describe_action(&empty_args) {
                    if action_desc.description.contains("allowed directories") {
                        context.push_str("- Application Data Directory: [Available through file tools]\n");
                        found_directories = true;
                    }
                }
            }
            
            if !found_directories {
                context.push_str("- Application Data Directory: [Use list_files to discover]\n");
                warn!("APP_PATHS is None when generating context information - LLM will need to use list_files");
            }
        }
        
        // Add tool availability context
        let tool_count = self.tools.len();
        let tool_names: Vec<&str> = self.tools.keys().map(|s| s.as_str()).collect();
        context.push_str(&format!(
            "- Available Tools ({}): {}\n",
            tool_count,
            tool_names.join(", ")
        ));
        
        // Add session management and action guidance notes
        context.push_str(
            "- **Session Management**: Persistent session where previous context and actions may influence current behavior\n\
            - **Tool State**: Some tools maintain state across calls (e.g., todo_write tracks progress per session)\n\
            - **File Operations**: All file operations are sandboxed to the application directory for security\n\
            - **Path Requirements**: ALWAYS use fully qualified (absolute) paths for all file operations\n\
            \n\
            ### Action Guidelines\n\
            - **When given clear instructions**: Take immediate action without explanation or hesitation\n\
            - **Use tools directly**: Do not explain what you will do - just do it\n\
            - **Be decisive**: Follow instructions promptly and execute the requested operations\n\
            - **Complete tasks**: Execute all requested steps in sequence\n"
        );
        
        context
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}