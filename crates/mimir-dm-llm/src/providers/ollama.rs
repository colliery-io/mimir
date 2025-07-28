//! # Ollama Provider
//!
//! This module provides an implementation of the [`LlmProvider`] trait for the Ollama API.
//!
//! ## Configuration
//!
//! The Ollama provider requires the following configuration:
//!
//! ```yaml
//! name: "llama3"
//! model: "llama3"
//! provider: "ollama"
//! supported_endpoints: ["chat", "completion", "embedding"]
//! config:
//!   base_url: "http://localhost:11434"
//! limit:
//!   renewal_period: "minutes"
//!   calls: 60
//! ```

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use url::Url;

use crate::config::{ModelConfig, EndpointType};
use crate::provider::{
    LlmProvider, LlmError, ChatResponse, CompletionResponse, EmbeddingResponse, 
    Message, RateLimitState, Usage, Timing
};

/// Ollama chat message for API requests
#[derive(Debug, Clone, Serialize, Deserialize)]
struct OllamaChatMessage {
    role: String,
    content: String,
}

/// Ollama chat request
#[derive(Debug, Serialize)]
struct OllamaChatRequest {
    model: String,
    messages: Vec<OllamaChatMessage>,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<OllamaOptions>,
}

/// Ollama completion request  
#[derive(Debug, Serialize)]
struct OllamaCompletionRequest {
    model: String,
    prompt: String,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<OllamaOptions>,
}

/// Ollama embedding request
#[derive(Debug, Serialize)]
struct OllamaEmbeddingRequest {
    model: String,
    prompt: String,
}

/// Ollama model options
#[derive(Debug, Serialize)]
struct OllamaOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_predict: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop: Option<Vec<String>>,
}

/// Ollama chat response
#[derive(Debug, Deserialize)]
struct OllamaChatResponse {
    message: OllamaChatMessage,
    done: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    load_duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prompt_eval_duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eval_duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prompt_eval_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eval_count: Option<u32>,
}

/// Ollama completion response
#[derive(Debug, Deserialize)]
struct OllamaCompletionResponse {
    response: String,
    done: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    load_duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prompt_eval_duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eval_duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prompt_eval_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eval_count: Option<u32>,
}

/// Ollama embedding response
#[derive(Debug, Deserialize)]
struct OllamaEmbeddingResponse {
    embedding: Vec<f32>,
}

/// Ollama provider implementation
pub struct OllamaProvider {
    config: ModelConfig,
    rate_limit_state: RateLimitState,
    client: reqwest::Client,
    base_url: String,
}

impl OllamaProvider {
    /// Create a new Ollama provider
    pub fn new(config: ModelConfig) -> Result<Self, LlmError> {
        // Get base_url from config
        let base_url = config
            .config
            .as_ref()
            .and_then(|c| c.get("base_url"))
            .ok_or_else(|| LlmError::ConfigError("Missing base_url in config".to_string()))?
            .clone();

        // Validate URL
        let _url = Url::parse(&base_url)
            .map_err(|e| LlmError::ConfigError(format!("Invalid base_url: {}", e)))?;

        let rate_limit_state = config.limit.as_ref().map_or_else(
            || RateLimitState::default(),
            |limit| RateLimitState::new(limit)
        );

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(300))
            .build()
            .map_err(|e| LlmError::ProviderError(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self {
            config,
            rate_limit_state,
            client,
            base_url,
        })
    }

    /// Convert timing information from nanoseconds to milliseconds
    fn convert_timing(
        total_duration: Option<u64>,
        load_duration: Option<u64>,
        prompt_eval_duration: Option<u64>,
        eval_duration: Option<u64>,
    ) -> Option<Timing> {
        Some(Timing {
            total_duration_ms: total_duration.unwrap_or(0) / 1_000_000,
            load_duration_ms: load_duration.unwrap_or(0) / 1_000_000,
            prompt_eval_duration_ms: prompt_eval_duration.unwrap_or(0) / 1_000_000,
            completion_duration_ms: eval_duration.unwrap_or(0) / 1_000_000,
        })
    }
}

#[async_trait]
impl LlmProvider for OllamaProvider {
    fn config(&self) -> &ModelConfig {
        &self.config
    }

    fn rate_limit_state(&self) -> &RateLimitState {
        &self.rate_limit_state
    }

    async fn chat(
        &self,
        messages: Vec<Message>,
        _n: Option<u32>,
        temperature: Option<f32>,
        max_tokens: Option<u32>,
        stop: Option<Vec<String>>,
        _extra_config: Option<HashMap<String, String>>,
    ) -> Result<ChatResponse, LlmError> {
        if !self.supports_endpoint(EndpointType::Chat) {
            return Err(LlmError::UnsupportedEndpoint("chat".to_string()));
        }

        self.check_rate_limit().await?;

        let ollama_messages: Vec<OllamaChatMessage> = messages
            .into_iter()
            .map(|msg| OllamaChatMessage {
                role: msg.role,
                content: msg.content,
            })
            .collect();

        let options = if temperature.is_some() || max_tokens.is_some() || stop.is_some() {
            Some(OllamaOptions {
                temperature,
                num_predict: max_tokens.map(|t| t as i32),
                stop,
            })
        } else {
            None
        };

        let request = OllamaChatRequest {
            model: self.config.model.clone(),
            messages: ollama_messages,
            stream: false,
            options,
        };

        let response = self
            .client
            .post(&format!("{}/api/chat", self.base_url))
            .json(&request)
            .send()
            .await
            .map_err(|e| LlmError::ProviderError(format!("HTTP request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(LlmError::ProviderError(format!(
                "Ollama API error: {}",
                response.status()
            )));
        }

