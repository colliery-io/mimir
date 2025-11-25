//! Database-backed monster catalog commands.
//!
//! Provides Tauri commands for searching, filtering, and retrieving monster data
//! from the 5e catalog database. Used for encounter building and monster browsing.

use crate::state::AppState;
use mimir_dm_core::models::catalog::monster::{Monster, MonsterFilters, MonsterSummary};
use mimir_dm_core::services::MonsterService;
use tauri::State;
use tracing::{debug, error};

/// Search the monster catalog with filters.
///
/// Returns a list of monster summaries matching the provided filter criteria.
/// Supports filtering by name, CR range, size, type, alignment, and source.
///
/// # Parameters
/// - `filters` - `MonsterFilters` struct containing search criteria
///
/// # Returns
/// List of `MonsterSummary` objects with basic monster information.
///
/// # Errors
/// Returns an error string if the database connection or query fails.
#[tauri::command]
pub async fn search_monsters(
    filters: MonsterFilters,
    state: State<'_, AppState>,
) -> Result<Vec<MonsterSummary>, String> {
    debug!("Searching monsters with filters: {:?}", filters);

    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut service = MonsterService::new(&mut conn);
    service
        .search_monsters(filters)
        .map_err(|e| format!("Database query failed: {}", e))
}

/// Get complete monster details by name and source.
///
/// Retrieves the full monster stat block including abilities, actions,
/// legendary actions, lair actions, and all other properties.
///
/// # Parameters
/// - `monster_name` - Exact monster name (case-sensitive)
/// - `monster_source` - Source book abbreviation (e.g., "MM", "VGM")
///
/// # Returns
/// The complete `Monster` object if found, or `None` if no match exists.
///
/// # Errors
/// Returns an error string if the database connection or query fails.
#[tauri::command]
pub async fn get_monster_details(
    monster_name: String,
    monster_source: String,
    state: State<'_, AppState>,
) -> Result<Option<Monster>, String> {
    debug!(
        "Getting monster details for name: {}, source: {}",
        monster_name, monster_source
    );

    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut service = MonsterService::new(&mut conn);
    service
        .get_monster_by_name_and_source(&monster_name, &monster_source)
        .map_err(|e| format!("Database query failed: {}", e))
}

/// Get all unique monster sizes in the catalog.
///
/// Returns size categories present in the monster catalog.
/// Used to populate filter dropdowns in the UI.
///
/// # Returns
/// List of size names (e.g., `["Tiny", "Small", "Medium", "Large", "Huge", "Gargantuan"]`).
///
/// # Errors
/// Returns an error string if the database connection or query fails.
#[tauri::command]
pub async fn get_monster_sizes(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    debug!("Getting all monster sizes");

    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut service = MonsterService::new(&mut conn);
    service
        .get_all_sizes()
        .map_err(|e| format!("Database query failed: {}", e))
}

/// Get all unique creature types in the monster catalog.
///
/// Returns creature type categories present in the catalog.
/// Used to populate filter dropdowns in the UI.
///
/// # Returns
/// List of type names (e.g., `["Aberration", "Beast", "Dragon", "Undead"]`).
///
/// # Errors
/// Returns an error string if the database connection or query fails.
#[tauri::command]
pub async fn get_monster_types(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    debug!("Getting all monster creature types");

    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut service = MonsterService::new(&mut conn);
    service
        .get_all_creature_types()
        .map_err(|e| format!("Database query failed: {}", e))
}

/// Get all unique alignments in the monster catalog.
///
/// Returns alignment values present in the catalog.
/// Used to populate filter dropdowns in the UI.
///
/// # Returns
/// List of alignment strings (e.g., `["lawful good", "chaotic evil", "unaligned"]`).
///
/// # Errors
/// Returns an error string if the database connection or query fails.
#[tauri::command]
pub async fn get_monster_alignments(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    debug!("Getting all monster alignments");

    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut service = MonsterService::new(&mut conn);
    service
        .get_all_alignments()
        .map_err(|e| format!("Database query failed: {}", e))
}

/// Get the minimum and maximum CR values in the monster catalog.
///
/// Returns the CR range for use in filter sliders.
///
/// # Returns
/// Tuple of (min_cr, max_cr) as floating point values.
///
/// # Errors
/// Returns an error string if the database connection or query fails.
#[tauri::command]
pub async fn get_monster_cr_range(state: State<'_, AppState>) -> Result<(f64, f64), String> {
    debug!("Getting monster CR range");

    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut service = MonsterService::new(&mut conn);
    service
        .get_cr_range()
        .map_err(|e| format!("Database query failed: {}", e))
}

/// Get monster count statistics grouped by source book.
///
/// Returns the number of monsters from each source book.
/// Useful for displaying catalog coverage statistics.
///
/// # Returns
/// List of tuples containing (source_abbreviation, monster_count).
///
/// # Errors
/// Returns an error string if the database connection or query fails.
#[tauri::command]
pub async fn get_monster_statistics(
    state: State<'_, AppState>,
) -> Result<Vec<(String, i64)>, String> {
    debug!("Getting monster statistics");

    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut service = MonsterService::new(&mut conn);
    service
        .get_monster_count_by_source()
        .map_err(|e| format!("Database query failed: {}", e))
}
