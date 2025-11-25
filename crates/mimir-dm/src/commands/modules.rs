//! Module management commands

use crate::state::AppState;
use crate::types::{ApiError, ApiResponse};
use mimir_dm_core::{
    services::ModuleService,
    models::campaign::modules::{Module, UpdateModule},
    domain::BoardCompletionStatus,
};
use serde::{Deserialize, Serialize};
use tauri::State;
use tracing::{error, info};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateModuleRequest {
    pub campaign_id: i32,
    pub name: String,
    pub expected_sessions: i32,
    pub module_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateModuleRequest {
    pub name: Option<String>,
    pub expected_sessions: Option<i32>,
    pub actual_sessions: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransitionModuleRequest {
    pub module_id: i32,
    pub new_stage: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InitializeDocumentsRequest {
    pub module_id: i32,
    pub campaign_directory: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListModulesRequest {
    pub campaign_id: i32,
}

/// Create a new module
#[tauri::command]
pub async fn create_module(
    request: CreateModuleRequest,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Module>, ApiError> {
    info!("Creating module: {} for campaign {} with type: {:?}",
        request.name, request.campaign_id, request.module_type);

    let mut conn = state.db.get_connection()?;
    let mut service = ModuleService::new(&mut *conn);

    match service.create_module_with_documents(
        request.campaign_id,
        request.name,
        request.expected_sessions,
        request.module_type,
    ) {
        Ok(module) => {
            info!("Module created successfully with ID: {}", module.id);
            Ok(ApiResponse::success(module))
        }
        Err(e) => {
            error!("Failed to create module: {}", e);
            Ok(ApiResponse::error(format!("Failed to create module: {}", e)))
        }
    }
}

/// Get a module by ID
#[tauri::command]
pub async fn get_module(
    id: i32,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Module>, ApiError> {
    info!("Getting module with ID: {}", id);
    
    let mut conn = state.db.get_connection()?;
    let mut service = ModuleService::new(&mut *conn);
    
    match service.get_module(id) {
        Ok(Some(module)) => {
            info!("Module found: {}", module.name);
            Ok(ApiResponse::success(module))
        }
        Ok(None) => {
            info!("Module not found with ID: {}", id);
            Ok(ApiResponse::error(format!("Module not found with ID: {}", id)))
        }
        Err(e) => {
            error!("Failed to get module: {}", e);
            Ok(ApiResponse::error(format!("Failed to get module: {}", e)))
        }
    }
}

/// List all modules for a campaign
#[tauri::command]
pub async fn list_campaign_modules(
    request: ListModulesRequest,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<Module>>, ApiError> {
    info!("Listing modules for campaign: {}", request.campaign_id);
    
    let mut conn = state.db.get_connection()?;
    let mut service = ModuleService::new(&mut *conn);
    
    match service.list_campaign_modules(request.campaign_id) {
        Ok(modules) => {
            info!("Found {} modules", modules.len());
            Ok(ApiResponse::success(modules))
        }
        Err(e) => {
            error!("Failed to list modules: {}", e);
            Ok(ApiResponse::error(format!("Failed to list modules: {}", e)))
        }
    }
}

/// Update a module
#[tauri::command]
pub async fn update_module(
    id: i32,
    request: UpdateModuleRequest,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Module>, ApiError> {
    info!("Updating module with ID: {}", id);
    
    let mut conn = state.db.get_connection()?;
    let mut service = ModuleService::new(&mut *conn);
    
    let update = UpdateModule {
        name: request.name,
        status: None,
        expected_sessions: request.expected_sessions,
        actual_sessions: request.actual_sessions,
        started_at: None,
        completed_at: None,
    };
    
    match service.update_module(id, update) {
        Ok(module) => {
            info!("Module updated successfully");
            Ok(ApiResponse::success(module))
        }
        Err(e) => {
            error!("Failed to update module: {}", e);
            Ok(ApiResponse::error(format!("Failed to update module: {}", e)))
        }
    }
}

/// Transition a module to a new stage
#[tauri::command]
pub async fn transition_module_stage(
    request: TransitionModuleRequest,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Module>, ApiError> {
    info!("Transitioning module {} to stage: {}", request.module_id, request.new_stage);
    
    let mut conn = state.db.get_connection()?;
    let mut service = ModuleService::new(&mut *conn);
    
    match service.transition_module_stage(request.module_id, &request.new_stage) {
        Ok(module) => {
            info!("Module transitioned successfully to: {}", module.status);
            Ok(ApiResponse::success(module))
        }
        Err(e) => {
            error!("Failed to transition module: {}", e);
            Ok(ApiResponse::error(format!("Failed to transition module: {}", e)))
        }
    }
}

/// Initialize documents for a module stage
#[tauri::command]
pub async fn initialize_module_documents(
    request: InitializeDocumentsRequest,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<String>>, ApiError> {
    info!("Initializing documents for module: {}", request.module_id);
    
    let mut conn = state.db.get_connection()?;
    let mut service = ModuleService::new(&mut *conn);
    
    match service.initialize_module_documents(
        request.module_id,
        &request.campaign_directory,
    ) {
        Ok(files) => {
            info!("Initialized {} documents", files.len());
            Ok(ApiResponse::success(files))
        }
        Err(e) => {
            error!("Failed to initialize documents: {}", e);
            Ok(ApiResponse::error(format!("Failed to initialize documents: {}", e)))
        }
    }
}

/// Request for getting module documents
#[derive(Debug, Serialize, Deserialize)]
pub struct GetModuleDocumentsRequest {
    pub module_id: i32,
}

/// Get module documents
#[tauri::command]
pub async fn get_module_documents(
    request: GetModuleDocumentsRequest,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<mimir_dm_core::models::campaign::documents::Document>>, ApiError> {
    info!("Getting documents for module: {}", request.module_id);
    
    let mut conn = state.db.get_connection()?;
    let mut service = ModuleService::new(&mut *conn);
    
    match service.get_module_documents(request.module_id) {
        Ok(documents) => {
            info!("Found {} documents", documents.len());
            Ok(ApiResponse::success(documents))
        }
        Err(e) => {
            error!("Failed to get module documents: {}", e);
            Ok(ApiResponse::error(format!("Failed to get module documents: {}", e)))
        }
    }
}

/// Check module completion status
#[tauri::command]
pub async fn check_module_completion(
    module_id: i32,
    state: State<'_, AppState>,
) -> Result<ApiResponse<BoardCompletionStatus>, ApiError> {
    info!("Checking completion status for module: {}", module_id);
    
    let mut conn = state.db.get_connection()?;
    let mut service = ModuleService::new(&mut *conn);
    
    match service.check_module_completion(module_id) {
        Ok(status) => {
            info!("Module completion: {}/{} required documents", 
                status.completed_required_documents, 
                status.total_required_documents
            );
            Ok(ApiResponse::success(status))
        }
        Err(e) => {
            error!("Failed to check module completion: {}", e);
            Ok(ApiResponse::error(format!("Failed to check module completion: {}", e)))
        }
    }
}

/// Find modules needing next module planning
#[tauri::command]
pub async fn find_modules_needing_next(
    campaign_id: i32,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<Module>>, ApiError> {
    info!("Finding modules needing next planning for campaign: {}", campaign_id);
    
    let mut conn = state.db.get_connection()?;
    let mut service = ModuleService::new(&mut *conn);
    
    match service.find_modules_needing_next(campaign_id) {
        Ok(modules) => {
            info!("Found {} modules needing next planning", modules.len());
            Ok(ApiResponse::success(modules))
        }
        Err(e) => {
            error!("Failed to find modules needing next: {}", e);
            Ok(ApiResponse::error(format!("Failed to find modules needing next: {}", e)))
        }
    }
}

/// Increment module session count
#[tauri::command]
pub async fn increment_module_sessions(
    module_id: i32,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Module>, ApiError> {
    info!("Incrementing session count for module: {}", module_id);
    
    let mut conn = state.db.get_connection()?;
    let mut service = ModuleService::new(&mut *conn);
    
    match service.increment_module_sessions(module_id) {
        Ok(module) => {
            info!("Module session count updated to: {}", module.actual_sessions);
            Ok(ApiResponse::success(module))
        }
        Err(e) => {
            error!("Failed to increment module sessions: {}", e);
            Ok(ApiResponse::error(format!("Failed to increment module sessions: {}", e)))
        }
    }
}

/// Delete a module
#[tauri::command]
pub async fn delete_module(
    id: i32,
    state: State<'_, AppState>,
) -> Result<ApiResponse<()>, ApiError> {
    info!("Deleting module with ID: {}", id);
    
    let mut conn = state.db.get_connection()?;
    let mut service = ModuleService::new(&mut *conn);
    
    match service.delete_module(id) {
        Ok(_) => {
            info!("Module deleted successfully");
            Ok(ApiResponse::success(()))
        }
        Err(e) => {
            error!("Failed to delete module: {}", e);
            Ok(ApiResponse::error(format!("Failed to delete module: {}", e)))
        }
    }
}