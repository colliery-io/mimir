//! Campaign service for business logic operations
//!
//! This service handles all campaign-related business logic including:
//! - Campaign creation with directory structure
//! - Stage transitions with validation
//! - Document generation

use crate::{
    connection::DbConnection,
    dal::campaign::campaigns::CampaignRepository,
    dal::campaign::documents::DocumentRepository,
    dal::campaign::template_documents::TemplateRepository,
    domain::{BoardRegistry},
    error::{DbError, Result},
    models::campaign::campaigns::{Campaign, NewCampaign},
    models::campaign::documents::{NewDocument},
};
use std::fs;
use std::path::{Path, PathBuf};

pub struct CampaignService<'a> {
    conn: &'a mut DbConnection,
}

impl<'a> CampaignService<'a> {
    /// Create a new campaign service
    pub fn new(conn: &'a mut DbConnection) -> Self {
        Self { conn }
    }
    
    /// Create a new campaign with directory structure
    pub fn create_campaign(
        &mut self,
        name: &str,
        _description: Option<String>,
        directory_location: &str,
    ) -> Result<Campaign> {
        // Validate inputs
        if name.trim().is_empty() {
            return Err(DbError::InvalidData("Campaign name cannot be empty".to_string()));
        }
        
        // Create directory structure
        let base_path = Path::new(directory_location);
        let campaign_path = self.create_campaign_directory_structure(base_path, name)?;
        
        // Create database record
        let mut repo = CampaignRepository::new(self.conn);
        let new_campaign = NewCampaign {
            name: name.to_string(),
            status: "concept".to_string(),
            directory_path: campaign_path.to_string_lossy().to_string(),
        };
        
        // Try to create the campaign
        let campaign = match repo.create(new_campaign) {
            Ok(c) => c,
            Err(e) => {
                // Rollback: try to remove the created directory
                if let Err(remove_err) = fs::remove_dir_all(&campaign_path) {
                    eprintln!("Failed to cleanup campaign directory after database error: {}", remove_err);
                }
                return Err(e);
            }
        };
        
        // Create initial documents for the concept stage
        if let Err(e) = self.create_initial_documents(&campaign) {
            eprintln!("Failed to create initial documents: {}", e);
            // Continue anyway - campaign is created, documents can be created later
        }
        
        Ok(campaign)
    }
    
    /// Transition a campaign to a new stage
    pub fn transition_campaign_stage(
        &mut self,
        campaign_id: i32,
        new_stage: &str,
    ) -> Result<Campaign> {
        // Get the campaign
        let mut repo = CampaignRepository::new(self.conn);
        let campaign = repo.find_by_id(campaign_id)?
            .ok_or_else(|| DbError::NotFound { 
                entity_type: "Campaign".to_string(),
                id: campaign_id.to_string()
            })?;
        
        // Validate transition using board definition
        let board_registry = BoardRegistry::new();
        let board = board_registry.get("campaign")
            .ok_or_else(|| DbError::InvalidData("Campaign board definition not found".to_string()))?;
        
        // Check if transition is allowed
        if !board.can_transition(&campaign.status, new_stage) {
            return Err(DbError::InvalidData(format!(
                "Cannot transition from {} to {}",
                campaign.status, new_stage
            )));
        }
        
        // Perform the transition
        let updated_campaign = repo.transition_status(campaign_id, new_stage)?;
        
        // Create stage-specific documents
        if let Err(e) = self.create_stage_documents(&updated_campaign, new_stage) {
            eprintln!("Failed to create stage documents: {}", e);
            // Continue anyway - transition succeeded
        }
        
        Ok(updated_campaign)
    }
    
    /// List all campaigns
    pub fn list_campaigns(&mut self) -> Result<Vec<Campaign>> {
        let mut repo = CampaignRepository::new(self.conn);
        repo.list()
    }
    
    /// Get a campaign by ID
    pub fn get_campaign(&mut self, campaign_id: i32) -> Result<Option<Campaign>> {
        let mut repo = CampaignRepository::new(self.conn);
        repo.find_by_id(campaign_id)
    }
    
