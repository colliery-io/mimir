//! Application info commands

use crate::app_init::AppPaths;
use crate::types::ApiResponse;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppInfo {
    pub database_path: String,
    pub app_dir: String,
    pub config_dir: String,
    pub data_dir: String,
}

#[tauri::command]
pub async fn get_app_info(
    app_paths: State<'_, Arc<AppPaths>>
) -> Result<ApiResponse<AppInfo>, String> {
    let app_info = AppInfo {
        database_path: app_paths.database_path_str(),
        app_dir: app_paths.app_dir.to_string_lossy().to_string(),
        config_dir: app_paths.config_dir.to_string_lossy().to_string(),
        data_dir: app_paths.data_dir.to_string_lossy().to_string(),
    };
    Ok(ApiResponse::success(app_info))
}

#[tauri::command]
pub async fn greet(name: String) -> String {
    format!("Hello, {}! Welcome to Mimir.", name)
}

#[tauri::command]
pub async fn get_default_campaigns_directory() -> Result<ApiResponse<String>, String> {
    use directories::UserDirs;
    
    match UserDirs::new() {
        Some(user_dirs) => {
            let documents_dir = user_dirs.document_dir()
                .unwrap_or_else(|| user_dirs.home_dir())
                .join("Mimir Campaigns");
            
            Ok(ApiResponse::success(documents_dir.to_string_lossy().to_string()))
        }
        None => {
            Ok(ApiResponse::error("Could not determine user directories".to_string()))
        }
    }
}