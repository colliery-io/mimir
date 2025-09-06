//! Tool for updating document content
//! 
//! This tool allows the LLM to update the content of campaign/module/session documents.
//! It requires user confirmation before making changes.

use async_trait::async_trait;
use mimir_dm_llm::{ToolTrait, traits::ActionDescription};
use mimir_dm_core::{
    dal::campaign::documents::DocumentRepository,
};
use serde_json::{json, Value};
use std::error::Error;
use std::sync::Arc;
use std::fs;
use std::path::Path;
use similar::{ChangeTag, TextDiff};

use crate::services::database::DatabaseService;

/// Generate a diff display between current and new content
fn generate_diff_display(current_content: &str, new_content: &str) -> String {
    let diff = TextDiff::from_lines(current_content, new_content);
    let mut diff_output = String::new();
    let mut line_count = 0;
    const MAX_LINES: usize = 50;
    
    for change in diff.iter_all_changes() {
        if line_count >= MAX_LINES {
            diff_output.push_str(&format!("\n... ({} more lines changed) ...", 
                diff.iter_all_changes().count() - line_count));
            break;
        }
        
        let sign = match change.tag() {
            ChangeTag::Delete => "- ",
            ChangeTag::Insert => "+ ",
            ChangeTag::Equal => "  ",
        };
        
        // Only show changed lines and minimal context
        match change.tag() {
            ChangeTag::Delete | ChangeTag::Insert => {
                diff_output.push_str(&format!("{}{}", sign, change));
                line_count += 1;
            },
            ChangeTag::Equal => {
                // Show context lines (unchanged lines around changes)
                let line = change.to_string();
                if !line.trim().is_empty() && line.len() < 100 {
                    diff_output.push_str(&format!("{}{}", sign, change));
                    line_count += 1;
                }
            }
        }
    }
    
    // Check if there are actually any changes (insertions/deletions)
    let has_changes = diff.iter_all_changes().any(|change| {
        matches!(change.tag(), ChangeTag::Delete | ChangeTag::Insert)
    });
    
    if !has_changes {
        "No changes detected".to_string()
    } else if diff_output.trim().is_empty() {
        "No changes detected".to_string()
    } else {
        format!("```diff\n{}\n```", diff_output)
    }
}

/// Tool for updating document content
pub struct UpdateDocumentTool {
    db_service: Arc<DatabaseService>,
}

impl UpdateDocumentTool {
    pub fn new(db_service: Arc<DatabaseService>) -> Self {
        Self { db_service }
    }
    
