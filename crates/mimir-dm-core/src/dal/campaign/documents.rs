//! Document repository for managing campaign documents

use crate::{
    connection::DbConnection,
    error::{DbError, Result},
    models::campaign::documents::{Document, NewDocument, UpdateDocument},
    schema::documents,
};
use diesel::prelude::*;

pub struct DocumentRepository;

impl DocumentRepository {
    /// Create a new document
    pub fn create(conn: &mut DbConnection, new_document: NewDocument) -> Result<Document> {
        diesel::insert_into(documents::table)
            .values(&new_document)
            .returning(Document::as_returning())
            .get_result(conn)
            .map_err(|e| DbError::Query(e))
    }

    /// Get document by ID
    pub fn find_by_id(conn: &mut DbConnection, document_id: i32) -> Result<Document> {
        documents::table
            .find(document_id)
            .first(conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => DbError::NotFound {
                    entity_type: "Document".to_string(),
                    id: document_id.to_string(),
                },
                _ => DbError::Query(e),
            })
    }

    /// Get all documents for a campaign
    pub fn find_by_campaign(conn: &mut DbConnection, campaign_id: i32) -> Result<Vec<Document>> {
        documents::table
            .filter(documents::campaign_id.eq(campaign_id))
            .order(documents::created_at.asc())
            .load(conn)
            .map_err(|e| DbError::Query(e))
    }

    /// Get all documents for a module
    pub fn find_by_module(conn: &mut DbConnection, module_id: i32) -> Result<Vec<Document>> {
        documents::table
            .filter(documents::module_id.eq(module_id))
            .order(documents::created_at.asc())
            .load(conn)
            .map_err(|e| DbError::Query(e))
    }

    /// Get all documents for a session
    pub fn find_by_session(conn: &mut DbConnection, session_id: i32) -> Result<Vec<Document>> {
        documents::table
            .filter(documents::session_id.eq(session_id))
            .order(documents::created_at.asc())
            .load(conn)
            .map_err(|e| DbError::Query(e))
    }

    /// Get all documents by template ID
    pub fn find_by_template(conn: &mut DbConnection, template_id: &str) -> Result<Vec<Document>> {
        documents::table
            .filter(documents::template_id.eq(template_id))
            .order(documents::created_at.asc())
            .load(conn)
            .map_err(|e| DbError::Query(e))
    }

    /// Get all incomplete documents for a campaign
    pub fn find_incomplete_by_campaign(conn: &mut DbConnection, campaign_id: i32) -> Result<Vec<Document>> {
        documents::table
            .filter(documents::campaign_id.eq(campaign_id))
            .filter(documents::completed_at.is_null())
            .order(documents::created_at.asc())
            .load(conn)
            .map_err(|e| DbError::Query(e))
    }

    /// Get all completed documents for a campaign
    pub fn find_completed_by_campaign(conn: &mut DbConnection, campaign_id: i32) -> Result<Vec<Document>> {
        documents::table
            .filter(documents::campaign_id.eq(campaign_id))
            .filter(documents::completed_at.is_not_null())
            .order(documents::created_at.asc())
            .load(conn)
            .map_err(|e| DbError::Query(e))
    }

    /// Update a document
    pub fn update(conn: &mut DbConnection, document_id: i32, update: UpdateDocument) -> Result<Document> {
        diesel::update(documents::table.find(document_id))
            .set(&update)
            .returning(Document::as_returning())
            .get_result(conn)
            .map_err(|e| DbError::Query(e))
    }

    /// Mark a document as completed
    pub fn mark_completed(conn: &mut DbConnection, document_id: i32) -> Result<Document> {
        let update = UpdateDocument {
            title: None,
            updated_at: Some(chrono::Utc::now().to_rfc3339()),
            completed_at: Some(chrono::Utc::now().to_rfc3339()),
        };
        
        Self::update(conn, document_id, update)
    }

    /// Delete a document
    pub fn delete(conn: &mut DbConnection, document_id: i32) -> Result<usize> {
        diesel::delete(documents::table.find(document_id))
            .execute(conn)
            .map_err(|e| DbError::Query(e))
    }

    /// Check if a document exists by file path
    pub fn exists_by_path(conn: &mut DbConnection, file_path: &str) -> Result<bool> {
        let count = documents::table
            .filter(documents::file_path.eq(file_path))
            .count()
            .get_result::<i64>(conn)
            .map_err(|e| DbError::Query(e))?;
        
        Ok(count > 0)
    }

    /// Get all handout documents for a campaign
    pub fn find_handouts_by_campaign(conn: &mut DbConnection, campaign_id: i32) -> Result<Vec<Document>> {
        documents::table
            .filter(documents::campaign_id.eq(campaign_id))
            .filter(documents::document_type.eq("handout"))
            .order(documents::created_at.asc())
            .load(conn)
            .map_err(|e| DbError::Query(e))
    }
}