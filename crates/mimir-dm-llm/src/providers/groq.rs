//! # Groq Provider
//!
//! This module provides an implementation of the [`LlmProvider`] trait for the Groq API.
//! Groq provides ultra-fast LLM inference using their custom LPU (Language Processing Unit) architecture.
//!
//! ## Configuration
//!
//! The Groq provider requires an API key in the configuration. The upstream application
//! is responsible for sourcing this key (from environment variables, secure storage, etc.).
//!
//! Example configuration:
//!
//! ```rust
//! use std::collections::HashMap;
//! use mimir_dm_llm::config::{ModelConfig, EndpointType};
//!
//! let mut config_map = HashMap::new();
//! config_map.insert("api_key".to_string(), "gsk_...".to_string());
//! // Optional: custom base URL
//! // config_map.insert("base_url".to_string(), "https://api.groq.com/openai/v1".to_string());
//!
//! let config = ModelConfig {
//!     name: "groq-llama3".to_string(),
//!     model: "llama-3.3-70b-versatile".to_string(),
//!     provider: "groq".to_string(),
//!     supported_endpoints: vec![EndpointType::Chat, EndpointType::Completion],
//!     config: Some(config_map),
//!     limit: None,
//! };
//! ```
//!
//! ## Supported Models
//!
//! - llama-3.3-70b-versatile (recommended)
//! - llama-3.1-8b-instant
//! - llama-3.2-90b-vision-preview
//! - mixtral-8x7b-32768
//! - gemma2-9b-it
//!
//! See https://console.groq.com/docs/models for the current list.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error};

use crate::config::{EndpointType, ModelConfig};
use crate::traits::{
    ChatResponse, CompletionResponse, EmbeddingResponse, LlmError, LlmProvider, Message,
    RateLimitState, Tool, ToolCall, Usage,
};

/// Groq chat request following OpenAI API format
#[derive(Debug, Serialize)]
struct GroqChatRequest {
    model: String,
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<Tool>>,
    stream: bool,
}

/// Groq chat response message
#[derive(Debug, Deserialize)]
struct GroqChatMessage {
    #[allow(dead_code)]
    role: String,
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<ToolCall>>,
}

/// Groq chat response choice
#[derive(Debug, Deserialize)]
struct GroqChatChoice {
    #[allow(dead_code)]
    index: u32,
    message: GroqChatMessage,
    #[allow(dead_code)]
    finish_reason: String,
}

/// Groq token usage
#[derive(Debug, Deserialize)]
struct GroqUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

/// Groq chat response
#[derive(Debug, Deserialize)]
struct GroqChatResponse {
    #[allow(dead_code)]
    id: String,
    #[allow(dead_code)]
    object: String,
    #[allow(dead_code)]
    created: u64,
    model: String,
    choices: Vec<GroqChatChoice>,
    usage: GroqUsage,
}

/// Groq error response
#[derive(Debug, Deserialize)]
struct GroqError {
    error: GroqErrorDetail,
}

#[derive(Debug, Deserialize)]
struct GroqErrorDetail {
    message: String,
    #[serde(rename = "type")]
    error_type: String,
}

/// Groq provider implementation
pub struct GroqProvider {
    config: ModelConfig,
    rate_limit_state: RateLimitState,
    client: reqwest::Client,
    base_url: String,
    api_key: String,
}

impl GroqProvider {
    /// Create a new Groq provider
    pub fn new(config: ModelConfig) -> Result<Self, LlmError> {
        // Get API key from config (required)
        let api_key = config
            .config
            .as_ref()
            .and_then(|c| c.get("api_key"))
            .ok_or_else(|| {
                LlmError::ConfigError(
                    "Missing api_key in config - upstream application must provide API key"
                        .to_string(),
                )
            })?
            .to_string();

        // Get base_url from config or use default
        let base_url = config
            .config
            .as_ref()
            .and_then(|c| c.get("base_url"))
            .map(|s| s.to_string())
            .unwrap_or_else(|| "https://api.groq.com/openai/v1".to_string())
            .trim_end_matches('/')
            .to_string();

        let rate_limit_state = config.limit.as_ref().map_or_else(
            || RateLimitState::default(),
            |limit| RateLimitState::new(limit),
        );

        // Create HTTP client with timeout
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(300)) // 5 minute timeout
            .build()
            .map_err(|e| LlmError::ProviderError(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self {
            config,
            rate_limit_state,
            client,
            base_url,
            api_key,
        })
    }

    /// Make a request to the Groq API
    async fn make_request<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
        body: impl Serialize,
    ) -> Result<T, LlmError> {
        let url = format!("{}/{}", self.base_url, endpoint);

        debug!("Making Groq API request to {}", url);

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| LlmError::ProviderError(format!("Request failed: {}", e)))?;

        let status = response.status();

        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

            // Try to parse as Groq error
            if let Ok(groq_error) = serde_json::from_str::<GroqError>(&error_text) {
                error!("Groq API error: {}", groq_error.error.message);
                return Err(LlmError::ProviderError(format!(
                    "Groq API error ({}): {}",
                    groq_error.error.error_type, groq_error.error.message
                )));
            }

            error!("Groq API error (status {}): {}", status, error_text);
            return Err(LlmError::ProviderError(format!(
                "Groq API error (status {}): {}",
                status, error_text
            )));
        }

        let response_text = response
            .text()
            .await
            .map_err(|e| LlmError::ProviderError(format!("Failed to read response: {}", e)))?;

        serde_json::from_str(&response_text).map_err(|e| {
            error!("Failed to parse Groq response: {}", e);
            error!("Response text: {}", response_text);
            LlmError::ProviderError(format!("Failed to parse response: {}", e))
        })
    }
}

