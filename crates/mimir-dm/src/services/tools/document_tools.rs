//! Document management tools for LLM
//! 
//! Tools for retrieving and listing campaign/module/session documents

use async_trait::async_trait;
use mimir_dm_llm::ToolTrait;
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

/// Tool for retrieving specific document content
pub struct GetDocumentTool {
    db_service: Arc<DatabaseService>,
}

impl GetDocumentTool {
    pub fn new(db_service: Arc<DatabaseService>) -> Self {
        Self { db_service }
    }
}

#[async_trait]
impl ToolTrait for GetDocumentTool {
    fn name(&self) -> &str {
        "get_document"
    }
    
    fn description(&self) -> &str {
        "Retrieve the content of a specific document by type and context (campaign/module/session)"
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
                }
            },
            "required": ["document_type", "campaign_id"]
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
        
        // Read the file content
        let file_path = Path::new(&document.file_path);
        if !file_path.exists() {
            return Err(format!("Document file not found: {}", document.file_path).into());
        }
        
        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read document file: {}", e))?;
        
        // Return content with metadata header
        Ok(format!(
            "# {}\n\n**Type:** {}\n**Status:** {}\n**Last Updated:** {}\n\n---\n\n{}",
            document.title,
            document.document_type,
            if document.completed_at.is_some() { "Completed" } else { "In Progress" },
            document.updated_at,
            content
        ))
    }
}

/// Tool for listing documents in a context
pub struct ListDocumentsTool {
    db_service: Arc<DatabaseService>,
}

impl ListDocumentsTool {
    pub fn new(db_service: Arc<DatabaseService>) -> Self {
        Self { db_service }
    }
}

#[async_trait]
impl ToolTrait for ListDocumentsTool {
    fn name(&self) -> &str {
        "list_documents"
    }
    
    fn description(&self) -> &str {
        "List all documents for a given campaign, module, or session"
    }
    
    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "campaign_id": {
                    "type": "integer",
                    "description": "Campaign ID (required)"
                },
                "module_id": {
                    "type": "integer",
                    "description": "Module ID (optional, filter by module)"
                },
                "session_id": {
                    "type": "integer",
                    "description": "Session ID (optional, filter by session)"
                },
                "completed_only": {
                    "type": "boolean",
                    "description": "Only show completed documents (default: false)"
                }
            },
            "required": ["campaign_id"]
        })
    }
    
    async fn execute(&self, arguments: Value) -> Result<String, Box<dyn Error + Send + Sync>> {
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
        
        let completed_only = arguments
            .get("completed_only")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        
        
        // Get database connection
        let mut conn = self.db_service.get_connection()
            .map_err(|e| format!("Database connection failed: {}", e))?;
        
        // Get documents based on context
        let mut documents = if let Some(session_id) = session_id {
            DocumentRepository::find_by_session(&mut conn, session_id)
                .map_err(|e| format!("Failed to query documents: {}", e))?
        } else if let Some(module_id) = module_id {
            DocumentRepository::find_by_module(&mut conn, module_id)
                .map_err(|e| format!("Failed to query documents: {}", e))?
        } else if completed_only {
            DocumentRepository::find_completed_by_campaign(&mut conn, campaign_id)
                .map_err(|e| format!("Failed to query documents: {}", e))?
        } else {
            DocumentRepository::find_by_campaign(&mut conn, campaign_id)
                .map_err(|e| format!("Failed to query documents: {}", e))?
        };
        
        // Filter by completed status if requested (and not already filtered)
        if completed_only && session_id.is_none() && module_id.is_some() {
            documents.retain(|doc| doc.completed_at.is_some());
        }
        
        if documents.is_empty() {
            return Ok("No documents found for the specified criteria.".to_string());
        }
        
        // Format the document list
        let mut output = format!("Found {} document(s):\n\n", documents.len());
        
        for doc in documents {
            let level = if doc.session_id.is_some() {
                format!("Session {}", doc.session_id.unwrap())
            } else if doc.module_id.is_some() {
                format!("Module {}", doc.module_id.unwrap())
            } else {
                "Campaign".to_string()
            };
            
            let status = if doc.completed_at.is_some() {
                "✓ Completed"
            } else {
                "○ In Progress"
            };
            
            output.push_str(&format!(
                "- **{}** ({}) - {} - {}\n",
                doc.title,
                doc.document_type,
                level,
                status
            ));
        }
        
        Ok(output)
    }
}

