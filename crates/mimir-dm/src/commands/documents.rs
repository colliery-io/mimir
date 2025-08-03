//! Document management commands

use mimir_dm_db::{
    dal::{
        documents::DocumentRepository,
        campaigns::CampaignRepository,
        template_documents::TemplateRepository,
    },
    models::documents::{Document, NewDocument, UpdateDocument},
    models::campaigns::Campaign,
};
use std::sync::Arc;
use std::path::PathBuf;
use std::fs;
use tauri::State;

use crate::{
    services::database::DatabaseService,
    types::ApiResponse,
};
use serde::Serialize;

/// Get all documents for a campaign
#[tauri::command]
pub async fn get_campaign_documents(
    campaign_id: i32,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<ApiResponse<Vec<Document>>, String> {
    let mut pooled_conn = db_service.get_connection().map_err(|e| e.to_string())?;
    let conn = &mut *pooled_conn;
    
    match DocumentRepository::find_by_campaign(conn, campaign_id) {
        Ok(documents) => Ok(ApiResponse::success(documents)),
        Err(e) => Ok(ApiResponse::error(format!(
            "Failed to load documents: {}",
            e
        ))),
    }
}

/// Get documents by level (campaign, module, session, or handout)
#[tauri::command]
pub async fn get_documents_by_level(
    campaign_id: i32,
    level: String,
    module_id: Option<i32>,
    session_id: Option<i32>,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<ApiResponse<Vec<Document>>, String> {
    let mut pooled_conn = db_service.get_connection().map_err(|e| e.to_string())?;
    let conn = &mut *pooled_conn;
    
    let documents = match level.as_str() {
        "campaign" => {
            // Get campaign-level documents (no module or session id)
            DocumentRepository::find_by_campaign(conn, campaign_id)
                .map(|docs| docs.into_iter()
                    .filter(|d| d.module_id.is_none() && d.session_id.is_none() && d.document_type != "handout")
                    .collect())
        },
        "module" => {
            if let Some(mid) = module_id {
                DocumentRepository::find_by_module(conn, mid)
            } else {
                Ok(vec![])
            }
        },
        "session" => {
            if let Some(sid) = session_id {
                DocumentRepository::find_by_session(conn, sid)
            } else {
                Ok(vec![])
            }
        },
        "handout" => {
            DocumentRepository::find_handouts_by_campaign(conn, campaign_id)
        },
        _ => Err(mimir_dm_db::error::DbError::InvalidData(format!("Invalid document level: {}", level))),
    };
    
    match documents {
        Ok(docs) => Ok(ApiResponse::success(docs)),
        Err(e) => Ok(ApiResponse::error(format!(
            "Failed to load documents: {}",
            e
        ))),
    }
}

/// Create a new document
#[tauri::command]
pub async fn create_document(
    new_document: NewDocument,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<ApiResponse<Document>, String> {
    let mut pooled_conn = db_service.get_connection().map_err(|e| e.to_string())?;
    let conn = &mut *pooled_conn;
    
    match DocumentRepository::create(conn, new_document) {
        Ok(document) => Ok(ApiResponse::success(document)),
        Err(e) => Ok(ApiResponse::error(format!(
            "Failed to create document: {}",
            e
        ))),
    }
}

/// Update a document
#[tauri::command]
pub async fn update_document(
    document_id: i32,
    update: UpdateDocument,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<ApiResponse<Document>, String> {
    let mut pooled_conn = db_service.get_connection().map_err(|e| e.to_string())?;
    let conn = &mut *pooled_conn;
    
    match DocumentRepository::update(conn, document_id, update) {
        Ok(document) => Ok(ApiResponse::success(document)),
        Err(e) => Ok(ApiResponse::error(format!(
            "Failed to update document: {}",
            e
        ))),
    }
}

/// Mark a document as completed
#[tauri::command]
pub async fn complete_document(
    document_id: i32,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<ApiResponse<Document>, String> {
    let mut pooled_conn = db_service.get_connection().map_err(|e| e.to_string())?;
    let conn = &mut *pooled_conn;
    
    match DocumentRepository::mark_completed(conn, document_id) {
        Ok(document) => Ok(ApiResponse::success(document)),
        Err(e) => Ok(ApiResponse::error(format!(
            "Failed to complete document: {}",
            e
        ))),
    }
}

/// Delete a document
#[tauri::command]
pub async fn delete_document(
    document_id: i32,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<ApiResponse<()>, String> {
    let mut pooled_conn = db_service.get_connection().map_err(|e| e.to_string())?;
    let conn = &mut *pooled_conn;
    
    match DocumentRepository::delete(conn, document_id) {
        Ok(_) => Ok(ApiResponse::success(())),
        Err(e) => Ok(ApiResponse::error(format!(
            "Failed to delete document: {}",
            e
        ))),
    }
}

/// Get incomplete documents for a campaign
#[tauri::command]
pub async fn get_incomplete_documents(
    campaign_id: i32,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<ApiResponse<Vec<Document>>, String> {
    let mut pooled_conn = db_service.get_connection().map_err(|e| e.to_string())?;
    let conn = &mut *pooled_conn;
    
    match DocumentRepository::find_incomplete_by_campaign(conn, campaign_id) {
        Ok(documents) => Ok(ApiResponse::success(documents)),
        Err(e) => Ok(ApiResponse::error(format!(
            "Failed to load incomplete documents: {}",
            e
        ))),
    }
}

/// Get completed documents for a campaign
#[tauri::command]
pub async fn get_completed_documents(
    campaign_id: i32,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<ApiResponse<Vec<Document>>, String> {
    let mut pooled_conn = db_service.get_connection().map_err(|e| e.to_string())?;
    let conn = &mut *pooled_conn;
    
    match DocumentRepository::find_completed_by_campaign(conn, campaign_id) {
        Ok(documents) => Ok(ApiResponse::success(documents)),
        Err(e) => Ok(ApiResponse::error(format!(
            "Failed to load completed documents: {}",
            e
        ))),
    }
}

/// Create a document from a template
#[tauri::command]
pub async fn create_document_from_template(
    campaign_id: i32,
    template_id: String,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<ApiResponse<Document>, String> {
    let mut pooled_conn = db_service.get_connection().map_err(|e| e.to_string())?;
    let conn = &mut *pooled_conn;
    
    // Get the campaign
    let mut campaign_repo = CampaignRepository::new(conn);
    let campaign = match campaign_repo.find_by_id(campaign_id) {
        Ok(Some(c)) => c,
        Ok(None) => return Ok(ApiResponse::error("Campaign not found".to_string())),
        Err(e) => return Ok(ApiResponse::error(format!("Database error: {}", e))),
    };
    
    // Check if document already exists
    let existing = DocumentRepository::find_by_campaign(conn, campaign_id)
        .map_err(|e| e.to_string())?;
    if existing.iter().any(|d| d.template_id == template_id) {
        return Ok(ApiResponse::error("Document already exists".to_string()));
    }
    
    // Get the template
    let template = match TemplateRepository::get_latest(conn, &template_id) {
        Ok(t) => t,
        Err(_) => return Ok(ApiResponse::error(format!("Template '{}' not found", template_id))),
    };
    
    // Create the document file
    let file_name = format!("{}.md", template_id);
    let file_path = PathBuf::from(&campaign.directory_path).join(&file_name);
    
    // Process template using the create_context method
    let mut context = template.create_context();
    let mut tera = tera::Tera::default();
    tera.add_raw_template(&template.document_id, &template.document_content)
        .map_err(|e| format!("Failed to add template: {}", e))?;
    
    let content = tera.render(&template.document_id, &context)
        .map_err(|e| format!("Failed to render template: {}", e))?;
    
    // Write file to disk
    fs::write(&file_path, content)
        .map_err(|e| format!("Failed to write document file: {}", e))?;
    
    // Create database record
    // Generate title from template_id
    let title = template_id
        .split('-')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ");
    
    let new_doc = NewDocument {
        campaign_id: campaign.id,
        module_id: None,
        session_id: None,
        template_id: template_id.clone(),
        document_type: template_id.replace('-', "_"),
        title,
        file_path: file_path.to_string_lossy().to_string(),
    };
    
    match DocumentRepository::create(conn, new_doc) {
        Ok(document) => Ok(ApiResponse::success(document)),
        Err(e) => Ok(ApiResponse::error(format!("Failed to create document: {}", e))),
    }
}

/// Read a document file from disk
#[tauri::command]
pub async fn read_document_file(
    file_path: String,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<ApiResponse<String>, String> {
    let path = PathBuf::from(&file_path);
    
    // Check if file exists
    if !path.exists() {
        return Ok(ApiResponse::error("Document file not found".to_string()));
    }
    
    // Read the markdown file directly
    match fs::read_to_string(&path) {
        Ok(content) => Ok(ApiResponse::success(content)),
        Err(e) => Ok(ApiResponse::error(format!("Failed to read document: {}", e))),
    }
}

/// Save a document file to disk
#[tauri::command]
pub async fn save_document_file(
    file_path: String,
    content: String,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<ApiResponse<()>, String> {
    let path = PathBuf::from(&file_path);
    
    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            if let Err(e) = fs::create_dir_all(parent) {
                return Ok(ApiResponse::error(format!("Failed to create directory: {}", e)));
            }
        }
    }
    
    // Write the file
    match fs::write(&path, content) {
        Ok(_) => Ok(ApiResponse::success(())),
        Err(e) => Ok(ApiResponse::error(format!("Failed to save document: {}", e))),
    }
}

#[derive(Serialize, Debug)]
pub struct StageCompletionStatus {
    pub current_stage: String,
    pub total_documents: usize,
    pub completed_documents: usize,
    pub is_complete: bool,
    pub can_progress: bool,
    pub next_stage: Option<String>,
    pub missing_documents: Vec<String>,
}

/// Stage document requirements configuration
fn get_stage_required_documents(stage: &str) -> Vec<&'static str> {
    match stage {
        "concept" => vec!["campaign_pitch"],
        "session_zero" => vec![
            "starting_scenario",
            "world_primer", 
            "character_guidelines",
            "table_expectations",
            "character_integration_forms",
            "session_zero_packet"
        ],
        "integration" => vec![
            "campaign_bible",
            "character_integration_notes",
            "major_npcs",
            "world_events_timeline"
        ],
        "active" => vec![], // No required documents for active stage
        _ => vec![],
    }
}

/// Check if a stage is complete and ready for progression
#[tauri::command]
pub async fn check_stage_completion(
    campaign_id: i32,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<ApiResponse<StageCompletionStatus>, String> {
    let mut pooled_conn = db_service.get_connection().map_err(|e| e.to_string())?;
    let conn = &mut *pooled_conn;
    
    // Get the campaign to check current status
    let mut campaign_repo = CampaignRepository::new(conn);
    let campaign = match campaign_repo.find_by_id(campaign_id) {
        Ok(Some(c)) => c,
        Ok(None) => return Ok(ApiResponse::error("Campaign not found".to_string())),
        Err(e) => return Ok(ApiResponse::error(format!("Database error: {}", e))),
    };
    
    let current_stage = &campaign.status;
    let required_doc_types = get_stage_required_documents(current_stage);
    
    // Get all documents for this campaign
    let all_documents = DocumentRepository::find_by_campaign(conn, campaign_id)
        .map_err(|e| e.to_string())?;
    
    // Filter documents by current stage requirements
    let stage_documents: Vec<_> = all_documents.into_iter()
        .filter(|doc| required_doc_types.contains(&doc.document_type.as_str()))
        .collect();
    
    // Find missing document types
    let existing_types: Vec<&str> = stage_documents.iter()
        .map(|d| d.document_type.as_str())
        .collect();
    
    let missing_documents: Vec<String> = required_doc_types.iter()
        .filter(|&&doc_type| !existing_types.contains(&doc_type))
        .map(|&s| s.to_string())
        .collect();
    
    let total_documents = required_doc_types.len();
    let completed_documents = stage_documents
        .iter()
        .filter(|doc| doc.completed_at.is_some())
        .count();
    
    let is_complete = total_documents > 0 && 
                     total_documents == stage_documents.len() && 
                     total_documents == completed_documents;
    
    // Determine next stage using the campaign's can_transition_to method
    let next_stage = match current_stage.as_str() {
        "concept" => Some("session_zero".to_string()),
        "session_zero" => Some("integration".to_string()),
        "integration" => Some("active".to_string()),
        "active" => Some("concluding".to_string()),
        _ => None,
    };
    
    let can_progress = if let Some(ref next) = next_stage {
        is_complete && campaign.can_transition_to(next)
    } else {
        false
    };
    
    Ok(ApiResponse::success(StageCompletionStatus {
        current_stage: current_stage.clone(),
        total_documents,
        completed_documents,
        is_complete,
        can_progress,
        next_stage,
        missing_documents,
    }))
}