#[async_trait]
impl LlmProvider for GroqProvider {
    fn config(&self) -> &ModelConfig {
        &self.config
    }

    fn rate_limit_state(&self) -> &RateLimitState {
        &self.rate_limit_state
    }

    async fn chat(
        &self,
        messages: Vec<Message>,
        tools: Option<Vec<Tool>>,
        _n: Option<u32>,
        temperature: Option<f32>,
        max_tokens: Option<u32>,
        stop: Option<Vec<String>>,
        _extra_config: Option<HashMap<String, String>>,
        _cancellation_token: Option<CancellationToken>,
    ) -> Result<ChatResponse, LlmError> {
        if !self.supports_endpoint(EndpointType::Chat) {
            return Err(LlmError::UnsupportedEndpoint("chat".to_string()));
        }

        self.check_rate_limit().await?;

        let request = GroqChatRequest {
            model: self.config.model.clone(),
            messages,
            temperature,
            max_tokens,
            stop,
            tools,
            stream: false,
        };

        debug!(
            "Sending chat request to Groq with model: {}",
            request.model
        );

        let response: GroqChatResponse = self.make_request("chat/completions", request).await?;

        // Extract the first choice
        let choice = response
            .choices
            .first()
            .ok_or_else(|| LlmError::ProviderError("No choices in response".to_string()))?;

        Ok(ChatResponse {
            content: choice.message.content.clone(),
            usage: Some(Usage {
                prompt_tokens: response.usage.prompt_tokens,
                completion_tokens: response.usage.completion_tokens,
                total_tokens: response.usage.total_tokens,
            }),
            timing: None, // Groq doesn't provide detailed timing in standard response
            model: response.model,
            tool_calls: choice.message.tool_calls.clone(),
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

        // Convert completion to chat format (OpenAI-style providers use chat for everything)
        let messages = vec![Message {
            role: "user".to_string(),
            content: prompt,
        }];

        let request = GroqChatRequest {
            model: self.config.model.clone(),
            messages,
            temperature,
            max_tokens,
            stop,
            tools: None,
            stream: false,
        };

        debug!(
            "Sending completion request (via chat) to Groq with model: {}",
            request.model
        );

        let response: GroqChatResponse = self.make_request("chat/completions", request).await?;

        let choice = response
            .choices
            .first()
            .ok_or_else(|| LlmError::ProviderError("No choices in response".to_string()))?;

        Ok(CompletionResponse {
            text: choice.message.content.clone(),
            usage: Some(Usage {
                prompt_tokens: response.usage.prompt_tokens,
                completion_tokens: response.usage.completion_tokens,
                total_tokens: response.usage.total_tokens,
            }),
            timing: None,
            model: response.model,
        })
    }

    async fn embed(
        &self,
        _input: Vec<String>,
        _extra_config: Option<HashMap<String, String>>,
    ) -> Result<EmbeddingResponse, LlmError> {
        // Groq doesn't currently support embeddings via their API
        Err(LlmError::NotSupported)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_creation_with_api_key() {
        let mut config_map = HashMap::new();
        config_map.insert("api_key".to_string(), "test-api-key".to_string());

        let config = ModelConfig {
            name: "test-groq".to_string(),
            model: "llama-3.3-70b-versatile".to_string(),
            provider: "groq".to_string(),
            supported_endpoints: vec![EndpointType::Chat],
            config: Some(config_map),
            limit: None,
        };

        let provider = GroqProvider::new(config);
        assert!(provider.is_ok());

        let provider = provider.unwrap();
        assert_eq!(provider.api_key, "test-api-key");
        assert_eq!(provider.base_url, "https://api.groq.com/openai/v1");
    }

    #[test]
    fn test_provider_creation_fails_without_api_key() {
        let config = ModelConfig {
            name: "test-groq".to_string(),
            model: "llama-3.3-70b-versatile".to_string(),
            provider: "groq".to_string(),
            supported_endpoints: vec![EndpointType::Chat],
            config: Some(HashMap::new()),
            limit: None,
        };

        let provider = GroqProvider::new(config);
        assert!(provider.is_err());

        if let Err(LlmError::ConfigError(msg)) = provider {
            assert!(msg.contains("Missing api_key"));
        } else {
            panic!("Expected ConfigError");
        }
    }

    #[test]
    fn test_custom_base_url() {
        let mut config_map = HashMap::new();
        config_map.insert("api_key".to_string(), "test-key".to_string());
        config_map.insert(
            "base_url".to_string(),
            "https://custom.groq.com/v1".to_string(),
        );

        let config = ModelConfig {
            name: "test-groq".to_string(),
            model: "llama-3.3-70b-versatile".to_string(),
            provider: "groq".to_string(),
            supported_endpoints: vec![EndpointType::Chat],
            config: Some(config_map),
            limit: None,
        };

        let provider = GroqProvider::new(config).unwrap();
        assert_eq!(provider.base_url, "https://custom.groq.com/v1");
    }

    #[test]
    fn test_base_url_trailing_slash_removed() {
        let mut config_map = HashMap::new();
        config_map.insert("api_key".to_string(), "test-key".to_string());
        config_map.insert(
            "base_url".to_string(),
            "https://api.groq.com/openai/v1/".to_string(),
        );

        let config = ModelConfig {
            name: "test-groq".to_string(),
            model: "llama-3.3-70b-versatile".to_string(),
            provider: "groq".to_string(),
            supported_endpoints: vec![EndpointType::Chat],
            config: Some(config_map),
            limit: None,
        };

        let provider = GroqProvider::new(config).unwrap();
        assert_eq!(provider.base_url, "https://api.groq.com/openai/v1");
    }
}
