use crate::state::AppState;
use mimir_dm_core::models::catalog::monster::{Monster, MonsterFilters, MonsterSummary};
use mimir_dm_core::services::MonsterService;
use tauri::State;
use tracing::{debug, error};

/// Search monsters from database with filters
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
    service.search_monsters(filters)
        .map_err(|e| format!("Database query failed: {}", e))
}

/// Get monster details by name and source
#[tauri::command]
pub async fn get_monster_details(
    monster_name: String,
    monster_source: String,
    state: State<'_, AppState>,
) -> Result<Option<Monster>, String> {
    debug!("Getting monster details for name: {}, source: {}", monster_name, monster_source);

    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut service = MonsterService::new(&mut conn);
    service.get_monster_by_name_and_source(&monster_name, &monster_source)
        .map_err(|e| format!("Database query failed: {}", e))
}

/// Get all monster sizes for filter dropdowns
#[tauri::command]
pub async fn get_monster_sizes(
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    debug!("Getting all monster sizes");

    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut service = MonsterService::new(&mut conn);
    service.get_all_sizes()
        .map_err(|e| format!("Database query failed: {}", e))
}

/// Get all monster creature types for filter dropdowns
#[tauri::command]
pub async fn get_monster_types(
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    debug!("Getting all monster creature types");

    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut service = MonsterService::new(&mut conn);
    service.get_all_creature_types()
        .map_err(|e| format!("Database query failed: {}", e))
}

/// Get all monster alignments for filter dropdowns
#[tauri::command]
pub async fn get_monster_alignments(
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    debug!("Getting all monster alignments");

    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut service = MonsterService::new(&mut conn);
    service.get_all_alignments()
        .map_err(|e| format!("Database query failed: {}", e))
}

/// Get monster CR range for filter sliders
#[tauri::command]
pub async fn get_monster_cr_range(
    state: State<'_, AppState>,
) -> Result<(f64, f64), String> {
    debug!("Getting monster CR range");

    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut service = MonsterService::new(&mut conn);
    service.get_cr_range()
        .map_err(|e| format!("Database query failed: {}", e))
}

/// Get monster count by source for statistics
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
    service.get_monster_count_by_source()
        .map_err(|e| format!("Database query failed: {}", e))
}