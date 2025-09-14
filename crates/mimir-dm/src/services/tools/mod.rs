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
                    ### Usage Guidelines\n\
                    - Always use the complete path: `{}/your_filename.txt`\n\
                    - Use the full path for all file operations\n\
                    - If uncertain about structure, run list_files first\n\
                    \n\
                    ### When Taking Action\n\
                    - Use the exact path shown above for ALL file operations\n\
                    - Take direct action when given clear file operation instructions",
                    app_paths.data_dir.display(),
                    app_paths.data_dir.display()
                ));
            } else {
                rules.push(
                    "## FILE OPERATIONS - DISCOVERY REQUIRED\n\
                    \n\
                    ### Required Workflow\n\
                    1. Run: `list_files()` (no arguments) to discover the allowed directory\n\
                    2. Use the exact path returned for all file operations\n\
                    3. Take immediate action when file operations are needed\n".to_string()
                );
                warn!("APP_PATHS is None when generating file tool rules");
            }
        }
        
        // Add tool awareness and relationship guidance
        rules.push(self.generate_tool_awareness_guidance());
        
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
                "**FILE OPERATIONS DIRECTORY**\n\
                - **REQUIRED PATH**: {}\n\
                - **ALL file operations must use this exact path**\n\n",
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
        
        // Add session management notes  
        context.push_str(
            "- **Session Management**: Persistent session where previous context and actions may influence current behavior\n\
            - **Tool State**: Some tools maintain state across calls (e.g., todo_write tracks progress per session)\n\
            - **File Operations**: All file operations are sandboxed to the application directory for security\n\
            - **Path Requirements**: ALWAYS use fully qualified (absolute) paths for all file operations\n"
        );
        
        context
    }
    
    /// Generate tool awareness and relationship guidance
    fn generate_tool_awareness_guidance(&self) -> String {
        let mut guidance = String::from("## TOOL AWARENESS\n\n");
        
        guidance.push_str("### Available Tools\n");
        let tool_names: Vec<&str> = self.tools.keys().map(|s| s.as_str()).collect();
        guidance.push_str(&format!("You have access to these tools: {}\n\n", tool_names.join(", ")));
        
        guidance.push_str("### Tool Workflow Guidance\n");
        
        // Collect workflow guidance from each tool
        let mut has_guidance = false;
        for (name, tool) in &self.tools {
            if let Some(tool_guidance) = tool.workflow_guidance() {
                guidance.push_str(&format!("**{}:**\n{}\n\n", name, tool_guidance));
                has_guidance = true;
            }
        }
        
        if !has_guidance {
            guidance.push_str("- Tools are independent and can be used as needed\n");
            guidance.push_str("- Follow tool descriptions for specific usage patterns\n\n");
        }
        
        guidance.push_str("### General Action Patterns\n");
        guidance.push_str("- Take direct action when user requests clear operations\n");
        guidance.push_str("- Use tools in logical sequence based on their guidance\n");
        guidance.push_str("- Always complete requested actions rather than just explaining them\n");
        
        guidance
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}