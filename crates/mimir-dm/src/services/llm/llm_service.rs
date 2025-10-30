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
    providers::groq::GroqProvider,
    traits::ActionDescription,
    ChatResponse, CompletionResponse, EmbeddingResponse, LlmProvider, Message, ModelPullProgress, RateLimitState, TodoListTool, TodoStateManager, Tool,
};
use crate::services::provider_settings::{ProviderSettings, ProviderType};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::{oneshot, Mutex};
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::services::chat_logger::ChatLogger;
use mimir_dm_core::DatabaseService;
use crate::services::tools::ToolRegistry;
use crate::app_init::AppPaths;

/// Provider enum that wraps concrete provider implementations
/// This is needed because LlmProvider trait is not dyn-compatible due to generic methods
#[derive(Clone)]
pub enum Provider {
    Ollama(Arc<OllamaProvider>),
    Groq(Arc<GroqProvider>),
}

#[async_trait]
impl LlmProvider for Provider {
    fn config(&self) -> &ModelConfig {
        match self {
            Provider::Ollama(p) => p.config(),
            Provider::Groq(p) => p.config(),
        }
    }

    fn rate_limit_state(&self) -> &RateLimitState {
        match self {
            Provider::Ollama(p) => p.rate_limit_state(),
            Provider::Groq(p) => p.rate_limit_state(),
        }
    }

    async fn chat(
        &self,
        messages: Vec<Message>,
        tools: Option<Vec<Tool>>,
        n: Option<u32>,
        temperature: Option<f32>,
        max_tokens: Option<u32>,
        stop: Option<Vec<String>>,
        extra_config: Option<HashMap<String, String>>,
        cancellation_token: Option<CancellationToken>,
    ) -> Result<ChatResponse, mimir_dm_llm::LlmError> {
        match self {
            Provider::Ollama(p) => p.chat(messages, tools, n, temperature, max_tokens, stop, extra_config, cancellation_token).await,
            Provider::Groq(p) => p.chat(messages, tools, n, temperature, max_tokens, stop, extra_config, cancellation_token).await,
        }
    }

    async fn complete(
        &self,
        prompt: String,
        n: Option<u32>,
        temperature: Option<f32>,
        max_tokens: Option<u32>,
        stop: Option<Vec<String>>,
        extra_config: Option<HashMap<String, String>>,
    ) -> Result<CompletionResponse, mimir_dm_llm::LlmError> {
        match self {
            Provider::Ollama(p) => p.complete(prompt, n, temperature, max_tokens, stop, extra_config).await,
            Provider::Groq(p) => p.complete(prompt, n, temperature, max_tokens, stop, extra_config).await,
        }
    }

    async fn embed(
        &self,
        input: Vec<String>,
        extra_config: Option<HashMap<String, String>>,
    ) -> Result<EmbeddingResponse, mimir_dm_llm::LlmError> {
        match self {
            Provider::Ollama(p) => p.embed(input, extra_config).await,
            Provider::Groq(p) => p.embed(input, extra_config).await,
        }
    }

    async fn check_service(&self) -> Result<bool, mimir_dm_llm::LlmError> {
        match self {
            Provider::Ollama(p) => p.check_service().await,
            Provider::Groq(p) => p.check_service().await,
        }
    }

    async fn model_exists(&self, model_name: &str) -> Result<bool, mimir_dm_llm::LlmError> {
        match self {
            Provider::Ollama(p) => p.model_exists(model_name).await,
            Provider::Groq(p) => p.model_exists(model_name).await,
        }
    }

    async fn pull_model(&self, model_name: &str) -> Result<(), mimir_dm_llm::LlmError> {
        match self {
            Provider::Ollama(p) => p.pull_model(model_name).await,
            Provider::Groq(p) => p.pull_model(model_name).await,
        }
    }

    async fn pull_model_with_progress<F>(
        &self,
        model_name: &str,
        progress_callback: F,
    ) -> Result<(), mimir_dm_llm::LlmError>
    where
        F: Fn(ModelPullProgress) + Send + 'static,
    {
        match self {
            Provider::Ollama(p) => p.pull_model_with_progress(model_name, progress_callback).await,
            Provider::Groq(p) => p.pull_model_with_progress(model_name, progress_callback).await,
        }
    }
}

