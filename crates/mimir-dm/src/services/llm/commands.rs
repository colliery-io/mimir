//! Tauri command handlers for LLM service
//!
//! This module contains all Tauri commands that expose LLM functionality
//! to the frontend application.

use crate::services::llm::chat_processor::ChatProcessor;
use crate::services::llm::LlmService;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};
use uuid::Uuid;

use super::{CancellationTokens, ConfirmationReceivers, OLLAMA_BASE_URL};

/// Chat message structure for Tauri commands
#[derive(Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

/// Chat response with token usage
#[derive(Clone, Serialize, Deserialize)]
pub struct ChatResponseWithUsage {
    pub content: String,
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

/// Tauri command to check LLM status
#[tauri::command]
pub async fn check_llm_status(
    service: tauri::State<'_, Arc<Mutex<Option<LlmService>>>>,
) -> Result<bool, String> {
    let service = service.lock().await;

    if let Some(llm) = service.as_ref() {
        llm.check_service().await.map_err(|e| e.to_string())
    } else {
        Ok(false)
    }
}

/// Tauri command to get model info
#[tauri::command]
pub async fn get_llm_model_info(
    service: tauri::State<'_, Arc<Mutex<Option<LlmService>>>>,
) -> Result<String, String> {
    let service = service.lock().await;

    if let Some(llm) = service.as_ref() {
        Ok(llm.model_name().to_string())
    } else {
        Err("LLM service not initialized".to_string())
    }
}

/// Tauri command to send a chat message (with optional tool support)
#[tauri::command]
pub async fn send_chat_message(
    service: tauri::State<'_, Arc<Mutex<Option<LlmService>>>>,
    cancellation_tokens: tauri::State<'_, CancellationTokens>,
    messages: Vec<ChatMessage>,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
    enable_tools: Option<bool>,
    session_id: Option<String>,
    _model_name: Option<String>,
    ollama_url: Option<String>,
    campaign_directory_path: Option<String>,
) -> Result<ChatResponseWithUsage, String> {
    let service = service.lock().await;

    let llm = service
        .as_ref()
        .ok_or_else(|| "LLM service not initialized".to_string())?;

    // Require a valid session ID for all chat messages
    let session_id =
        session_id.ok_or_else(|| "Session ID is required for chat messages".to_string())?;

    // Create and register cancellation token for this session
    let cancellation_token = CancellationToken::new();

    {
        let mut tokens = cancellation_tokens.lock().await;
        tokens.insert(session_id.clone(), cancellation_token.clone());
    }

    // Convert to provider messages
    let provider_messages: Vec<mimir_dm_llm::Message> = messages
        .into_iter()
        .map(|msg| mimir_dm_llm::Message {
            role: msg.role,
            content: msg.content,
        })
        .collect();

    // Use the ChatProcessor to handle the complex message processing
    let processor = ChatProcessor::new(llm);
    let result = processor
        .process_chat(
            provider_messages,
            max_tokens,
            temperature,
            enable_tools.unwrap_or(false),
            &session_id,
            ollama_url.as_deref(),
            campaign_directory_path.as_deref(),
            cancellation_token,
        )
        .await;

    // Clean up cancellation token
    {
        let mut tokens = cancellation_tokens.lock().await;
        tokens.remove(&session_id);
    }

    // Convert result
    result.map(|response| ChatResponseWithUsage {
        content: response.content,
        prompt_tokens: response.prompt_tokens,
        completion_tokens: response.completion_tokens,
        total_tokens: response.total_tokens,
    })
}

/// Tauri command to get model context info
#[tauri::command]
pub async fn get_model_context_info(
    service: tauri::State<'_, Arc<Mutex<Option<LlmService>>>>,
) -> Result<serde_json::Value, String> {
    let service = service.lock().await;

    let llm = service
        .as_ref()
        .ok_or_else(|| "LLM service not initialized".to_string())?;

    // For now, return hardcoded info for gpt-oss:20b
    // In the future, we could query this from Ollama
    Ok(serde_json::json!({
        "model": llm.model_name(),
        "context_length": 262144,  // From our curl query
        "default_max_tokens": 16384, // Increased for thinking models (thinking section gets discarded)
        "architecture": "qwen3moe"
    }))
}

/// Tauri command to confirm or reject a tool action
#[tauri::command]
pub async fn confirm_tool_action(
    confirmation_receivers: tauri::State<'_, ConfirmationReceivers>,
    confirmation_id: String,
    confirmed: bool,
) -> Result<(), String> {
    info!(
        "Received confirmation request: ID={}, confirmed={}",
        confirmation_id, confirmed
    );

    let id = Uuid::parse_str(&confirmation_id)
        .map_err(|e| format!("Invalid confirmation ID: {}", e))?;

    // Find and remove the sender for this confirmation
    let sender = {
        let mut receivers = confirmation_receivers.lock().await;
        info!("Current receivers in map: {}", receivers.len());
        for (key, _) in receivers.iter() {
            info!("  - Receiver ID: {}", key);
        }
        receivers.remove(&id)
    };

    if let Some(tx) = sender {
        // Send the response back to the waiting tool execution
        tx.send(confirmed)
            .map_err(|_| "Failed to send confirmation - receiver dropped".to_string())?;
        info!(
            "Confirmation {} sent successfully: {}",
            confirmation_id, confirmed
        );
    } else {
        error!(
            "Confirmation ID {} not found in receivers map",
            confirmation_id
        );
        return Err("Confirmation ID not found or already processed".to_string());
    }

    Ok(())
}

/// Tauri command to list available models from Ollama
#[tauri::command]
pub async fn list_available_models() -> Result<Vec<serde_json::Value>, String> {
    // For now, we'll use a simple approach - just try to call Ollama's list endpoint
    let client = reqwest::Client::new();
    let url = format!("{}/api/tags", OLLAMA_BASE_URL);

    match client.get(&url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<serde_json::Value>().await {
                    Ok(data) => {
                        if let Some(models) = data.get("models").and_then(|m| m.as_array()) {
                            let model_list: Vec<serde_json::Value> = models
                                .iter()
                                .map(|model| {
                                    serde_json::json!({
                                        "name": model.get("name").and_then(|n| n.as_str()).unwrap_or("unknown"),
                                        "size": model.get("size").and_then(|s| s.as_u64()).unwrap_or(0),
                                        "modified_at": model.get("modified_at").and_then(|m| m.as_str()).unwrap_or("")
                                    })
                                })
                                .collect();
                            Ok(model_list)
                        } else {
                            Ok(vec![])
                        }
                    }
                    Err(e) => Err(format!("Failed to parse Ollama response: {}", e)),
                }
            } else {
                Err(format!("Ollama API returned status: {}", response.status()))
            }
        }
        Err(e) => Err(format!("Failed to connect to Ollama: {}", e)),
    }
}

/// Tauri command to cancel an ongoing chat message
#[tauri::command]
pub async fn cancel_chat_message(
    cancellation_tokens: tauri::State<'_, CancellationTokens>,
    session_id: Option<String>,
) -> Result<(), String> {
    info!(
        "cancel_chat_message called with session_id: {:?}",
        session_id
    );
    if let Some(session_id) = session_id {
        let mut tokens = cancellation_tokens.lock().await;
        info!(
            "Current active tokens: {:?}",
            tokens.keys().collect::<Vec<_>>()
        );
        if let Some(token) = tokens.remove(&session_id) {
            token.cancel();
            info!(
                "Successfully cancelled chat message for session: {}",
                session_id
            );
            Ok(())
        } else {
            warn!(
                "No active request found for session: {} (available: {:?})",
                session_id,
                tokens.keys().collect::<Vec<_>>()
            );
            Err("No active request found for this session".to_string())
        }
    } else {
        error!("Session ID is required for cancellation but was None");
        Err("Session ID is required for cancellation".to_string())
    }
}
