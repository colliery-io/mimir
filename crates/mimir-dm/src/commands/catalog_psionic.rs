use crate::state::AppState;
use mimir_dm_core::models::catalog::PsionicFilters;
use mimir_dm_core::services::PsionicService;
use tauri::State;

#[tauri::command]
pub async fn search_psionics(
    query: Option<String>,
    psionic_types: Option<Vec<String>>, // "D", "T"
    orders: Option<Vec<String>>, // Avatar, Awakened, etc.
    sources: Option<Vec<String>>,
    state: State<'_, AppState>,
) -> Result<Vec<serde_json::Value>, String> {
    let mut conn = state.db.get_connection()
        .map_err(|e| format!("Database connection failed: {}", e))?;

    let filters = PsionicFilters {
        name: query,
        psionic_types,
        orders,
        sources,
    };

    let psionics = PsionicService::search_psionics(&mut conn, filters)
        .map_err(|e| format!("Failed to search psionics: {}", e))?;
    
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
    state: State<'_, AppState>,
) -> Result<Option<serde_json::Value>, String> {
    let mut conn = state.db.get_connection()
        .map_err(|e| format!("Database connection failed: {}", e))?;

    let psionic = PsionicService::get_psionic_by_name_and_source(&mut conn, &name, &source)
        .map_err(|e| format!("Failed to get psionic details: {}", e))?;
    
    Ok(psionic.map(|p| serde_json::to_value(p).unwrap_or_default()))
}

#[tauri::command]
pub async fn get_psionic_types(
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    let mut conn = state.db.get_connection()
        .map_err(|e| format!("Database connection failed: {}", e))?;

    PsionicService::get_all_psionic_types(&mut conn)
        .map_err(|e| format!("Failed to get psionic types: {}", e))
}

#[tauri::command]
pub async fn get_psionic_orders(
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    let mut conn = state.db.get_connection()
        .map_err(|e| format!("Database connection failed: {}", e))?;

    PsionicService::get_all_psionic_orders(&mut conn)
        .map_err(|e| format!("Failed to get psionic orders: {}", e))
}

#[tauri::command]
pub async fn get_psionic_sources(
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    let mut conn = state.db.get_connection()
        .map_err(|e| format!("Database connection failed: {}", e))?;

    PsionicService::get_all_psionic_sources(&mut conn)
        .map_err(|e| format!("Failed to get psionic sources: {}", e))
}