/// Model names for different providers
pub const OLLAMA_MODEL: &str = "gpt-oss:20b";
pub const GROQ_MODEL: &str = "openai/gpt-oss-20b";
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
    pub(super) provider: Provider,
    model_name: String,
    provider_type: ProviderType,
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
    /// Application paths for file operations
    app_paths: Arc<AppPaths>,
}

impl LlmService {
    /// Create a new LLM service instance with shared confirmation receivers
    pub fn new(
        db_service: Arc<DatabaseService>,
        confirmation_receivers: ConfirmationReceivers,
        app_handle: AppHandle,
        app_paths: Arc<AppPaths>,
    ) -> Result<Self> {
        // Load provider settings
        let settings = ProviderSettings::load(&app_paths.config_dir)
            .context("Failed to load provider settings")?;

        // Create provider based on settings
        let (provider, model_name, provider_type) = Self::create_provider_from_settings(&settings)?;

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
            provider,
            model_name,
            provider_type,
            tool_registry: Arc::new(tool_registry),
            _db_service: db_service,
            confirmation_receivers,
            app_handle: Some(app_handle),
            todo_state_manager,
            chat_loggers: Arc::new(Mutex::new(HashMap::new())),
            app_paths,
        })
    }

    /// Create provider from settings
    fn create_provider_from_settings(
        settings: &ProviderSettings,
    ) -> Result<(Provider, String, ProviderType)> {
        match settings.provider_type {
            ProviderType::Ollama => {
                let ollama_config = settings
                    .ollama_config
                    .as_ref()
                    .context("Missing Ollama configuration")?;

                let config = Self::create_ollama_config(&ollama_config.base_url);
                let provider = OllamaProvider::new(config)
                    .context("Failed to create Ollama provider")?;

                info!("Created Ollama provider with base URL: {}", ollama_config.base_url);
                Ok((Provider::Ollama(Arc::new(provider)), OLLAMA_MODEL.to_string(), ProviderType::Ollama))
            }
            ProviderType::Groq => {
                let groq_config = settings
                    .groq_config
                    .as_ref()
                    .context("Missing Groq configuration")?;

                let config = Self::create_groq_config(&groq_config.api_key);
                let provider = GroqProvider::new(config)
                    .context("Failed to create Groq provider")?;

                info!("Created Groq provider");
                Ok((Provider::Groq(Arc::new(provider)), GROQ_MODEL.to_string(), ProviderType::Groq))
            }
        }
    }

    /// Create Ollama model configuration
    fn create_ollama_config(base_url: &str) -> ModelConfig {
        let mut config_map = HashMap::new();
        config_map.insert("base_url".to_string(), base_url.to_string());

        ModelConfig {
            name: format!("{}-dm", OLLAMA_MODEL),
            supported_endpoints: vec![
                EndpointType::Chat,
                EndpointType::Completion,
                EndpointType::Embedding,
            ],
            provider: "ollama".to_string(),
            model: OLLAMA_MODEL.to_string(),
            config: Some(config_map),
            limit: None,
        }
    }

    /// Create Groq model configuration
    fn create_groq_config(api_key: &str) -> ModelConfig {
        let mut config_map = HashMap::new();
        config_map.insert("api_key".to_string(), api_key.to_string());

        ModelConfig {
            name: format!("{}-dm", GROQ_MODEL),
            supported_endpoints: vec![EndpointType::Chat, EndpointType::Completion],
            provider: "groq".to_string(),
            model: GROQ_MODEL.to_string(),
            config: Some(config_map),
            limit: None,
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
        // Only perform these checks for Ollama (local provider)
        // Groq is a cloud service and doesn't need service/model checks
        match self.provider_type {
            ProviderType::Ollama => {
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
            ProviderType::Groq => {
                // Groq is a cloud service - no local checks needed
                info!("Using Groq cloud provider with model: {}", self.model_name);
                Ok(())
            }
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
    pub fn provider(&self) -> Provider {
        self.provider.clone()
    }

    /// Get the configured provider
    /// Note: The endpoint parameter is deprecated and ignored - configure provider via settings
    pub(super) fn get_provider_with_endpoint(
        &self,
        _endpoint: Option<&str>,
    ) -> Result<Provider> {
        Ok(self.provider.clone())
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
        let logger = ChatLogger::new(session_id.to_string(), &self.app_paths.logs_dir)
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
    app_paths: Arc<AppPaths>,
) -> Result<LlmService> {
    info!("Initializing LLM service...");

    let service = LlmService::new(db_service, confirmation_receivers, app_handle, app_paths)
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
