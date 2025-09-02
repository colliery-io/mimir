//! Tool trait for LLM function calling
//!
//! This module defines the trait that all tools must implement to be callable by the LLM.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;

use super::provider::{Tool as LlmTool, ToolFunction};

/// Risk level for tool actions that modify state
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RiskLevel {
    /// Low risk - e.g., updating documentation
    Low,
    /// Medium risk - e.g., modifying configuration
    Medium,
    /// High risk - e.g., deleting data
    High,
}

/// Description of an action that requires confirmation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionDescription {
    /// Brief title of the action
    pub title: String,
    /// Detailed description of what will happen
    pub description: String,
    /// List of specific changes that will be made
    pub changes: Vec<String>,
}

/// Trait that all callable tools must implement
#[async_trait]
pub trait Tool: Send + Sync {
    /// Get the tool's unique name
    fn name(&self) -> &str;
    
    /// Get the tool's description for the LLM
    fn description(&self) -> &str;
    
    /// Get the JSON Schema for the tool's parameters
    fn parameters_schema(&self) -> Value;
    
    /// Whether this tool requires user confirmation before execution
    /// 
    /// Default implementation returns false (no confirmation needed)
    fn requires_confirmation(&self) -> bool {
        false
    }
    
    /// Generate a human-readable description of the action for confirmation
    /// 
    /// This is only called when `requires_confirmation()` returns true.
    /// Default implementation returns None.
    fn describe_action(&self, _arguments: &Value) -> Option<ActionDescription> {
        None
    }
    
    /// Execute the tool with the given arguments
    async fn execute(&self, arguments: Value) -> Result<String, Box<dyn Error + Send + Sync>>;
    
    /// Convert to LLM tool definition
    fn to_llm_tool(&self) -> LlmTool {
        LlmTool {
            tool_type: "function".to_string(),
            function: ToolFunction {
                name: self.name().to_string(),
                description: self.description().to_string(),
                parameters: self.parameters_schema(),
            },
        }
    }
}