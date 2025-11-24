use tauri::State;
use mimir_dm_core::services::FeatService;
use mimir_dm_core::DatabaseService;
use mimir_dm_core::models::catalog::FeatFilters;
use std::sync::Arc;
use tracing::error;

#[tauri::command]
pub async fn search_feats(
    query: Option<String>,
    sources: Option<Vec<String>>,
    has_prerequisites: Option<bool>,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<Vec<serde_json::Value>, String> {
    let mut conn = db_service.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let filters = FeatFilters {
        search_pattern: query,
        sources,
        has_prerequisites,
    };

    let feats = FeatService::search_feats(&mut conn, filters)
        .map_err(|e| e.to_string())?;
    
    // Convert to JSON for frontend
    let json_feats: Vec<serde_json::Value> = feats
        .into_iter()
        .map(|feat| serde_json::to_value(feat).unwrap_or(serde_json::Value::Null))
        .collect();
    
    Ok(json_feats)
}

#[tauri::command]
pub async fn get_feat_details(
    name: String,
    source: String,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<serde_json::Value, String> {
    let mut conn = db_service.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let feat = FeatService::get_feat_by_name_and_source(&mut conn, &name, &source)
        .map_err(|e| e.to_string())?;
    
    // Parse the full JSON data
    let full_data: serde_json::Value = serde_json::from_str(&feat.full_feat_json)
        .map_err(|e| format!("Failed to parse feat JSON: {}", e))?;
    
    Ok(full_data)
}

#[tauri::command]
pub async fn get_feat_sources(
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<Vec<String>, String> {
    let mut conn = db_service.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    FeatService::get_feat_sources(&mut conn)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_feat_count(
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<i64, String> {
    let mut conn = db_service.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    FeatService::get_feat_count(&mut conn)
        .map_err(|e| e.to_string())
}