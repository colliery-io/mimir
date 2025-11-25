use crate::state::AppState;
use mimir_dm_core::models::catalog::cult::{CatalogCult, CultBoonSummary, CultFilters};
use mimir_dm_core::services::CultService;
use tauri::State;
use tracing::{debug, error};

#[tauri::command]
pub async fn search_cults(
    name: Option<String>,
    sources: Option<Vec<String>>,
    categories: Option<Vec<String>>,
    cult_types: Option<Vec<String>>,
    state: State<'_, AppState>,
) -> Result<Vec<CultBoonSummary>, String> {
    debug!("search_cults called with name: {:?}", name);

    let filters = CultFilters {
        name,
        source: sources,
        category: categories,
        cult_type: cult_types,
    };

    let mut conn = state.db.get_connection()
        .map_err(|e| {
            error!("Failed to get database connection: {}", e);
            format!("Database connection error: {}", e)
        })?;
    
    let service = CultService;
    service.search_cults(&mut conn, filters)
        .map_err(|e| {
            error!("Failed to search cults: {}", e);
            format!("Search error: {}", e)
        })
}

#[tauri::command]
pub async fn get_cult_details(
    name: String,
    source: String,
    state: State<'_, AppState>,
) -> Result<Option<CatalogCult>, String> {
    debug!("get_cult_details called for: {} from {}", name, source);

    let mut conn = state.db.get_connection()
        .map_err(|e| {
            error!("Failed to get database connection: {}", e);
            format!("Database connection error: {}", e)
        })?;
    
    let service = CultService;
    service.get_cult_details(&mut conn, name, source)
        .map_err(|e| {
            error!("Failed to get cult details: {}", e);
            format!("Database error: {}", e)
        })
}

#[tauri::command]
pub async fn get_cult_sources(
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    debug!("get_cult_sources called");

    let mut conn = state.db.get_connection()
        .map_err(|e| {
            error!("Failed to get database connection: {}", e);
            format!("Database connection error: {}", e)
        })?;
    
    let service = CultService;
    service.get_cult_sources(&mut conn)
        .map_err(|e| {
            error!("Failed to get cult sources: {}", e);
            format!("Database error: {}", e)
        })
}

#[tauri::command]
pub async fn get_cult_count(
    state: State<'_, AppState>,
) -> Result<i64, String> {
    debug!("get_cult_count called");

    let mut conn = state.db.get_connection()
        .map_err(|e| {
            error!("Failed to get database connection: {}", e);
            format!("Database connection error: {}", e)
        })?;
    
    let service = CultService;
    service.get_cult_count(&mut conn)
        .map_err(|e| {
            error!("Failed to get cult count: {}", e);
            format!("Database error: {}", e)
        })
}

#[tauri::command]
pub async fn get_cult_types(
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    debug!("get_cult_types called");

    let mut conn = state.db.get_connection()
        .map_err(|e| {
            error!("Failed to get database connection: {}", e);
            format!("Database connection error: {}", e)
        })?;
    
    let service = CultService;
    service.get_cult_types(&mut conn)
        .map_err(|e| {
            error!("Failed to get cult types: {}", e);
            format!("Database error: {}", e)
        })
}

#[tauri::command]
pub async fn get_cult_categories(
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    debug!("get_cult_categories called");

    let mut conn = state.db.get_connection()
        .map_err(|e| {
            error!("Failed to get database connection: {}", e);
            format!("Database connection error: {}", e)
        })?;
    
    let service = CultService;
    service.get_cult_categories(&mut conn)
        .map_err(|e| {
            error!("Failed to get cult categories: {}", e);
            format!("Database error: {}", e)
        })
}