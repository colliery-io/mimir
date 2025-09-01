//! Traits for LLM providers and related functionality

pub mod provider;
pub mod tool;
pub mod context;

// Re-export commonly used types
pub use provider::{
    LlmProvider, LlmError, 
    ChatResponse, CompletionResponse, EmbeddingResponse,
    Message, Usage, Timing,
    ModelInfo, ModelPullProgress,
    RateLimitState,
    Tool, ToolFunction, ToolCall, ToolCallFunction
};

pub use tool::{Tool as ToolTrait, ActionDescription, RiskLevel};
pub use context::ToolContext;