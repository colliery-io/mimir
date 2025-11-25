//! Database-backed table catalog commands

use crate::state::AppState;
use mimir_dm_core::models::catalog::table::{Table, TableFilters, TableSummary};
use mimir_dm_core::services::TableService;
use tauri::State;
use tracing::{debug, info};

/// Search tables using database with filters
#[tauri::command]
pub async fn search_tables(
    query: Option<String>,
    categories: Option<Vec<String>>,
    sources: Option<Vec<String>>,
    state: State<'_, AppState>,
) -> Result<Vec<TableSummary>, String> {
    debug!("Database table search - query: {:?}, categories: {:?}, sources: {:?}",
           query, categories, sources);

    let mut conn = state.db.get_connection()
        .map_err(|e| format!("Database connection failed: {}", e))?;

    let mut service = TableService::new(&mut conn);

    let filters = TableFilters {
        name: query,
        categories,
        sources,
    };

    let tables = service.search_tables(filters)
        .map_err(|e| format!("Failed to search tables: {}", e))?;

    info!("Found {} tables in database search", tables.len());
    Ok(tables)
}

/// Get table by ID
#[tauri::command]
pub async fn get_table(
    id: i32,
    state: State<'_, AppState>,
) -> Result<Option<Table>, String> {
    debug!("Getting table by ID: {}", id);

    let mut conn = state.db.get_connection()
        .map_err(|e| format!("Database connection failed: {}", e))?;

    let mut service = TableService::new(&mut conn);

    service.get_table_by_id(id)
        .map_err(|e| format!("Failed to get table: {}", e))
}

/// Get detailed table information by name and source
#[tauri::command]
pub async fn get_table_details(
    name: String,
    source: String,
    state: State<'_, AppState>,
) -> Result<Option<Table>, String> {
    debug!("Getting table details: {} from {}", name, source);

    let mut conn = state.db.get_connection()
        .map_err(|e| format!("Database connection failed: {}", e))?;

    let mut service = TableService::new(&mut conn);

    service.get_table_by_name_and_source(&name, &source)
        .map_err(|e| format!("Failed to get table details: {}", e))
}

/// Get available table categories for filter dropdown
#[tauri::command]
pub async fn get_table_categories(
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    debug!("Getting table categories from database");

    let mut conn = state.db.get_connection()
        .map_err(|e| format!("Database connection failed: {}", e))?;

    let mut service = TableService::new(&mut conn);

    let categories = service.get_table_categories()
        .map_err(|e| format!("Failed to get table categories: {}", e))?;

    info!("Found {} table categories in database", categories.len());
    Ok(categories)
}

/// Get available table sources for filter dropdown
#[tauri::command]
pub async fn get_table_sources(
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    debug!("Getting table sources from database");

    let mut conn = state.db.get_connection()
        .map_err(|e| format!("Database connection failed: {}", e))?;

    let mut service = TableService::new(&mut conn);

    let sources = service.get_table_sources()
        .map_err(|e| format!("Failed to get table sources: {}", e))?;

    info!("Found {} table sources in database", sources.len());
    Ok(sources)
}