    /// Create the campaign directory structure
    fn create_campaign_directory_structure(
        &self,
        base_path: &Path,
        campaign_name: &str,
    ) -> Result<PathBuf> {
        let campaign_path = base_path.join(campaign_name);
        
        // Check if campaign directory already exists
        if campaign_path.exists() {
            return Err(DbError::InvalidData(format!(
                "Campaign directory '{}' already exists",
                campaign_path.display()
            )));
        }
        
        println!("Creating campaign directory structure at: {}", campaign_path.display());
        
        // Create main campaign directory
        fs::create_dir_all(&campaign_path)?;
        
        // Create all the required directories
        let directories = [
            "session_zero",
            "world",
            "world/factions",
            "regions",
            "modules",
            "sessions",
            "characters",
            "npcs",
            "npcs/recurring",
            "resources",
            "resources/maps",
            "resources/handouts",
            "resources/references",
        ];
        
        for dir in directories {
            let dir_path = campaign_path.join(dir);
            fs::create_dir_all(&dir_path)?;
            println!("Created directory: {}", dir_path.display());
        }
        
        // Create initial README
        let readme_content = format!(
            "# {}\n\nCampaign created on {}\n\nUse the Mimir application to generate additional campaign documents as needed.",
            campaign_name,
            chrono::Local::now().format("%Y-%m-%d")
        );
        
        fs::write(campaign_path.join("README.md"), readme_content)?;
        
        println!("Successfully created campaign directory structure");
        Ok(campaign_path)
    }
    
