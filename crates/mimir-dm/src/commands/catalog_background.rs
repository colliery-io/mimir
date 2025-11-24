use tauri::State;
use mimir_dm_core::services::BackgroundService;
use mimir_dm_core::DatabaseService;
use mimir_dm_core::models::catalog::BackgroundFilters;
use std::sync::Arc;
use tracing::error;

#[tauri::command]
pub async fn search_backgrounds(
    query: Option<String>,
    sources: Option<Vec<String>>,
    has_tools: Option<bool>,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<Vec<serde_json::Value>, String> {
    let mut conn = db_service.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let filters = BackgroundFilters {
        search_pattern: query,
        sources,
        has_tools,
    };

    let backgrounds = BackgroundService::search_backgrounds(&mut conn, filters)
        .map_err(|e| e.to_string())?;
    
    // Convert to JSON for frontend
    let json_backgrounds: Vec<serde_json::Value> = backgrounds
        .into_iter()
        .map(|bg| serde_json::to_value(bg).unwrap_or(serde_json::Value::Null))
        .collect();
    
    Ok(json_backgrounds)
}

#[tauri::command]
pub async fn get_background_details(
    name: String,
    source: String,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<serde_json::Value, String> {
    let mut conn = db_service.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let background = BackgroundService::get_background_by_name_and_source(&mut conn, &name, &source)
        .map_err(|e| e.to_string())?;
    
    // Parse the full JSON data
    let full_data: serde_json::Value = serde_json::from_str(&background.full_background_json)
        .map_err(|e| format!("Failed to parse background JSON: {}", e))?;
    
    Ok(full_data)
}

#[tauri::command]
pub async fn get_background_sources(
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<Vec<String>, String> {
    let mut conn = db_service.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    BackgroundService::get_background_sources(&mut conn)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_background_count(
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<i64, String> {
    let mut conn = db_service.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    BackgroundService::get_background_count(&mut conn)
        .map_err(|e| e.to_string())
}