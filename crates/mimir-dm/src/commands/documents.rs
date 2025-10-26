//! Document management commands

use mimir_dm_core::{
    services::DocumentService,
    models::campaign::documents::{Document, NewDocument, UpdateDocument},
    DatabaseService,
};
use std::sync::Arc;
use tauri::State;
use crate::types::ApiResponse;
use serde::Serialize;

/// Get all documents for a campaign
#[tauri::command]
pub async fn get_campaign_documents(
    campaign_id: i32,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<ApiResponse<Vec<Document>>, String> {
    let mut conn = db_service.get_connection().map_err(|e| e.to_string())?;
    let mut service = DocumentService::new(&mut *conn);

    match service.get_campaign_documents(campaign_id) {
        Ok(documents) => Ok(ApiResponse::success(documents)),
        Err(e) => Ok(ApiResponse::error(format!("Failed to load documents: {}", e))),
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
    let mut conn = db_service.get_connection().map_err(|e| e.to_string())?;
    let mut service = DocumentService::new(&mut *conn);

    match service.get_documents_by_level(campaign_id, &level, module_id, session_id) {
        Ok(docs) => Ok(ApiResponse::success(docs)),
        Err(e) => Ok(ApiResponse::error(format!("Failed to load documents: {}", e))),
    }
}

/// Create a new document
#[tauri::command]
pub async fn create_document(
    new_document: NewDocument,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<ApiResponse<Document>, String> {
    let mut conn = db_service.get_connection().map_err(|e| e.to_string())?;
    let mut service = DocumentService::new(&mut *conn);

    match service.create_document(new_document) {
        Ok(document) => Ok(ApiResponse::success(document)),
        Err(e) => Ok(ApiResponse::error(format!("Failed to create document: {}", e))),
    }
}

/// Update a document
#[tauri::command]
pub async fn update_document(
    document_id: i32,
    update: UpdateDocument,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<ApiResponse<Document>, String> {
    let mut conn = db_service.get_connection().map_err(|e| e.to_string())?;
    let mut service = DocumentService::new(&mut *conn);

    match service.update_document(document_id, update) {
        Ok(document) => Ok(ApiResponse::success(document)),
        Err(e) => Ok(ApiResponse::error(format!("Failed to update document: {}", e))),
    }
}

/// Mark a document as completed
#[tauri::command]
pub async fn complete_document(
    document_id: i32,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<ApiResponse<Document>, String> {
    let mut conn = db_service.get_connection().map_err(|e| e.to_string())?;
    let mut service = DocumentService::new(&mut *conn);

    match service.complete_document(document_id) {
        Ok(document) => Ok(ApiResponse::success(document)),
        Err(e) => Ok(ApiResponse::error(format!("Failed to complete document: {}", e))),
    }
}

/// Delete a document
#[tauri::command]
pub async fn delete_document(
    document_id: i32,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<ApiResponse<()>, String> {
    let mut conn = db_service.get_connection().map_err(|e| e.to_string())?;
    let mut service = DocumentService::new(&mut *conn);

    match service.delete_document(document_id) {
        Ok(_) => Ok(ApiResponse::success(())),
        Err(e) => Ok(ApiResponse::error(format!("Failed to delete document: {}", e))),
    }
}

/// Get incomplete documents for a campaign
#[tauri::command]
pub async fn get_incomplete_documents(
    campaign_id: i32,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<ApiResponse<Vec<Document>>, String> {
    let mut conn = db_service.get_connection().map_err(|e| e.to_string())?;
    let mut service = DocumentService::new(&mut *conn);

    match service.get_incomplete_documents(campaign_id) {
        Ok(documents) => Ok(ApiResponse::success(documents)),
        Err(e) => Ok(ApiResponse::error(format!("Failed to load incomplete documents: {}", e))),
    }
}

/// Get completed documents for a campaign
#[tauri::command]
pub async fn get_completed_documents(
    campaign_id: i32,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<ApiResponse<Vec<Document>>, String> {
    let mut conn = db_service.get_connection().map_err(|e| e.to_string())?;
    let mut service = DocumentService::new(&mut *conn);

    match service.get_completed_documents(campaign_id) {
        Ok(documents) => Ok(ApiResponse::success(documents)),
        Err(e) => Ok(ApiResponse::error(format!("Failed to load completed documents: {}", e))),
    }
}

/// Create a document from a template
#[tauri::command]
pub async fn create_document_from_template(
    campaign_id: i32,
    template_id: String,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<ApiResponse<Document>, String> {
    let mut conn = db_service.get_connection().map_err(|e| e.to_string())?;
    let mut service = DocumentService::new(&mut *conn);

    match service.create_document_from_template(campaign_id, &template_id) {
        Ok(document) => Ok(ApiResponse::success(document)),
        Err(e) => Ok(ApiResponse::error(format!("Failed to create document from template: {}", e))),
    }
}

/// Read a document file from disk
#[tauri::command]
pub async fn read_document_file(
    file_path: String,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<ApiResponse<String>, String> {
    let mut conn = db_service.get_connection().map_err(|e| e.to_string())?;
    let service = DocumentService::new(&mut *conn);

    match service.read_document_file(&file_path) {
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
    let mut conn = db_service.get_connection().map_err(|e| e.to_string())?;
    let service = DocumentService::new(&mut *conn);

    match service.save_document_file(&file_path, &content) {
        Ok(_) => Ok(ApiResponse::success(())),
        Err(e) => Ok(ApiResponse::error(format!("Failed to save document: {}", e))),
    }
}

#[derive(Serialize, Debug)]
#[allow(dead_code)]
pub struct StageCompletionStatus {
    pub current_stage: String,
    pub total_documents: usize,
    pub completed_documents: usize,
    pub is_complete: bool,
    pub can_progress: bool,
    pub next_stage: Option<String>,
    pub missing_documents: Vec<String>,
}

