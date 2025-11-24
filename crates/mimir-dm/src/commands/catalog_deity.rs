use std::sync::Arc;
use tauri::State;
use tracing::{debug, error};

use mimir_dm_core::DatabaseService;
use mimir_dm_core::services::DeityService;
use mimir_dm_core::models::catalog::deity::{DeitySummary, DeityFilters, Deity};

/// Search deities from database with filters
#[tauri::command]
pub async fn search_deities(
    filters: DeityFilters,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<Vec<DeitySummary>, String> {
    debug!("Searching deities with filters: {:?}", filters);

    let mut conn = db_service.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut service = DeityService::new(&mut conn);
    service.search_deities(filters)
        .map_err(|e| format!("Database query failed: {}", e))
}

/// Get deity details by name and source
#[tauri::command]
pub async fn get_deity_details(
    deity_name: String,
    deity_source: String,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<Option<Deity>, String> {
    debug!("Getting deity details for name: {}, source: {}", deity_name, deity_source);

    let mut conn = db_service.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut service = DeityService::new(&mut conn);
    service.get_deity_by_name_and_source(&deity_name, &deity_source)
        .map_err(|e| format!("Database query failed: {}", e))
}

/// Get all pantheons for filter dropdowns
#[tauri::command]
pub async fn get_deity_pantheons(
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<Vec<String>, String> {
    debug!("Getting all deity pantheons");

    let mut conn = db_service.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut service = DeityService::new(&mut conn);
    service.get_all_pantheons()
        .map_err(|e| format!("Database query failed: {}", e))
}

/// Get all domains for filter dropdowns
#[tauri::command]
pub async fn get_deity_domains(
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<Vec<String>, String> {
    debug!("Getting all deity domains");

    let mut conn = db_service.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut service = DeityService::new(&mut conn);
    service.get_all_domains()
        .map_err(|e| format!("Database query failed: {}", e))
}

/// Get all alignments for filter dropdowns
#[tauri::command]
pub async fn get_deity_alignments(
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<Vec<String>, String> {
    debug!("Getting all deity alignments");

    let mut conn = db_service.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut service = DeityService::new(&mut conn);
    service.get_all_alignments()
        .map_err(|e| format!("Database query failed: {}", e))
}

/// Get deity count by source for statistics
#[tauri::command]
pub async fn get_deity_statistics(
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<Vec<(String, i64)>, String> {
    debug!("Getting deity statistics");

    let mut conn = db_service.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut service = DeityService::new(&mut conn);
    service.get_deity_count_by_source()
        .map_err(|e| format!("Database query failed: {}", e))
}