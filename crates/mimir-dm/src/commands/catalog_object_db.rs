use tauri::State;
use mimir_dm_core::services::ObjectService;
use mimir_dm_core::DatabaseService;
use mimir_dm_core::models::catalog::ObjectFilters;
use std::sync::Arc;
use tracing::error;

#[tauri::command]
pub async fn search_objects(
    search: Option<String>,
    sources: Option<Vec<String>>,
    object_types: Option<Vec<String>>,
    sizes: Option<Vec<String>>,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<Vec<serde_json::Value>, String> {
    let mut conn = db_service.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let filters = ObjectFilters {
        search_pattern: search,
        sources,
        object_types,
        sizes,
    };

    let results = ObjectService::search_objects(&mut conn, filters)?;

    // Convert ObjectSummary to JSON values for frontend compatibility
    let json_results: Vec<serde_json::Value> = results
        .into_iter()
        .map(|obj| serde_json::to_value(&obj).unwrap_or_default())
        .collect();

    Ok(json_results)
}

#[tauri::command]
pub async fn get_object_details(
    name: String,
    source: String,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<Option<String>, String> {
    let mut conn = db_service.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    ObjectService::get_object_details(&mut conn, &name, &source)
}

#[tauri::command]
pub async fn get_object_sources(
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<Vec<String>, String> {
    let mut conn = db_service.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    ObjectService::get_object_sources(&mut conn)
}

#[tauri::command]
pub async fn get_object_count(
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<i64, String> {
    let mut conn = db_service.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    ObjectService::get_object_count(&mut conn)
}

#[tauri::command]
pub async fn get_object_types(
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<Vec<String>, String> {
    let mut conn = db_service.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    ObjectService::get_object_types(&mut conn)
}

#[tauri::command]
pub async fn get_object_sizes(
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<Vec<String>, String> {
    let mut conn = db_service.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    ObjectService::get_object_sizes(&mut conn)
}