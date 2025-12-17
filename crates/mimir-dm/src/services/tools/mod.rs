//! Tool system for LLM function calling
//!
//! This module provides tools that can be called by the LLM to fetch data
//! and perform actions within the application.

use anyhow::Result;
use mimir_dm_llm::traits::ToolCallContext as ToolCall;
use mimir_dm_llm::{Tool as LlmTool, ToolTrait};
use serde_json::Value;
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::path::Path;
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
        self.tools.values().map(|tool| tool.to_llm_tool()).collect()
    }

    /// Execute a tool by name with the given arguments
    pub async fn execute_tool(&self, name: &str, arguments: Value) -> Result<String> {
        // Record the tool call before execution
        self.record_tool_call(name, &arguments);

        match self.tools.get(name) {
            Some(tool) => {
                let result = tool
                    .execute_with_context(arguments, self.recent_calls.clone())
                    .await
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
        let file_path = arguments
            .get("file_path")
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
        self.tools
            .get(name)
            .map(|tool| tool.requires_confirmation())
            .unwrap_or(false)
    }

    /// Get action description for a tool
    pub fn get_action_description(
        &self,
        name: &str,
        arguments: &Value,
    ) -> Option<mimir_dm_llm::traits::ActionDescription> {
        self.tools
            .get(name)
            .and_then(|tool| tool.describe_action(arguments))
    }

    /// Generate system prompt rules based on registered tools
    ///
    /// This method examines the available tools and generates guidance rules
    /// that help the LLM understand tool dependencies and proper usage patterns.
    pub fn generate_system_rules(&self, session_id: Option<&str>) -> Vec<String> {
        self.generate_system_rules_with_directory(session_id, None)
    }

    /// Generate system prompt rules with optional custom directory override
    ///
    /// This method allows overriding the directory path for campaign-specific operations
    pub fn generate_system_rules_with_directory(
        &self,
        session_id: Option<&str>,
        custom_directory: Option<&str>,
    ) -> Vec<String> {
        let mut rules = Vec::new();

        // When campaign directory is provided, we work exclusively with that directory
        if let Some(campaign_dir) = custom_directory {
            info!(
                "Using campaign directory for file operations: {}",
                campaign_dir
            );
        }

        // Add general context information
        rules.push(self.generate_context_information(session_id, custom_directory));

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
        if self.has_tool("read_file") && self.has_tool("write_file") && self.has_tool("list_files")
        {
            if let Some(custom_dir) = custom_directory {
                // Use campaign directory exclusively - no fallback to data directory
                rules.push(format!(
                    "## FILE PATH REQUIREMENTS\n\
                    \n\
                    **CAMPAIGN DIRECTORY**: {}\n\
                    \n\
                    ### Usage Guidelines\n\
                    - ALL file operations must use this campaign directory\n\
                    - Always use the complete path: `{}/your_filename.txt`\n\
                    - If uncertain about structure, run list_files first\n\
                    \n\
                    ### Campaign Structure\n\
                    - You are working within a campaign directory structure\n\
                    - Standard subdirectories: session_zero/, world/, modules/, sessions/, characters/, npcs/, resources/, templates/\n\
                    - Templates are in the templates/ subdirectory\n\
                    \n\
                    ### When Taking Action\n\
                    - Use the exact campaign path for ALL file operations\n\
                    - Take direct action when given clear file operation instructions",
                    custom_dir,
                    custom_dir
                ));
            } else {
                // When no campaign directory is provided, require discovery
                rules.push(
                    "## FILE OPERATIONS - CAMPAIGN REQUIRED\n\
                    \n\
                    ### Required Workflow\n\
                    1. A campaign must be selected before file operations can be performed\n\
                    2. File operations are only available within campaign context\n\
                    3. Use list_files() only after campaign context is established\n"
                        .to_string(),
                );
                warn!("No campaign directory provided - file operations limited");
            }
        }

        // Add tool awareness and relationship guidance
        rules.push(self.generate_tool_awareness_guidance());

        // Debug: Log what rules we generated
        info!("Generated {} system rules total", rules.len());
        for (i, rule) in rules.iter().enumerate() {
            if rule.contains("FILE TOOL USAGE RULES") {
                info!(
                    "Rule {}: FILE TOOL USAGE RULES (first 200 chars): {}",
                    i,
                    &rule.chars().take(200).collect::<String>()
                );
            }
        }

        rules
    }

    /// Generate context information for the LLM session
    fn generate_context_information(
        &self,
        session_id: Option<&str>,
        custom_directory: Option<&str>,
    ) -> String {
        let mut context = String::from("## Session Context\n");

        // Add campaign directory information - this is critical for LLM to know
        if let Some(custom_dir) = custom_directory {
            context.push_str(&format!(
                "**CAMPAIGN DIRECTORY**\n\
                - **PATH**: {}\n\
                - **ALL file operations must use this campaign directory**\n\
                - **CONTEXT**: Active campaign - files will be created in organized campaign structure\n\n",
                custom_dir
            ));
        } else {
            context.push_str(
                "**FILE OPERATIONS**\n\
                - **STATUS**: No campaign selected\n\
                - **REQUIREMENT**: Campaign must be selected for file operations\n\n",
            );
        }

        // Add session ID if available
        if let Some(session_id) = session_id {
            context.push_str(&format!("- Session ID: {}\n", session_id));
        }

        // Add campaign directory structure information
        if let Some(custom_dir) = custom_directory {
            // Get available files in the campaign directory
            let available_files = self.get_campaign_files(custom_dir);

            context.push_str(&format!(
                "- **Campaign Path**: {}\n\
                - **Structure**: Organized campaign directory with standard subdirectories\n\
                - **Templates**: Available in templates/ subdirectory for structured content creation\n\
                - **Path Requirement**: ALL file operations must use paths starting with: {}\n",
                custom_dir,
                custom_dir
            ));

            if !available_files.is_empty() {
                context.push_str("- **Available Files**:\n");
                for file in available_files.iter().take(20) {
                    // Limit to 20 files to avoid overwhelming
                    context.push_str(&format!("  - {}\n", file));
                }
                if available_files.len() > 20 {
                    context.push_str(&format!(
                        "  - ... and {} more files\n",
                        available_files.len() - 20
                    ));
                }
            } else {
                context.push_str(
                    "- **Available Files**: No files found - this appears to be a new campaign\n",
                );
            }
        } else {
            context.push_str(
                "- **Campaign Status**: No active campaign\n\
                - **File Operations**: Unavailable - campaign selection required\n\
                - **Next Step**: Select or create a campaign to enable file operations\n",
            );
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
        guidance.push_str(&format!(
            "You have access to these tools: {}\n\n",
            tool_names.join(", ")
        ));

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

        guidance.push_str("### ReAct Pattern for Multi-Step Tasks\n\n");
        guidance.push_str("For tasks requiring multiple tool calls, use explicit reasoning:\n\n");
        guidance.push_str("1. **THOUGHT**: Analyze what needs to be done and plan your approach\n");
        guidance.push_str("   - Use `<thought>` blocks to show your reasoning\n");
        guidance.push_str("   - Break complex tasks into clear steps\n");
        guidance.push_str("2. **ACTION**: Execute the appropriate tool(s)\n");
        guidance.push_str("3. **OBSERVATION**: Examine tool results and determine next steps\n");
        guidance.push_str("4. **REPEAT**: Continue until task is complete\n\n");

        guidance.push_str("Example:\n");
        guidance.push_str("```\n");
        guidance.push_str("<thought>\n");
        guidance.push_str("User wants to add an NPC to the session. I need to:\n");
        guidance.push_str("1. Read the current session NPCs file\n");
        guidance.push_str("2. Add the new NPC with appropriate stats\n");
        guidance.push_str("3. Save the updated file\n");
        guidance.push_str("</thought>\n");
        guidance.push_str("[executes read_file]\n");
        guidance.push_str("<thought>\n");
        guidance.push_str("Got the NPCs file. Now I'll add the blacksmith with:\n");
        guidance.push_str("- Name, role, and personality\n");
        guidance.push_str("- Key information and secrets\n");
        guidance.push_str("</thought>\n");
        guidance.push_str("[executes write_file with updated content]\n");
        guidance.push_str("```\n\n");

        guidance.push_str("### General Action Patterns\n");
        guidance.push_str("- Take direct action when user requests clear operations\n");
        guidance.push_str("- Use tools in logical sequence based on their guidance\n");
        guidance.push_str("- Always complete requested actions rather than just explaining them\n");
        guidance.push_str("- Show your reasoning in `<thought>` blocks for complex tasks\n");

        guidance
    }

    /// Get list of files in a campaign directory
    fn get_campaign_files(&self, campaign_dir: &str) -> Vec<String> {
        let campaign_path = Path::new(campaign_dir);
        let mut files = Vec::new();

        if !campaign_path.exists() {
            warn!("Campaign directory does not exist: {}", campaign_dir);
            return files;
        }

        // Recursively walk the directory and collect file paths
        if let Ok(entries) = self.walk_directory(campaign_path, campaign_path) {
            files = entries;
        }

        // Sort files for consistent display
        files.sort();
        files
    }

    /// Recursively walk a directory and return relative file paths
    #[allow(clippy::only_used_in_recursion)]
    fn walk_directory(&self, dir: &Path, base_path: &Path) -> Result<Vec<String>, std::io::Error> {
        let mut files = Vec::new();

        let entries = fs::read_dir(dir)?;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                // Skip hidden directories and common ignore patterns
                if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                    if dir_name.starts_with('.')
                        || dir_name == "node_modules"
                        || dir_name == "target"
                    {
                        continue;
                    }
                }

                // Recursively process subdirectory
                let mut sub_files = self.walk_directory(&path, base_path)?;
                files.append(&mut sub_files);
            } else {
                // Add file with relative path
                if let Ok(relative_path) = path.strip_prefix(base_path) {
                    if let Some(path_str) = relative_path.to_str() {
                        files.push(path_str.to_string());
                    }
                }
            }
        }

        Ok(files)
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// Character tools modules
pub mod character_tools;
pub mod character_write_tools;

// Catalog query tools
pub mod catalog_tools;

// Module management tools
pub mod module_tools;

#[cfg(test)]
mod character_tools_test;