        let ollama_response: OllamaChatResponse = response
            .json()
            .await
            .map_err(|e| LlmError::ProviderError(format!("JSON parsing failed: {}", e)))?;

        let usage = Some(Usage {
            prompt_tokens: ollama_response.prompt_eval_count.unwrap_or(0),
            completion_tokens: ollama_response.eval_count.unwrap_or(0),
            total_tokens: ollama_response.prompt_eval_count.unwrap_or(0) 
                + ollama_response.eval_count.unwrap_or(0),
        });

        let timing = Self::convert_timing(
            ollama_response.total_duration,
            ollama_response.load_duration,
            ollama_response.prompt_eval_duration,
            ollama_response.eval_duration,
        );

        Ok(ChatResponse {
            content: ollama_response.message.content,
            usage,
            timing,
            model: self.config.model.clone(),
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

        let options = if temperature.is_some() || max_tokens.is_some() || stop.is_some() {
            Some(OllamaOptions {
                temperature,
                num_predict: max_tokens.map(|t| t as i32),
                stop,
            })
        } else {
            None
        };

        let request = OllamaCompletionRequest {
            model: self.config.model.clone(),
            prompt,
            stream: false,
            options,
        };

        let response = self
            .client
            .post(&format!("{}/api/generate", self.base_url))
            .json(&request)
            .send()
            .await
            .map_err(|e| LlmError::ProviderError(format!("HTTP request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(LlmError::ProviderError(format!(
                "Ollama API error: {}",
                response.status()
            )));
        }

        let ollama_response: OllamaCompletionResponse = response
            .json()
            .await
            .map_err(|e| LlmError::ProviderError(format!("JSON parsing failed: {}", e)))?;

        let usage = Some(Usage {
            prompt_tokens: ollama_response.prompt_eval_count.unwrap_or(0),
            completion_tokens: ollama_response.eval_count.unwrap_or(0),
            total_tokens: ollama_response.prompt_eval_count.unwrap_or(0) 
                + ollama_response.eval_count.unwrap_or(0),
        });

        let timing = Self::convert_timing(
            ollama_response.total_duration,
            ollama_response.load_duration,
            ollama_response.prompt_eval_duration,
            ollama_response.eval_duration,
        );

        Ok(CompletionResponse {
            text: ollama_response.response,
            usage,
            timing,
            model: self.config.model.clone(),
        })
    }

    async fn embed(
        &self,
        input: Vec<String>,
        _extra_config: Option<HashMap<String, String>>,
    ) -> Result<EmbeddingResponse, LlmError> {
        if !self.supports_endpoint(EndpointType::Embedding) {
            return Err(LlmError::UnsupportedEndpoint("embedding".to_string()));
        }

        self.check_rate_limit().await?;

        // For multiple inputs, we'll concatenate them with spaces
        // This is a simplification - in production you might want to handle this differently
        let text = input.join(" ");

        let request = OllamaEmbeddingRequest {
            model: self.config.model.clone(),
            prompt: text.clone(),
        };

        let response = self
            .client
            .post(&format!("{}/api/embeddings", self.base_url))
            .json(&request)
            .send()
            .await
            .map_err(|e| LlmError::ProviderError(format!("HTTP request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(LlmError::ProviderError(format!(
                "Ollama API error: {}",
                response.status()
            )));
        }

        let ollama_response: OllamaEmbeddingResponse = response
            .json()
            .await
            .map_err(|e| LlmError::ProviderError(format!("JSON parsing failed: {}", e)))?;

        // Estimate token usage based on text length (rough approximation)
        let estimated_tokens = (text.len() as f32 / 4.0).ceil() as u32;

        let usage = Some(Usage {
            prompt_tokens: estimated_tokens,
            completion_tokens: 0,
            total_tokens: estimated_tokens,
        });

        Ok(EmbeddingResponse {
            embedding: ollama_response.embedding,
            usage,
            timing: None, // Ollama embeddings don't return timing info
            model: self.config.model.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{ModelConfig, EndpointType};
    use std::collections::HashMap;

    fn create_test_config() -> ModelConfig {
        let mut config_map = HashMap::new();
        config_map.insert("base_url".to_string(), "http://localhost:11434".to_string());

        ModelConfig {
            name: "test-ollama".to_string(),
            supported_endpoints: vec![
                EndpointType::Chat,
                EndpointType::Completion,
                EndpointType::Embedding,
            ],
            provider: "ollama".to_string(),
            model: "llama3".to_string(),
            config: Some(config_map),
            limit: None,
        }
    }

    #[test]
    fn test_ollama_provider_creation() {
        let config = create_test_config();
        let provider = OllamaProvider::new(config);
        assert!(provider.is_ok());
    }

    #[test]
    fn test_ollama_provider_invalid_url() {
        let mut config = create_test_config();
        config.config.as_mut().unwrap().insert(
            "base_url".to_string(),
            "invalid-url".to_string(),
        );

        let provider = OllamaProvider::new(config);
        assert!(provider.is_err());
    }

    #[test]
    fn test_ollama_provider_missing_base_url() {
        let mut config = create_test_config();
        config.config = None;

        let provider = OllamaProvider::new(config);
        assert!(provider.is_err());
    }

    #[test]
    fn test_supported_endpoints() {
        let config = create_test_config();
        let provider = OllamaProvider::new(config).unwrap();

        assert!(provider.supports_endpoint(EndpointType::Chat));
        assert!(provider.supports_endpoint(EndpointType::Completion));
        assert!(provider.supports_endpoint(EndpointType::Embedding));
    }
}