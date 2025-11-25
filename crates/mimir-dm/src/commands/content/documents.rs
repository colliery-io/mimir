//! Document management commands.
//!
//! Provides Tauri commands for creating, updating, and managing campaign documents.
//! Documents include session notes, module plans, campaign materials, and player handouts.

use crate::state::AppState;
use crate::types::{ApiError, ApiResponse};
use mimir_dm_core::{
    models::campaign::documents::{Document, NewDocument, UpdateDocument},
    services::DocumentService,
};
use serde::Serialize;
use tauri::State;

/// Get all documents for a campaign.
///
/// # Parameters
/// - `campaign_id` - The database ID of the campaign
/// - `state` - Application state containing the database connection
///
/// # Returns
/// `ApiResponse` containing a vector of `Document` objects.
///
/// # Errors
/// Returns an error response if database operations fail.
#[tauri::command]
pub async fn get_campaign_documents(
    campaign_id: i32,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<Document>>, ApiError> {
    let mut conn = state.db.get_connection()?;
    let mut service = DocumentService::new(&mut conn);

    match service.get_campaign_documents(campaign_id) {
        Ok(documents) => Ok(ApiResponse::success(documents)),
        Err(e) => Ok(ApiResponse::error(format!(
            "Failed to load documents: {}",
            e
        ))),
    }
}

/// Get documents by level (campaign, module, session, or handout).
///
/// Filters documents by their scope within the campaign hierarchy.
///
/// # Parameters
/// - `campaign_id` - The database ID of the campaign
/// - `level` - Document scope level ("campaign", "module", "session", "handout")
/// - `module_id` - Optional module ID filter for module/session level docs
/// - `session_id` - Optional session ID filter for session level docs
/// - `state` - Application state containing the database connection
///
/// # Returns
/// `ApiResponse` containing a vector of `Document` objects.
///
/// # Errors
/// Returns an error response if database operations fail.
#[tauri::command]
pub async fn get_documents_by_level(
    campaign_id: i32,
    level: String,
    module_id: Option<i32>,
    session_id: Option<i32>,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<Document>>, ApiError> {
    let mut conn = state.db.get_connection()?;
    let mut service = DocumentService::new(&mut conn);

    match service.get_documents_by_level(campaign_id, &level, module_id, session_id) {
        Ok(docs) => Ok(ApiResponse::success(docs)),
        Err(e) => Ok(ApiResponse::error(format!(
            "Failed to load documents: {}",
            e
        ))),
    }
}

/// Create a new document.
///
/// # Parameters
/// - `new_document` - Document creation data with campaign, template, and file info
/// - `state` - Application state containing the database connection
///
/// # Returns
/// `ApiResponse` containing the created `Document`.
///
/// # Errors
/// Returns an error response if database operations fail.
#[tauri::command]
pub async fn create_document(
    new_document: NewDocument,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Document>, ApiError> {
    let mut conn = state.db.get_connection()?;
    let mut service = DocumentService::new(&mut conn);

    match service.create_document(new_document) {
        Ok(document) => Ok(ApiResponse::success(document)),
        Err(e) => Ok(ApiResponse::error(format!(
            "Failed to create document: {}",
            e
        ))),
    }
}

/// Update a document.
///
/// # Parameters
/// - `document_id` - The database ID of the document to update
/// - `update` - Fields to update on the document
/// - `state` - Application state containing the database connection
///
/// # Returns
/// `ApiResponse` containing the updated `Document`.
///
/// # Errors
/// Returns an error response if database operations fail.
#[tauri::command]
pub async fn update_document(
    document_id: i32,
    update: UpdateDocument,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Document>, ApiError> {
    let mut conn = state.db.get_connection()?;
    let mut service = DocumentService::new(&mut conn);

    match service.update_document(document_id, update) {
        Ok(document) => Ok(ApiResponse::success(document)),
        Err(e) => Ok(ApiResponse::error(format!(
            "Failed to update document: {}",
            e
        ))),
    }
}

/// Mark a document as completed.
///
/// Sets the document's completion status, which affects stage progression.
///
/// # Parameters
/// - `document_id` - The database ID of the document to complete
/// - `state` - Application state containing the database connection
///
/// # Returns
/// `ApiResponse` containing the updated `Document`.
///
/// # Errors
/// Returns an error response if database operations fail.
#[tauri::command]
pub async fn complete_document(
    document_id: i32,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Document>, ApiError> {
    let mut conn = state.db.get_connection()?;
    let mut service = DocumentService::new(&mut conn);

    match service.complete_document(document_id) {
        Ok(document) => Ok(ApiResponse::success(document)),
        Err(e) => Ok(ApiResponse::error(format!(
            "Failed to complete document: {}",
            e
        ))),
    }
}

/// Delete a document.
///
/// Removes the document record from the database.
///
/// # Parameters
/// - `document_id` - The database ID of the document to delete
/// - `state` - Application state containing the database connection
///
/// # Returns
/// `ApiResponse` with success or error status.
///
/// # Errors
/// Returns an error response if database operations fail.
#[tauri::command]
pub async fn delete_document(
    document_id: i32,
    state: State<'_, AppState>,
) -> Result<ApiResponse<()>, ApiError> {
    let mut conn = state.db.get_connection()?;
    let mut service = DocumentService::new(&mut conn);

    match service.delete_document(document_id) {
        Ok(_) => Ok(ApiResponse::success(())),
        Err(e) => Ok(ApiResponse::error(format!(
            "Failed to delete document: {}",
            e
        ))),
    }
}

/// Get incomplete documents for a campaign.
///
/// Returns documents that have not been marked as completed.
///
/// # Parameters
/// - `campaign_id` - The database ID of the campaign
/// - `state` - Application state containing the database connection
///
/// # Returns
/// `ApiResponse` containing a vector of incomplete `Document` objects.
///
/// # Errors
/// Returns an error response if database operations fail.
#[tauri::command]
pub async fn get_incomplete_documents(
    campaign_id: i32,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<Document>>, ApiError> {
    let mut conn = state.db.get_connection()?;
    let mut service = DocumentService::new(&mut conn);

    match service.get_incomplete_documents(campaign_id) {
        Ok(documents) => Ok(ApiResponse::success(documents)),
        Err(e) => Ok(ApiResponse::error(format!(
            "Failed to load incomplete documents: {}",
            e
        ))),
    }
}

/// Get completed documents for a campaign.
///
/// Returns documents that have been marked as completed.
///
/// # Parameters
/// - `campaign_id` - The database ID of the campaign
/// - `state` - Application state containing the database connection
///
/// # Returns
/// `ApiResponse` containing a vector of completed `Document` objects.
///
/// # Errors
/// Returns an error response if database operations fail.
#[tauri::command]
pub async fn get_completed_documents(
    campaign_id: i32,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<Document>>, ApiError> {
    let mut conn = state.db.get_connection()?;
    let mut service = DocumentService::new(&mut conn);

    match service.get_completed_documents(campaign_id) {
        Ok(documents) => Ok(ApiResponse::success(documents)),
        Err(e) => Ok(ApiResponse::error(format!(
            "Failed to load completed documents: {}",
            e
        ))),
    }
}

/// Create a document from a template.
///
/// Generates a new document using the specified template's structure and content.
///
/// # Parameters
/// - `campaign_id` - The database ID of the campaign
/// - `template_id` - The ID of the template to use
/// - `state` - Application state containing the database connection
///
/// # Returns
/// `ApiResponse` containing the created `Document`.
///
/// # Errors
/// Returns an error response if the template is not found or database operations fail.
#[tauri::command]
pub async fn create_document_from_template(
    campaign_id: i32,
    template_id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Document>, ApiError> {
    let mut conn = state.db.get_connection()?;
    let mut service = DocumentService::new(&mut conn);

    match service.create_document_from_template(campaign_id, &template_id) {
        Ok(document) => Ok(ApiResponse::success(document)),
        Err(e) => Ok(ApiResponse::error(format!(
            "Failed to create document from template: {}",
            e
        ))),
    }
}

/// Read a document file from disk.
///
/// Reads the content of a document's markdown file.
///
/// # Parameters
/// - `file_path` - Path to the document file on disk
/// - `state` - Application state containing the database connection
///
/// # Returns
/// `ApiResponse` containing the file content as a string.
///
/// # Errors
/// Returns an error response if the file cannot be read.
#[tauri::command]
pub async fn read_document_file(
    file_path: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<String>, ApiError> {
    let mut conn = state.db.get_connection()?;
    let service = DocumentService::new(&mut conn);

    match service.read_document_file(&file_path) {
        Ok(content) => Ok(ApiResponse::success(content)),
        Err(e) => Ok(ApiResponse::error(format!(
            "Failed to read document: {}",
            e
        ))),
    }
}

/// Save a document file to disk.
///
/// Writes content to a document's markdown file.
///
/// # Parameters
/// - `file_path` - Path to the document file on disk
/// - `content` - The content to write to the file
/// - `state` - Application state containing the database connection
///
/// # Returns
/// `ApiResponse` with success or error status.
///
/// # Errors
/// Returns an error response if the file cannot be written.
#[tauri::command]
pub async fn save_document_file(
    file_path: String,
    content: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<()>, ApiError> {
    let mut conn = state.db.get_connection()?;
    let service = DocumentService::new(&mut conn);

    match service.save_document_file(&file_path, &content) {
        Ok(_) => Ok(ApiResponse::success(())),
        Err(e) => Ok(ApiResponse::error(format!(
            "Failed to save document: {}",
            e
        ))),
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
