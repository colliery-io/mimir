//! LLM Service Module
//!
//! This module provides LLM (Large Language Model) functionality for the application,
//! organized into focused submodules:
//!
//! - `llm_service`: Core service for model management and initialization
//! - `chat_processor`: Chat message processing and tool execution
//! - `commands`: Tauri command handlers for frontend integration

mod chat_processor;
pub mod commands;
mod llm_service;

// Re-export main types from llm_service
pub use llm_service::{
    initialize_llm, CancellationTokens, ConfirmationReceivers, LlmService, OLLAMA_BASE_URL,
    REQUIRED_MODEL,
};
