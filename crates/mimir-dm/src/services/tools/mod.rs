//! Tool system for LLM function calling
//! 
//! This module provides tools that can be called by the LLM to fetch data
//! and perform actions within the application.

use anyhow::Result;
use mimir_dm_llm::{Tool as LlmTool, ToolTrait};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{info, warn};

pub mod implementations;
pub mod document_tools;
pub mod update_document_tool;
pub mod todo_list_tool;

#[cfg(test)]
mod tests;

/// Registry of available tools
pub struct ToolRegistry {
    tools: HashMap<String, Arc<dyn ToolTrait>>,
}

impl ToolRegistry {
    /// Create a new empty tool registry
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
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
        match self.tools.get(name) {
            Some(tool) => {
                let result = tool.execute(arguments).await
                    .map_err(|e| anyhow::anyhow!("Tool execution failed: {}", e))?;
                Ok(result)
            }
            None => {
                warn!("Tool not found: {}", name);
                Err(anyhow::anyhow!("Tool not found: {}", name))
            }
        }
    }
    
    /// Check if a tool exists
    pub fn has_tool(&self, name: &str) -> bool {
        self.tools.contains_key(name)
    }
    
    /// Get the number of registered tools
    pub fn tool_count(&self) -> usize {
        self.tools.len()
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
    pub fn generate_system_rules(&self) -> Vec<String> {
        let mut rules = Vec::new();
        
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
        
        // Add more tool-specific rules here as needed in the future
        
        rules
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}