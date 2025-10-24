//! # GGUF Provider Implementation
//!
//! Main implementation of the LlmProvider trait for GGUF models using llama.cpp.

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio_util::sync::CancellationToken;
use tracing::{debug, info, warn};

use llama_cpp_rs::{LlamaModel, LlamaParams};

use crate::config::{ModelConfig, EndpointType};
use crate::traits::{
    LlmProvider, LlmError, ChatResponse, CompletionResponse, EmbeddingResponse,
    Message, RateLimitState, Usage, Timing, Tool
};

use super::config::GgufConfig;
use super::session::SessionPool;

/// Model metadata extracted from the loaded model
#[derive(Debug, Clone)]
pub struct ModelMetadata {
    pub vocab_size: usize,
    pub context_size: usize,
    pub model_type: String,
    pub parameter_count: u64,
    pub architecture: String,
}

/// GGUF provider implementation
pub struct GgufProvider {
    /// Model configuration
    config: ModelConfig,
    
    /// GGUF-specific configuration
    gguf_config: GgufConfig,
    
    /// Rate limiting state
    rate_limit_state: RateLimitState,
    
    /// The loaded llama.cpp model
    model: Arc<LlamaModel>,
    
    /// Session pool for concurrent requests
    session_pool: SessionPool,
    
    /// Model metadata
    metadata: ModelMetadata,
}

impl GgufProvider {
    /// Create a new GGUF provider
    pub async fn new(config: ModelConfig) -> Result<Self, LlmError> {
        let gguf_config = GgufConfig::from_model_config(&config)?;
        
        // Validate configuration
        gguf_config.validate()?;
        
        info!("Loading GGUF model from: {}", gguf_config.model_path.display());
        
        // Configure llama.cpp parameters
        let params = LlamaParams {
            n_ctx: gguf_config.context_size as i32,
            n_batch: gguf_config.batch_size as i32,
            n_gpu_layers: gguf_config.gpu_layers,
            n_threads: gguf_config.threads.map(|t| t as i32).unwrap_or(4),
            use_mlock: gguf_config.mlock,
            use_mmap: gguf_config.mmap,
            seed: gguf_config.seed.unwrap_or(42) as i32,
            ..Default::default()
        };
        
        debug!("llama.cpp params: ctx={}, batch={}, gpu_layers={}, threads={:?}", 
               params.n_ctx, params.n_batch, params.n_gpu_layers, 
               gguf_config.threads.unwrap_or(4));
        
        // Load the model
        let model = LlamaModel::load_from_file(
            &gguf_config.model_path,
            params
        ).map_err(|e| LlmError::ProviderError(
            format!("Failed to load GGUF model: {}", e)
        ))?;
        
        // Extract metadata
        let metadata = ModelMetadata {
            vocab_size: model.n_vocab() as usize,
            context_size: model.n_ctx() as usize,
            model_type: model.model_type().unwrap_or("unknown".to_string()),
            parameter_count: model.model_size(),
            architecture: model.model_arch().unwrap_or("transformer".to_string()),
        };
        
        info!("Loaded {} model: {} parameters, {} context, {} vocab",
              metadata.model_type,
              Self::format_param_count(metadata.parameter_count),
              metadata.context_size,
              metadata.vocab_size);
        
        // Create session pool
        let session_pool = SessionPool::new(Arc::clone(&model), 2)?;
        
        // Initialize rate limiting
        let rate_limit_state = config.limit.as_ref().map_or_else(
            || RateLimitState::default(),
            |limit| RateLimitState::new(limit)
        );
        
        Ok(Self {
            config,
            gguf_config,
            rate_limit_state,
            model,
            session_pool,
            metadata,
        })
    }
    
    /// Format parameter count for display
    fn format_param_count(count: u64) -> String {
        if count >= 1_000_000_000 {
            format!("{:.1}B", count as f64 / 1_000_000_000.0)
        } else if count >= 1_000_000 {
            format!("{:.1}M", count as f64 / 1_000_000.0)
        } else {
            format!("{}", count)
        }
    }
    
