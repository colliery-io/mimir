//! # Mimir LLM Provider Abstraction
//!
//! This crate provides a provider-agnostic abstraction layer for Large Language Models (LLMs).
//! It supports multiple endpoints (chat, completion, embedding) with configurable rate limiting
//! and provider-specific implementations.
//!
//! ## Features
//!
//! - **Provider abstraction**: Unified interface for different LLM providers
//! - **Rate limiting**: Configurable rate limiting with token bucket algorithm
//! - **Multiple endpoints**: Support for chat, completion, and embedding endpoints
//! - **Configuration**: YAML-based configuration system
//! - **Async support**: Full async/await support with tokio
//!
//! ## Quick Start
//!
//! ```rust
//! use mimir_dm_llm::{
//!     config::{ModelConfig, EndpointType},
//!     providers::ollama::OllamaProvider,
//!     provider::{LlmProvider, Message},
//! };
//! use std::collections::HashMap;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create configuration
//! let mut config_map = HashMap::new();
//! config_map.insert("base_url".to_string(), "http://localhost:11434".to_string());
//!
//! let config = ModelConfig {
//!     name: "llama3".to_string(),
//!     supported_endpoints: vec![EndpointType::Chat, EndpointType::Embedding],
//!     provider: "ollama".to_string(),
//!     model: "llama3".to_string(),
//!     config: Some(config_map),
//!     limit: None,
//! };
//!
//! // Create provider
//! let provider = OllamaProvider::new(config)?;
//! 
//! // Use chat endpoint
//! let messages = vec![Message {
//!     role: "user".to_string(),
//!     content: "Hello, world!".to_string(),
//! }];
//!
//! let response = provider.chat(messages, None, None, None, None, None).await?;
//! println!("Response: {}", response.content);
//! # Ok(())
//! # }
//! ```

pub mod config;
pub mod traits;
pub mod providers;
pub mod tools;

// Re-export commonly used types from config
pub use config::{ModelConfig, EndpointType, RateLimit, RenewalPeriod, ConfigError, FileToolsConfig};

// Re-export provider trait and types
pub use traits::provider::{
    LlmProvider, LlmError, ChatResponse, CompletionResponse, EmbeddingResponse,
    Message, Usage, Timing, ModelInfo, ModelPullProgress, RateLimitState,
    Tool, ToolFunction, ToolCall, ToolCallFunction
};

// Re-export tool trait
pub use traits::ToolTrait;

// Re-export tools
pub use tools::{
    TodoListTool, TodoItem, TodoStateManager,
    ReadFileTool, WriteFileTool, ListFilesTool, EditFileTool,
};