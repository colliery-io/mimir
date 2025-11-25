use crate::state::AppState;
use mimir_dm_core::models::catalog::RaceFilters;
use mimir_dm_core::services::RaceService;
use tauri::State;
use tracing::error;

#[tauri::command]
pub async fn search_races(
    search: Option<String>,
    sources: Option<Vec<String>>,
    sizes: Option<Vec<String>>,
    has_darkvision: Option<bool>,
    has_flight: Option<bool>,
    state: State<'_, AppState>,
) -> Result<Vec<serde_json::Value>, String> {
    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let filters = RaceFilters {
        search_pattern: search,
        sources,
        sizes,
        has_darkvision,
        has_flight,
    };

    let results = RaceService::search_races(&mut conn, filters)
        .map_err(|e| e.to_string())?;

    // Convert RaceSummary to JSON values for frontend compatibility
    let json_results: Vec<serde_json::Value> = results
        .into_iter()
        .map(|race| serde_json::to_value(&race).unwrap_or_default())
        .collect();

    Ok(json_results)
}

#[tauri::command]
pub async fn get_race_details(
    name: String,
    source: String,
    state: State<'_, AppState>,
) -> Result<Option<String>, String> {
    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    RaceService::get_race_details(&mut conn, &name, &source)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_race_sources(
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    RaceService::get_race_sources(&mut conn)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_race_count(
    state: State<'_, AppState>,
) -> Result<i64, String> {
    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    RaceService::get_race_count(&mut conn)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_race_sizes(
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    RaceService::get_race_sizes(&mut conn)
        .map_err(|e| e.to_string())
}