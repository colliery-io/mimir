//! Database-backed action catalog commands

use mimir_dm_core::models::catalog::{Action, ActionFilters, ActionSummary};
use mimir_dm_core::services::ActionService;
use mimir_dm_core::DatabaseService;
use std::sync::Arc;
use tauri::State;
use tracing::{debug, error, info};

/// Search actions using database with filters
#[tauri::command]
pub async fn search_actions(
    name: Option<String>,
    search: Option<String>,
    time_types: Option<Vec<String>>,
    sources: Option<Vec<String>>,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<Vec<ActionSummary>, String> {
    debug!("Database action search - name: {:?}, search: {:?}, time_types: {:?}, sources: {:?}",
           name, search, time_types, sources);

    let filters = ActionFilters {
        name,
        search,
        time_types,
        sources,
    };

    let mut conn = db_service.get_connection().map_err(|e| {
        error!("Database connection error during action search: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    match ActionService::search_actions(&mut conn, filters) {
        Ok(actions) => {
            info!("Found {} actions in database search", actions.len());
            Ok(actions)
        }
        Err(e) => {
            error!("Database action search failed: {}", e);
            Err(format!("Failed to search actions: {}", e))
        }
    }
}

/// Get a specific action by ID for modal display
#[tauri::command]
pub async fn get_action(
    action_id: i32,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<Option<Action>, String> {
    debug!("Getting action by ID: {}", action_id);

    let mut conn = db_service.get_connection().map_err(|e| {
        error!("Database connection error getting action: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    match ActionService::get_action_by_id(&mut conn, action_id) {
        Ok(action) => Ok(action),
        Err(e) => {
            error!("Failed to get action: {}", e);
            Err(format!("Failed to get action: {}", e))
        }
    }
}

/// Get all available time types for filter dropdowns
#[tauri::command]
pub async fn get_action_time_types(
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<Vec<String>, String> {
    debug!("Getting action time types");

    let mut conn = db_service.get_connection().map_err(|e| {
        error!("Database connection error getting time types: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    match ActionService::get_time_types(&mut conn) {
        Ok(time_types) => Ok(time_types),
        Err(e) => {
            error!("Failed to get time types: {}", e);
            Err(format!("Failed to get time types: {}", e))
        }
    }
}

/// Get all available sources for filter dropdowns
#[tauri::command]
pub async fn get_action_sources(
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<Vec<String>, String> {
    debug!("Getting action sources");

    let mut conn = db_service.get_connection().map_err(|e| {
        error!("Database connection error getting sources: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    match ActionService::get_sources(&mut conn) {
        Ok(sources) => Ok(sources),
        Err(e) => {
            error!("Failed to get sources: {}", e);
            Err(format!("Failed to get sources: {}", e))
        }
    }
}

/// Get action count for statistics
#[tauri::command]
pub async fn get_action_count(
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<i64, String> {
    debug!("Getting action count");

    let mut conn = db_service.get_connection().map_err(|e| {
        error!("Database connection error getting action count: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    match ActionService::get_action_count(&mut conn) {
        Ok(count) => Ok(count),
        Err(e) => {
            error!("Failed to get action count: {}", e);
            Err(format!("Failed to get action count: {}", e))
        }
    }
}