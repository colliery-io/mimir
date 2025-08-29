//! Tool trait for LLM function calling
//!
//! This module defines the trait that all tools must implement to be callable by the LLM.

use async_trait::async_trait;
use serde_json::Value;
use std::error::Error;

use super::provider::{Tool as LlmTool, ToolFunction};

/// Trait that all callable tools must implement
#[async_trait]
pub trait Tool: Send + Sync {
    /// Get the tool's unique name
    fn name(&self) -> &str;
    
    /// Get the tool's description for the LLM
    fn description(&self) -> &str;
    
    /// Get the JSON Schema for the tool's parameters
    fn parameters_schema(&self) -> Value;
    
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