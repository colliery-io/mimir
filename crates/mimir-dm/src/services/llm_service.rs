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
    TodoListTool, TodoStateManager,
    ReadFileTool, WriteFileTool, ListFilesTool, EditFileTool, FileToolsConfig, SayHelloTool,
};
use serde::{Deserialize, Serialize};
use serde_json::{self, json};
use std::collections::HashMap;
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::{oneshot, Mutex};
use tracing::{debug, error, info, warn};

/// Helper macro for bifurcated logging - full content to file, truncated to console
macro_rules! debug_content {
    ($msg:expr, $full_content:expr, $truncate_at:expr) => {
        // Full content to file (debug level)
        debug!(target: "file_only", "{}: {}", $msg, $full_content);
        // Truncated content to console (debug level, but with default target)
        if $full_content.len() > $truncate_at {
            debug!("{}: {}... [truncated from {} chars]", $msg, 
                &$full_content.chars().take($truncate_at).collect::<String>(), 
                $full_content.len());
        } else {
            debug!("{}: {}", $msg, $full_content);
        }
    };
}

/// Helper macro for bifurcated info logging
macro_rules! info_content {
    ($msg:expr, $full_content:expr, $truncate_at:expr) => {
        // Full content to file
        info!(target: "file_only", "{}: {}", $msg, $full_content);
        // Truncated content to console
        if $full_content.len() > $truncate_at {
            info!("{}: {}... [truncated from {} chars]", $msg, 
                &$full_content.chars().take($truncate_at).collect::<String>(), 
                $full_content.len());
        } else {
            info!("{}: {}", $msg, $full_content);
        }
    };
}
use uuid::Uuid;

use super::tools::ToolRegistry;
use super::database::DatabaseService;

/// Strip thinking blocks from content for logging (simple string replacement)
fn strip_thinking_blocks(content: &str) -> String {
    let mut result = content.to_string();
    
    // Remove <thinking> blocks (simple approach)
    while let (Some(start), Some(end)) = (result.find("<thinking>"), result.find("</thinking>")) {
        if start < end {
            result = format!("{}{}", &result[..start], &result[end + 12..]);
        } else {
            break;
        }
    }
    
    // Remove <think> blocks
    while let (Some(start), Some(end)) = (result.find("<think>"), result.find("</think>")) {
        if start < end {
            result = format!("{}{}", &result[..start], &result[end + 8..]);
        } else {
            break;
        }
    }
    
    result.trim().to_string()
}

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

/// Intermediate message from LLM (during multi-turn tool execution)
#[derive(Clone, Serialize, Deserialize)]
pub struct IntermediateMessage {
    pub role: String,
    pub content: String,
    pub tool_calls: Vec<String>,
    pub iteration: usize,
    pub session_id: Option<String>,
}

/// Tool result message
#[derive(Clone, Serialize, Deserialize)]
pub struct ToolResultMessage {
    pub tool_name: String,
    pub result: String,
    pub success: bool,
    pub iteration: usize,
    pub session_id: Option<String>,
}

/// Global confirmation state that can be shared across the app
pub type ConfirmationReceivers = Arc<Mutex<HashMap<Uuid, oneshot::Sender<bool>>>>;

/// LLM Service state
pub struct LlmService {
    provider: Arc<OllamaProvider>,
    model_name: String,
    tool_registry: Arc<ToolRegistry>,
    _db_service: Arc<DatabaseService>,
    /// Channel senders for pending confirmations (shared globally)
    confirmation_receivers: ConfirmationReceivers,
    /// App handle for emitting events
    app_handle: Option<AppHandle>,
    /// Todo state manager for ephemeral todos
    todo_state_manager: TodoStateManager,
}

