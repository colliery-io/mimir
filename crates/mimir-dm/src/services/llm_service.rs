//! LLM Service for managing Ollama integration
//! 
//! This service handles:
//! - Checking Ollama availability
//! - Ensuring required models are available
//! - Downloading models with progress tracking
//! - Providing LLM access to the application

use anyhow::{anyhow, Context, Result};
use mimir_dm_llm::{
    config::{EndpointType, ModelConfig},
    providers::ollama::OllamaProvider,
    traits::ActionDescription,
    LlmProvider, ModelPullProgress,
};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle, Emitter};
use tokio::sync::{oneshot, Mutex};
use tokio::time::timeout;
use tracing::{error, info, warn};
use uuid::Uuid;

use super::tools::{
    ToolRegistry, 
    implementations::SayHelloTool,
    document_tools::{GetDocumentTool, ListDocumentsTool},
    update_document_tool::UpdateDocumentTool,
};
use super::database::DatabaseService;

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

/// Request for tool confirmation sent to frontend
#[derive(Clone, Serialize, Deserialize)]
pub struct ToolConfirmationRequest {
    pub id: String,
    pub tool_name: String,
    pub action: ActionDescription,
}

/// Global confirmation state that can be shared across the app
pub type ConfirmationReceivers = Arc<Mutex<HashMap<Uuid, oneshot::Sender<bool>>>>;

/// LLM Service state
pub struct LlmService {
    provider: Arc<OllamaProvider>,
    model_name: String,
    tool_registry: Arc<ToolRegistry>,
    db_service: Arc<DatabaseService>,
    /// Channel senders for pending confirmations (shared globally)
    confirmation_receivers: ConfirmationReceivers,
    /// App handle for emitting events
    app_handle: Option<AppHandle>,
}

impl LlmService {
    /// Create a new LLM service instance with shared confirmation receivers
    pub fn new(db_service: Arc<DatabaseService>, confirmation_receivers: ConfirmationReceivers) -> Result<Self> {
        let config = Self::create_config(REQUIRED_MODEL);
        let provider = OllamaProvider::new(config)
            .context("Failed to create Ollama provider")?;
        
        // Create tool registry and register tools
        let mut tool_registry = ToolRegistry::new();
        tool_registry.register(Arc::new(SayHelloTool));
        tool_registry.register(Arc::new(GetDocumentTool::new(db_service.clone())));
        tool_registry.register(Arc::new(ListDocumentsTool::new(db_service.clone())));
        tool_registry.register(Arc::new(UpdateDocumentTool::new(db_service.clone())));
        
        Ok(Self {
            provider: Arc::new(provider),
            model_name: REQUIRED_MODEL.to_string(),
            tool_registry: Arc::new(tool_registry),
            db_service,
            confirmation_receivers,
            app_handle: None,
        })
    }
    
