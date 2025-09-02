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
use tracing::{debug, error};
use url::Url;

use crate::config::{ModelConfig, EndpointType};
use crate::traits::{
    LlmProvider, LlmError, ChatResponse, CompletionResponse, EmbeddingResponse, 
    Message, RateLimitState, Usage, Timing, ModelInfo, ModelPullProgress,
    Tool, ToolCall
};

/// Ollama chat message for API requests
#[derive(Debug, Clone, Serialize, Deserialize)]
struct OllamaChatMessage {
    role: String,
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<ToolCall>>,
}

/// Ollama chat request
#[derive(Debug, Serialize)]
struct OllamaChatRequest {
    model: String,
    messages: Vec<OllamaChatMessage>,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<OllamaOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<Tool>>,
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
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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

/// Response from Ollama's /api/tags endpoint
#[derive(Debug, Deserialize)]
struct OllamaTagsResponse {
    models: Vec<OllamaModel>,
}

/// Represents a model available in Ollama
#[derive(Debug, Clone, Deserialize)]
struct OllamaModel {
    name: String,
    #[allow(dead_code)]
    digest: String,
    #[allow(dead_code)]
    size: u64,
    #[allow(dead_code)]
    modified_at: String,
}

/// Request to pull a model
#[derive(Debug, Serialize)]
struct OllamaPullRequest {
    name: String,
    stream: bool,
}

/// Response from model pull (streaming)
#[derive(Debug, Deserialize)]
struct OllamaPullStreamResponse {
    status: String,
    #[serde(default)]
    #[allow(dead_code)]
    digest: String,
    #[serde(default)]
    total: u64,
    #[serde(default)]
    completed: u64,
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
        tools: Option<Vec<Tool>>,
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
                tool_calls: None,
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
            tools,
        };

        debug!("Ollama API request: model={} messages={} tools={}", 
            request.model, 
            request.messages.len(), 
            request.tools.as_ref().map_or(0, |t| t.len())
        );

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

        // Get the response text to log its size before parsing
        let response_text = response
            .text()
            .await
            .map_err(|e| LlmError::ProviderError(format!("Failed to read response text: {}", e)))?;
        
        debug!("Raw response size: {} bytes", response_text.len());

        let ollama_response: OllamaChatResponse = serde_json::from_str(&response_text)
            .map_err(|e| {
                error!("JSON parsing failed for response of {} bytes: {}", response_text.len(), e);
                if response_text.len() > 1000 {
                    error!("Response preview (first 500 chars): {}", &response_text[..500]);
                    error!("Response preview (last 500 chars): {}", &response_text[response_text.len()-500..]);
                } else {
                    error!("Full response: {}", response_text);
                }
                LlmError::ProviderError(format!("JSON parsing failed: {}", e))
            })?;

        // Calculate thinking block size
        let thinking_block_size = if ollama_response.message.content.contains("<thinking>") {
            let thinking_start = ollama_response.message.content.find("<thinking>").unwrap_or(0);
            let thinking_end = ollama_response.message.content.rfind("</thinking>").unwrap_or(ollama_response.message.content.len());
            if thinking_end > thinking_start {
                thinking_end - thinking_start + "</thinking>".len()
            } else {
                0
            }
        } else {
            0
        };

        debug!("Ollama API response: content_length={} thinking_block_size={} tool_calls={}", 
            ollama_response.message.content.len(),
            thinking_block_size,
            ollama_response.message.tool_calls.is_some()
        );
        
        if let Some(ref tool_calls) = ollama_response.message.tool_calls {
            debug!("  Tool calls returned: {}", tool_calls.len());
            for tool_call in tool_calls {
                debug!("    Tool: {}", tool_call.function.name);
            }
        } else {
            debug!("  No tool calls in response");
        }

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
            tool_calls: ollama_response.message.tool_calls,
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
    
    // Model Management implementations
    
    /// Check if the Ollama service is available and responding
    /// 
    /// Returns `Ok(true)` if the service is running and accessible,
    /// `Ok(false)` if the service is not responding.
    /// 
    /// # Example
    /// ```no_run
    /// # use mimir_dm_llm::{providers::ollama::OllamaProvider, LlmProvider};
    /// # async fn example(provider: OllamaProvider) -> Result<(), Box<dyn std::error::Error>> {
    /// if provider.check_service().await? {
    ///     println!("Ollama is running");
    /// } else {
    ///     println!("Ollama is not available");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    async fn check_service(&self) -> Result<bool, LlmError> {
        let url = format!("{}/api/tags", self.base_url);
        
