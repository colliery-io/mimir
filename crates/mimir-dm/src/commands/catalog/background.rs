//! Database-backed background catalog commands.
//!
//! Provides Tauri commands for searching and retrieving character background data
//! from the 5e catalog database. Used for character creation.

use crate::state::AppState;
use mimir_dm_core::models::catalog::BackgroundFilters;
use mimir_dm_core::services::BackgroundService;
use tauri::State;
use tracing::error;

/// Search the background catalog with optional filters.
///
/// Returns a list of background summaries matching the provided criteria.
/// All filter parameters are optional and can be combined.
///
/// # Parameters
/// - `query` - Text to search in background names (case-insensitive)
/// - `sources` - Filter by source books
/// - `has_tools` - Filter for backgrounds that grant tool proficiencies
///
/// # Returns
/// List of background objects as JSON values.
///
/// # Errors
/// Returns an error string if the database connection or query fails.
#[tauri::command]
pub async fn search_backgrounds(
    query: Option<String>,
    sources: Option<Vec<String>>,
    has_tools: Option<bool>,
    state: State<'_, AppState>,
) -> Result<Vec<serde_json::Value>, String> {
    let mut conn = state.db.get_connection().map_err(|e| {
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

/// Get complete background details by name and source.
///
/// Retrieves the full background data including features, proficiencies, and equipment.
///
/// # Parameters
/// - `name` - Exact background name (case-sensitive)
/// - `source` - Source book abbreviation (e.g., "PHB")
///
/// # Returns
/// The complete background data as a JSON value.
///
/// # Errors
/// Returns an error string if the database connection or query fails.
#[tauri::command]
pub async fn get_background_details(
    name: String,
    source: String,
    state: State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    let mut conn = state.db.get_connection().map_err(|e| {
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

/// Get all unique source books containing backgrounds.
///
/// Returns source book abbreviations that contain at least one background.
/// Used to populate filter dropdowns in the UI.
///
/// # Returns
/// List of source abbreviations.
///
/// # Errors
/// Returns an error string if the database connection or query fails.
#[tauri::command]
pub async fn get_background_sources(
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    BackgroundService::get_background_sources(&mut conn)
        .map_err(|e| e.to_string())
}

/// Get total number of backgrounds in the catalog.
///
/// Returns the total count of all backgrounds across all source books.
///
/// # Returns
/// Total background count as a 64-bit integer.
///
/// # Errors
/// Returns an error string if the database connection or query fails.
#[tauri::command]
pub async fn get_background_count(
    state: State<'_, AppState>,
) -> Result<i64, String> {
    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    BackgroundService::get_background_count(&mut conn)
        .map_err(|e| e.to_string())
}