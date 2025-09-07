use crate::services::database::DatabaseService;
use mimir_dm_core::models::catalog::{CatalogTrap, TrapFilters, TrapSummary};
use mimir_dm_core::services::TrapService;
use std::sync::Arc;
use tauri::State;
use tracing::{debug, error};

#[tauri::command]
pub async fn search_traps(
    search: Option<String>,
    sources: Option<Vec<String>>,
    categories: Option<Vec<String>>,
    trap_types: Option<Vec<String>>,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<Vec<TrapSummary>, String> {
    debug!("search_traps called with search: {:?}", search);
    
    let filters = TrapFilters {
        search,
        sources,
        categories,
        trap_types,
    };
    
    let mut conn = db_service.get_connection()
        .map_err(|e| {
            error!("Failed to get database connection: {}", e);
            format!("Database connection error: {}", e)
        })?;
    
    let service = TrapService;
    service.search_traps(&mut conn, filters)
        .map_err(|e| {
            error!("Failed to search traps: {}", e);
            format!("Search error: {}", e)
        })
}

#[tauri::command]
pub async fn get_trap_details(
    name: String,
    source: String,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<Option<CatalogTrap>, String> {
    debug!("get_trap_details called for: {} from {}", name, source);
    
    let mut conn = db_service.get_connection()
        .map_err(|e| {
            error!("Failed to get database connection: {}", e);
            format!("Database connection error: {}", e)
        })?;
    
    let service = TrapService;
    service.get_trap_details(&mut conn, name, source)
        .map_err(|e| {
            error!("Failed to get trap details: {}", e);
            format!("Database error: {}", e)
        })
}

#[tauri::command]
pub async fn get_trap_sources(
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<Vec<String>, String> {
    debug!("get_trap_sources called");
    
    let mut conn = db_service.get_connection()
        .map_err(|e| {
            error!("Failed to get database connection: {}", e);
            format!("Database connection error: {}", e)
        })?;
    
    let service = TrapService;
    service.get_trap_sources(&mut conn)
        .map_err(|e| {
            error!("Failed to get trap sources: {}", e);
            format!("Database error: {}", e)
        })
}

#[tauri::command]
pub async fn get_trap_count(
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<i64, String> {
    debug!("get_trap_count called");
    
    let mut conn = db_service.get_connection()
        .map_err(|e| {
            error!("Failed to get database connection: {}", e);
            format!("Database connection error: {}", e)
        })?;
    
    let service = TrapService;
    service.get_trap_count(&mut conn)
        .map_err(|e| {
            error!("Failed to get trap count: {}", e);
            format!("Database error: {}", e)
        })
}

#[tauri::command]
pub async fn get_trap_types(
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<Vec<String>, String> {
    debug!("get_trap_types called");
    
    let mut conn = db_service.get_connection()
        .map_err(|e| {
            error!("Failed to get database connection: {}", e);
            format!("Database connection error: {}", e)
        })?;
    
    let service = TrapService;
    service.get_trap_types(&mut conn)
        .map_err(|e| {
            error!("Failed to get trap types: {}", e);
            format!("Database error: {}", e)
        })
}

#[tauri::command]
pub async fn get_trap_categories(
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<Vec<String>, String> {
    debug!("get_trap_categories called");
    
    let mut conn = db_service.get_connection()
        .map_err(|e| {
            error!("Failed to get database connection: {}", e);
            format!("Database connection error: {}", e)
        })?;
    
    let service = TrapService;
    service.get_trap_categories(&mut conn)
        .map_err(|e| {
            error!("Failed to get trap categories: {}", e);
            format!("Database error: {}", e)
        })
}