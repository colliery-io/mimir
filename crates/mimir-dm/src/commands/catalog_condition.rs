//! Database-backed condition catalog commands

use crate::state::AppState;
use mimir_dm_core::models::catalog::{ConditionOrDisease, ConditionFilters, ConditionSummary};
use mimir_dm_core::services::ConditionService;
use tauri::State;
use tracing::{debug, error, info};

/// Search conditions and diseases using database with filters
#[tauri::command]
pub async fn search_conditions(
    state: State<'_, AppState>,
    name: Option<String>,
    search: Option<String>,
    item_types: Option<Vec<String>>,
    sources: Option<Vec<String>>,
) -> Result<Vec<ConditionSummary>, String> {
    debug!("Database condition search - name: {:?}, search: {:?}, item_types: {:?}, sources: {:?}",
           name, search, item_types, sources);

    let filters = ConditionFilters {
        name,
        search,
        item_types,
        sources,
    };

    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Database connection error during condition search: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    match ConditionService::search_conditions(&mut conn, filters) {
        Ok(conditions) => {
            info!("Found {} conditions in database search", conditions.len());
            Ok(conditions)
        }
        Err(e) => {
            error!("Database condition search failed: {}", e);
            Err(format!("Failed to search conditions: {}", e))
        }
    }
}

/// Get a specific condition or disease by ID for modal display
#[tauri::command]
pub async fn get_condition(
    state: State<'_, AppState>,
    condition_id: i32,
) -> Result<Option<ConditionOrDisease>, String> {
    debug!("Getting condition by ID: {}", condition_id);

    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Database connection error getting condition: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    match ConditionService::get_condition_by_id(&mut conn, condition_id) {
        Ok(condition) => Ok(condition),
        Err(e) => {
            error!("Failed to get condition: {}", e);
            Err(format!("Failed to get condition: {}", e))
        }
    }
}

/// Get all available item types for filter dropdowns
#[tauri::command]
pub async fn get_condition_item_types(
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    debug!("Getting condition item types");

    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Database connection error getting item types: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    match ConditionService::get_item_types(&mut conn) {
        Ok(item_types) => Ok(item_types),
        Err(e) => {
            error!("Failed to get item types: {}", e);
            Err(format!("Failed to get item types: {}", e))
        }
    }
}

/// Get all available sources for filter dropdowns
#[tauri::command]
pub async fn get_condition_sources(
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    debug!("Getting condition sources");

    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Database connection error getting sources: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    match ConditionService::get_sources(&mut conn) {
        Ok(sources) => Ok(sources),
        Err(e) => {
            error!("Failed to get sources: {}", e);
            Err(format!("Failed to get sources: {}", e))
        }
    }
}

/// Get condition count for statistics
#[tauri::command]
pub async fn get_condition_count(
    state: State<'_, AppState>,
) -> Result<i64, String> {
    debug!("Getting condition count");

    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Database connection error getting condition count: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    match ConditionService::get_condition_count(&mut conn) {
        Ok(count) => Ok(count),
        Err(e) => {
            error!("Failed to get condition count: {}", e);
            Err(format!("Failed to get condition count: {}", e))
        }
    }
}