    /// Set the app handle for emitting events
    pub fn set_app_handle(&mut self, app: AppHandle) {
        self.app_handle = Some(app);
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
    
    /// Request confirmation from the user for a tool action
    async fn request_confirmation(
        &self,
        action: ActionDescription,
        tool_name: String,
    ) -> Result<bool> {
        let app = self.app_handle.as_ref()
            .ok_or_else(|| anyhow!("App handle not set for confirmation requests"))?;
        
        let confirmation_id = Uuid::new_v4();
        info!("Creating confirmation request with ID: {}", confirmation_id);
        
        // Create a oneshot channel for this specific confirmation
        let (tx, rx) = oneshot::channel::<bool>();
        
        // Store the sender so the Tauri command can send the response
        {
            let mut receivers = self.confirmation_receivers.lock().await;
            receivers.insert(confirmation_id, tx);
            info!("Stored confirmation receiver, total receivers: {}", receivers.len());
        }
        
        // Emit event to frontend
        app.emit("tool-confirmation-request", ToolConfirmationRequest {
            id: confirmation_id.to_string(),
            tool_name,
            action,
        })?;
        info!("Emitted confirmation request to frontend");
        
        // Wait for response with timeout
        match timeout(Duration::from_secs(60), rx).await {
            Ok(Ok(confirmed)) => {
                info!("Received confirmation response: {}", confirmed);
                Ok(confirmed)
            },
            Ok(Err(_)) => Err(anyhow!("Confirmation channel closed")),
            Err(_) => {
                // Timeout - clean up the receiver
                self.confirmation_receivers.lock().await.remove(&confirmation_id);
                Err(anyhow!("Confirmation timeout - no response from user"))
            }
        }
    }
}

/// Initialize the LLM service during application startup
pub async fn initialize_llm(
    app: Option<AppHandle>, 
    db_service: Arc<DatabaseService>,
    confirmation_receivers: ConfirmationReceivers
) -> Result<LlmService> {
    info!("Initializing LLM service...");
    
    let mut service = LlmService::new(db_service, confirmation_receivers)
        .context("Failed to create LLM service")?;
    
    // Set app handle if provided
    if let Some(app_handle) = app.clone() {
        service.set_app_handle(app_handle);
    }
    
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

/// Tauri command to send a chat message (with optional tool support)
#[tauri::command]
pub async fn send_chat_message(
    service: tauri::State<'_, Arc<Mutex<Option<LlmService>>>>,
    messages: Vec<ChatMessage>,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
    enable_tools: Option<bool>,
) -> Result<ChatResponseWithUsage, String> {
    let service = service.lock().await;
    
    let llm = service.as_ref()
        .ok_or_else(|| "LLM service not initialized".to_string())?;
    
    // Convert to provider messages
    let mut provider_messages: Vec<mimir_dm_llm::Message> = messages
        .into_iter()
        .map(|msg| mimir_dm_llm::Message {
            role: msg.role,
            content: msg.content,
        })
        .collect();
    
    // Get tools if enabled
    let tools = if enable_tools.unwrap_or(false) {
        Some(llm.tool_registry.get_tool_definitions())
    } else {
        None
    };
    
    // Inject system rules if tools are enabled
    if tools.is_some() {
        let system_rules = llm.tool_registry.generate_system_rules();
        if !system_rules.is_empty() {
            // Check if there's already a system message, or create one
            let system_content = system_rules.join("\n\n");
            
            // If the first message is a system message, append to it
            if let Some(first_msg) = provider_messages.first_mut() {
                if first_msg.role == "system" {
                    first_msg.content.push_str("\n\n");
                    first_msg.content.push_str(&system_content);
                } else {
                    // Insert system message at the beginning
                    provider_messages.insert(0, mimir_dm_llm::Message {
                        role: "system".to_string(),
                        content: system_content,
                    });
                }
            } else {
                // No messages yet, add system message
                provider_messages.push(mimir_dm_llm::Message {
                    role: "system".to_string(),
                    content: system_content,
                });
            }
            
            info!("Injected {} system rules for tool guidance", system_rules.len());
        }
    }
    
    // Tool execution loop (max 5 iterations to prevent infinite loops)
    const MAX_TOOL_ITERATIONS: usize = 5;
    let mut tool_call_count = 0;
    let mut final_response = None;
    
    while tool_call_count < MAX_TOOL_ITERATIONS {
        // Log message flow before LLM call
        info!("=== LLM Call {} ===", tool_call_count + 1);
        info!("Sending {} messages to LLM ({})", 
            provider_messages.len(),
            provider_messages.iter().map(|m| m.role.as_str()).collect::<Vec<_>>().join(", ")
        );
        
        // Call the provider's chat method
        let response = llm.provider()
            .chat(
                provider_messages.clone(),
                tools.clone(),
                None,                          // n (number of completions)
                temperature.or(Some(0.5)),     // temperature (default to 0.5 for better instruction following)
                max_tokens.or(Some(16384)),    // max_tokens (default to 16384 for thinking models)
                None,                          // stop sequences
                None,                          // extra config
            )
            .await
            .map_err(|e| format!("Chat request failed: {}", e))?;
        
        // Log response structure
        info!("LLM Response: content_length={}, tool_calls={}", 
            response.content.len(),
            response.tool_calls.as_ref().map_or(0, |tc| tc.len())
        );
        
        if let Some(tool_calls) = &response.tool_calls {
            if !tool_calls.is_empty() {
                let tool_names: Vec<&str> = tool_calls.iter().map(|tc| tc.function.name.as_str()).collect();
                info!("Tool calls requested: [{}]", tool_names.join(", "));
            }
        }
        
        // Check if there are tool calls
        if let Some(tool_calls) = &response.tool_calls {
            if !tool_calls.is_empty() {
                tool_call_count += 1;
                info!("Processing {} tool calls (iteration {})", tool_calls.len(), tool_call_count);
                
                // Add assistant message with tool calls
                provider_messages.push(mimir_dm_llm::Message {
                    role: "assistant".to_string(),
                    content: response.content.clone(),
                });
                
                // Execute each tool call
                for (idx, tool_call) in tool_calls.iter().enumerate() {
                    let tool_name = &tool_call.function.name;
                    let tool_args = &tool_call.function.arguments;
                    
                    // Extract key parameters for logging
                    let doc_type = tool_args.get("document_type").and_then(|v| v.as_str()).unwrap_or("unknown");
                    let campaign_id = tool_args.get("campaign_id").and_then(|v| v.as_i64()).unwrap_or(-1);
                    
                    info!("Tool {}: {} (campaign: {}, doc: {})", 
                        idx + 1, tool_name, campaign_id, doc_type
                    );
                    
                    // Check if tool requires confirmation
                    if llm.tool_registry.requires_confirmation(tool_name) {
                        // Get action description
                        if let Some(action_desc) = llm.tool_registry.get_action_description(tool_name, tool_args) {
                            info!("Tool {} requires confirmation, requesting from user", tool_name);
                            
                            // Request confirmation from user
                            match llm.request_confirmation(action_desc, tool_name.clone()).await {
                                Ok(confirmed) => {
                                    if !confirmed {
                                        info!("User rejected tool {} execution", tool_name);
                                        // User rejected - add cancellation message
                                        provider_messages.push(mimir_dm_llm::Message {
                                            role: "tool".to_string(),
                                            content: format!("Action cancelled by user: {}", tool_name),
                                        });
                                        continue; // Skip to next tool call
                                    }
                                    info!("User confirmed tool {} execution", tool_name);
                                }
                                Err(e) => {
                                    error!("Confirmation request failed: {}", e);
                                    provider_messages.push(mimir_dm_llm::Message {
                                        role: "tool".to_string(),
                                        content: format!("Confirmation failed: {}", e),
                                    });
                                    continue;
                                }
                            }
                        } else {
                            error!("Tool {} requires confirmation but provided no action description", tool_name);
                            provider_messages.push(mimir_dm_llm::Message {
                                role: "tool".to_string(),
                                content: format!("Tool configuration error: missing action description"),
                            });
                            continue;
                        }
                    }
                    
                    // Execute the tool (either no confirmation needed or user confirmed)
                    let tool_result = llm.tool_registry
                        .execute_tool(tool_name, tool_args.clone())
                        .await
                        .unwrap_or_else(|e| {
                            error!("Tool {} execution failed: {}", tool_name, e);
                            format!("Tool execution failed: {}", e)
                        });
                    
                    info!("Tool {} result: {} chars", tool_name, tool_result.len());
                    
                    // Check if this is a structured success response
                    let is_success_response = if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&tool_result) {
                        parsed.get("status").and_then(|s| s.as_str()) == Some("success")
                    } else {
                        false
                    };
                    
                    // Add tool response to messages
                    provider_messages.push(mimir_dm_llm::Message {
                        role: "tool".to_string(),
                        content: tool_result.clone(),
                    });
                    
                    // If this was a successful update action and it's the only/last tool call,
                    // we can short-circuit and return a simple success message
                    if is_success_response && idx == tool_calls.len() - 1 {
                        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&tool_result) {
                            if let Some(message) = parsed.get("message").and_then(|m| m.as_str()) {
                                warn!("=== EARLY EXIT: Tool {} returned success, short-circuiting ===", tool_name);
                                info!("Success message: {}", message);
                                // Return early with a simple success message
                                return Ok(ChatResponseWithUsage {
                                    content: message.to_string(),
                                    prompt_tokens: 0,  // We didn't make another LLM call
                                    completion_tokens: 0,
                                    total_tokens: 0,
                                });
                            }
                        }
                    }
                }
                
                // Continue loop to get next response
                info!("=== Continuing loop for next LLM call ===");
                continue;
            }
        }
        
        // No tool calls, we have the final response
        info!("=== No tool calls found, ending loop ===");
        final_response = Some(response);
        break;
    }
    
    if tool_call_count >= MAX_TOOL_ITERATIONS {
        warn!("Reached maximum tool iterations ({})", MAX_TOOL_ITERATIONS);
    }
    
    let response = final_response
        .ok_or_else(|| "Maximum tool iterations reached".to_string())?;
    
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
    info!("Received confirmation request: ID={}, confirmed={}", confirmation_id, confirmed);
    
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
        info!("Confirmation {} sent successfully: {}", confirmation_id, confirmed);
    } else {
        error!("Confirmation ID {} not found in receivers map", confirmation_id);
        return Err("Confirmation ID not found or already processed".to_string());
    }
    
    Ok(())
}