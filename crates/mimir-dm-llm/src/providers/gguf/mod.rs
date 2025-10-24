//! # GGUF Provider Module
//!
//! This module provides a generic GGUF model provider that can load and serve
//! ANY GGUF model file using llama.cpp as the inference engine.
//!
//! ## Key Features
//!
//! - Generic GGUF support: Any transformer model works without code changes
//! - Hardware acceleration: CPU, CUDA, Metal support
//! - Memory efficiency: Memory-mapped model loading
//! - Session pooling: Concurrent request handling
//! - Drop-in Ollama replacement: Same interface, no external services

pub mod config;
pub mod provider;
pub mod session;

pub use provider::GgufProvider;
pub use config::GgufConfig;