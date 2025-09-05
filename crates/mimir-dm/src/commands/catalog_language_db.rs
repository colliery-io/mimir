//! Database-backed language catalog commands

use mimir_dm_core::models::catalog::{Language, LanguageFilters, LanguageSummary};
use mimir_dm_core::services::LanguageService;
use tracing::{debug, error, info};

/// Search languages using database with filters
#[tauri::command]
pub async fn search_languages(
    query: Option<String>,
    language_types: Option<Vec<String>>,
    scripts: Option<Vec<String>>,
    sources: Option<Vec<String>>,
) -> Result<Vec<LanguageSummary>, String> {
    debug!("Database language search - query: {:?}, types: {:?}, scripts: {:?}, sources: {:?}", 
           query, language_types, scripts, sources);
    
    let filters = LanguageFilters {
        name: None,
        search: query,
        language_types,
        scripts,
        sources,
    };
    
    match crate::db_connection::get_connection() {
        Ok(mut conn) => {
            match LanguageService::search_languages(&mut conn, filters) {
                Ok(languages) => {
                    info!("Found {} languages in database search", languages.len());
                    Ok(languages)
                }
                Err(e) => {
                    error!("Database language search failed: {}", e);
                    Err(format!("Failed to search languages: {}", e))
                }
            }
        }
        Err(e) => {
            error!("Database connection error during language search: {}", e);
            Err("Database connection failed".to_string())
        }
    }
}

/// Get a specific language by name and source for modal display
#[tauri::command]
pub async fn get_language_details(
    name: String,
    source: String,
) -> Result<Language, String> {
    debug!("Getting language details for '{}' from '{}'", name, source);
    
    match crate::db_connection::get_connection() {
        Ok(mut conn) => {
            match LanguageService::get_language_by_name_and_source(&mut conn, &name, &source) {
                Ok(Some(language)) => Ok(language),
                Ok(None) => Err(format!("Language not found: {} from {}", name, source)),
                Err(e) => {
                    error!("Failed to get language: {}", e);
                    Err(format!("Failed to get language: {}", e))
                }
            }
        }
        Err(e) => {
            error!("Database connection error getting language: {}", e);
            Err("Database connection failed".to_string())
        }
    }
}

/// Get all available language types for filter dropdowns
#[tauri::command]
pub async fn get_language_types() -> Result<Vec<String>, String> {
    debug!("Getting language types");
    
    match crate::db_connection::get_connection() {
        Ok(mut conn) => {
            match LanguageService::get_language_types(&mut conn) {
                Ok(types) => Ok(types),
                Err(e) => {
                    error!("Failed to get language types: {}", e);
                    Err(format!("Failed to get language types: {}", e))
                }
            }
        }
        Err(e) => {
            error!("Database connection error getting language types: {}", e);
            Err("Database connection failed".to_string())
        }
    }
}

/// Get all available scripts for filter dropdowns
#[tauri::command]
pub async fn get_language_scripts() -> Result<Vec<String>, String> {
    debug!("Getting language scripts");
    
    match crate::db_connection::get_connection() {
        Ok(mut conn) => {
            match LanguageService::get_scripts(&mut conn) {
                Ok(scripts) => Ok(scripts),
                Err(e) => {
                    error!("Failed to get scripts: {}", e);
                    Err(format!("Failed to get scripts: {}", e))
                }
            }
        }
        Err(e) => {
            error!("Database connection error getting scripts: {}", e);
            Err("Database connection failed".to_string())
        }
    }
}

/// Get all available sources for filter dropdowns  
#[tauri::command]
pub async fn get_language_sources() -> Result<Vec<String>, String> {
    debug!("Getting language sources");
    
    match crate::db_connection::get_connection() {
        Ok(mut conn) => {
            match LanguageService::get_sources(&mut conn) {
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

/// Get language count for statistics
#[tauri::command]
pub async fn get_language_count() -> Result<i64, String> {
    debug!("Getting language count");
    
    match crate::db_connection::get_connection() {
        Ok(mut conn) => {
            match LanguageService::get_language_count(&mut conn) {
                Ok(count) => Ok(count),
                Err(e) => {
                    error!("Failed to get language count: {}", e);
                    Err(format!("Failed to get language count: {}", e))
                }
            }
        }
        Err(e) => {
            error!("Database connection error getting language count: {}", e);
            Err("Database connection failed".to_string())
        }
    }
}