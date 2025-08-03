//! Document management commands

use mimir_dm_db::{
    dal::documents::DocumentRepository,
    models::documents::{Document, NewDocument, UpdateDocument},
};
use std::sync::Arc;
use tauri::State;

use crate::{
    services::database::DatabaseService,
    types::ApiResponse,
};

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