    /// Format chat messages into a prompt based on model type
    fn format_chat_prompt(&self, messages: &[Message]) -> String {
        let model_type = &self.metadata.model_type.to_lowercase();
        
        if model_type.contains("llama") || model_type.contains("mistral") {
            // Llama/Mistral format
            let mut prompt = String::new();
            for msg in messages {
                match msg.role.as_str() {
                    "system" => prompt.push_str(&format!("[INST] <<SYS>>\n{}\n<</SYS>> [/INST]\n", msg.content)),
                    "user" => prompt.push_str(&format!("[INST] {} [/INST]\n", msg.content)),
                    "assistant" => prompt.push_str(&format!("{}\n", msg.content)),
                    _ => {}
                }
            }
            prompt
        } else if model_type.contains("chatml") || model_type.contains("qwen") {
            // ChatML format
            let mut prompt = String::new();
            for msg in messages {
                prompt.push_str(&format!("<|im_start|>{}\n{}<|im_end|>\n", 
                                         msg.role, msg.content));
            }
            prompt.push_str("<|im_start|>assistant\n");
            prompt
        } else {
            // Generic format
            let mut prompt = String::new();
            for msg in messages {
                prompt.push_str(&format!("{}: {}\n", msg.role, msg.content));
            }
            prompt.push_str("assistant: ");
            prompt
        }
    }
    
    /// Generate text with the model
    async fn generate_text(
        &self,
        prompt: &str,
        max_tokens: u32,
        temperature: f32,
        stop_sequences: Option<&[String]>,
        cancellation_token: Option<&CancellationToken>,
    ) -> Result<(String, Usage, Timing), LlmError> {
        // Acquire session from pool
        let mut session_guard = self.session_pool.acquire()?;
        let session = session_guard.session_mut();
        
        // Configure sampling parameters
        session.set_temperature(temperature);
        if let Some(top_p) = self.gguf_config.top_p {
            session.set_top_p(top_p);
        }
        if let Some(top_k) = self.gguf_config.top_k {
            session.set_top_k(top_k as i32);
        }
        session.set_repeat_penalty(self.gguf_config.repeat_penalty);
        
        let start_time = Instant::now();
        
        // Process prompt
        session.advance_context(prompt)
            .map_err(|e| LlmError::ProviderError(
                format!("Failed to process prompt: {}", e)
            ))?;
        
        let prompt_eval_time = start_time.elapsed();
        let prompt_tokens = session.n_past() as u32;
        
        // Generate response
        let mut response = String::new();
        let mut completion_tokens = 0u32;
        
        for _ in 0..max_tokens {
            // Check cancellation
            if let Some(token) = cancellation_token {
                if token.is_cancelled() {
                    debug!("Generation cancelled");
                    return Err(LlmError::Cancelled);
                }
            }
            
            // Generate next token
            let token = session.sample_token()
                .map_err(|e| LlmError::ProviderError(
                    format!("Failed to sample token: {}", e)
                ))?;
            
            // Check for EOS
            if token == self.model.token_eos() {
                debug!("Hit EOS token, stopping generation");
                break;
            }
            
            // Convert token to text
            let token_str = self.model.token_to_str(token)
                .unwrap_or_default();
            
            // Check stop sequences
            if let Some(stop_seqs) = stop_sequences {
                if stop_seqs.iter().any(|seq| response.ends_with(seq) || token_str.contains(seq)) {
                    debug!("Hit stop sequence, stopping generation");
                    break;
                }
            }
            
            response.push_str(&token_str);
            completion_tokens += 1;
            
            // Advance context with new token
            session.advance_context_with_token(token)
                .map_err(|e| LlmError::ProviderError(
                    format!("Failed to advance context: {}", e)
                ))?;
        }
        
        let total_time = start_time.elapsed();
        
        // Calculate usage and timing
        let usage = Usage {
            prompt_tokens,
            completion_tokens,
            total_tokens: prompt_tokens + completion_tokens,
        };
        
        let timing = Timing {
            total_duration_ms: total_time.as_millis() as u64,
            load_duration_ms: 0, // Model is already loaded
            prompt_eval_duration_ms: prompt_eval_time.as_millis() as u64,
            completion_duration_ms: (total_time - prompt_eval_time).as_millis() as u64,
        };
        
        Ok((response, usage, timing))
    }
    
