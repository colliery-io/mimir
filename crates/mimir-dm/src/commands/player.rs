//! Player management commands

use tauri::State;
use mimir_dm_core::models::player::Player;
use mimir_dm_core::services::PlayerService;
use mimir_dm_core::DatabaseService;
use std::sync::Arc;
use tracing::error;

/// Create a new player
#[tauri::command]
pub async fn create_player(
    name: String,
    email: Option<String>,
    notes: Option<String>,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<Player, String> {
    let mut conn = db_service.get_connection().map_err(|e| {
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
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<Player, String> {
    let mut conn = db_service.get_connection().map_err(|e| {
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
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<Vec<Player>, String> {
    let mut conn = db_service.get_connection().map_err(|e| {
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
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<Player, String> {
    let mut conn = db_service.get_connection().map_err(|e| {
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
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<(), String> {
    let mut conn = db_service.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut player_service = PlayerService::new(&mut conn);
    player_service.delete_player(player_id)
        .map_err(|e| format!("Failed to delete player: {}", e))
}
