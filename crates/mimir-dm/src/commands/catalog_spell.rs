//! Database-backed spell catalog commands

use mimir_dm_core::models::catalog::{Spell, SpellFilters, SpellSummary};
use mimir_dm_core::services::SpellService;
use tracing::{debug, error, info};

/// Search spells using database with filters
#[tauri::command]
pub async fn search_spells(
    query: Option<String>,
    sources: Option<Vec<String>>,
    levels: Option<Vec<i32>>,
    schools: Option<Vec<String>>,
    tags: Option<Vec<String>>,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<Vec<SpellSummary>, String> {
    debug!("Database spell search - query: {:?}, sources: {:?}, levels: {:?}", 
           query, sources, levels);
    
    let filters = SpellFilters {
        query,
        levels: levels.unwrap_or_default(),
        schools: schools.unwrap_or_default(),
        sources: sources.unwrap_or_default(),
        tags: tags.unwrap_or_default(),
        limit,
        offset,
    };
    
    match crate::db_connection::get_connection() {
        Ok(mut conn) => {
            match SpellService::search_spells(&mut conn, filters) {
                Ok(spells) => {
                    info!("Found {} spells in database search", spells.len());
                    Ok(spells)
                }
                Err(e) => {
                    error!("Database spell search failed: {}", e);
                    Err(format!("Failed to search spells: {}", e))
                }
            }
        }
        Err(e) => {
            error!("Database connection error during spell search: {}", e);
            Err("Database connection failed".to_string())
        }
    }
}

/// Get detailed spell information by name and source
#[tauri::command]
pub async fn get_spell_details(
    name: String,
    source: String,
) -> Result<Option<Spell>, String> {
    debug!("Getting spell details from database: {} from {}", name, source);
    
    match crate::db_connection::get_connection() {
        Ok(mut conn) => {
            match SpellService::get_spell_details(&mut conn, &name, &source) {
                Ok(spell) => {
                    if spell.is_some() {
                        debug!("Found spell details for: {}", name);
                    } else {
                        debug!("Spell not found: {} from {}", name, source);
                    }
                    Ok(spell)
                }
                Err(e) => {
                    error!("Failed to get spell details: {}", e);
                    Err(format!("Failed to get spell details: {}", e))
                }
            }
        }
        Err(e) => {
            error!("Database connection error during spell details fetch: {}", e);
            Err("Database connection failed".to_string())
        }
    }
}

/// Get available spell sources for filter dropdown
#[tauri::command]
pub async fn get_spell_sources() -> Result<Vec<String>, String> {
    debug!("Getting spell sources from database");
    
    match crate::db_connection::get_connection() {
        Ok(mut conn) => {
            match SpellService::get_spell_sources(&mut conn) {
                Ok(sources) => {
                    info!("Found {} spell sources in database", sources.len());
                    Ok(sources)
                }
                Err(e) => {
                    error!("Failed to get spell sources: {}", e);
                    Err(format!("Failed to get spell sources: {}", e))
                }
            }
        }
        Err(e) => {
            error!("Database connection error during spell sources fetch: {}", e);
            Err("Database connection failed".to_string())
        }
    }
}

/// Get available spell schools for filter dropdown
#[tauri::command]
pub async fn get_spell_schools() -> Result<Vec<String>, String> {
    debug!("Getting spell schools from database");
    
    match crate::db_connection::get_connection() {
        Ok(mut conn) => {
            match SpellService::get_spell_schools(&mut conn) {
                Ok(schools) => {
                    info!("Found {} spell schools in database", schools.len());
                    Ok(schools)
                }
                Err(e) => {
                    error!("Failed to get spell schools: {}", e);
                    Err(format!("Failed to get spell schools: {}", e))
                }
            }
        }
        Err(e) => {
            error!("Database connection error during spell schools fetch: {}", e);
            Err("Database connection failed".to_string())
        }
    }
}

/// Get spell statistics by source
#[tauri::command]
pub async fn get_spell_statistics() -> Result<Vec<(String, i64)>, String> {
    debug!("Getting spell statistics from database");
    
    match crate::db_connection::get_connection() {
        Ok(mut conn) => {
            match SpellService::get_spell_count_by_source(&mut conn) {
                Ok(stats) => {
                    info!("Retrieved spell statistics for {} sources", stats.len());
                    Ok(stats)
                }
                Err(e) => {
                    error!("Failed to get spell statistics: {}", e);
                    Err(format!("Failed to get spell statistics: {}", e))
                }
            }
        }
        Err(e) => {
            error!("Database connection error during spell statistics fetch: {}", e);
            Err("Database connection failed".to_string())
        }
    }
}

/// Get total number of spells in the database
#[tauri::command]
pub async fn get_spell_count() -> Result<i64, String> {
    debug!("Getting total spell count from database");
    
    match crate::db_connection::get_connection() {
        Ok(mut conn) => {
            match SpellService::get_total_spell_count(&mut conn) {
                Ok(count) => {
                    info!("Total spells in database: {}", count);
                    Ok(count)
                }
                Err(e) => {
                    error!("Failed to get spell count: {}", e);
                    Err(format!("Failed to get spell count: {}", e))
                }
            }
        }
        Err(e) => {
            error!("Database connection error during spell count fetch: {}", e);
            Err("Database connection failed".to_string())
        }
    }
}