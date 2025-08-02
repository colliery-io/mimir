//! Application info commands

use crate::{types::ApiResponse, APP_PATHS};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppInfo {
    pub database_path: String,
    pub app_dir: String,
    pub config_dir: String,
    pub data_dir: String,
}

#[tauri::command]
pub async fn get_app_info() -> ApiResponse<AppInfo> {
    match APP_PATHS.get() {
        Some(paths) => {
            let app_info = AppInfo {
                database_path: paths.database_path_str(),
                app_dir: paths.app_dir.to_string_lossy().to_string(),
                config_dir: paths.config_dir.to_string_lossy().to_string(),
                data_dir: paths.data_dir.to_string_lossy().to_string(),
            };
            ApiResponse::success(app_info)
        }
        None => ApiResponse::error("Application not initialized".to_string())
    }
}

#[tauri::command]
pub async fn greet(name: String) -> String {
    format!("Hello, {}! Welcome to Mimir.", name)
}

#[tauri::command]
pub async fn get_default_campaigns_directory() -> ApiResponse<String> {
    use directories::UserDirs;
    
    match UserDirs::new() {
        Some(user_dirs) => {
            let documents_dir = user_dirs.document_dir()
                .unwrap_or_else(|| user_dirs.home_dir())
                .join("Mimir Campaigns");
            
            ApiResponse::success(documents_dir.to_string_lossy().to_string())
        }
        None => {
            ApiResponse::error("Could not determine user directories".to_string())
        }
    }
}