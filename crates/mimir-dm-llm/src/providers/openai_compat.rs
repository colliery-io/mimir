//! OpenAI-Compatible Client
//!
//! Provides a shared HTTP client for OpenAI-compatible API endpoints.
//! This client works with any provider that implements the OpenAI chat completions API:
//! - Ollama (via /v1/chat/completions)
//! - Groq (via /openai/v1/chat/completions)
//! - OpenAI (via /v1/chat/completions)
//! - vLLM, LM Studio, and other compatible providers

use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error};

use crate::traits::{
    ChatResponse, CompletionResponse, LlmError, Message, Tool, ToolCall, Usage,
};

/// OpenAI-compatible chat request
#[derive(Debug, Serialize)]
pub struct OpenAiChatRequest {
    /// Model identifier
    pub model: String,
    /// Conversation messages
    pub messages: Vec<OpenAiMessage>,
    /// Sampling temperature (0.0 to 2.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// Maximum tokens to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    /// Stop sequences
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
    /// Tools available to the model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    /// Whether to stream the response
    #[serde(default)]
    pub stream: bool,
}

/// OpenAI-compatible message format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAiMessage {
    /// Message role (system, user, assistant, tool)
    pub role: String,
    /// Message content (may be null when tool_calls present)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Tool call ID (required for tool role messages)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
    /// Tool calls made by the assistant
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
}

impl From<Message> for OpenAiMessage {
    fn from(msg: Message) -> Self {
        Self {
            role: msg.role,
            content: Some(msg.content),
            tool_call_id: msg.tool_call_id,
            tool_calls: None,
        }
    }
}

/// OpenAI-compatible chat response
#[derive(Debug, Deserialize)]
pub struct OpenAiChatResponse {
    /// Response ID
    #[allow(dead_code)]
    pub id: String,
    /// Object type
    #[allow(dead_code)]
    pub object: String,
    /// Creation timestamp
    #[allow(dead_code)]
    pub created: u64,
    /// Model used
    pub model: String,
    /// Response choices
    pub choices: Vec<OpenAiChoice>,
    /// Token usage
    pub usage: OpenAiUsage,
}

/// Response choice
#[derive(Debug, Deserialize)]
pub struct OpenAiChoice {
    /// Choice index
    #[allow(dead_code)]
    pub index: u32,
    /// Generated message
    pub message: OpenAiMessage,
    /// Reason for stopping
    #[allow(dead_code)]
    pub finish_reason: Option<String>,
}

/// Token usage information
#[derive(Debug, Deserialize)]
pub struct OpenAiUsage {
    /// Tokens in prompt
    pub prompt_tokens: u32,
    /// Tokens in completion
    pub completion_tokens: u32,
    /// Total tokens
    pub total_tokens: u32,
}

/// OpenAI-compatible error response
#[derive(Debug, Deserialize)]
pub struct OpenAiErrorResponse {
    /// Error details
    pub error: OpenAiErrorDetail,
}

/// Error detail
#[derive(Debug, Deserialize)]
pub struct OpenAiErrorDetail {
    /// Error message
    pub message: String,
    /// Error type
    #[serde(rename = "type")]
    pub error_type: Option<String>,
    /// Error code
    pub code: Option<String>,
}

/// Client for OpenAI-compatible APIs
pub struct OpenAiCompatClient {
    client: reqwest::Client,
    base_url: String,
    api_key: Option<String>,
}

impl OpenAiCompatClient {
    /// Create a new OpenAI-compatible client
    ///
    /// # Arguments
    /// * `base_url` - Base URL for the API (e.g., "http://localhost:11434/v1")
    /// * `api_key` - Optional API key for authentication
    /// * `timeout_secs` - Request timeout in seconds
    pub fn new(base_url: String, api_key: Option<String>, timeout_secs: u64) -> Result<Self, LlmError> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .build()
            .map_err(|e| LlmError::ProviderError(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self {
            client,
            base_url: base_url.trim_end_matches('/').to_string(),
            api_key,
        })
    }

    /// Send a chat completion request
    pub async fn chat(
        &self,
        request: OpenAiChatRequest,
        cancellation_token: Option<CancellationToken>,
    ) -> Result<ChatResponse, LlmError> {
        let url = format!("{}/chat/completions", self.base_url);

        debug!("OpenAI-compat request to {}: model={} messages={}",
            url, request.model, request.messages.len());

        // Build the request
        let mut req_builder = self.client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request);

        // Add authorization if API key is present
        if let Some(ref key) = self.api_key {
            req_builder = req_builder.header("Authorization", format!("Bearer {}", key));
        }

        // Execute with optional cancellation
        let response = if let Some(ref token) = cancellation_token {
            tokio::select! {
                result = req_builder.send() => {
                    result.map_err(|e| LlmError::ProviderError(format!("Request failed: {}", e)))?
                }
                _ = token.cancelled() => {
                    debug!("Chat request cancelled");
                    return Err(LlmError::Cancelled);
                }
            }
        } else {
            req_builder.send().await
                .map_err(|e| LlmError::ProviderError(format!("Request failed: {}", e)))?
        };

