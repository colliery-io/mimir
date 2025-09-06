//! Todo management commands

use crate::{APP_PATHS, types::ApiResponse};
use serde::{Deserialize, Serialize};
use std::fs;
use tracing::{debug, error, info};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoItem {
    pub content: String,
    pub status: String, // "pending", "in_progress", "completed"
    #[serde(rename = "activeForm")]
    pub active_form: String, // Present tense form for display during execution
}

/// Get todos for a specific session
#[tauri::command]
pub async fn get_todos(
    session_id: String,
) -> Result<ApiResponse<Vec<TodoItem>>, String> {
    info!("Loading todos for session: {}", session_id);
    
    let app_paths = APP_PATHS.get()
        .ok_or_else(|| "App paths not initialized".to_string())?;
    
    let todos_dir = app_paths.data_dir.join("todos");
    let file_path = todos_dir.join(format!("{}.json", session_id));
    
    if !file_path.exists() {
        debug!("Todo file does not exist for session {}, returning empty list", session_id);
        return Ok(ApiResponse::success(Vec::new()));
    }
    
    match fs::read_to_string(&file_path) {
        Ok(content) => {
            match serde_json::from_str::<Vec<TodoItem>>(&content) {
                Ok(todos) => {
                    info!("Loaded {} todos for session {}", todos.len(), session_id);
                    Ok(ApiResponse::success(todos))
                }
                Err(e) => {
                    error!("Failed to parse todo JSON for session {}: {}", session_id, e);
                    Ok(ApiResponse::error(format!("Failed to parse todo file: {}", e)))
                }
            }
        }
        Err(e) => {
            error!("Failed to read todo file for session {}: {}", session_id, e);
            Ok(ApiResponse::error(format!("Failed to read todo file: {}", e)))
        }
    }
}

/// Clear all todos for a specific session
#[tauri::command]
pub async fn clear_todos(
    session_id: String,
) -> Result<ApiResponse<()>, String> {
    info!("Clearing todos for session: {}", session_id);
    
    let app_paths = APP_PATHS.get()
        .ok_or_else(|| "App paths not initialized".to_string())?;
    
    let todos_dir = app_paths.data_dir.join("todos");
    let file_path = todos_dir.join(format!("{}.json", session_id));
    
    if file_path.exists() {
        match fs::remove_file(&file_path) {
            Ok(_) => {
                info!("Successfully cleared todos for session {}", session_id);
                Ok(ApiResponse::success(()))
            }
            Err(e) => {
                error!("Failed to remove todo file for session {}: {}", session_id, e);
                Ok(ApiResponse::error(format!("Failed to clear todos: {}", e)))
            }
        }
    } else {
        debug!("No todo file to clear for session {}", session_id);
        Ok(ApiResponse::success(()))
    }
}