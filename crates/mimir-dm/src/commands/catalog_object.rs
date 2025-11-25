use crate::state::AppState;
use mimir_dm_core::models::catalog::ObjectFilters;
use mimir_dm_core::services::ObjectService;
use tauri::State;
use tracing::error;

#[tauri::command]
pub async fn search_objects(
    search: Option<String>,
    sources: Option<Vec<String>>,
    object_types: Option<Vec<String>>,
    sizes: Option<Vec<String>>,
    state: State<'_, AppState>,
) -> Result<Vec<serde_json::Value>, String> {
    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let filters = ObjectFilters {
        search_pattern: search,
        sources,
        object_types,
        sizes,
    };

    let results = ObjectService::search_objects(&mut conn, filters)
        .map_err(|e| format!("Failed to search objects: {}", e))?;

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
    state: State<'_, AppState>,
) -> Result<Option<String>, String> {
    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    ObjectService::get_object_details(&mut conn, &name, &source)
        .map_err(|e| format!("Failed to get object details: {}", e))
}

#[tauri::command]
pub async fn get_object_sources(
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    ObjectService::get_object_sources(&mut conn)
        .map_err(|e| format!("Failed to get object sources: {}", e))
}

#[tauri::command]
pub async fn get_object_count(
    state: State<'_, AppState>,
) -> Result<i64, String> {
    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    ObjectService::get_object_count(&mut conn)
        .map_err(|e| format!("Failed to get object count: {}", e))
}

#[tauri::command]
pub async fn get_object_types(
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    ObjectService::get_object_types(&mut conn)
        .map_err(|e| format!("Failed to get object types: {}", e))
}

#[tauri::command]
pub async fn get_object_sizes(
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    ObjectService::get_object_sizes(&mut conn)
        .map_err(|e| format!("Failed to get object sizes: {}", e))
}