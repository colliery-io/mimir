//! Example tools for testing and demonstration

use async_trait::async_trait;
use crate::ToolTrait;
use serde_json::{json, Value};
use std::error::Error;

/// Simple test tool that says hello
pub struct SayHelloTool;

#[async_trait]
impl ToolTrait for SayHelloTool {
    fn name(&self) -> &str {
        "say_hello"
    }
    
    fn description(&self) -> &str {
        "Say hello to someone"
    }
    
    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "name": {
                    "type": "string",
                    "description": "The name of the person to greet"
                }
            },
            "required": ["name"]
        })
    }
    
    async fn execute(&self, arguments: Value) -> Result<String, Box<dyn Error + Send + Sync>> {
        let name = arguments
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or("Missing 'name' parameter")?;
        
        Ok(format!("Hello, {}!", name))
    }
}