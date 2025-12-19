//! Document management service.
//!
//! Provides business logic for managing campaign documents. Documents include
//! campaign-level materials, module planning docs, session notes, and player
//! handouts. Handles template rendering, file system operations, and database
//! record management.

use crate::connection::DbConnection;
use crate::dal::campaign::{
    campaigns::CampaignRepository, documents::DocumentRepository,
    template_documents::TemplateRepository,
};
use crate::error::Result;
use crate::models::campaign::documents::{Document, NewDocument, UpdateDocument};
use std::fs;
use std::path::PathBuf;

/// Service for managing documents
pub struct DocumentService<'a> {
    conn: &'a mut DbConnection,
}

impl<'a> DocumentService<'a> {
    /// Create a new document service.
    ///
    /// # Arguments
    /// * `conn` - Mutable reference to the database connection
    pub fn new(conn: &'a mut DbConnection) -> Self {
        Self { conn }
    }

    /// Get all documents for a campaign.
    ///
    /// # Arguments
    /// * `campaign_id` - Database ID of the campaign
    ///
    /// # Returns
    /// * `Ok(Vec<Document>)` - All documents associated with the campaign
    pub fn get_campaign_documents(&mut self, campaign_id: i32) -> Result<Vec<Document>> {
        DocumentRepository::find_by_campaign(self.conn, campaign_id)
    }

    /// Get documents by level with filtering logic.
    ///
    /// Filters documents by their scope in the campaign hierarchy.
    ///
    /// # Arguments
    /// * `campaign_id` - Database ID of the campaign
    /// * `level` - Document scope: "campaign", "module", "session", or "handout"
    /// * `module_id` - Required for "module" level filtering
    /// * `session_id` - Required for "session" level filtering
    ///
    /// # Returns
    /// * `Ok(Vec<Document>)` - Documents matching the level and filters
    /// * `Err(DbError::InvalidData)` - If level is invalid
    pub fn get_documents_by_level(
        &mut self,
        campaign_id: i32,
        level: &str,
        module_id: Option<i32>,
        session_id: Option<i32>,
    ) -> Result<Vec<Document>> {
        match level {
            "campaign" => {
                // Get campaign-level documents (no module or session id)
                DocumentRepository::find_by_campaign(self.conn, campaign_id).map(|docs| {
                    docs.into_iter()
                        .filter(|d| {
                            d.module_id.is_none()
                                && d.session_id.is_none()
                                && d.document_type != "handout"
                        })
                        .collect()
                })
            }
            "module" => {
                if let Some(mid) = module_id {
                    DocumentRepository::find_by_module(self.conn, mid)
                } else {
                    Ok(vec![])
                }
            }
            "session" => {
                if let Some(sid) = session_id {
                    DocumentRepository::find_by_session(self.conn, sid)
                } else {
                    Ok(vec![])
                }
            }
            "handout" => DocumentRepository::find_handouts_by_campaign(self.conn, campaign_id),
            _ => Err(crate::error::DbError::InvalidData(format!(
                "Invalid document level: {}",
                level
            ))),
        }
    }

    /// Create a new document.
    ///
    /// Creates a database record for an existing document file. Does not
    /// create the file on disk.
    ///
    /// # Arguments
    /// * `new_document` - Document creation data
    ///
    /// # Returns
    /// * `Ok(Document)` - The created document record
    pub fn create_document(&mut self, new_document: NewDocument) -> Result<Document> {
        DocumentRepository::create(self.conn, new_document)
    }

    /// Update a document.
    ///
    /// # Arguments
    /// * `document_id` - Database ID of the document
    /// * `update` - Fields to update (None fields are left unchanged)
    ///
    /// # Returns
    /// * `Ok(Document)` - The updated document record
    pub fn update_document(
        &mut self,
        document_id: i32,
        update: UpdateDocument,
    ) -> Result<Document> {
        DocumentRepository::update(self.conn, document_id, update)
    }

    /// Mark a document as completed.
    ///
    /// Sets the completed_at timestamp, which affects stage progression checks.
    ///
    /// # Arguments
    /// * `document_id` - Database ID of the document
    ///
    /// # Returns
    /// * `Ok(Document)` - The updated document with completed_at set
    pub fn complete_document(&mut self, document_id: i32) -> Result<Document> {
        DocumentRepository::mark_completed(self.conn, document_id)
    }

    /// Delete a document.
    ///
    /// Removes the database record. Does not delete the file on disk.
    ///
    /// # Arguments
    /// * `document_id` - Database ID of the document
    ///
    /// # Returns
    /// * `Ok(())` - If deletion succeeds
    pub fn delete_document(&mut self, document_id: i32) -> Result<()> {
        DocumentRepository::delete(self.conn, document_id).map(|_| ())
    }

