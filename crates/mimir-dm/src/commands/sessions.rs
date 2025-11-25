//! Session-related Tauri commands

use crate::state::AppState;
use crate::types::ApiResponse;
use mimir_dm_core::domain::BoardRegistry;
use mimir_dm_core::services::SessionService;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSessionRequest {
    pub module_id: i32,
    pub campaign_id: i32,
    pub campaign_directory: String,
    pub module_number: i32,
}

#[derive(Debug, Serialize)]
pub struct SessionResponse {
    pub data: mimir_dm_core::models::campaign::sessions::Session,
}

/// Create a new session for a module
#[tauri::command]
pub async fn create_session(
    request: CreateSessionRequest,
    state: State<'_, AppState>,
) -> Result<SessionResponse, String> {
    let mut conn = state.db.get_connection()
        .map_err(|e| format!("Database connection failed: {}", e))?;
    
    let session = SessionService::create_session(
        &mut conn,
        request.module_id,
        request.campaign_id,
        &request.campaign_directory,
        request.module_number,
    ).map_err(|e| format!("Failed to create session: {}", e))?;
    
    Ok(SessionResponse { data: session })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListSessionsRequest {
    pub module_id: i32,
}

#[derive(Debug, Serialize)]
pub struct SessionListResponse {
    pub data: Vec<mimir_dm_core::models::campaign::sessions::Session>,
}

/// List all sessions for a module
#[tauri::command]
pub async fn list_module_sessions(
    request: ListSessionsRequest,
    state: State<'_, AppState>,
) -> Result<SessionListResponse, String> {
    use mimir_dm_core::dal::campaign::sessions::SessionRepository;

    let mut conn = state.db.get_connection()
        .map_err(|e| format!("Database connection failed: {}", e))?;
    let mut repo = SessionRepository::new(&mut conn);
    let sessions = repo.list_by_module(request.module_id)
        .map_err(|e| format!("Failed to list sessions: {}", e))?;
    
    Ok(SessionListResponse { data: sessions })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransitionSessionRequest {
    pub session_id: i32,
    pub new_status: String,
}

/// Transition a session to a new status
#[tauri::command]
pub async fn transition_session_status(
    request: TransitionSessionRequest,
    state: State<'_, AppState>,
) -> Result<SessionResponse, String> {
    use mimir_dm_core::dal::campaign::sessions::SessionRepository;

    let mut conn = state.db.get_connection()
        .map_err(|e| format!("Database connection failed: {}", e))?;
    let mut repo = SessionRepository::new(&mut conn);
    let session = repo.transition_status(request.session_id, &request.new_status)
        .map_err(|e| format!("Failed to transition session: {}", e))?;
    
    Ok(SessionResponse { data: session })
}

/// Get session board configuration
#[tauri::command]
pub async fn get_session_board_config() -> Result<ApiResponse<SessionBoardConfig>, String> {
    let board_registry = BoardRegistry::new();
    let session_board = board_registry.get("session")
        .ok_or_else(|| "Session board not found".to_string())?;
    
    let stages: Vec<SessionStageInfo> = session_board.stages()
        .into_iter()
        .map(|stage| {
            let metadata = session_board.stage_metadata(stage);
            let next_stage = session_board.next_stage(stage);
            
            SessionStageInfo {
                key: stage.to_string(),
                display_name: metadata.display_name,
                next_stage: next_stage.map(|s| s.to_string()),
                can_transition_to: session_board.stages()
                    .into_iter()
                    .filter(|&to| session_board.can_transition(stage, to))
                    .map(|s| s.to_string())
                    .collect(),
            }
        })
        .collect();
    
    Ok(ApiResponse::success(SessionBoardConfig {
        board_type: "session".to_string(),
        stages,
        initial_stage: session_board.stages().first().map(|s| s.to_string()),
    }))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionBoardConfig {
    pub board_type: String,
    pub stages: Vec<SessionStageInfo>,
    pub initial_stage: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionStageInfo {
    pub key: String,
    pub display_name: String,
    pub next_stage: Option<String>,
    pub can_transition_to: Vec<String>,
}