    /// Create initial documents for a new campaign
    fn create_initial_documents(&mut self, campaign: &Campaign) -> Result<()> {
        // Get the board definition to know what documents are needed
        let board_registry = BoardRegistry::new();
        let board = board_registry.get("campaign")
            .ok_or_else(|| DbError::InvalidData("Campaign board definition not found".to_string()))?;
        
        let required_docs = board.required_documents(&campaign.status);
        
        for doc_type in required_docs {
            // Use doc_type directly as template_id (both use snake_case now)
            let template_id = doc_type.to_string();
            let file_path = format!("{}/{}.md", campaign.directory_path, template_id);
            
            // Create directory if needed
            let full_path = std::path::Path::new(&file_path);
            if let Some(parent) = full_path.parent() {
                fs::create_dir_all(parent)?;
            }
            
            // Get the template from the database - this MUST exist
            let template = TemplateRepository::get_latest(self.conn, &template_id)
                .map_err(|e| DbError::InvalidData(
                    format!("Required template '{}' not found in database: {}", template_id, e)
                ))?;
            
            // Render the template with its default context
            let context = template.create_context();
            let mut tera = tera::Tera::default();
            tera.add_raw_template(&template.document_id, &template.document_content)
                .map_err(|e| DbError::InvalidData(format!("Failed to add template: {}", e)))?;
            
            let rendered_content = tera.render(&template.document_id, &context)
                .map_err(|e| DbError::InvalidData(format!("Failed to render template: {}", e)))?;
            
            // Write the rendered template content to the file
            fs::write(&full_path, rendered_content)?;
            
            let new_doc = NewDocument {
                campaign_id: campaign.id,
                module_id: None,
                session_id: None,
                template_id,
                document_type: doc_type.to_string(),
                title: doc_type.replace('_', " ").split_whitespace()
                    .map(|w| {
                        let mut chars = w.chars();
                        match chars.next() {
                            None => String::new(),
                            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(" "),
                file_path,
            };
            
            if let Err(e) = DocumentRepository::create(self.conn, new_doc) {
                eprintln!("Failed to create {} document: {}", doc_type, e);
            }
        }
        
        Ok(())
    }
    
    /// Create stage-specific documents when transitioning
    fn create_stage_documents(&mut self, campaign: &Campaign, stage: &str) -> Result<()> {
        // Get the board definition
        let board_registry = BoardRegistry::new();
        let board = board_registry.get("campaign")
            .ok_or_else(|| DbError::InvalidData("Campaign board definition not found".to_string()))?;
        
        let required_docs = board.required_documents(stage);
        
        // Check which documents already exist
        let existing_docs = DocumentRepository::find_by_campaign(self.conn, campaign.id)?;
        let existing_template_ids: Vec<String> = existing_docs.iter()
            .map(|d| d.template_id.clone())
            .collect();
        
        // Create missing required documents
        for doc_type in required_docs {
            // Use doc_type directly as template_id (both use snake_case now)
            let template_id = doc_type.to_string();
            
            // Skip if document already exists
            if existing_template_ids.contains(&template_id) {
                continue;
            }
            
            let file_path = format!("{}/{}.md", campaign.directory_path, template_id);
            
            // Create directory if needed
            let full_path = std::path::Path::new(&file_path);
            if let Some(parent) = full_path.parent() {
                fs::create_dir_all(parent)?;
            }
            
            // Try to get the template from the database and render it
            let content = match TemplateRepository::get_latest(self.conn, &template_id) {
                Ok(template) => {
                    // Render the template with its default context
                    let context = template.create_context();
                    let mut tera = tera::Tera::default();
                    tera.add_raw_template(&template.document_id, &template.document_content)
                        .map_err(|e| DbError::InvalidData(format!("Failed to add template: {}", e)))?;
                    
                    tera.render(&template.document_id, &context)
                        .map_err(|e| DbError::InvalidData(format!("Failed to render template: {}", e)))?
                },
                Err(_) => {
                    // If template doesn't exist, create a basic markdown file
                    format!("# {}\n\n*This document will be created for the {} stage.*\n\n## Overview\n\n[Document content will be added here]\n", 
                        doc_type.replace('_', " ").split_whitespace()
                            .map(|w| {
                                let mut chars = w.chars();
                                match chars.next() {
                                    None => String::new(),
                                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                                }
                            })
                            .collect::<Vec<_>>()
                            .join(" "),
                        stage
                    )
                }
            };
            
            // Write the content to the file
            fs::write(&full_path, content)?;
            
            // Create database record
            let new_doc = NewDocument {
                campaign_id: campaign.id,
                module_id: None,
                session_id: None,
                template_id: template_id.clone(),
                document_type: doc_type.to_string(),
                title: doc_type.replace('_', " ").split_whitespace()
                    .map(|w| {
                        let mut chars = w.chars();
                        match chars.next() {
                            None => String::new(),
                            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(" "),
                file_path,
            };
            
            if let Err(e) = DocumentRepository::create(self.conn, new_doc) {
                eprintln!("Failed to create {} document: {}", doc_type, e);
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::connection::establish_connection;
    use tempfile::TempDir;
    
    #[test]
    fn test_create_campaign_directory_structure() {
        let temp_dir = TempDir::new().unwrap();
        let mut conn = establish_connection(":memory:").unwrap();
        crate::run_migrations(&mut conn).unwrap();
        
        let service = CampaignService::new(&mut conn);
        
        let campaign_path = service.create_campaign_directory_structure(
            temp_dir.path(),
            "Test Campaign"
        ).unwrap();
        
        // Verify directories were created
        assert!(campaign_path.exists());
        assert!(campaign_path.join("session_zero").exists());
        assert!(campaign_path.join("world/factions").exists());
        assert!(campaign_path.join("README.md").exists());
    }
    
    #[test]
    fn test_create_campaign_with_empty_name() {
        let mut conn = establish_connection(":memory:").unwrap();
        crate::run_migrations(&mut conn).unwrap();
        
        let mut service = CampaignService::new(&mut conn);
        
        let result = service.create_campaign(
            "",
            None,
            "/tmp"
        );
        
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), DbError::InvalidData(_)));
    }
    
    #[test]
    fn test_create_campaign_with_whitespace_name() {
        let mut conn = establish_connection(":memory:").unwrap();
        crate::run_migrations(&mut conn).unwrap();
        
        let mut service = CampaignService::new(&mut conn);
        
        let result = service.create_campaign(
            "   ",
            None,
            "/tmp"
        );
        
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), DbError::InvalidData(_)));
    }
    
    #[test]
    fn test_create_campaign_success() {
        let temp_dir = TempDir::new().unwrap();
        let mut conn = establish_connection(":memory:").unwrap();
        crate::run_migrations(&mut conn).unwrap();
        
        let mut service = CampaignService::new(&mut conn);
        
        let campaign = service.create_campaign(
            "My Test Campaign",
            Some("A test campaign description".to_string()),
            temp_dir.path().to_str().unwrap()
        ).unwrap();
        
        assert_eq!(campaign.name, "My Test Campaign");
        assert_eq!(campaign.status, "concept");
        assert!(campaign.directory_path.contains("My Test Campaign"));
        
        // Verify directory was created
        let campaign_dir = Path::new(&campaign.directory_path);
        assert!(campaign_dir.exists());
        assert!(campaign_dir.join("session_zero").exists());
    }
    
    #[test]
    fn test_create_duplicate_campaign_directory() {
        let temp_dir = TempDir::new().unwrap();
        let mut conn = establish_connection(":memory:").unwrap();
        crate::run_migrations(&mut conn).unwrap();
        
        let mut service = CampaignService::new(&mut conn);
        
        // Create first campaign
        let campaign1 = service.create_campaign(
            "Duplicate Test",
            None,
            temp_dir.path().to_str().unwrap()
        ).unwrap();
        
        assert_eq!(campaign1.name, "Duplicate Test");
        
        // Try to create second campaign with same name in same location
        let result = service.create_campaign(
            "Duplicate Test",
            None,
            temp_dir.path().to_str().unwrap()
        );
        
        assert!(result.is_err());
        // Should fail because directory already exists
    }
    
    #[test]
    fn test_list_campaigns_empty() {
        let mut conn = establish_connection(":memory:").unwrap();
        crate::run_migrations(&mut conn).unwrap();
        
        let mut service = CampaignService::new(&mut conn);
        
        let campaigns = service.list_campaigns().unwrap();
        assert_eq!(campaigns.len(), 0);
    }
    
    #[test]
    fn test_list_campaigns_multiple() {
        let temp_dir = TempDir::new().unwrap();
        let mut conn = establish_connection(":memory:").unwrap();
        crate::run_migrations(&mut conn).unwrap();
        
        let mut service = CampaignService::new(&mut conn);
        
        // Create multiple campaigns
        service.create_campaign(
            "Campaign 1",
            None,
            temp_dir.path().to_str().unwrap()
        ).unwrap();
        
        // Create subdirectory for second campaign
        let subdir = temp_dir.path().join("other");
        fs::create_dir_all(&subdir).unwrap();
        
        service.create_campaign(
            "Campaign 2",
            None,
            subdir.to_str().unwrap()
        ).unwrap();
        
        let campaigns = service.list_campaigns().unwrap();
        assert_eq!(campaigns.len(), 2);
        
        let names: Vec<String> = campaigns.iter().map(|c| c.name.clone()).collect();
        assert!(names.contains(&"Campaign 1".to_string()));
        assert!(names.contains(&"Campaign 2".to_string()));
    }
    
    #[test]
    fn test_get_campaign() {
        let temp_dir = TempDir::new().unwrap();
        let mut conn = establish_connection(":memory:").unwrap();
        crate::run_migrations(&mut conn).unwrap();
        
        let mut service = CampaignService::new(&mut conn);
        
        let created = service.create_campaign(
            "Get Test Campaign",
            Some("Description".to_string()),
            temp_dir.path().to_str().unwrap()
        ).unwrap();
        
        // Get existing campaign
        let found = service.get_campaign(created.id).unwrap();
        assert!(found.is_some());
        
        let campaign = found.unwrap();
        assert_eq!(campaign.id, created.id);
        assert_eq!(campaign.name, "Get Test Campaign");
        
        // Get non-existent campaign
        let not_found = service.get_campaign(99999).unwrap();
        assert!(not_found.is_none());
    }
    
    #[test]
    fn test_transition_campaign_stage() {
        let temp_dir = TempDir::new().unwrap();
        let mut conn = establish_connection(":memory:").unwrap();
        crate::run_migrations(&mut conn).unwrap();
        
        let mut service = CampaignService::new(&mut conn);
        
        let campaign = service.create_campaign(
            "Transition Test",
            None,
            temp_dir.path().to_str().unwrap()
        ).unwrap();
        
        assert_eq!(campaign.status, "concept");
        
        // Transition to session_zero
        let updated = service.transition_campaign_stage(
            campaign.id,
            "session_zero"
        ).unwrap();
        
        assert_eq!(updated.status, "session_zero");
        
        // Verify we can't transition to invalid stage
        let invalid_result = service.transition_campaign_stage(
            campaign.id,
            "completed"  // Can't jump from session_zero to completed
        );
        
        assert!(invalid_result.is_err());
        assert!(matches!(invalid_result.unwrap_err(), DbError::InvalidData(_)));
    }
    
    #[test]
    fn test_transition_nonexistent_campaign() {
        let mut conn = establish_connection(":memory:").unwrap();
        crate::run_migrations(&mut conn).unwrap();
        
        let mut service = CampaignService::new(&mut conn);
        
        let result = service.transition_campaign_stage(
            99999,
            "session_zero"
        );
        
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), DbError::NotFound { .. }));
    }
}