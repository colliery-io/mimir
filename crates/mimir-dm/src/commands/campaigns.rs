//! Campaign management commands

use crate::{
    boards::{BoardCompletionStatus, BoardRegistry},
    services::database::DatabaseService,
    types::ApiResponse,
    APP_PATHS,
};
use mimir_dm_db::{
    dal::campaigns::CampaignRepository,
    dal::documents::DocumentRepository,
    dal::template_documents::TemplateRepository,
    models::campaigns::{Campaign as DbCampaign, NewCampaign},
};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tauri::State;
use tera::{Context, Tera};
use tracing::{error, info, warn};

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

/// Create campaign directory structure based on the specification
fn create_campaign_directory_structure(base_path: &Path, campaign_name: &str) -> Result<PathBuf, std::io::Error> {
    let campaign_path = base_path.join(campaign_name);
    
    // Check if campaign directory already exists
    if campaign_path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            format!("Campaign directory '{}' already exists", campaign_path.display())
        ));
    }
    
    info!("Creating campaign directory structure at: {}", campaign_path.display());
    
    // Create main campaign directory
    fs::create_dir_all(&campaign_path)?;
    
    // Create all the required directories
    let directories = [
        "session_zero",
        "world",
        "world/factions",
        "regions",
        "modules", 
        "sessions",
        "characters",
        "npcs",
        "npcs/recurring",
        "resources",
        "resources/maps",
        "resources/handouts", 
        "resources/references",
    ];
    
    for dir in directories {
        let dir_path = campaign_path.join(dir);
        fs::create_dir_all(&dir_path)?;
        info!("Created directory: {}", dir_path.display());
    }
    
    // Create only essential starter files - others will be created as needed
    // For now, just create a simple README to mark the campaign as initialized
    let readme_content = format!(
        "# {}\n\nCampaign created on {}\n\nUse the Mimir application to generate additional campaign documents as needed.",
        campaign_name,
        chrono::Local::now().format("%Y-%m-%d")
    );
    fs::write(campaign_path.join("README.md"), readme_content)?;
    info!("Created campaign README.md");
    
    info!("Successfully created campaign directory structure");
    Ok(campaign_path)
}