        match self.client.get(&url).send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false), // Service not running
        }
    }
    
    /// List all models available in the Ollama service
    /// 
    /// Returns a vector of `ModelInfo` containing the names of all locally available models.
    /// 
    /// # Errors
    /// Returns an error if the service is not available or if the response cannot be parsed.
    /// 
    /// # Example
    /// ```no_run
    /// # use mimir_dm_llm::{providers::ollama::OllamaProvider, LlmProvider};
    /// # async fn example(provider: OllamaProvider) -> Result<(), Box<dyn std::error::Error>> {
    /// let models = provider.list_models().await?;
    /// for model in models {
    ///     println!("Available model: {}", model.name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    async fn list_models(&self) -> Result<Vec<ModelInfo>, LlmError> {
        let url = format!("{}/api/tags", self.base_url);
        
        let response = self.client.get(&url)
            .send()
            .await
            .map_err(|e| LlmError::ProviderError(format!(
                "Failed to list models from {}: {}", self.base_url, e
            )))?;
            
        if !response.status().is_success() {
            return Err(LlmError::ServiceUnavailable(format!(
                "Ollama service at {} returned status: {}", 
                self.base_url, response.status()
            )));
        }
        
        let tags_response: OllamaTagsResponse = response.json().await
            .map_err(|e| LlmError::ProviderError(format!("Failed to parse model list: {}", e)))?;
            
        Ok(tags_response.models.into_iter()
            .map(|m| ModelInfo { name: m.name })
            .collect())
    }
    
    /// Check if a specific model exists locally in Ollama
    /// 
    /// This method checks if a model with the given name (or starting with the given name)
    /// exists in the local Ollama installation. It handles partial matches, so "llama3.1"
    /// will match "llama3.1:latest" or "llama3.1-instruct".
    /// 
    /// # Arguments
    /// * `model_name` - The name of the model to check for
    /// 
    /// # Example
    /// ```no_run
    /// # use mimir_dm_llm::{providers::ollama::OllamaProvider, LlmProvider};
    /// # async fn example(provider: OllamaProvider) -> Result<(), Box<dyn std::error::Error>> {
    /// if provider.model_exists("llama3.1").await? {
    ///     println!("Model is available locally");
    /// } else {
    ///     println!("Model needs to be pulled");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    async fn model_exists(&self, model_name: &str) -> Result<bool, LlmError> {
        let models = self.list_models().await?;
        // Check if any model name starts with the requested model
        // This handles cases like "qwen2:8b" matching "qwen2:8b-instruct-q4_0"
        Ok(models.iter().any(|m| m.name.starts_with(model_name)))
    }
    
    /// Pull (download) a model from the Ollama library
    /// 
    /// This method downloads a model from the Ollama library if it's not already available locally.
    /// The download happens synchronously (blocking until complete).
    /// 
    /// # Arguments
    /// * `model_name` - The name of the model to pull (e.g., "llama3.1", "mistral:latest")
    /// 
    /// # Errors
    /// Returns an error if the model cannot be found or if the download fails.
    /// 
    /// # Example
    /// ```no_run
    /// # use mimir_dm_llm::{providers::ollama::OllamaProvider, LlmProvider};
    /// # async fn example(provider: OllamaProvider) -> Result<(), Box<dyn std::error::Error>> {
    /// provider.pull_model("tinyllama").await?;
    /// println!("Model downloaded successfully");
    /// # Ok(())
    /// # }
    /// ```
    async fn pull_model(&self, model_name: &str) -> Result<(), LlmError> {
        let url = format!("{}/api/pull", self.base_url);
        
        let request = OllamaPullRequest {
            name: model_name.to_string(),
            stream: false,
        };
        
        let response = self.client.post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| LlmError::ProviderError(format!(
                "Failed to pull model '{}' from {}: {}", model_name, self.base_url, e
            )))?;
            
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(LlmError::ModelPullFailed(
                format!("Failed to pull model '{}': HTTP {} - {}", 
                    model_name, status, error_text)
            ));
        }
        
        // Wait for the response to complete (non-streaming)
        let _body = response.text().await
            .map_err(|e| LlmError::ProviderError(format!("Failed to read pull response: {}", e)))?;
            
        Ok(())
    }
    
    /// Pull (download) a model with progress updates
    /// 
    /// This method downloads a model from the Ollama library and provides progress updates
    /// through a callback function. This is useful for showing download progress to users.
    /// 
    /// # Arguments
    /// * `model_name` - The name of the model to pull
    /// * `progress_callback` - A callback function that receives progress updates
    /// 
    /// # Example
    /// ```no_run
    /// # use mimir_dm_llm::{providers::ollama::OllamaProvider, LlmProvider, ModelPullProgress};
    /// # async fn example(provider: OllamaProvider) -> Result<(), Box<dyn std::error::Error>> {
    /// provider.pull_model_with_progress("llama3.1", |progress: ModelPullProgress| {
    ///     println!("{}: {}/{} bytes", progress.status, progress.downloaded, progress.total);
    /// }).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn pull_model_with_progress<F>(
        &self,
        model_name: &str,
        progress_callback: F,
    ) -> Result<(), LlmError>
    where
        F: Fn(ModelPullProgress) + Send + 'static,
    {
        let url = format!("{}/api/pull", self.base_url);
        
        let request = OllamaPullRequest {
            name: model_name.to_string(),
            stream: true,
        };
        
        let response = self.client.post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| LlmError::ProviderError(format!(
                "Failed to initiate pull for model '{}' from {}: {}", 
                model_name, self.base_url, e
            )))?;
            
        if !response.status().is_success() {
            let status = response.status();
            return Err(LlmError::ModelPullFailed(
                format!("Failed to pull model '{}': HTTP {} from {}", 
                    model_name, status, self.base_url)
            ));
        }
        
        // Process streaming response
        use futures::StreamExt;
        let mut stream = response.bytes_stream();
        
        while let Some(chunk) = stream.next().await {
            let chunk = chunk
                .map_err(|e| LlmError::ProviderError(format!("Stream error: {}", e)))?;
            
            // Parse each line as JSON (Ollama sends newline-delimited JSON)
            for line in chunk.split(|&b| b == b'\n') {
                if line.is_empty() {
                    continue;
                }
                
                match serde_json::from_slice::<OllamaPullStreamResponse>(line) {
                    Ok(progress_data) => {
                        let progress = ModelPullProgress {
                            status: progress_data.status.clone(),
                            downloaded: progress_data.completed,
                            total: progress_data.total,
                        };
                        
                        progress_callback(progress);
                        
                        // Check if done
                        if progress_data.status.contains("success") || 
                           progress_data.status.contains("already exists") {
                            return Ok(());
                        }
                    }
                    Err(_) => {
                        // Ignore parse errors for individual lines
                        // Ollama sends various status messages we don't need to parse
                    }
                }
            }
        }
        
        Ok(())
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