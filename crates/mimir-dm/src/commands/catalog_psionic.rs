use std::sync::Arc;
use tauri::State;
use mimir_dm_core::services::PsionicService;
use mimir_dm_core::DatabaseService;
use mimir_dm_core::models::catalog::{PsionicFilters};

#[tauri::command]
pub async fn search_psionics(
    query: Option<String>,
    psionic_types: Option<Vec<String>>, // "D", "T"  
    orders: Option<Vec<String>>, // Avatar, Awakened, etc.
    sources: Option<Vec<String>>,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<Vec<serde_json::Value>, String> {
    let mut conn = db_service.get_connection()
        .map_err(|e| format!("Database connection failed: {}", e))?;

    let filters = PsionicFilters {
        name: query,
        psionic_types,
        orders,
        sources,
    };

    let psionics = PsionicService::search_psionics(&mut conn, filters)?;
    
    // Convert to JSON for frontend
    let json_psionics: Vec<serde_json::Value> = psionics
        .into_iter()
        .map(|p| serde_json::to_value(p).unwrap_or_default())
        .collect();
    
    Ok(json_psionics)
}

#[tauri::command]
pub async fn get_psionic_details(
    name: String,
    source: String,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<Option<serde_json::Value>, String> {
    let mut conn = db_service.get_connection()
        .map_err(|e| format!("Database connection failed: {}", e))?;

    let psionic = PsionicService::get_psionic_by_name_and_source(&mut conn, &name, &source)?;
    
    Ok(psionic.map(|p| serde_json::to_value(p).unwrap_or_default()))
}

#[tauri::command]
pub async fn get_psionic_types(
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<Vec<String>, String> {
    let mut conn = db_service.get_connection()
        .map_err(|e| format!("Database connection failed: {}", e))?;

    PsionicService::get_all_psionic_types(&mut conn)
}

#[tauri::command]
pub async fn get_psionic_orders(
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<Vec<String>, String> {
    let mut conn = db_service.get_connection()
        .map_err(|e| format!("Database connection failed: {}", e))?;

    PsionicService::get_all_psionic_orders(&mut conn)
}

#[tauri::command]
pub async fn get_psionic_sources(
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<Vec<String>, String> {
    let mut conn = db_service.get_connection()
        .map_err(|e| format!("Database connection failed: {}", e))?;

    PsionicService::get_all_psionic_sources(&mut conn)
}