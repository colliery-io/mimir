//! Database-backed language catalog commands

use crate::state::AppState;
use mimir_dm_core::models::catalog::{Language, LanguageFilters, LanguageSummary};
use mimir_dm_core::services::LanguageService;
use tauri::State;
use tracing::{debug, error, info};

/// Search languages using database with filters
#[tauri::command]
pub async fn search_languages(
    state: State<'_, AppState>,
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

    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Database connection error during language search: {}", e);
        format!("Database connection failed: {}", e)
    })?;

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

/// Get a specific language by name and source for modal display
#[tauri::command]
pub async fn get_language_details(
    state: State<'_, AppState>,
    name: String,
    source: String,
) -> Result<Language, String> {
    debug!("Getting language details for '{}' from '{}'", name, source);

    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Database connection error getting language: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    match LanguageService::get_language_by_name_and_source(&mut conn, &name, &source) {
        Ok(Some(language)) => Ok(language),
        Ok(None) => Err(format!("Language not found: {} from {}", name, source)),
        Err(e) => {
            error!("Failed to get language: {}", e);
            Err(format!("Failed to get language: {}", e))
        }
    }
}

/// Get all available language types for filter dropdowns
#[tauri::command]
pub async fn get_language_types(
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    debug!("Getting language types");

    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Database connection error getting language types: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    match LanguageService::get_language_types(&mut conn) {
        Ok(types) => Ok(types),
        Err(e) => {
            error!("Failed to get language types: {}", e);
            Err(format!("Failed to get language types: {}", e))
        }
    }
}

/// Get all available scripts for filter dropdowns
#[tauri::command]
pub async fn get_language_scripts(
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    debug!("Getting language scripts");

    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Database connection error getting scripts: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    match LanguageService::get_scripts(&mut conn) {
        Ok(scripts) => Ok(scripts),
        Err(e) => {
            error!("Failed to get scripts: {}", e);
            Err(format!("Failed to get scripts: {}", e))
        }
    }
}

/// Get all available sources for filter dropdowns
#[tauri::command]
pub async fn get_language_sources(
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    debug!("Getting language sources");

    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Database connection error getting sources: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    match LanguageService::get_sources(&mut conn) {
        Ok(sources) => Ok(sources),
        Err(e) => {
            error!("Failed to get sources: {}", e);
            Err(format!("Failed to get sources: {}", e))
        }
    }
}

/// Get language count for statistics
#[tauri::command]
pub async fn get_language_count(
    state: State<'_, AppState>,
) -> Result<i64, String> {
    debug!("Getting language count");

    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Database connection error getting language count: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    match LanguageService::get_language_count(&mut conn) {
        Ok(count) => Ok(count),
        Err(e) => {
            error!("Failed to get language count: {}", e);
            Err(format!("Failed to get language count: {}", e))
        }
    }
}