    /// Get incomplete documents for a campaign.
    ///
    /// Returns documents that have not been marked as completed.
    ///
    /// # Arguments
    /// * `campaign_id` - Database ID of the campaign
    ///
    /// # Returns
    /// * `Ok(Vec<Document>)` - Documents where completed_at is NULL
    pub fn get_incomplete_documents(&mut self, campaign_id: i32) -> Result<Vec<Document>> {
        DocumentRepository::find_incomplete_by_campaign(self.conn, campaign_id)
    }

    /// Get completed documents for a campaign.
    ///
    /// Returns documents that have been marked as completed.
    ///
    /// # Arguments
    /// * `campaign_id` - Database ID of the campaign
    ///
    /// # Returns
    /// * `Ok(Vec<Document>)` - Documents where completed_at is set
    pub fn get_completed_documents(&mut self, campaign_id: i32) -> Result<Vec<Document>> {
        DocumentRepository::find_completed_by_campaign(self.conn, campaign_id)
    }

    /// Create a document from a template.
    ///
    /// Retrieves the template, renders it with default values, writes the file
    /// to the campaign directory, and creates the database record.
    ///
    /// # Arguments
    /// * `campaign_id` - Database ID of the campaign
    /// * `template_id` - ID of the template to use
    ///
    /// # Returns
    /// * `Ok(Document)` - The created document record
    /// * `Err(DbError::NotFound)` - If campaign or template not found
    /// * `Err(DbError::InvalidData)` - If document already exists
    pub fn create_document_from_template(
        &mut self,
        campaign_id: i32,
        template_id: &str,
    ) -> Result<Document> {
        // Get the campaign
        let mut campaign_repo = CampaignRepository::new(self.conn);
        let campaign = campaign_repo.find_by_id(campaign_id)?.ok_or_else(|| {
            crate::error::DbError::NotFound {
                entity_type: "Campaign".to_string(),
                id: campaign_id.to_string(),
            }
        })?;

        // Check if document already exists
        let existing = DocumentRepository::find_by_campaign(self.conn, campaign_id)?;
        if existing.iter().any(|d| d.template_id == template_id) {
            return Err(crate::error::DbError::InvalidData(
                "Document already exists".into(),
            ));
        }

        // Get the template
        let template = TemplateRepository::get_latest(self.conn, template_id)?;

        // Create the document file
        let file_name = format!("{}.md", template_id);
        let file_path = PathBuf::from(&campaign.directory_path).join(&file_name);

        // Process template using the create_context method
        let context = template.create_context();
        let mut tera = tera::Tera::default();
        tera.add_raw_template(&template.document_id, &template.document_content)
            .map_err(|e| {
                crate::error::DbError::InvalidData(format!("Failed to add template: {}", e))
            })?;

        let template_content = tera.render(&template.document_id, &context).map_err(|e| {
            crate::error::DbError::InvalidData(format!("Failed to render template: {}", e))
        })?;

        // Generate title from template_id (e.g., "campaign_pitch" -> "Campaign Pitch")
        let title = template_id
            .split('_')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ");

        // Document type uses underscores (e.g., "campaign_pitch")
        let document_type = template_id.replace('-', "_");

        // Add YAML frontmatter with title and type
        let content_with_frontmatter = format!(
            "---\ntitle: \"{}\"\ntype: {}\n---\n\n{}",
            title, document_type, template_content
        );

        // Write file to disk with frontmatter
        fs::write(&file_path, content_with_frontmatter)?;

        let new_doc = NewDocument {
            campaign_id: campaign.id,
            module_id: None,
            session_id: None,
            template_id: template_id.to_string(),
            document_type,
            title,
            file_path: file_path.to_string_lossy().to_string(),
        };

        DocumentRepository::create(self.conn, new_doc)
    }

    /// Read a document file from disk.
    ///
    /// # Arguments
    /// * `file_path` - Absolute path to the document file
    ///
    /// # Returns
    /// * `Ok(String)` - The file contents
    /// * `Err(DbError::NotFound)` - If the file does not exist
    /// * `Err(DbError::Io)` - If reading fails
    pub fn read_document_file(&self, file_path: &str) -> Result<String> {
        let path = PathBuf::from(file_path);

        // Check if file exists
        if !path.exists() {
            return Err(crate::error::DbError::NotFound {
                entity_type: "Document file".to_string(),
                id: file_path.to_string(),
            });
        }

        // Read the markdown file directly
        Ok(fs::read_to_string(&path)?)
    }

    /// Save a document file to disk.
    ///
    /// Creates parent directories if they don't exist.
    ///
    /// # Arguments
    /// * `file_path` - Absolute path to the document file
    /// * `content` - The content to write
    ///
    /// # Returns
    /// * `Ok(())` - If writing succeeds
    /// * `Err(DbError::Io)` - If writing fails
    pub fn save_document_file(&self, file_path: &str, content: &str) -> Result<()> {
        let path = PathBuf::from(file_path);

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        // Write the file
        fs::write(&path, content)?;
        Ok(())
    }
}
