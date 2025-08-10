//! Campaign management commands

use crate::{
    services::database::DatabaseService,
    types::ApiResponse,
};
use mimir_dm_core::{
    dal::campaign::{campaigns::CampaignRepository, documents::DocumentRepository},
    domain::{BoardCompletionStatus, BoardRegistry},
    models::campaign::campaigns::Campaign as DbCampaign,
};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::sync::Arc;
use tauri::State;
use tracing::{error, info};

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
) -> Result<ApiResponse<Vec<Campaign>>, String> {
    info!("Listing campaigns");
    
    let mut conn = db_service.get_connection().map_err(|e| e.to_string())?;
    let mut service = mimir_dm_core::services::CampaignService::new(&mut *conn);
    
    match service.list_campaigns() {
        Ok(campaigns) => {
            let campaigns: Vec<Campaign> = campaigns.into_iter().map(Campaign::from).collect();
            info!("Found {} campaigns", campaigns.len());
            Ok(ApiResponse::success(campaigns))
        }
        Err(e) => {
            error!("Failed to list campaigns: {}", e);
            Ok(ApiResponse::error(format!("Failed to list campaigns: {}", e)))
        }
    }
}

#[tauri::command]
pub async fn create_campaign(
    request: CreateCampaignRequest,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<ApiResponse<Campaign>, String> {
    info!("Creating new campaign: {} at location: {}", request.name, request.directory_location);
    
    let mut conn = db_service.get_connection().map_err(|e| e.to_string())?;
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
) -> Result<ApiResponse<GeneratedDocument>, String> {
    info!("Generating document from template '{}' for campaign {}", request.template_id, request.campaign_id);
    
    let mut conn = db_service.get_connection().map_err(|e| e.to_string())?;
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

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateInfo {
    pub id: String,
    pub title: String,
    pub purpose: String,
    pub level: String,
    pub template_type: String,
    pub variables: Vec<TemplateVariable>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateVariable {
    pub name: String,
    pub var_type: String,
    pub description: String,
    pub default: JsonValue,
    pub required: bool,
}

/// List all available templates
#[tauri::command]
pub async fn list_templates(
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<ApiResponse<Vec<TemplateInfo>>, String> {
    info!("Listing available templates");
    
    let mut conn = db_service.get_connection().map_err(|e| e.to_string())?;
    let mut service = mimir_dm_core::services::TemplateService::new(&mut *conn);
    
    match service.list_templates() {
        Ok(templates) => {
            let template_infos: Vec<TemplateInfo> = templates.into_iter()
                .filter_map(|template| {
                    // Parse variables from the variables_schema JSON
                    let variables = match &template.variables_schema {
                        Some(schema_str) => {
                            serde_json::from_str::<Vec<serde_json::Value>>(schema_str)
                                .ok()
                                .map(|vars| {
                                    vars.into_iter()
                                        .filter_map(|v| {
                                            Some(TemplateVariable {
                                                name: v.get("name")?.as_str()?.to_string(),
                                                var_type: v.get("var_type").or(v.get("type"))?.as_str()?.to_string(),
                                                description: v.get("description")?.as_str()?.to_string(),
                                                default: v.get("default")?.clone(),
                                                required: v.get("required").and_then(|r| r.as_bool()).unwrap_or(true),
                                            })
                                        })
                                        .collect()
                                })
                                .unwrap_or_default()
                        }
                        None => vec![]
                    };
                    
                    Some(TemplateInfo {
                        id: template.document_id,
                        title: template.metadata
                            .as_ref()
                            .and_then(|m| serde_json::from_str::<serde_json::Value>(m).ok())
                            .and_then(|m| m.get("title")?.as_str().map(String::from))
                            .unwrap_or_else(|| "Untitled Template".to_string()),
                        purpose: template.purpose.unwrap_or_else(|| "No purpose specified".to_string()),
                        level: template.document_level.unwrap_or_else(|| "unknown".to_string()),
                        template_type: template.document_type.unwrap_or_else(|| "unknown".to_string()),
                        variables,
                    })
                })
                .collect();
            
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
) -> Result<ApiResponse<Campaign>, String> {
    info!("Getting campaign with id: {}", id);
    
    let mut conn = db_service.get_connection().map_err(|e| e.to_string())?;
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
) -> Result<ApiResponse<BoardCompletionStatus>, String> {
    info!("Checking stage completion for campaign {}", campaign_id);
    
    let mut pooled_conn = db_service.get_connection().map_err(|e| e.to_string())?;
    let conn = &mut *pooled_conn;
    
    // Get the campaign
    let mut campaign_repo = CampaignRepository::new(conn);
    let campaign = match campaign_repo.find_by_id(campaign_id) {
        Ok(Some(c)) => c,
        Ok(None) => {
            error!("Campaign {} not found", campaign_id);
            return Ok(ApiResponse::error(format!("Campaign {} not found", campaign_id)));
        }
        Err(e) => {
            error!("Failed to find campaign: {}", e);
            return Ok(ApiResponse::error(format!("Database error: {}", e)));
        }
    };
    
    // Get the board definition
    let board_registry = BoardRegistry::new();
    let board = match board_registry.get("campaign") {
        Some(b) => b,
        None => {
            error!("Campaign board definition not found");
            return Ok(ApiResponse::error("Campaign board definition not found".to_string()));
        }
    };
    
    let current_stage = &campaign.status;
    
    // Get required and optional documents for current stage
    let required_docs = board.required_documents(current_stage);
    let optional_docs = board.optional_documents(current_stage);
    
    // Get all documents for this campaign
    let all_documents = match DocumentRepository::find_by_campaign(conn, campaign_id) {
        Ok(docs) => docs,
        Err(e) => {
            error!("Failed to find documents: {}", e);
            return Ok(ApiResponse::error(format!("Failed to find documents: {}", e)));
        }
    };
    
    // Count completed required documents
    let mut completed_required = 0;
    let mut missing_required = Vec::new();
    
    for doc_type in &required_docs {
        if let Some(doc) = all_documents.iter().find(|d| d.document_type == *doc_type) {
            if doc.completed_at.is_some() {
                completed_required += 1;
            }
        } else {
            missing_required.push(doc_type.to_string());
        }
    }
    
    // Count completed optional documents
    let mut completed_optional = 0;
    for doc_type in &optional_docs {
        if let Some(doc) = all_documents.iter().find(|d| d.document_type == *doc_type) {
            if doc.completed_at.is_some() {
                completed_optional += 1;
            }
        }
    }
    
    let is_stage_complete = required_docs.len() == completed_required && missing_required.is_empty();
    let next_stage = board.next_stage(current_stage).map(|s| s.to_string());
    let can_progress = is_stage_complete && next_stage.is_some();
    
    let status = BoardCompletionStatus {
        board_type: board.board_type().to_string(),
        current_stage: current_stage.clone(),
        total_required_documents: required_docs.len(),
        completed_required_documents: completed_required,
        total_optional_documents: optional_docs.len(),
        completed_optional_documents: completed_optional,
        missing_required_documents: missing_required,
        is_stage_complete,
        can_progress,
        next_stage,
        stage_metadata: board.stage_metadata(current_stage),
    };
    
    info!("Stage completion status: {:?}", status);
    Ok(ApiResponse::success(status))
}

/// Transition campaign to the next stage
#[tauri::command]
pub async fn transition_campaign_stage(
    campaign_id: i32,
    new_stage: String,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<ApiResponse<Campaign>, String> {
    info!("Transitioning campaign {} to stage {}", campaign_id, new_stage);
    
    let mut conn = db_service.get_connection().map_err(|e| e.to_string())?;
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

