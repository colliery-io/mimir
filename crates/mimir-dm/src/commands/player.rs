//! Player management commands

use crate::state::AppState;
use mimir_dm_core::models::player::Player;
use mimir_dm_core::services::PlayerService;
use tauri::State;
use tracing::error;

/// Create a new player
#[tauri::command]
pub async fn create_player(
    name: String,
    email: Option<String>,
    notes: Option<String>,
    state: State<'_, AppState>,
) -> Result<Player, String> {
    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut player_service = PlayerService::new(&mut conn);
    player_service.create_player(&name, email, notes)
        .map_err(|e| format!("Failed to create player: {}", e))
}

/// Get player by ID
#[tauri::command]
pub async fn get_player(
    player_id: i32,
    state: State<'_, AppState>,
) -> Result<Player, String> {
    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut player_service = PlayerService::new(&mut conn);
    player_service.get_player(player_id)
        .map_err(|e| format!("Failed to get player: {}", e))
}

/// List all players
#[tauri::command]
pub async fn list_players(
    state: State<'_, AppState>,
) -> Result<Vec<Player>, String> {
    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut player_service = PlayerService::new(&mut conn);
    player_service.list_players()
        .map_err(|e| format!("Failed to list players: {}", e))
}

/// Update player
#[tauri::command]
pub async fn update_player(
    player_id: i32,
    name: Option<String>,
    email: Option<Option<String>>,
    notes: Option<Option<String>>,
    state: State<'_, AppState>,
) -> Result<Player, String> {
    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut player_service = PlayerService::new(&mut conn);
    player_service.update_player(player_id, name, email, notes)
        .map_err(|e| format!("Failed to update player: {}", e))
}

/// Delete player
#[tauri::command]
pub async fn delete_player(
    player_id: i32,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut player_service = PlayerService::new(&mut conn);
    player_service.delete_player(player_id)
        .map_err(|e| format!("Failed to delete player: {}", e))
}