#[tauri::command]
pub async fn list_campaigns(
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<ApiResponse<Vec<Campaign>>, String> {
    info!("Listing campaigns");
    
    let Some(_paths) = APP_PATHS.get() else {
        error!("Application not initialized");
        return Ok(ApiResponse::error("Application not initialized".to_string()));
    };

    let mut pooled_conn = db_service.get_connection().map_err(|e| e.to_string())?;
    let conn = &mut *pooled_conn;
    
    let mut repo = CampaignRepository::new(conn);
    match repo.list() {
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
    
    let Some(_paths) = APP_PATHS.get() else {
        error!("Application not initialized");
        return Ok(ApiResponse::error("Application not initialized".to_string()));
    };

    // First, try to create the campaign directory structure
    let base_path = Path::new(&request.directory_location);
    let campaign_path = match create_campaign_directory_structure(base_path, &request.name) {
        Ok(path) => path,
        Err(e) => {
            error!("Failed to create campaign directory: {}", e);
            return Ok(ApiResponse::error(format!("Failed to create campaign directory: {}", e)));
        }
    };

    // If directory creation succeeded, create the database record
    let mut pooled_conn = db_service.get_connection().map_err(|e| {
        // Rollback: try to remove the created directory
        if let Err(remove_err) = fs::remove_dir_all(&campaign_path) {
            warn!("Failed to cleanup campaign directory after database connection error: {}", remove_err);
        }
        e.to_string()
    })?;
    let conn = &mut *pooled_conn;
    
    let mut repo = CampaignRepository::new(conn);
    let new_campaign = NewCampaign {
        name: request.name.clone(),
        status: "concept".to_string(),
        directory_path: campaign_path.to_string_lossy().to_string(),
    };
    
    match repo.create(new_campaign) {
        Ok(campaign) => {
            info!("Created campaign: {} with directory: {}", campaign.name, campaign.directory_path);
            
            // Initialize concept stage documents
            if let Err(e) = crate::commands::stage_transitions::create_initial_documents(conn, &campaign) {
                warn!("Failed to create initial documents: {}", e);
                // Continue anyway - campaign is created, documents can be created later
            }
            
            Ok(ApiResponse::success(Campaign::from(campaign)))
        }
        Err(e) => {
            error!("Failed to create campaign '{}': {}", request.name, e);
            
            // Rollback: try to remove the created directory
            if let Err(remove_err) = fs::remove_dir_all(&campaign_path) {
                warn!("Failed to cleanup campaign directory after database error: {}", remove_err);
            }
            
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
    
    let mut pooled_conn = db_service.get_connection().map_err(|e| e.to_string())?;
    let conn = &mut *pooled_conn;
    
    // Get the campaign to find its directory
    let mut campaign_repo = CampaignRepository::new(conn);
    let campaign = match campaign_repo.find_by_id(request.campaign_id) {
        Ok(Some(campaign)) => campaign,
        Ok(None) => {
            error!("Campaign {} not found", request.campaign_id);
            return Ok(ApiResponse::error(format!("Campaign {} not found", request.campaign_id)));
        }
        Err(e) => {
            error!("Failed to find campaign: {}", e);
            return Ok(ApiResponse::error(format!("Failed to find campaign: {}", e)));
        }
    };
    
    // Get the template
    let template = match TemplateRepository::get_latest(conn, &request.template_id) {
        Ok(template) => template,
        Err(e) => {
            error!("Failed to get template '{}': {}", request.template_id, e);
            return Ok(ApiResponse::error(format!("Failed to get template: {}", e)));
        }
    };
    
    // Create Tera context with provided variables
    let mut context = Context::new();
    
    // Add campaign-level variables
    context.insert("campaign_name", &campaign.name);
    context.insert("campaign_status", &campaign.status);
    
    // Add user-provided variables
    for (key, value) in request.variables {
        context.insert(&key, &value);
    }
    
    // Render the template
    let mut tera = Tera::default();
    match tera.add_raw_template(&request.template_id, &template.document_content) {
        Ok(_) => {},
        Err(e) => {
            error!("Failed to parse template: {}", e);
            return Ok(ApiResponse::error(format!("Failed to parse template: {}", e)));
        }
    }
    
    let rendered_content = match tera.render(&request.template_id, &context) {
        Ok(content) => content,
        Err(e) => {
            error!("Failed to render template: {}", e);
            return Ok(ApiResponse::error(format!("Failed to render template: {}", e)));
        }
    };
    
    // Determine the file path based on template type
    let file_path = determine_template_file_path(&campaign.directory_path, &request.template_id);
    let full_path = PathBuf::from(&file_path);
    
    // Create parent directory if needed
    if let Some(parent) = full_path.parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            error!("Failed to create directory: {}", e);
            return Ok(ApiResponse::error(format!("Failed to create directory: {}", e)));
        }
    }
    
    // Write the file
    match fs::write(&full_path, rendered_content) {
        Ok(_) => {
            info!("Generated document at: {}", full_path.display());
            Ok(ApiResponse::success(GeneratedDocument {
                file_path: file_path.clone(),
                template_id: request.template_id,
                success: true,
                error_message: None,
            }))
        }
        Err(e) => {
            error!("Failed to write file: {}", e);
            Ok(ApiResponse::error(format!("Failed to write file: {}", e)))
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
    
    let mut pooled_conn = db_service.get_connection().map_err(|e| e.to_string())?;
    let conn = &mut *pooled_conn;
    
    match TemplateRepository::get_all_active(conn) {
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

/// Determine where to save a template based on its ID
fn determine_template_file_path(campaign_dir: &str, template_id: &str) -> String {
    let campaign_path = Path::new(campaign_dir);
    
    let file_path = match template_id {
        // Campaign level documents
        "campaign-bible" => campaign_path.join("campaign_bible.md"),
        "campaign-pitch" => campaign_path.join("pitch.md"),
        "starting-scenario" => campaign_path.join("session_zero/starting_scenario.md"),
        "quick-start-kit" => campaign_path.join("quick_start_kit.md"),
        
        // World building
        "world-overview" => campaign_path.join("world/overview.md"),
        "region-overview" => campaign_path.join("regions/region_overview.md"),
        "faction-template" => campaign_path.join("world/factions/faction.md"),
        
        // Characters and NPCs  
        "character-integration" => campaign_path.join("characters/character_integration.md"),
        "major-npc-tracker" => campaign_path.join("npcs/major_npcs.md"),
        "quick-npc-reference" => campaign_path.join("npcs/quick_reference.md"),
        "pc-arc-tracker" => campaign_path.join("characters/pc_arc_tracker.md"),
        
        // Session management
        "session-outline" => campaign_path.join("sessions/session_outline.md"),
        "clue-tracker" => campaign_path.join("sessions/clue_tracker.md"),
        "document-tracker" => campaign_path.join("document_tracker.md"),
        
        // Module templates
        "module-overview" => campaign_path.join("modules/module_overview.md"),
        template if template.starts_with("module-") => {
            campaign_path.join(format!("modules/{}.md", template))
        }
        
        // Default fallback
        _ => campaign_path.join(format!("{}.md", template_id)),
    };
    
    file_path.to_string_lossy().to_string()
}

#[tauri::command]
pub async fn get_campaign(
    id: i32,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<ApiResponse<Campaign>, String> {
    info!("Getting campaign with id: {}", id);
    
    let Some(_paths) = APP_PATHS.get() else {
        error!("Application not initialized");
        return Ok(ApiResponse::error("Application not initialized".to_string()));
    };

    let mut pooled_conn = db_service.get_connection().map_err(|e| e.to_string())?;
    let conn = &mut *pooled_conn;
    
    let mut repo = CampaignRepository::new(conn);
    match repo.find_by_id(id) {
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
    
    let mut pooled_conn = db_service.get_connection().map_err(|e| e.to_string())?;
    let conn = &mut *pooled_conn;
    
    let mut campaign_repo = CampaignRepository::new(conn);
    
    // Verify the transition is valid using board definition
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
    
    let board_registry = BoardRegistry::new();
    let board = match board_registry.get("campaign") {
        Some(b) => b,
        None => {
            error!("Campaign board definition not found");
            return Ok(ApiResponse::error("Campaign board definition not found".to_string()));
        }
    };
    
    // Check if transition is allowed
    if !board.can_transition(&campaign.status, &new_stage) {
        error!("Cannot transition from {} to {}", campaign.status, new_stage);
        return Ok(ApiResponse::error(format!(
            "Cannot transition from {} to {}",
            campaign.status, new_stage
        )));
    }
    
    // Perform the transition
    match campaign_repo.transition_status(campaign_id, &new_stage) {
        Ok(updated_campaign) => {
            info!("Successfully transitioned campaign to {}", new_stage);
            
            // Create initial documents for the new stage if needed
            if let Err(e) = crate::commands::stage_transitions::create_stage_documents(conn, &updated_campaign, &new_stage) {
                warn!("Failed to create stage documents: {}", e);
                // Continue anyway - transition succeeded
            }
            
            Ok(ApiResponse::success(Campaign::from(updated_campaign)))
        }
        Err(e) => {
            error!("Failed to transition campaign: {}", e);
            Ok(ApiResponse::error(format!("Failed to transition campaign: {}", e)))
        }
    }
}

