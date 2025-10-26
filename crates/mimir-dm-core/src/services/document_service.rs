//! Document management service

use crate::connection::DbConnection;
use crate::dal::campaign::{
    documents::DocumentRepository,
    campaigns::CampaignRepository,
    template_documents::TemplateRepository,
};
use crate::error::Result;
use crate::models::campaign::documents::{Document, NewDocument, UpdateDocument};
use std::path::PathBuf;
use std::fs;

/// Service for managing documents
pub struct DocumentService<'a> {
    conn: &'a mut DbConnection,
}

impl<'a> DocumentService<'a> {
    /// Create a new document service
    pub fn new(conn: &'a mut DbConnection) -> Self {
        Self { conn }
    }

    /// Get all documents for a campaign
    pub fn get_campaign_documents(&mut self, campaign_id: i32) -> Result<Vec<Document>> {
        DocumentRepository::find_by_campaign(self.conn, campaign_id)
    }

    /// Get documents by level with filtering logic
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
                DocumentRepository::find_by_campaign(self.conn, campaign_id)
                    .map(|docs| docs.into_iter()
                        .filter(|d| d.module_id.is_none() && d.session_id.is_none() && d.document_type != "handout")
                        .collect())
            },
            "module" => {
                if let Some(mid) = module_id {
                    DocumentRepository::find_by_module(self.conn, mid)
                } else {
                    Ok(vec![])
                }
            },
            "session" => {
                if let Some(sid) = session_id {
                    DocumentRepository::find_by_session(self.conn, sid)
                } else {
                    Ok(vec![])
                }
            },
            "handout" => {
                DocumentRepository::find_handouts_by_campaign(self.conn, campaign_id)
            },
            _ => Err(crate::error::DbError::InvalidData(format!("Invalid document level: {}", level))),
        }
    }

    /// Create a new document
    pub fn create_document(&mut self, new_document: NewDocument) -> Result<Document> {
        DocumentRepository::create(self.conn, new_document)
    }

    /// Update a document
    pub fn update_document(&mut self, document_id: i32, update: UpdateDocument) -> Result<Document> {
        DocumentRepository::update(self.conn, document_id, update)
    }

    /// Mark a document as completed
    pub fn complete_document(&mut self, document_id: i32) -> Result<Document> {
        DocumentRepository::mark_completed(self.conn, document_id)
    }

    /// Delete a document
    pub fn delete_document(&mut self, document_id: i32) -> Result<()> {
        DocumentRepository::delete(self.conn, document_id).map(|_| ())
    }

    /// Get incomplete documents for a campaign
    pub fn get_incomplete_documents(&mut self, campaign_id: i32) -> Result<Vec<Document>> {
        DocumentRepository::find_incomplete_by_campaign(self.conn, campaign_id)
    }

    /// Get completed documents for a campaign
    pub fn get_completed_documents(&mut self, campaign_id: i32) -> Result<Vec<Document>> {
        DocumentRepository::find_completed_by_campaign(self.conn, campaign_id)
    }

    /// Create a document from a template
    pub fn create_document_from_template(
        &mut self,
        campaign_id: i32,
        template_id: &str,
    ) -> Result<Document> {
        // Get the campaign
        let mut campaign_repo = CampaignRepository::new(self.conn);
        let campaign = campaign_repo.find_by_id(campaign_id)?
            .ok_or_else(|| crate::error::DbError::NotFound {
                entity_type: "Campaign".to_string(),
                id: campaign_id.to_string(),
            })?;

        // Check if document already exists
        let existing = DocumentRepository::find_by_campaign(self.conn, campaign_id)?;
        if existing.iter().any(|d| d.template_id == template_id) {
            return Err(crate::error::DbError::InvalidData("Document already exists".into()));
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
            .map_err(|e| crate::error::DbError::InvalidData(format!("Failed to add template: {}", e)))?;

        let content = tera.render(&template.document_id, &context)
            .map_err(|e| crate::error::DbError::InvalidData(format!("Failed to render template: {}", e)))?;

        // Write file to disk
        fs::write(&file_path, content)?;

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
            template_id: template_id.to_string(),
            document_type: template_id.replace('-', "_"),
            title,
            file_path: file_path.to_string_lossy().to_string(),
        };

        DocumentRepository::create(self.conn, new_doc)
    }

    /// Read a document file from disk
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

    /// Save a document file to disk
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