impl LlmService {
    /// Create a new LLM service instance with shared confirmation receivers
    pub fn new(db_service: Arc<DatabaseService>, confirmation_receivers: ConfirmationReceivers, app_handle: AppHandle) -> Result<Self> {
        let config = Self::create_config(REQUIRED_MODEL, None);
        let provider = OllamaProvider::new(config)
            .context("Failed to create Ollama provider")?;
        
        // Create todo state manager
        let todo_state_manager = TodoStateManager::new();
        
        // Create file tools configuration
        let app_paths = crate::APP_PATHS.get()
            .ok_or_else(|| anyhow!("App paths not initialized"))?;
        
        let file_config = Arc::new(FileToolsConfig::with_root(app_paths.data_dir.clone()));
        info!("Configured file tool access to: {}", app_paths.data_dir.display());
        
        // Create tool registry and register tools
        let mut tool_registry = ToolRegistry::new();
        tool_registry.register(Arc::new(SayHelloTool));
        tool_registry.register(Arc::new(ReadFileTool::new(file_config.clone())));
        tool_registry.register(Arc::new(WriteFileTool::new(file_config.clone())));
        tool_registry.register(Arc::new(ListFilesTool::new(file_config.clone())));
        tool_registry.register(Arc::new(EditFileTool::new(file_config.clone())));
        
        // Configure default todo storage path using app handle
        if let Ok(app_data_dir) = app_handle.path().app_data_dir() {
            let todos_dir = app_data_dir.join("todos");
            if let Err(e) = todo_state_manager.configure_storage(todos_dir.clone()) {
                warn!("Failed to configure default todo storage: {}", e);
            } else {
                info!("Configured default todo storage: {:?}", todos_dir);
            }
        } else {
            warn!("Could not determine app data directory for todos");
        }
        
        // Register TodoListTool with configurable state manager
        let todo_tool = TodoListTool::new(todo_state_manager.clone());
        tool_registry.register(Arc::new(todo_tool));
        info!("Registered TodoListTool with configurable state manager");
        
        Ok(Self {
            provider: Arc::new(provider),
            model_name: REQUIRED_MODEL.to_string(),
            tool_registry: Arc::new(tool_registry),
            _db_service: db_service,
            confirmation_receivers,
            app_handle: Some(app_handle),
            todo_state_manager,
        })
    }
    
    
    
