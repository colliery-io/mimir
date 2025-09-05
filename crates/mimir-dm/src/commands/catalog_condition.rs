//! Database-backed condition catalog commands

use mimir_dm_core::models::catalog::{ConditionOrDisease, ConditionFilters, ConditionSummary};
use mimir_dm_core::services::ConditionService;
use tracing::{debug, error, info};

/// Search conditions and diseases using database with filters
#[tauri::command]
pub async fn search_conditions(
    name: Option<String>,
    search: Option<String>,
    item_types: Option<Vec<String>>,
    sources: Option<Vec<String>>,
) -> Result<Vec<ConditionSummary>, String> {
    debug!("Database condition search - name: {:?}, search: {:?}, item_types: {:?}, sources: {:?}", 
           name, search, item_types, sources);
    
    let filters = ConditionFilters {
        name,
        search,
        item_types,
        sources,
    };
    
    match crate::db_connection::get_connection() {
        Ok(mut conn) => {
            match ConditionService::search_conditions(&mut conn, filters) {
                Ok(conditions) => {
                    info!("Found {} conditions in database search", conditions.len());
                    Ok(conditions)
                }
                Err(e) => {
                    error!("Database condition search failed: {}", e);
                    Err(format!("Failed to search conditions: {}", e))
                }
            }
        }
        Err(e) => {
            error!("Database connection error during condition search: {}", e);
            Err("Database connection failed".to_string())
        }
    }
}

/// Get a specific condition or disease by ID for modal display
#[tauri::command]
pub async fn get_condition(condition_id: i32) -> Result<Option<ConditionOrDisease>, String> {
    debug!("Getting condition by ID: {}", condition_id);
    
    match crate::db_connection::get_connection() {
        Ok(mut conn) => {
            match ConditionService::get_condition_by_id(&mut conn, condition_id) {
                Ok(condition) => Ok(condition),
                Err(e) => {
                    error!("Failed to get condition: {}", e);
                    Err(format!("Failed to get condition: {}", e))
                }
            }
        }
        Err(e) => {
            error!("Database connection error getting condition: {}", e);
            Err("Database connection failed".to_string())
        }
    }
}

/// Get all available item types for filter dropdowns
#[tauri::command]
pub async fn get_condition_item_types() -> Result<Vec<String>, String> {
    debug!("Getting condition item types");
    
    match crate::db_connection::get_connection() {
        Ok(mut conn) => {
            match ConditionService::get_item_types(&mut conn) {
                Ok(item_types) => Ok(item_types),
                Err(e) => {
                    error!("Failed to get item types: {}", e);
                    Err(format!("Failed to get item types: {}", e))
                }
            }
        }
        Err(e) => {
            error!("Database connection error getting item types: {}", e);
            Err("Database connection failed".to_string())
        }
    }
}

/// Get all available sources for filter dropdowns
#[tauri::command]
pub async fn get_condition_sources() -> Result<Vec<String>, String> {
    debug!("Getting condition sources");
    
    match crate::db_connection::get_connection() {
        Ok(mut conn) => {
            match ConditionService::get_sources(&mut conn) {
                Ok(sources) => Ok(sources),
                Err(e) => {
                    error!("Failed to get sources: {}", e);
                    Err(format!("Failed to get sources: {}", e))
                }
            }
        }
        Err(e) => {
            error!("Database connection error getting sources: {}", e);
            Err("Database connection failed".to_string())
        }
    }
}

/// Get condition count for statistics
#[tauri::command]
pub async fn get_condition_count() -> Result<i64, String> {
    debug!("Getting condition count");
    
    match crate::db_connection::get_connection() {
        Ok(mut conn) => {
            match ConditionService::get_condition_count(&mut conn) {
                Ok(count) => Ok(count),
                Err(e) => {
                    error!("Failed to get condition count: {}", e);
                    Err(format!("Failed to get condition count: {}", e))
                }
            }
        }
        Err(e) => {
            error!("Database connection error getting condition count: {}", e);
            Err("Database connection failed".to_string())
        }
    }
}