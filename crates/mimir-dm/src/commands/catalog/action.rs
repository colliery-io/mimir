//! Database-backed action catalog commands.
//!
//! Provides Tauri commands for searching and retrieving combat action data
//! from the 5e catalog database. Actions include standard combat options
//! like Attack, Dash, Dodge, etc.

use crate::state::AppState;
use mimir_dm_core::models::catalog::{Action, ActionFilters, ActionSummary};
use mimir_dm_core::services::ActionService;
use tauri::State;
use tracing::{debug, error, info};

/// Search the action catalog with optional filters.
///
/// Returns a list of action summaries matching the provided criteria.
/// All filter parameters are optional and can be combined.
///
/// # Parameters
/// - `name` - Text to search in action names (case-insensitive)
/// - `search` - Text to search in action descriptions
/// - `time_types` - Filter by action time (e.g., `["Action", "Bonus Action"]`)
/// - `sources` - Filter by source books
///
/// # Returns
/// List of `ActionSummary` objects containing basic action information.
///
/// # Errors
/// Returns an error string if the database connection or query fails.
#[tauri::command]
pub async fn search_actions(
    name: Option<String>,
    search: Option<String>,
    time_types: Option<Vec<String>>,
    sources: Option<Vec<String>>,
    state: State<'_, AppState>,
) -> Result<Vec<ActionSummary>, String> {
    debug!("Database action search - name: {:?}, search: {:?}, time_types: {:?}, sources: {:?}",
           name, search, time_types, sources);

    let filters = ActionFilters {
        name,
        search,
        time_types,
        sources,
    };

    let mut conn = state.db.get_connection().map_err(|e| {
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

/// Get complete action details by database ID.
///
/// Retrieves the full action record including description and rules text.
///
/// # Parameters
/// - `action_id` - Database ID of the action
///
/// # Returns
/// The complete `Action` object if found, or `None` if no match exists.
///
/// # Errors
/// Returns an error string if the database connection or query fails.
#[tauri::command]
pub async fn get_action(
    action_id: i32,
    state: State<'_, AppState>,
) -> Result<Option<Action>, String> {
    debug!("Getting action by ID: {}", action_id);

    let mut conn = state.db.get_connection().map_err(|e| {
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

/// Get all unique action time types in the catalog.
///
/// Returns time type values present in the action catalog.
/// Used to populate filter dropdowns in the UI.
///
/// # Returns
/// List of time types (e.g., `["Action", "Bonus Action", "Reaction"]`).
///
/// # Errors
/// Returns an error string if the database connection or query fails.
#[tauri::command]
pub async fn get_action_time_types(
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    debug!("Getting action time types");

    let mut conn = state.db.get_connection().map_err(|e| {
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

/// Get all unique source books containing actions.
///
/// Returns source book abbreviations that contain at least one action.
/// Used to populate filter dropdowns in the UI.
///
/// # Returns
/// List of source abbreviations.
///
/// # Errors
/// Returns an error string if the database connection or query fails.
#[tauri::command]
pub async fn get_action_sources(
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    debug!("Getting action sources");

    let mut conn = state.db.get_connection().map_err(|e| {
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

/// Get total number of actions in the catalog.
///
/// Returns the total count of all actions across all source books.
///
/// # Returns
/// Total action count as a 64-bit integer.
///
/// # Errors
/// Returns an error string if the database connection or query fails.
#[tauri::command]
pub async fn get_action_count(
    state: State<'_, AppState>,
) -> Result<i64, String> {
    debug!("Getting action count");

    let mut conn = state.db.get_connection().map_err(|e| {
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