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
    LlmProvider, ModelPullProgress, TodoListTool, TodoStateManager,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::{oneshot, Mutex};
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::services::chat_logger::ChatLogger;
use crate::services::database::DatabaseService;
use crate::services::tools::ToolRegistry;

/// The model we want to use for the DM assistant
pub const REQUIRED_MODEL: &str = "gpt-oss:20b";
pub const OLLAMA_BASE_URL: &str = "http://localhost:11434";

/// Event emitted during model download progress
#[derive(Clone, Serialize)]
pub struct ModelDownloadProgress {
    pub model: String,
    pub status: String,
    pub downloaded: u64,
    pub total: u64,
    pub percentage: f32,
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
pub type CancellationTokens = Arc<Mutex<HashMap<String, CancellationToken>>>;

/// LLM Service state
pub struct LlmService {
    pub(super) provider: Arc<OllamaProvider>,
    model_name: String,
    pub(super) tool_registry: Arc<ToolRegistry>,
    _db_service: Arc<DatabaseService>,
    /// Channel senders for pending confirmations (shared globally)
    confirmation_receivers: ConfirmationReceivers,
    /// App handle for emitting events
    pub(super) app_handle: Option<AppHandle>,
    /// Todo state manager for ephemeral todos
    pub(super) todo_state_manager: TodoStateManager,
    /// Chat loggers by session ID
    chat_loggers: Arc<Mutex<HashMap<String, Arc<ChatLogger>>>>,
}

impl LlmService {
    /// Create a new LLM service instance with shared confirmation receivers
    pub fn new(
        db_service: Arc<DatabaseService>,
        confirmation_receivers: ConfirmationReceivers,
        app_handle: AppHandle,
    ) -> Result<Self> {
        let config = Self::create_config(REQUIRED_MODEL, None);
        let provider =
            OllamaProvider::new(config).context("Failed to create Ollama provider")?;

        // Create todo state manager
        let todo_state_manager = TodoStateManager::new();

        // Create tool registry - file tools will be added dynamically when campaign directory is provided
        let mut tool_registry = ToolRegistry::new();
        info!("Tool registry created - file tools will be configured per campaign");

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
            chat_loggers: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    /// Create the model configuration
    fn create_config(model: &str, base_url: Option<&str>) -> ModelConfig {
        let mut config_map = HashMap::new();
        config_map.insert(
            "base_url".to_string(),
            base_url.unwrap_or(OLLAMA_BASE_URL).to_string(),
        );

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
        self.provider
            .check_service()
            .await
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
        let model_exists = self
            .provider
            .model_exists(&self.model_name)
            .await
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
            self.provider
                .pull_model(&self.model_name)
                .await
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
                if progress.status.contains("success")
                    || progress.status.contains("already exists")
                {
                    info!("Model download completed: {}", progress.status);
                    break;
                }
            }
        });

        // Start the download with progress callback
        self.provider
            .pull_model_with_progress(&model_name, move |progress| {
                if let Err(e) = tx.send(progress) {
                    warn!("Failed to send progress update: {}", e);
                }
            })
            .await
            .context("Failed to download model")?;

        // Emit completion event
        app.emit("model-download-complete", &model_name)
            .context("Failed to emit completion event")?;

        Ok(())
    }

    /// Get the provider for direct LLM operations
    #[allow(dead_code)]
    pub fn provider(&self) -> Arc<OllamaProvider> {
        Arc::clone(&self.provider)
    }

    /// Get or create a provider with a specific endpoint
    pub(super) fn get_provider_with_endpoint(
        &self,
        endpoint: Option<&str>,
    ) -> Result<Arc<OllamaProvider>> {
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
        self.todo_state_manager
            .configure_storage(storage_path)
            .map_err(|e| anyhow!("Failed to configure todo storage: {}", e))?;
        Ok(())
    }

    /// Get or create a chat logger for a session
    pub async fn get_chat_logger(&self, session_id: &str) -> Result<Arc<ChatLogger>> {
        let mut loggers = self.chat_loggers.lock().await;

        if let Some(logger) = loggers.get(session_id) {
            return Ok(Arc::clone(logger));
        }

        // Create new logger
        let app_paths = crate::APP_PATHS
            .get()
            .ok_or_else(|| anyhow!("App paths not initialized"))?;

        let logger = ChatLogger::new(session_id.to_string(), &app_paths.logs_dir)
            .context("Failed to create chat logger")?;
        let logger_arc = Arc::new(logger);

        loggers.insert(session_id.to_string(), Arc::clone(&logger_arc));
        info!("Created chat logger for session: {}", session_id);

        Ok(logger_arc)
    }

    /// Request confirmation from the user for a tool action
    pub(super) async fn request_confirmation(
        &self,
        action: ActionDescription,
        tool_name: String,
    ) -> Result<bool> {
        let app = self
            .app_handle
            .as_ref()
            .ok_or_else(|| anyhow!("App handle not set for confirmation requests"))?;

        let confirmation_id = Uuid::new_v4();
        info!("Creating confirmation request with ID: {}", confirmation_id);

        // Create a oneshot channel for this specific confirmation
        let (tx, rx) = oneshot::channel::<bool>();

        // Store the sender so the Tauri command can send the response
        {
            let mut receivers = self.confirmation_receivers.lock().await;
            receivers.insert(confirmation_id, tx);
            info!(
                "Stored confirmation receiver, total receivers: {}",
                receivers.len()
            );
        }

        // Emit event to frontend
        app.emit(
            "tool-confirmation-request",
            ToolConfirmationRequest {
                id: confirmation_id.to_string(),
                tool_name,
                action,
            },
        )?;
        info!("Emitted confirmation request to frontend");

        // Wait for response indefinitely
        match rx.await {
            Ok(confirmed) => {
                info!("Received confirmation response: {}", confirmed);
                Ok(confirmed)
            }
            Err(_) => Err(anyhow!("Confirmation channel closed")),
        }
    }
}

/// Initialize the LLM service during application startup
pub async fn initialize_llm(
    app_handle: AppHandle,
    db_service: Arc<DatabaseService>,
    confirmation_receivers: ConfirmationReceivers,
) -> Result<LlmService> {
    info!("Initializing LLM service...");

    let service = LlmService::new(db_service, confirmation_receivers, app_handle)
        .context("Failed to create LLM service")?;

    // Check and download model if needed
    match service.ensure_model(None).await {
        Ok(()) => {
            info!(
                "LLM service initialized successfully with model: {}",
                service.model_name()
            );
        }
        Err(e) => {
            error!("Failed to ensure model availability: {}", e);
            return Err(e);
        }
    }

    Ok(service)
}