        let status = response.status();

        if !status.is_success() {
            let error_text = response.text().await
                .unwrap_or_else(|_| "Unknown error".to_string());

            // Try to parse as OpenAI error format
            if let Ok(error_response) = serde_json::from_str::<OpenAiErrorResponse>(&error_text) {
                error!("OpenAI-compat API error: {}", error_response.error.message);
                return Err(LlmError::ProviderError(format!(
                    "API error: {}", error_response.error.message
                )));
            }

            error!("OpenAI-compat API error (status {}): {}", status, error_text);
            return Err(LlmError::ProviderError(format!(
                "API error (status {}): {}", status, error_text
            )));
        }

        // Read response with optional cancellation
        let response_text = if let Some(ref token) = cancellation_token {
            tokio::select! {
                result = response.text() => {
                    result.map_err(|e| LlmError::ProviderError(format!("Failed to read response: {}", e)))?
                }
                _ = token.cancelled() => {
                    debug!("Response reading cancelled");
                    return Err(LlmError::Cancelled);
                }
            }
        } else {
            response.text().await
                .map_err(|e| LlmError::ProviderError(format!("Failed to read response: {}", e)))?
        };

        debug!("Response size: {} bytes", response_text.len());

        let api_response: OpenAiChatResponse = serde_json::from_str(&response_text)
            .map_err(|e| {
                error!("Failed to parse response: {}", e);
                if response_text.len() > 500 {
                    error!("Response preview: {}...", &response_text[..500]);
                } else {
                    error!("Full response: {}", response_text);
                }
                LlmError::ProviderError(format!("Failed to parse response: {}", e))
            })?;

        // Extract first choice
        let choice = api_response.choices.first()
            .ok_or_else(|| LlmError::ProviderError("No choices in response".to_string()))?;

        Ok(ChatResponse {
            content: choice.message.content.clone().unwrap_or_default(),
            usage: Some(Usage {
                prompt_tokens: api_response.usage.prompt_tokens,
                completion_tokens: api_response.usage.completion_tokens,
                total_tokens: api_response.usage.total_tokens,
            }),
            timing: None, // OpenAI format doesn't include detailed timing
            model: api_response.model,
            tool_calls: choice.message.tool_calls.clone(),
        })
    }

    /// Send a completion request (uses chat endpoint with single user message)
    pub async fn complete(
        &self,
        model: String,
        prompt: String,
        temperature: Option<f32>,
        max_tokens: Option<u32>,
        stop: Option<Vec<String>>,
    ) -> Result<CompletionResponse, LlmError> {
        // Convert to chat format
        let messages = vec![OpenAiMessage {
            role: "user".to_string(),
            content: Some(prompt),
            tool_call_id: None,
            tool_calls: None,
        }];

        let request = OpenAiChatRequest {
            model: model.clone(),
            messages,
            temperature,
            max_tokens,
            stop,
            tools: None,
            stream: false,
        };

        let chat_response = self.chat(request, None).await?;

        Ok(CompletionResponse {
            text: chat_response.content,
            usage: chat_response.usage,
            timing: chat_response.timing,
            model: chat_response.model,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_conversion() {
        let msg = Message {
            role: "user".to_string(),
            content: "Hello".to_string(),
            tool_call_id: None,
        };

        let openai_msg: OpenAiMessage = msg.into();
        assert_eq!(openai_msg.role, "user");
        assert_eq!(openai_msg.content, Some("Hello".to_string()));
    }

    #[test]
    fn test_parse_chat_response() {
        let json = r#"{
            "id": "chatcmpl-123",
            "object": "chat.completion",
            "created": 1234567890,
            "model": "llama3",
            "choices": [{
                "index": 0,
                "message": {
                    "role": "assistant",
                    "content": "Hello!"
                },
                "finish_reason": "stop"
            }],
            "usage": {
                "prompt_tokens": 10,
                "completion_tokens": 5,
                "total_tokens": 15
            }
        }"#;

        let response: OpenAiChatResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.choices[0].message.content, Some("Hello!".to_string()));
        assert_eq!(response.usage.total_tokens, 15);
    }

    #[test]
    fn test_parse_response_with_tool_calls() {
        let json = r#"{
            "id": "chatcmpl-456",
            "object": "chat.completion",
            "created": 1234567890,
            "model": "llama3",
            "choices": [{
                "index": 0,
                "message": {
                    "role": "assistant",
                    "tool_calls": [{
                        "id": "call_abc",
                        "function": {
                            "name": "get_weather",
                            "arguments": {"location": "NYC"}
                        }
                    }]
                },
                "finish_reason": "tool_calls"
            }],
            "usage": {
                "prompt_tokens": 20,
                "completion_tokens": 10,
                "total_tokens": 30
            }
        }"#;

        let response: OpenAiChatResponse = serde_json::from_str(json).unwrap();
        assert!(response.choices[0].message.content.is_none());
        assert!(response.choices[0].message.tool_calls.is_some());
    }
}