    /// Read current document content for diff generation
    fn read_current_content(
        &self,
        document_type: &str,
        campaign_id: i32,
        module_id: Option<i32>,
        session_id: Option<i32>,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
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
        
        Ok(content)
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
            .as_i64()? as i32;
        
        let module_id = arguments
            .get("module_id")
            .and_then(|v| v.as_i64())
            .map(|id| id as i32);
        
        let session_id = arguments
            .get("session_id")
            .and_then(|v| v.as_i64())
            .map(|id| id as i32);
        
        let new_content = arguments
            .get("content")?
            .as_str()?;
        
        // Determine the context
        let context = if let Some(session_id) = session_id {
            format!("Session {}", session_id)
        } else if let Some(module_id) = module_id {
            format!("Module {}", module_id)
        } else {
            format!("Campaign {}", campaign_id)
        };
        
        // Try to generate a diff display
        let changes_display = match self.read_current_content(document_type, campaign_id, module_id, session_id) {
            Ok(current_content) => {
                generate_diff_display(&current_content, new_content)
            },
            Err(_) => {
                // Fallback to showing new content info if we can't read current
                format!("New content ({} characters):\n{}", 
                    new_content.len(),
                    if new_content.len() > 500 {
                        format!("{}...\n\n[Content truncated for display]", &new_content[..500])
                    } else {
                        new_content.to_string()
                    }
                )
            }
        };
        
        Some(ActionDescription {
            title: format!("Update {} Document", document_type),
            description: format!(
                "This will update the {} document for {}",
                document_type, context
            ),
            changes: vec![
                format!("Document: {} ({})", document_type, context),
                changes_display,
            ],
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_diff_display_simple_change() {
        let current = "# Campaign Bible\n\nThe heroes start in Frosthaven.\nThey must save the world.";
        let new_content = "# Campaign Bible\n\nThe heroes start in a pizza shop.\nThey must save the world.";
        
        let result = generate_diff_display(current, new_content);
        
        assert!(result.contains("```diff"));
        assert!(result.contains("- The heroes start in Frosthaven."));
        assert!(result.contains("+ The heroes start in a pizza shop."));
        assert!(result.contains("  They must save the world."));
    }

    #[test]
    fn test_generate_diff_display_no_changes() {
        let content = "# Campaign Bible\n\nNo changes here.";
        
        let result = generate_diff_display(content, content);
        
        assert_eq!(result, "No changes detected");
    }

    #[test]
    fn test_generate_diff_display_addition_only() {
        let current = "# Campaign Bible\n\nOriginal content.";
        let new_content = "# Campaign Bible\n\nOriginal content.\nNew additional content.";
        
        let result = generate_diff_display(current, new_content);
        
        assert!(result.contains("```diff"));
        assert!(result.contains("+ New additional content."));
        assert!(result.contains("  # Campaign Bible"));
    }

    #[test]
    fn test_generate_diff_display_deletion_only() {
        let current = "# Campaign Bible\n\nOriginal content.\nContent to remove.";
        let new_content = "# Campaign Bible\n\nOriginal content.";
        
        let result = generate_diff_display(current, new_content);
        
        assert!(result.contains("```diff"));
        assert!(result.contains("- Content to remove."));
        assert!(result.contains("  # Campaign Bible"));
    }

    #[test]
    fn test_generate_diff_display_large_diff_truncation() {
        // Create content with more than 50 lines of changes
        let mut current_lines = Vec::new();
        let mut new_lines = Vec::new();
        
        for i in 0..60 {
            current_lines.push(format!("Old line {}", i));
            new_lines.push(format!("New line {}", i));
        }
        
        let current = current_lines.join("\n");
        let new_content = new_lines.join("\n");
        
        let result = generate_diff_display(&current, &new_content);
        
        assert!(result.contains("```diff"));
        assert!(result.contains("more lines changed"));
        // Should truncate at 50 lines
        assert!(result.matches('\n').count() < 60); // Much fewer than the 120+ lines we'd get without truncation
    }

    #[test]
    fn test_generate_diff_display_mixed_changes() {
        let current = "# Campaign\n\nLine 1\nLine to change\nLine 3\nLine to remove\nLine 5";
        let new_content = "# Campaign\n\nLine 1\nChanged line\nLine 3\nLine 5\nNew line";
        
        let result = generate_diff_display(current, new_content);
        
        assert!(result.contains("```diff"));
        assert!(result.contains("- Line to change"));
        assert!(result.contains("+ Changed line"));
        assert!(result.contains("- Line to remove"));
        assert!(result.contains("+ New line"));
        assert!(result.contains("  Line 1"));
        assert!(result.contains("  Line 3"));
    }

    #[test]
    fn test_generate_diff_display_context_filtering() {
        // Create content with long lines that should be filtered out of context
        let current = format!(
            "Short line\n{}\nAnother short line", 
            "A".repeat(150) // Very long line that should be filtered from context
        );
        let new_content = format!(
            "Short line changed\n{}\nAnother short line", 
            "A".repeat(150)
        );
        
        let result = generate_diff_display(&current, &new_content);
        
        assert!(result.contains("- Short line"));
        assert!(result.contains("+ Short line changed"));
        // Long line should not appear in context due to length filter
        assert!(!result.contains(&"A".repeat(150)));
    }

    #[test]
    fn test_generate_diff_display_empty_strings() {
        let result = generate_diff_display("", "New content");
        assert!(result.contains("+ New content"));
        
        let result = generate_diff_display("Old content", "");
        assert!(result.contains("- Old content"));
    }

    #[test]
    fn test_generate_diff_display_whitespace_changes() {
        let current = "Line with spaces   \nSecond line";
        let new_content = "Line with spaces\nSecond line";
        
        let result = generate_diff_display(current, new_content);
        
        assert!(result.contains("```diff"));
        // Should detect whitespace changes
        assert!(result.contains("- Line with spaces   "));
        assert!(result.contains("+ Line with spaces"));
    }
}