    /// Create the model configuration
    fn create_config(model: &str, base_url: Option<&str>) -> ModelConfig {
        let mut config_map = HashMap::new();
        config_map.insert("base_url".to_string(), base_url.unwrap_or(OLLAMA_BASE_URL).to_string());
        
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
    
    /// Get or create a provider with a specific endpoint
    fn get_provider_with_endpoint(&self, endpoint: Option<&str>) -> Result<Arc<OllamaProvider>> {
        // If no custom endpoint provided, use the default provider
        if endpoint.is_none() || endpoint == Some(OLLAMA_BASE_URL) {
            return Ok(self.provider.clone());
        }
        
        // Create a new provider with the custom endpoint
        let config = Self::create_config(REQUIRED_MODEL, endpoint);
        let provider = OllamaProvider::new(config)
            .context("Failed to create Ollama provider with custom endpoint")?;
        Ok(Arc::new(provider))
    }
    
    /// Get the model name being used
    pub fn model_name(&self) -> &str {
        &self.model_name
    }
    
    /// Get todos for a session from the state manager
    pub fn get_session_todos(&self, session_id: &str) -> Vec<mimir_dm_llm::TodoItem> {
        self.todo_state_manager.get_todos(session_id)
    }
    
    /// Configure todo storage path
    pub fn configure_todo_storage(&self, storage_path: std::path::PathBuf) -> Result<()> {
        self.todo_state_manager.configure_storage(storage_path)
            .map_err(|e| anyhow!("Failed to configure todo storage: {}", e))?;
        Ok(())
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
        
        // Wait for response indefinitely
        match rx.await {
            Ok(confirmed) => {
                info!("Received confirmation response: {}", confirmed);
                Ok(confirmed)
            },
            Err(_) => Err(anyhow!("Confirmation channel closed"))
        }
    }
}

/// Initialize the LLM service during application startup
pub async fn initialize_llm(
    app_handle: AppHandle, 
    db_service: Arc<DatabaseService>,
    confirmation_receivers: ConfirmationReceivers
) -> Result<LlmService> {
    info!("Initializing LLM service...");
    
    let service = LlmService::new(db_service, confirmation_receivers, app_handle)
        .context("Failed to create LLM service")?;
    
    // Check and download model if needed
    match service.ensure_model(None).await {
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
    session_id: Option<String>,
    _model_name: Option<String>,
    ollama_url: Option<String>,
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
        let tool_definitions = llm.tool_registry.get_tool_definitions();
        debug!("Tools enabled: {} tools available", tool_definitions.len());
        for tool in &tool_definitions {
            debug!("  Tool: {}", tool.function.name);
        }
        Some(tool_definitions)
    } else {
        debug!("Tools disabled for this request");
        None
    };
    
    // Inject system rules if tools are enabled
    if tools.is_some() {
        let system_rules = llm.tool_registry.generate_system_rules(session_id.as_deref());
        if !system_rules.is_empty() {
            // Check if there's already a system message, or create one
            let system_content = system_rules.join("\n\n");
            
            info!("Generated {} system rules for LLM context", system_rules.len());
            debug_content!("System rules content", system_content, 200);
            
            // If the first message is a system message, prepend critical rules to the beginning
            if let Some(first_msg) = provider_messages.first_mut() {
                if first_msg.role == "system" {
                    // Put critical file path info at the very beginning
                    let original_content = first_msg.content.clone();
                    first_msg.content = format!("{}\n\n{}", system_content, original_content);
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
    
    // Tool execution loop (max 20 iterations to prevent infinite loops)
    const MAX_TOOL_ITERATIONS: usize = 20;
    let mut tool_call_count = 0;
    let mut final_response = None;
    
    while tool_call_count < MAX_TOOL_ITERATIONS {
        // Log message flow before LLM call
        info!("=== LLM Call {} ===", tool_call_count + 1);
        info!("Sending {} messages to LLM ({})", 
            provider_messages.len(),
            provider_messages.iter().map(|m| m.role.as_str()).collect::<Vec<_>>().join(", ")
        );
        
        debug!("Request parameters:");
        debug!("  Temperature: {:?}", temperature.or(Some(0.3)));
        debug!("  Max tokens: {:?}", max_tokens.or(Some(16384)));
        debug!("  Tools provided: {}", tools.is_some());
        if let Some(ref tools) = tools {
            debug!("  Tool names: [{}]", tools.iter().map(|t| t.function.name.as_str()).collect::<Vec<_>>().join(", "));
        }
        
        debug!("Message details:");
        for (i, msg) in provider_messages.iter().enumerate() {
            debug!("  Message {}: role='{}' content_length={}", i + 1, msg.role, msg.content.len());
            
            // Strip thinking blocks and show the actual content being sent
            let content_without_thinking = strip_thinking_blocks(&msg.content);
            if content_without_thinking.len() < 300 {
                debug!("    Content: {}", content_without_thinking);
            } else {
                // Safe UTF-8 truncation to avoid panics on character boundaries
                let truncated = content_without_thinking
                    .char_indices()
                    .take_while(|(i, _)| *i < 300)
                    .last()
                    .map(|(i, c)| i + c.len_utf8())
                    .unwrap_or(0);
                debug!("    Content preview: {}...", &content_without_thinking[..truncated]);
            }
            
            // Show if thinking blocks were present
            if content_without_thinking.len() != msg.content.len() {
                debug!("    [Thinking blocks removed: {} chars -> {} chars]", 
                    msg.content.len(), content_without_thinking.len());
            }
        }
        
        // Get the appropriate provider (with custom endpoint if specified)
        let provider = llm.get_provider_with_endpoint(ollama_url.as_deref())
            .map_err(|e| {
                error!("Failed to get provider with endpoint: {}", e);
                error!("Endpoint: {}", ollama_url.as_deref().unwrap_or(OLLAMA_BASE_URL));
                format!("Failed to get provider with endpoint: {}", e)
            })?;
        
        // Log the request details before making the call
        info!("Making LLM request: endpoint={}, model={}, messages={}, tools={}", 
            ollama_url.as_deref().unwrap_or(OLLAMA_BASE_URL), 
            REQUIRED_MODEL,
            provider_messages.len(),
            tools.as_ref().map_or(0, |t| t.len())
        );
        
        // Call the provider's chat method
        let response = provider
            .chat(
                provider_messages.clone(),
                tools.clone(),
                None,                          // n (number of completions)
                temperature.or(Some(0.3)),     // temperature (default to 0.3 for more deterministic tool calling)
                max_tokens.or(Some(16384)),    // max_tokens (default to 16384 for thinking models)
                None,                          // stop sequences
                None,                          // extra config
            )
            .await
            .map_err(|e| {
                error!("Chat request failed: {}", e);
                error!("Request details: endpoint={}, model={}, messages={}, tools={}", 
                    ollama_url.as_deref().unwrap_or(OLLAMA_BASE_URL), 
                    REQUIRED_MODEL,
                    provider_messages.len(),
                    tools.as_ref().map_or(0, |t| t.len())
                );
                format!("Chat request failed: {}", e)
            })?;
        
        // Log response structure and check for thinking blocks
        let has_thinking = response.content.contains("<think>") || response.content.contains("<thinking>");
        info!("LLM Response: content_length={}, tool_calls={}, has_thinking_blocks={}", 
            response.content.len(),
            response.tool_calls.as_ref().map_or(0, |tc| tc.len()),
            has_thinking
        );
        
        if has_thinking {
            warn!("LLM response contains thinking blocks despite think=false parameter");
            debug_content!("Response preview", response.content, 200);
        }
        
        debug!("Response details:");
        let response_without_thinking = strip_thinking_blocks(&response.content);
        debug_content!("Content preview", response_without_thinking, 150);
        if response_without_thinking.len() != response.content.len() {
            debug!("  [Response thinking blocks removed: {} chars -> {} chars]", 
                response.content.len(), response_without_thinking.len());
        }
        debug!("  Tool calls present: {}", response.tool_calls.is_some());
        
        if let Some(tool_calls) = &response.tool_calls {
            debug!("  Tool calls count: {}", tool_calls.len());
            for (i, tool_call) in tool_calls.iter().enumerate() {
                debug!("    Tool call {}: function='{}' args_length={}", 
                    i + 1, 
                    tool_call.function.name, 
                    serde_json::to_string(&tool_call.function.arguments).map_or(0, |s| s.len())
                );
                debug!("      Arguments: {}", 
                    serde_json::to_string_pretty(&tool_call.function.arguments).unwrap_or_else(|_| "Invalid JSON".to_string())
                );
            }
            
            if !tool_calls.is_empty() {
                let tool_names: Vec<&str> = tool_calls.iter().map(|tc| tc.function.name.as_str()).collect();
                info!("Tool calls requested: [{}]", tool_names.join(", "));
            }
        } else {
            debug!("  No tool calls in response - final answer mode");
        }
        
        // Check if there are tool calls
        if let Some(tool_calls) = &response.tool_calls {
            if !tool_calls.is_empty() {
                tool_call_count += 1;
                info!("Processing {} tool calls (iteration {})", tool_calls.len(), tool_call_count);
                
                // Emit intermediate LLM response
                if let Some(ref app) = llm.app_handle {
                    let tool_names: Vec<String> = tool_calls.iter()
                        .map(|tc| tc.function.name.clone())
                        .collect();
                    
                    let intermediate_msg = IntermediateMessage {
                        role: "assistant".to_string(),
                        content: response.content.clone(),
                        tool_calls: tool_names,
                        iteration: tool_call_count,
                        session_id: session_id.clone(),
                    };
                    
                    if let Err(e) = app.emit("llm-intermediate-message", &intermediate_msg) {
                        debug!("Failed to emit intermediate message: {}", e);
                    }
                }
                
                // Add assistant message with tool calls
                provider_messages.push(mimir_dm_llm::Message {
                    role: "assistant".to_string(),
                    content: response.content.clone(),
                });
                
                // Execute each tool call
                info!("=== Processing {} tool calls ===", tool_calls.len());
                for (idx, tool_call) in tool_calls.iter().enumerate() {
                    let tool_name = &tool_call.function.name;
                    let mut tool_args = tool_call.function.arguments.clone();
                    
                    info!("Processing tool call {}/{}: {}", idx + 1, tool_calls.len(), tool_name);
                    let args_json = serde_json::to_string_pretty(&tool_args).unwrap_or_else(|_| "Invalid JSON".to_string());
                    debug_content!("Tool arguments", args_json, 300);
                    
                    // Inject session_id for todo_write tool if session_id is provided
                    if tool_name == "todo_write" && session_id.is_some() {
                        if let Some(ref session_id_value) = session_id {
                            tool_args.as_object_mut().unwrap().insert(
                                "session_id".to_string(), 
                                serde_json::Value::String(session_id_value.clone())
                            );
                            debug!("Injected session_id '{}' into todo_write tool", session_id_value);
                        }
                    }
                    
                    // Extract key parameters for logging
                    let doc_type = tool_args.get("document_type").and_then(|v| v.as_str()).unwrap_or("unknown");
                    let campaign_id = tool_args.get("campaign_id").and_then(|v| v.as_i64()).unwrap_or(-1);
                    
                    info!("Tool {}: {} (campaign: {}, doc: {})", 
                        idx + 1, tool_name, campaign_id, doc_type
                    );
                    
                    // Check if tool requires confirmation
                    if llm.tool_registry.requires_confirmation(tool_name) {
                        // Get action description
                        if let Some(action_desc) = llm.tool_registry.get_action_description(tool_name, &tool_args) {
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
                    info!("Executing tool: {} with {} bytes of arguments", tool_name, serde_json::to_string(&tool_args).unwrap_or_default().len());
                    
                    // Try up to 2 times if it fails
                    let mut tool_result = None;
                    for attempt in 1..=2 {
                        info!("Tool {} execution attempt {}/2", tool_name, attempt);
                        match llm.tool_registry.execute_tool(tool_name, tool_args.clone()).await {
                            Ok(result) => {
                                info!("Tool {} succeeded on attempt {} - result length: {} chars", 
                                    tool_name, attempt, result.len());
                                tool_result = Some(result);
                                if attempt > 1 {
                                    info!("Tool {} recovered after retry", tool_name);
                                }
                                break;
                            }
                            Err(e) => {
                                error!("Tool {} execution failed on attempt {}: {}", tool_name, attempt, e);
                                if attempt == 2 {
                                    // Final attempt failed
                                    let error_msg = format!("Tool execution failed after {} attempts: {}", attempt, e);
                                    error!("FINAL FAILURE for tool {}: {}", tool_name, error_msg);
                                    tool_result = Some(error_msg);
                                } else {
                                    warn!("Tool {} failed on attempt {}, retrying in 100ms", tool_name, attempt);
                                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                                }
                            }
                        }
                    }
                    let tool_result = tool_result.unwrap();
                    
                    info!("Tool {} result: {} chars", tool_name, tool_result.len());
                    
                    // Emit tool result
                    if let Some(ref app) = llm.app_handle {
                        let success = !tool_result.contains("Tool execution failed");
                        let tool_result_msg = ToolResultMessage {
                            tool_name: tool_name.clone(),
                            result: tool_result.clone(),
                            success,
                            iteration: tool_call_count,
                            session_id: session_id.clone(),
                        };
                        
                        if let Err(e) = app.emit("tool-result-message", &tool_result_msg) {
                            debug!("Failed to emit tool result message: {}", e);
                        }
                        
                        // If this was a todo_write tool and successful, emit todos update
                        if tool_name == "todo_write" && success {
                            if let Some(session_id) = &session_id {
                                let current_todos = llm.get_session_todos(session_id);
                                if let Err(e) = app.emit("todos-updated", &json!({
                                    "session_id": session_id,
                                    "todos": current_todos
                                })) {
                                    debug!("Failed to emit todos update: {}", e);
                                }
                            }
                        }
                    }
                    
                    // Check if this is a structured success response
                    let is_success_response = if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&tool_result) {
                        parsed.get("status").and_then(|s| s.as_str()) == Some("success")
                    } else {
                        false
                    };
                    
                    // Add tool response to messages
                    let is_error = tool_result.contains("Tool execution failed") || tool_result.contains("error");
                    info!("Adding tool result to conversation: {} (error: {})", tool_name, is_error);
                    if is_error {
                        warn!("Tool error being added to LLM context: {}", tool_result);
                    }
                    debug_content!("Tool result content", tool_result, 200);
                    
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
                info!("Current conversation has {} messages", provider_messages.len());
                info!("Last message role: {}, content length: {} chars", 
                    provider_messages.last().map(|m| m.role.as_str()).unwrap_or("none"),
                    provider_messages.last().map(|m| m.content.len()).unwrap_or(0)
                );
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
    
    // Apply thinking block size limit to prevent future token issues
    let limited_content = limit_thinking_block_size(&response.content, 12000); // ~3k tokens worth of thinking
    
    Ok(ChatResponseWithUsage {
        content: limited_content,
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

/// Limit the size of thinking blocks to prevent token overflow
/// If thinking blocks exceed the limit, truncate them with a warning
fn limit_thinking_block_size(content: &str, max_thinking_chars: usize) -> String {
    if !content.contains("<thinking>") {
        return content.to_string();
    }
    
    let mut result = String::new();
    let mut remaining = content;
    let mut total_thinking_size = 0;
    
    while let Some(start_pos) = remaining.find("<thinking>") {
        // Add content before thinking block
        result.push_str(&remaining[..start_pos]);
        
        // Find the end of this thinking block
        let thinking_start = start_pos + "<thinking>".len();
        if let Some(end_pos) = remaining[thinking_start..].find("</thinking>") {
            let thinking_content = &remaining[thinking_start..thinking_start + end_pos];
            let thinking_size = thinking_content.len();
            
            total_thinking_size += thinking_size;
            
            if total_thinking_size <= max_thinking_chars {
                // Include full thinking block
                result.push_str("<thinking>");
                result.push_str(thinking_content);
                result.push_str("</thinking>");
            } else {
                // Truncate thinking block
                let available_space = max_thinking_chars - (total_thinking_size - thinking_size);
                if available_space > 100 {
                    result.push_str("<thinking>");
                    result.push_str(&thinking_content[..available_space]);
                    result.push_str("\n\n[THINKING TRUNCATED - too long for token limit]");
                    result.push_str("</thinking>");
                } else {
                    result.push_str("<thinking>[THINKING TRUNCATED - too long for token limit]</thinking>");
                }
                
                warn!("Truncated thinking block: {} chars -> {} chars (limit: {})", 
                    thinking_size, available_space, max_thinking_chars);
            }
            
            // Move past this thinking block
            remaining = &remaining[thinking_start + end_pos + "</thinking>".len()..];
        } else {
            // Malformed thinking block, just add it as-is
            result.push_str(&remaining[start_pos..]);
            break;
        }
    }
    
    // Add any remaining content
    result.push_str(remaining);
    result
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
                    Err(e) => Err(format!("Failed to parse Ollama response: {}", e))
                }
            } else {
                Err(format!("Ollama API returned status: {}", response.status()))
            }
        }
        Err(e) => Err(format!("Failed to connect to Ollama: {}", e))
    }
}