    /// Get model metadata
    pub fn metadata(&self) -> &ModelMetadata {
        &self.metadata
    }
    
    /// Get session pool statistics
    pub fn session_stats(&self) -> super::session::SessionPoolStats {
        self.session_pool.stats()
    }
}

#[async_trait]
impl LlmProvider for GgufProvider {
    fn config(&self) -> &ModelConfig {
        &self.config
    }
    
    fn rate_limit_state(&self) -> &RateLimitState {
        &self.rate_limit_state
    }
    
    async fn chat(
        &self,
        messages: Vec<Message>,
        _tools: Option<Vec<Tool>>,
        _n: Option<u32>,
        temperature: Option<f32>,
        max_tokens: Option<u32>,
        stop: Option<Vec<String>>,
        _extra_config: Option<HashMap<String, String>>,
        cancellation_token: Option<CancellationToken>,
    ) -> Result<ChatResponse, LlmError> {
        if !self.supports_endpoint(EndpointType::Chat) {
            return Err(LlmError::UnsupportedEndpoint("chat".to_string()));
        }
        
        self.check_rate_limit().await?;
        
        // Format messages into prompt
        let prompt = self.format_chat_prompt(&messages);
        debug!("Formatted chat prompt: {} chars", prompt.len());
        
        // Generate response
        let (content, usage, timing) = self.generate_text(
            &prompt,
            max_tokens.unwrap_or(512),
            temperature.unwrap_or(self.gguf_config.temperature),
            stop.as_deref(),
            cancellation_token.as_ref(),
        ).await?;
        
        Ok(ChatResponse {
            content,
            usage: Some(usage),
            timing: Some(timing),
            model: self.config.model.clone(),
            tool_calls: None, // TODO: Add tool support
        })
    }
    
    async fn complete(
        &self,
        prompt: String,
        _n: Option<u32>,
        temperature: Option<f32>,
        max_tokens: Option<u32>,
        stop: Option<Vec<String>>,
        _extra_config: Option<HashMap<String, String>>,
    ) -> Result<CompletionResponse, LlmError> {
        if !self.supports_endpoint(EndpointType::Completion) {
            return Err(LlmError::UnsupportedEndpoint("completion".to_string()));
        }
        
        self.check_rate_limit().await?;
        
        debug!("Completion request: {} chars", prompt.len());
        
        // Generate response
        let (text, usage, timing) = self.generate_text(
            &prompt,
            max_tokens.unwrap_or(512),
            temperature.unwrap_or(self.gguf_config.temperature),
            stop.as_deref(),
            None, // No cancellation token for completion
        ).await?;
        
        Ok(CompletionResponse {
            text,
            usage: Some(usage),
            timing: Some(timing),
            model: self.config.model.clone(),
        })
    }
    
    // Embedding not supported in this initial implementation
    async fn embed(
        &self,
        _input: Vec<String>,
        _extra_config: Option<HashMap<String, String>>,
    ) -> Result<EmbeddingResponse, LlmError> {
        Err(LlmError::NotSupported)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_format_param_count() {
        assert_eq!(GgufProvider::format_param_count(1_500_000_000), "1.5B");
        assert_eq!(GgufProvider::format_param_count(7_000_000), "7.0M");
        assert_eq!(GgufProvider::format_param_count(1000), "1000");
    }
    
    #[test]
    fn test_chat_prompt_formatting() {
        use std::collections::HashMap;
        
        let config = ModelConfig {
            name: "test".to_string(),
            supported_endpoints: vec![],
            provider: "gguf".to_string(),
            model: "test-model".to_string(),
            config: Some({
                let mut map = HashMap::new();
                map.insert("model_path".to_string(), "/dev/null".to_string());
                map
            }),
            limit: None,
        };
        
        // Can't actually create provider without real model file,
        // but we can test the logic structure
        assert!(GgufConfig::from_model_config(&config).is_ok());
    }
}