//! LLM Service for managing Ollama integration
//! 
//! This service handles:
//! - Checking Ollama availability
//! - Ensuring required models are available
//! - Downloading models with progress tracking
//! - Providing LLM access to the application

use anyhow::{Context, Result};
use mimir_dm_llm::{
    config::{EndpointType, ModelConfig},
    providers::ollama::OllamaProvider,
    LlmProvider, ModelPullProgress, Message, Usage,
};
use serde_json;
use std::collections::HashMap;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::sync::Mutex;
use tracing::{error, info, warn};

/// The model we want to use for the DM assistant
const REQUIRED_MODEL: &str = "qwen3:30b";
const OLLAMA_BASE_URL: &str = "http://localhost:11434";

/// Event emitted during model download progress
#[derive(Clone, serde::Serialize)]
struct ModelDownloadProgress {
    model: String,
    status: String,
    downloaded: u64,
    total: u64,
    percentage: f32,
}

/// LLM Service state
pub struct LlmService {
    provider: Arc<OllamaProvider>,
    model_name: String,
}

impl LlmService {
    /// Create a new LLM service instance
    pub fn new() -> Result<Self> {
        let config = Self::create_config(REQUIRED_MODEL);
        let provider = OllamaProvider::new(config)
            .context("Failed to create Ollama provider")?;
        
        Ok(Self {
            provider: Arc::new(provider),
            model_name: REQUIRED_MODEL.to_string(),
        })
    }
    
    /// Create the model configuration
    fn create_config(model: &str) -> ModelConfig {
        let mut config_map = HashMap::new();
        config_map.insert("base_url".to_string(), OLLAMA_BASE_URL.to_string());
        
        ModelConfig {
            name: format!("{}-dm", model),
            supported_endpoints: vec![
                EndpointType::Chat,
                EndpointType::Completion,
                EndpointType::Embedding,
            ],
            provider: "ollama".to_string(),
            model: model.to_string(),
            config: Some(config_map),
            limit: None, // No rate limiting for local Ollama
        }
    }
    
    /// Check if Ollama service is running
    pub async fn check_service(&self) -> Result<bool> {
        self.provider.check_service().await
            .context("Failed to check Ollama service")
    }
    
    /// Ensure the required model is available
    pub async fn ensure_model(&self, app: Option<AppHandle>) -> Result<()> {
        // First check if Ollama is running
        if !self.check_service().await? {
            return Err(anyhow::anyhow!(
                "Ollama service is not running. Please start Ollama first."
            ));
        }
        
        // Check if model exists
        info!("Checking for model: {}", self.model_name);
        let model_exists = self.provider.model_exists(&self.model_name).await
            .context("Failed to check model existence")?;
        
        if model_exists {
            info!("Model {} is already available", self.model_name);
            return Ok(());
        }
        
        // Model doesn't exist, need to download it
        info!("Model {} not found, downloading...", self.model_name);
        
        if let Some(app) = app {
            // Download with progress tracking
            self.download_model_with_progress(app).await
        } else {
            // Download without progress (for non-GUI contexts)
            self.provider.pull_model(&self.model_name).await
                .context("Failed to pull model")?;
            info!("Model {} downloaded successfully", self.model_name);
            Ok(())
        }
    }
    
    /// Download model with progress updates sent to the frontend
    async fn download_model_with_progress(&self, app: AppHandle) -> Result<()> {
        let model_name = self.model_name.clone();
        let app_clone = app.clone();
        
        // Create a channel for progress updates
        let (tx, rx) = std::sync::mpsc::channel::<ModelPullProgress>();
        
        // Spawn a task to handle progress updates
        let model_name_clone = model_name.clone();
        std::thread::spawn(move || {
            while let Ok(progress) = rx.recv() {
                let percentage = if progress.total > 0 {
                    (progress.downloaded as f32 / progress.total as f32) * 100.0
                } else {
                    0.0
                };
                
                let event = ModelDownloadProgress {
                    model: model_name_clone.clone(),
                    status: progress.status.clone(),
                    downloaded: progress.downloaded,
                    total: progress.total,
                    percentage,
                };
                
                // Emit progress event to frontend
                if let Err(e) = app_clone.emit("model-download-progress", &event) {
                    error!("Failed to emit progress event: {}", e);
                }
                
                // Check if download is complete
                if progress.status.contains("success") || 
                   progress.status.contains("already exists") {
                    info!("Model download completed: {}", progress.status);
                    break;
                }
            }
        });
        
        // Start the download with progress callback
        self.provider.pull_model_with_progress(
            &model_name,
            move |progress| {
                if let Err(e) = tx.send(progress) {
                    warn!("Failed to send progress update: {}", e);
                }
            }
        ).await
        .context("Failed to download model")?;
        
        // Emit completion event
        app.emit("model-download-complete", &model_name)
            .context("Failed to emit completion event")?;
        
        Ok(())
    }
    
