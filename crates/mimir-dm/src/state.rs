//! Consolidated application state for Tauri
//!
//! This module defines the `AppState` struct that consolidates all shared
//! application state into a single managed resource. This improves code
//! organization and makes state dependencies explicit.

use crate::app_init::AppPaths;
use crate::commands::chat_sessions::SessionManager;
use crate::services::context_service::ContextState;
use crate::services::llm::{ConfirmationReceivers, CancellationTokens, LlmService};
use mimir_dm_core::DatabaseService;
use std::sync::Arc;

/// Consolidated application state managed by Tauri.
///
/// All command handlers receive this via `State<'_, AppState>` and can
/// access the specific services they need.
///
/// # Example
///
/// ```ignore
/// #[tauri::command]
/// pub async fn some_command(
///     state: State<'_, AppState>,
/// ) -> Result<SomeResponse, ApiError> {
///     let conn = state.db.get_connection()?;
///     // Use connection...
/// }
/// ```
pub struct AppState {
    /// Database service for all database operations
    pub db: Arc<DatabaseService>,

    /// Application paths (config dir, data dir, etc.)
    pub paths: Arc<AppPaths>,

    /// Context service for managing conversation context
    pub context: ContextState,

    /// Session manager for chat session persistence
    pub sessions: SessionManager,

    /// Receivers for LLM tool confirmations
    pub confirmations: ConfirmationReceivers,

    /// Cancellation tokens for LLM operations
    pub cancellations: CancellationTokens,

    /// LLM service (initialized asynchronously)
    pub llm: Arc<tokio::sync::Mutex<Option<LlmService>>>,
}

impl AppState {
    /// Create a new AppState with all required services
    pub fn new(
        db: Arc<DatabaseService>,
        paths: Arc<AppPaths>,
        context: ContextState,
        sessions: SessionManager,
        confirmations: ConfirmationReceivers,
        cancellations: CancellationTokens,
        llm: Arc<tokio::sync::Mutex<Option<LlmService>>>,
    ) -> Self {
        Self {
            db,
            paths,
            context,
            sessions,
            confirmations,
            cancellations,
            llm,
        }
    }
}
