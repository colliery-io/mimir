//! Campaign service for business logic operations
//!
//! This service handles all campaign-related business logic including:
//! - Campaign creation with directory structure
//! - Stage transitions with validation
//! - Document generation

use crate::{
    connection::DbConnection,
    dal::campaigns::CampaignRepository,
    dal::documents::DocumentRepository,
    domain::{BoardRegistry, BoardDefinition},
    error::{DbError, Result},
    models::campaigns::{Campaign, NewCampaign},
    models::documents::{NewDocument},
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
        name: String,
        description: Option<String>,
        directory_location: String,
    ) -> Result<Campaign> {
        // Validate inputs
        if name.trim().is_empty() {
            return Err(DbError::InvalidData("Campaign name cannot be empty".to_string()));
        }
        
        // Create directory structure
        let base_path = Path::new(&directory_location);
        let campaign_path = self.create_campaign_directory_structure(base_path, &name)?;
        
        // Create database record
        let mut repo = CampaignRepository::new(self.conn);
        let new_campaign = NewCampaign {
            name: name.clone(),
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
        new_stage: String,
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
        if !board.can_transition(&campaign.status, &new_stage) {
            return Err(DbError::InvalidData(format!(
                "Cannot transition from {} to {}",
                campaign.status, new_stage
            )));
        }
        
        // Perform the transition
        let updated_campaign = repo.transition_status(campaign_id, &new_stage)?;
        
        // Create stage-specific documents
        if let Err(e) = self.create_stage_documents(&updated_campaign, &new_stage) {
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
            let file_path = format!("{}/{}.md", campaign.directory_path, doc_type);
            let new_doc = NewDocument {
                campaign_id: campaign.id,
                module_id: None,
                session_id: None,
                template_id: format!("default-{}", doc_type),
                document_type: doc_type.to_string(),
                title: format!("{} - {}", campaign.name, doc_type),
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
        let existing_types: Vec<String> = existing_docs.iter()
            .map(|d| d.document_type.clone())
            .collect();
        
        // Create missing required documents
        for doc_type in required_docs {
            if !existing_types.contains(&doc_type.to_string()) {
                let file_path = format!("{}/{}.md", campaign.directory_path, doc_type);
                let new_doc = NewDocument {
                    campaign_id: campaign.id,
                    module_id: None,
                    session_id: None,
                    template_id: format!("default-{}", doc_type),
                    document_type: doc_type.to_string(),
                    title: format!("{} - {}", campaign.name, doc_type),
                    file_path,
                };
                
                if let Err(e) = DocumentRepository::create(self.conn, new_doc) {
                    eprintln!("Failed to create {} document: {}", doc_type, e);
                }
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
            "".to_string(),
            None,
            "/tmp".to_string()
        );
        
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), DbError::InvalidData(_)));
    }
}