    /// Get the provider for direct LLM operations
    pub fn provider(&self) -> Arc<OllamaProvider> {
        Arc::clone(&self.provider)
    }
    
    /// Get the model name being used
    pub fn model_name(&self) -> &str {
        &self.model_name
    }
}

/// Initialize the LLM service during application startup
pub async fn initialize_llm(app: Option<AppHandle>) -> Result<LlmService> {
    info!("Initializing LLM service...");
    
    let service = LlmService::new()
        .context("Failed to create LLM service")?;
    
    // Check and download model if needed
    match service.ensure_model(app).await {
        Ok(()) => {
            info!("LLM service initialized successfully with model: {}", service.model_name());
        }
        Err(e) => {
            error!("Failed to ensure model availability: {}", e);
            return Err(e);
        }
    }
    
    Ok(service)
}

/// Tauri command to check LLM status
#[tauri::command]
pub async fn check_llm_status(service: tauri::State<'_, Arc<Mutex<Option<LlmService>>>>) -> Result<bool, String> {
    let service = service.lock().await;
    
    if let Some(llm) = service.as_ref() {
        llm.check_service().await
            .map_err(|e| e.to_string())
    } else {
        Ok(false)
    }
}

/// Tauri command to get model info
#[tauri::command]
pub async fn get_llm_model_info(service: tauri::State<'_, Arc<Mutex<Option<LlmService>>>>) -> Result<String, String> {
    let service = service.lock().await;
    
    if let Some(llm) = service.as_ref() {
        Ok(llm.model_name().to_string())
    } else {
        Err("LLM service not initialized".to_string())
    }
}

/// Chat message structure for Tauri commands
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

/// Chat response with token usage
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct ChatResponseWithUsage {
    pub content: String,
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

/// Tauri command to send a chat message
#[tauri::command]
pub async fn send_chat_message(
    service: tauri::State<'_, Arc<Mutex<Option<LlmService>>>>,
    messages: Vec<ChatMessage>,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
) -> Result<ChatResponseWithUsage, String> {
    let service = service.lock().await;
    
    let llm = service.as_ref()
        .ok_or_else(|| "LLM service not initialized".to_string())?;
    
    // Convert to provider messages
    let provider_messages: Vec<mimir_dm_llm::Message> = messages
        .into_iter()
        .map(|msg| mimir_dm_llm::Message {
            role: msg.role,
            content: msg.content,
        })
        .collect();
    
    // Call the provider's chat method
    let response = llm.provider()
        .chat(
            provider_messages,
            None,                          // n (number of completions)
            temperature.or(Some(0.5)),     // temperature (default to 0.5 for better instruction following)
            max_tokens,                    // max_tokens
            None,                          // stop sequences
            None,                          // extra config
        )
        .await
        .map_err(|e| format!("Chat request failed: {}", e))?;
    
    // Extract token usage
    let usage = response.usage.unwrap_or(mimir_dm_llm::Usage {
        prompt_tokens: 0,
        completion_tokens: 0,
        total_tokens: 0,
    });
    
    Ok(ChatResponseWithUsage {
        content: response.content,
        prompt_tokens: usage.prompt_tokens,
        completion_tokens: usage.completion_tokens,
        total_tokens: usage.total_tokens,
    })
}

/// Tauri command to get model context info
#[tauri::command]
pub async fn get_model_context_info(
    service: tauri::State<'_, Arc<Mutex<Option<LlmService>>>>,
) -> Result<serde_json::Value, String> {
    let service = service.lock().await;
    
    let llm = service.as_ref()
        .ok_or_else(|| "LLM service not initialized".to_string())?;
    
    // For now, return hardcoded info for qwen3:30b
    // In the future, we could query this from Ollama
    Ok(serde_json::json!({
        "model": llm.model_name(),
        "context_length": 262144,  // From our curl query
        "default_max_tokens": 2048,
        "architecture": "qwen3moe"
    }))
}