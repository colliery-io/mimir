//! Session todo query and configuration commands

use crate::{
    services::llm::LlmService,
    types::ApiResponse,
};
use mimir_dm_llm::TodoItem;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;
use tracing::{debug, info, warn};

/// Get todos for a specific session from the LLM service's ephemeral state
#[tauri::command]
pub async fn get_session_todos(
    llm_service: State<'_, Arc<Mutex<Option<LlmService>>>>,
    session_id: String,
) -> Result<ApiResponse<Vec<TodoItem>>, String> {
    info!("Getting todos for session: {}", session_id);
    
    let service = llm_service.lock().await;
    
    if let Some(llm) = service.as_ref() {
        let todos = llm.get_session_todos(&session_id);
        debug!("Found {} todos for session {}", todos.len(), session_id);
        Ok(ApiResponse::success(todos))
    } else {
        debug!("LLM service not initialized, returning empty todos");
        Ok(ApiResponse::success(Vec::new()))
    }
}

/// Configure where todos should be stored
#[tauri::command]
pub async fn configure_todo_storage(
    llm_service: State<'_, Arc<Mutex<Option<LlmService>>>>,
    storage_path: String,
) -> Result<ApiResponse<()>, String> {
    info!("Configuring todo storage path: {}", storage_path);
    
    let service = llm_service.lock().await;
    
    if let Some(llm) = service.as_ref() {
        let path = PathBuf::from(storage_path);
        match llm.configure_todo_storage(path.clone()) {
            Ok(()) => {
                info!("Todo storage configured successfully to: {:?}", path);
                Ok(ApiResponse::success(()))
            }
            Err(e) => {
                warn!("Failed to configure todo storage: {}", e);
                Ok(ApiResponse::error(format!("Failed to configure todo storage: {}", e)))
            }
        }
    } else {
        warn!("LLM service not initialized, cannot configure todo storage");
        Ok(ApiResponse::error("LLM service not initialized".to_string()))
    }
}