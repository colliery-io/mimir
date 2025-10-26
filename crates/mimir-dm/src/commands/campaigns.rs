//! Campaign management commands

use crate::types::{ApiError, ApiResponse};
use mimir_dm_core::{
    domain::{BoardCompletionStatus, TemplateInfo},
    models::campaign::campaigns::Campaign as DbCampaign,
    DatabaseService,
};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::sync::Arc;
use tauri::State;
use tracing::{debug, error, info};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Campaign {
    pub id: i32,
    pub name: String,
    pub status: String,
    pub directory_path: String,
    pub created_at: String,
}

impl From<DbCampaign> for Campaign {
    fn from(db_campaign: DbCampaign) -> Self {
        Self {
            id: db_campaign.id,
            name: db_campaign.name,
            status: db_campaign.status,
            directory_path: db_campaign.directory_path,
            created_at: db_campaign.created_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCampaignRequest {
    pub name: String,
    pub description: Option<String>,
    pub directory_location: String, // Base directory where campaign folder will be created
}


#[tauri::command]
pub async fn list_campaigns(
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<ApiResponse<Vec<Campaign>>, ApiError> {
    info!("Listing campaigns");

    let mut conn = db_service.get_connection()?;
    let mut service = mimir_dm_core::services::CampaignService::new(&mut *conn);
    
    match service.list_active_campaigns() {
        Ok(campaigns) => {
            let campaigns: Vec<Campaign> = campaigns.into_iter().map(Campaign::from).collect();
            info!("Found {} campaigns", campaigns.len());
            debug!("Campaign details: {:?}", campaigns);
            let response = ApiResponse::success(campaigns);
            debug!("Returning success response");
            Ok(response)
        }
        Err(e) => {
            error!("Failed to list campaigns: {}", e);
            let response = ApiResponse::error(format!("Failed to list campaigns: {}", e));
            debug!("Returning error response: {:?}", response);
            Ok(response)
        }
    }
}

#[tauri::command]
pub async fn create_campaign(
    request: CreateCampaignRequest,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<ApiResponse<Campaign>, ApiError> {
    info!("Creating new campaign: {} at location: {}", request.name, request.directory_location);

    let mut conn = db_service.get_connection()?;
    let mut service = mimir_dm_core::services::CampaignService::new(&mut *conn);
    
    match service.create_campaign(
        &request.name,
        request.description,
        &request.directory_location,
    ) {
        Ok(campaign) => {
            info!("Created campaign: {} with directory: {}", campaign.name, campaign.directory_path);
            Ok(ApiResponse::success(Campaign::from(campaign)))
        }
        Err(e) => {
            error!("Failed to create campaign '{}': {}", request.name, e);
            Ok(ApiResponse::error(format!("Failed to create campaign: {}", e)))
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateDocumentRequest {
    pub campaign_id: i32,
    pub template_id: String,
    pub variables: HashMap<String, JsonValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratedDocument {
    pub file_path: String,
    pub template_id: String,
    pub success: bool,
    pub error_message: Option<String>,
}

/// Generate a campaign document from a template
#[tauri::command]
pub async fn generate_campaign_document(
    request: GenerateDocumentRequest,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<ApiResponse<GeneratedDocument>, ApiError> {
    info!("Generating document from template '{}' for campaign {}", request.template_id, request.campaign_id);

    let mut conn = db_service.get_connection()?;
    let mut service = mimir_dm_core::services::TemplateService::new(&mut *conn);
    
    match service.generate_document(
        request.campaign_id,
        &request.template_id,
        request.variables,
    ) {
        Ok(file_path) => {
            info!("Generated document at: {}", file_path);
            Ok(ApiResponse::success(GeneratedDocument {
                file_path: file_path.clone(),
                template_id: request.template_id,
                success: true,
                error_message: None,
            }))
        }
        Err(e) => {
            error!("Failed to generate document: {}", e);
            Ok(ApiResponse::error(format!("Failed to generate document: {}", e)))
        }
    }
}

/// List all available templates
#[tauri::command]
pub async fn list_templates(
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<ApiResponse<Vec<TemplateInfo>>, ApiError> {
    info!("Listing available templates");

    let mut conn = db_service.get_connection()?;
    let mut service = mimir_dm_core::services::TemplateService::new(&mut *conn);

    match service.list_templates_with_details() {
        Ok(template_infos) => {
            info!("Found {} templates", template_infos.len());
            Ok(ApiResponse::success(template_infos))
        }
        Err(e) => {
            error!("Failed to list templates: {}", e);
            Ok(ApiResponse::error(format!("Failed to list templates: {}", e)))
        }
    }
}


#[tauri::command]
pub async fn get_campaign(
    id: i32,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<ApiResponse<Campaign>, ApiError> {
    info!("Getting campaign with id: {}", id);

    let mut conn = db_service.get_connection()?;
    let mut service = mimir_dm_core::services::CampaignService::new(&mut *conn);
    
    match service.get_campaign(id) {
        Ok(Some(campaign)) => {
            info!("Found campaign: {}", campaign.name);
            Ok(ApiResponse::success(Campaign::from(campaign)))
        }
        Ok(None) => {
            error!("Campaign {} not found", id);
            Ok(ApiResponse::error(format!("Campaign with id {} not found", id)))
        }
        Err(e) => {
            error!("Failed to find campaign {}: {}", id, e);
            Ok(ApiResponse::error(format!("Database error: {}", e)))
        }
    }
}

/// Check campaign stage completion status
#[tauri::command]
pub async fn check_campaign_stage_completion(
    campaign_id: i32,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<ApiResponse<BoardCompletionStatus>, ApiError> {
    info!("Checking stage completion for campaign {}", campaign_id);

    let mut conn = db_service.get_connection()?;
    let mut service = mimir_dm_core::services::CampaignService::new(&mut *conn);

    match service.check_stage_completion(campaign_id) {
        Ok(status) => {
            info!("Stage completion status: {:?}", status);
            Ok(ApiResponse::success(status))
        }
        Err(e) => {
            error!("Failed to check stage completion: {}", e);
            Ok(ApiResponse::error(format!("Failed to check stage completion: {}", e)))
        }
    }
}

/// Transition campaign to the next stage
#[tauri::command]
pub async fn transition_campaign_stage(
    campaign_id: i32,
    new_stage: String,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<ApiResponse<Campaign>, ApiError> {
    info!("Transitioning campaign {} to stage {}", campaign_id, new_stage);

    let mut conn = db_service.get_connection()?;
    let mut service = mimir_dm_core::services::CampaignService::new(&mut *conn);
    
    match service.transition_campaign_stage(campaign_id, &new_stage) {
        Ok(updated_campaign) => {
            info!("Successfully transitioned campaign to {}", new_stage);
            Ok(ApiResponse::success(Campaign::from(updated_campaign)))
        }
        Err(e) => {
            error!("Failed to transition campaign: {}", e);
            Ok(ApiResponse::error(format!("Failed to transition campaign: {}", e)))
        }
    }
}

/// Archive a campaign
#[tauri::command]
pub async fn archive_campaign(
    campaign_id: i32,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<ApiResponse<Campaign>, ApiError> {
    info!("Archiving campaign {}", campaign_id);

    let mut conn = db_service.get_connection()?;
    let mut service = mimir_dm_core::services::CampaignService::new(&mut *conn);
    
    match service.archive_campaign(campaign_id) {
        Ok(archived_campaign) => {
            info!("Successfully archived campaign {}", campaign_id);
            Ok(ApiResponse::success(Campaign::from(archived_campaign)))
        }
        Err(e) => {
            error!("Failed to archive campaign: {}", e);
            Ok(ApiResponse::error(format!("Failed to archive campaign: {}", e)))
        }
    }
}

/// Unarchive a campaign
#[tauri::command]
pub async fn unarchive_campaign(
    campaign_id: i32,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<ApiResponse<Campaign>, ApiError> {
    info!("Unarchiving campaign {}", campaign_id);

    let mut conn = db_service.get_connection()?;
    let mut service = mimir_dm_core::services::CampaignService::new(&mut *conn);
    
    match service.unarchive_campaign(campaign_id) {
        Ok(unarchived_campaign) => {
            info!("Successfully unarchived campaign {}", campaign_id);
            Ok(ApiResponse::success(Campaign::from(unarchived_campaign)))
        }
        Err(e) => {
            error!("Failed to unarchive campaign: {}", e);
            Ok(ApiResponse::error(format!("Failed to unarchive campaign: {}", e)))
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteCampaignRequest {
    pub campaign_id: i32,
    pub delete_files: bool,
}

/// Delete a campaign (hard delete - only allowed for archived campaigns)
#[tauri::command]
pub async fn delete_campaign(
    request: DeleteCampaignRequest,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<ApiResponse<()>, ApiError> {
    info!("Deleting campaign {} (delete_files: {})", request.campaign_id, request.delete_files);

    let mut conn = db_service.get_connection()?;
    let mut service = mimir_dm_core::services::CampaignService::new(&mut *conn);
    
    match service.delete_campaign(request.campaign_id, request.delete_files) {
        Ok(()) => {
            info!("Successfully deleted campaign {}", request.campaign_id);
            Ok(ApiResponse::success(()))
        }
        Err(e) => {
            error!("Failed to delete campaign: {}", e);
            Ok(ApiResponse::error(format!("Failed to delete campaign: {}", e)))
        }
    }
}

/// List archived campaigns
#[tauri::command]
pub async fn list_archived_campaigns(
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<ApiResponse<Vec<Campaign>>, ApiError> {
    info!("Listing archived campaigns");

    let mut conn = db_service.get_connection()?;
    let mut service = mimir_dm_core::services::CampaignService::new(&mut *conn);
    
    match service.list_archived_campaigns() {
        Ok(campaigns) => {
            let campaigns: Vec<Campaign> = campaigns.into_iter().map(Campaign::from).collect();
            info!("Found {} archived campaigns", campaigns.len());
            Ok(ApiResponse::success(campaigns))
        }
        Err(e) => {
            error!("Failed to list archived campaigns: {}", e);
            Ok(ApiResponse::error(format!("Failed to list archived campaigns: {}", e)))
        }
    }
}

