//! Tool for updating document content
//! 
//! This tool allows the LLM to update the content of campaign/module/session documents.
//! It requires user confirmation before making changes.

use async_trait::async_trait;
use mimir_dm_llm::{ToolTrait, traits::{ActionDescription, RiskLevel}};
use mimir_dm_core::{
    dal::campaign::documents::DocumentRepository,
};
use serde_json::{json, Value};
use std::error::Error;
use std::sync::Arc;
use std::fs;
use std::path::Path;
use tracing::info;

use crate::services::database::DatabaseService;

/// Tool for updating document content
pub struct UpdateDocumentTool {
    db_service: Arc<DatabaseService>,
}

impl UpdateDocumentTool {
    pub fn new(db_service: Arc<DatabaseService>) -> Self {
        Self { db_service }
    }
}

#[async_trait]
impl ToolTrait for UpdateDocumentTool {
    fn name(&self) -> &str {
        "update_document"
    }
    
    fn description(&self) -> &str {
        "Update the content of a specific document by type and context (campaign/module/session). IMPORTANT: You must first use get_document to read and review the current content before updating. Requires user confirmation."
    }
    
    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "document_type": {
                    "type": "string",
                    "description": "Type of document (e.g., campaign_bible, session_plan, campaign_pitch)"
                },
                "campaign_id": {
                    "type": "integer",
                    "description": "Campaign ID (required)"
                },
                "module_id": {
                    "type": "integer",
                    "description": "Module ID (optional, for module-specific documents)"
                },
                "session_id": {
                    "type": "integer",
                    "description": "Session ID (optional, for session-specific documents)"
                },
                "content": {
                    "type": "string",
                    "description": "The new content for the document (markdown format). Must have used get_document first to review current content."
                }
            },
            "required": ["document_type", "campaign_id", "content"]
        })
    }
    
    fn requires_confirmation(&self) -> bool {
        true // Always require confirmation for document updates
    }
    
    fn describe_action(&self, arguments: &Value) -> Option<ActionDescription> {
        let document_type = arguments
            .get("document_type")?
            .as_str()?;
        
        let campaign_id = arguments
            .get("campaign_id")?
            .as_i64()?;
        
        let module_id = arguments
            .get("module_id")
            .and_then(|v| v.as_i64());
        
        let session_id = arguments
            .get("session_id")
            .and_then(|v| v.as_i64());
        
        let content = arguments
            .get("content")?
            .as_str()?;
        
        // Show full content in the confirmation
        let content_display = content.to_string();
        
        // Determine the context
        let context = if let Some(session_id) = session_id {
            format!("Session {}", session_id)
        } else if let Some(module_id) = module_id {
            format!("Module {}", module_id)
        } else {
            format!("Campaign {}", campaign_id)
        };
        
        Some(ActionDescription {
            title: format!("Update {} Document", document_type),
            description: format!(
                "This will update the content of the {} document for {}",
                document_type, context
            ),
            changes: vec![
                format!("Document type: {}", document_type),
                format!("Context: {}", context),
                format!("New content:\n{}", content_display),
            ],
            risk_level: RiskLevel::Medium, // Medium risk since we're modifying content
        })
    }
    
    async fn execute(&self, arguments: Value) -> Result<String, Box<dyn Error + Send + Sync>> {
        let document_type = arguments
            .get("document_type")
            .and_then(|v| v.as_str())
            .ok_or("Missing 'document_type' parameter")?;
        
        let campaign_id = arguments
            .get("campaign_id")
            .and_then(|v| v.as_i64())
            .ok_or("Missing 'campaign_id' parameter")? as i32;
        
        let module_id = arguments
            .get("module_id")
            .and_then(|v| v.as_i64())
            .map(|id| id as i32);
        
        let session_id = arguments
            .get("session_id")
            .and_then(|v| v.as_i64())
            .map(|id| id as i32);
        
        let content = arguments
            .get("content")
            .and_then(|v| v.as_str())
            .ok_or("Missing 'content' parameter")?;
        
        
        // Get database connection
        let mut conn = self.db_service.get_connection()
            .map_err(|e| format!("Database connection failed: {}", e))?;
        
        // Find documents based on context
        let documents = if let Some(session_id) = session_id {
            DocumentRepository::find_by_session(&mut conn, session_id)
                .map_err(|e| format!("Failed to query documents: {}", e))?
        } else if let Some(module_id) = module_id {
            DocumentRepository::find_by_module(&mut conn, module_id)
                .map_err(|e| format!("Failed to query documents: {}", e))?
        } else {
            DocumentRepository::find_by_campaign(&mut conn, campaign_id)
                .map_err(|e| format!("Failed to query documents: {}", e))?
        };
        
        // Find the specific document by type
        let document = documents
            .into_iter()
            .find(|doc| doc.document_type == document_type)
            .ok_or_else(|| format!("No {} document found for the specified context", document_type))?;
        
        // Write the new content to the file
        let file_path = Path::new(&document.file_path);
        
        // Create backup of existing content
        if file_path.exists() {
            let backup_path = file_path.with_extension("md.backup");
            fs::copy(&file_path, &backup_path)
                .map_err(|e| format!("Failed to create backup: {}", e))?;
        }
        
        // Write new content
        fs::write(&file_path, content)
            .map_err(|e| format!("Failed to write document file: {}", e))?;
        
        // Update the document's updated_at timestamp in the database
        use mimir_dm_core::models::campaign::documents::UpdateDocument;
        use chrono::Utc;
        
        let update = UpdateDocument {
            title: Some(document.title.clone()),
            completed_at: None, // Don't change completion status
            updated_at: Some(Utc::now().naive_utc().to_string()),
        };
        
        DocumentRepository::update(&mut conn, document.id, update)
            .map_err(|e| format!("Failed to update document metadata: {}", e))?;
        
        
        // Return a structured success response that the LLM can easily parse
        let result = serde_json::json!({
            "status": "success",
            "action": "document_updated",
            "details": {
                "document_type": document_type,
                "document_title": document.title,
                "context": if session_id.is_some() {
                    format!("Session {}", session_id.unwrap())
                } else if module_id.is_some() {
                    format!("Module {}", module_id.unwrap())
                } else {
                    format!("Campaign {}", campaign_id)
                },
                "file_path": document.file_path,
                "backup_path": format!("{}.backup", document.file_path),
            },
            "message": format!("The {} document has been successfully updated and saved.", document_type)
        });
        
        Ok(serde_json::to_string_pretty(&result).unwrap())
    }
}