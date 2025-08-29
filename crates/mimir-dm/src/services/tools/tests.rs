//! Tests for the tool system

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::super::implementations::SayHelloTool;
    use async_trait::async_trait;
    use mimir_dm_llm::ToolTrait;
    use serde_json::{json, Value};
    use std::error::Error;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_say_hello_tool() {
        let tool = SayHelloTool;
        
        // Test metadata
        assert_eq!(tool.name(), "say_hello");
        assert_eq!(tool.description(), "Say hello to someone");
        
        // Test schema
        let schema = tool.parameters_schema();
        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["name"].is_object());
        
        // Test execution
        let args = json!({
            "name": "Alice"
        });
        
        let result = tool.execute(args).await.unwrap();
        assert_eq!(result, "Hello, Alice! 👋");
    }
    
    #[tokio::test]
    async fn test_say_hello_missing_parameter() {
        let tool = SayHelloTool;
        
        let args = json!({});
        let result = tool.execute(args).await;
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Missing 'name' parameter");
    }
    
    #[tokio::test]
    async fn test_tool_registry() {
        let mut registry = ToolRegistry::new();
        
        // Initially empty
        assert_eq!(registry.tool_count(), 0);
        assert!(!registry.has_tool("say_hello"));
        
        // Register a tool
        let tool = Arc::new(SayHelloTool);
        registry.register(tool);
        
        // Check registration
        assert_eq!(registry.tool_count(), 1);
        assert!(registry.has_tool("say_hello"));
        
        // Get tool definitions
        let definitions = registry.get_tool_definitions();
        assert_eq!(definitions.len(), 1);
        assert_eq!(definitions[0].function.name, "say_hello");
    }
    
    #[tokio::test]
    async fn test_tool_execution_through_registry() {
        let mut registry = ToolRegistry::new();
        registry.register(Arc::new(SayHelloTool));
        
        // Execute tool
        let args = json!({
            "name": "Bob"
        });
        
        let result = registry.execute_tool("say_hello", args).await.unwrap();
        assert_eq!(result, "Hello, Bob! 👋");
    }
    
    #[tokio::test]
    async fn test_execute_nonexistent_tool() {
        let registry = ToolRegistry::new();
        
        let args = json!({});
        let result = registry.execute_tool("nonexistent", args).await;
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Tool not found"));
    }
    
    // Mock tool for testing
    struct MockTool {
        name: String,
        call_count: std::sync::atomic::AtomicU32,
    }
    
    #[async_trait]
    impl ToolTrait for MockTool {
        fn name(&self) -> &str {
            &self.name
        }
        
        fn description(&self) -> &str {
            "Mock tool for testing"
        }
        
        fn parameters_schema(&self) -> Value {
            json!({
                "type": "object",
                "properties": {}
            })
        }
        
        async fn execute(&self, _arguments: Value) -> Result<String, Box<dyn Error + Send + Sync>> {
            self.call_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            Ok("Mock response".to_string())
        }
    }
    
    #[tokio::test]
    async fn test_multiple_tools() {
        let mut registry = ToolRegistry::new();
        
        // Register multiple tools
        registry.register(Arc::new(SayHelloTool));
        registry.register(Arc::new(MockTool {
            name: "mock1".to_string(),
            call_count: std::sync::atomic::AtomicU32::new(0),
        }));
        registry.register(Arc::new(MockTool {
            name: "mock2".to_string(),
            call_count: std::sync::atomic::AtomicU32::new(0),
        }));
        
        assert_eq!(registry.tool_count(), 3);
        assert!(registry.has_tool("say_hello"));
        assert!(registry.has_tool("mock1"));
        assert!(registry.has_tool("mock2"));
        
        // Test definitions
        let definitions = registry.get_tool_definitions();
        assert_eq!(definitions.len(), 3);
    }
}