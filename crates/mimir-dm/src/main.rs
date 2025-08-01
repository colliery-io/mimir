#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

impl<T> ApiResponse<T> {
    fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }
}

#[tauri::command]
async fn greet(name: String) -> String {
    format!("Hello, {}! Welcome to Mimir.", name)
}

#[tauri::command]
async fn get_message() -> ApiResponse<String> {
    ApiResponse::success("Hello from Tauri!".to_string())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, get_message])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}