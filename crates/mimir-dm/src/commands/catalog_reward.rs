//! Database-backed reward catalog commands

use crate::state::AppState;
use mimir_dm_core::models::catalog::{Reward, RewardFilters, RewardSummary};
use mimir_dm_core::services::RewardService;
use tauri::State;
use tracing::{debug, error, info};

/// Search rewards using database with filters
#[tauri::command]
pub async fn search_rewards(
    state: State<'_, AppState>,
    query: Option<String>,
    reward_types: Option<Vec<String>>,
    sources: Option<Vec<String>>,
    has_prerequisites: Option<bool>,
) -> Result<Vec<RewardSummary>, String> {
    debug!("Database reward search - query: {:?}, types: {:?}, sources: {:?}, prerequisites: {:?}",
           query, reward_types, sources, has_prerequisites);

    let filters = RewardFilters {
        name: None,
        search: query,
        reward_types,
        sources,
        has_prerequisites,
    };

    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Database connection error during reward search: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    match RewardService::search_rewards(&mut conn, filters) {
        Ok(rewards) => {
            info!("Found {} rewards in database search", rewards.len());
            Ok(rewards)
        }
        Err(e) => {
            error!("Database reward search failed: {}", e);
            Err(format!("Failed to search rewards: {}", e))
        }
    }
}

/// Get a specific reward by name and source for modal display
#[tauri::command]
pub async fn get_reward_details(
    state: State<'_, AppState>,
    name: String,
    source: String,
) -> Result<Reward, String> {
    debug!("Getting reward details for '{}' from '{}'", name, source);

    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Database connection error getting reward: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    match RewardService::get_reward_by_name_and_source(&mut conn, &name, &source) {
        Ok(Some(reward)) => Ok(reward),
        Ok(None) => Err(format!("Reward not found: {} from {}", name, source)),
        Err(e) => {
            error!("Failed to get reward: {}", e);
            Err(format!("Failed to get reward: {}", e))
        }
    }
}

/// Get all available reward types for filter dropdowns
#[tauri::command]
pub async fn get_reward_types(
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    debug!("Getting reward types");

    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Database connection error getting reward types: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    match RewardService::get_reward_types(&mut conn) {
        Ok(types) => Ok(types),
        Err(e) => {
            error!("Failed to get reward types: {}", e);
            Err(format!("Failed to get reward types: {}", e))
        }
    }
}

/// Get all available sources for filter dropdowns
#[tauri::command]
pub async fn get_reward_sources(
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    debug!("Getting reward sources");

    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Database connection error getting sources: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    match RewardService::get_sources(&mut conn) {
        Ok(sources) => Ok(sources),
        Err(e) => {
            error!("Failed to get sources: {}", e);
            Err(format!("Failed to get sources: {}", e))
        }
    }
}

/// Get reward count for statistics
#[tauri::command]
pub async fn get_reward_count(
    state: State<'_, AppState>,
) -> Result<i64, String> {
    debug!("Getting reward count");

    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Database connection error getting reward count: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    match RewardService::get_reward_count(&mut conn) {
        Ok(count) => Ok(count),
        Err(e) => {
            error!("Failed to get reward count: {}", e);
            Err(format!("Failed to get reward count: {}", e))
        }
    }
}