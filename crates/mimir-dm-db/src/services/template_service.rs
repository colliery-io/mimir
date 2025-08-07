//! Template service for document generation
//!
//! This service handles template rendering and document generation

use crate::{
    connection::DbConnection,
    dal::campaigns::CampaignRepository,
    dal::template_documents::TemplateRepository,
    error::{DbError, Result},
    models::campaigns::Campaign,
    models::template_documents::TemplateDocument,
};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use tera::{Context, Tera};

pub struct TemplateService<'a> {
    conn: &'a mut DbConnection,
}

pub struct RenderedDocument {
    pub file_path: String,
    pub content: String,
    pub template_id: String,
}

impl<'a> TemplateService<'a> {
    /// Create a new template service
    pub fn new(conn: &'a mut DbConnection) -> Self {
        Self { conn }
    }
    
    /// Render a template for a campaign
    pub fn render_template(
        &mut self,
        campaign_id: i32,
        template_id: String,
        variables: HashMap<String, JsonValue>,
    ) -> Result<RenderedDocument> {
        // Get the campaign
        let mut campaign_repo = CampaignRepository::new(self.conn);
        let campaign = campaign_repo.find_by_id(campaign_id)?
            .ok_or_else(|| DbError::NotFound {
                entity_type: "Campaign".to_string(),
                id: campaign_id.to_string()
            })?;
        
        // Get the template
        let template = TemplateRepository::get_latest(self.conn, &template_id)?;
        
        // Render the template content
        let rendered_content = self.render_template_content(
            &template,
            &campaign,
            variables
        )?;
        
        // Determine the file path
        let file_path = self.determine_template_file_path(
            &campaign.directory_path,
            &template_id
        );
        
        Ok(RenderedDocument {
            file_path,
            content: rendered_content,
            template_id,
        })
    }
    
    /// Generate and save a document from a template
    pub fn generate_document(
        &mut self,
        campaign_id: i32,
        template_id: String,
        variables: HashMap<String, JsonValue>,
    ) -> Result<String> {
        // Render the template
        let rendered = self.render_template(campaign_id, template_id, variables)?;
        
        // Save to file
        let full_path = PathBuf::from(&rendered.file_path);
        
        // Create parent directory if needed
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Write the file
        fs::write(&full_path, &rendered.content)?;
        
        println!("Generated document at: {}", full_path.display());
        Ok(rendered.file_path)
    }
    
    /// List all available templates
    pub fn list_templates(&mut self) -> Result<Vec<TemplateDocument>> {
        TemplateRepository::get_all_active(self.conn)
            .map_err(|e| e.into())
    }
    
    /// Get a specific template
    pub fn get_template(&mut self, template_id: &str) -> Result<TemplateDocument> {
        TemplateRepository::get_latest(self.conn, template_id)
            .map_err(|e| e.into())
    }
    
    /// Render template content with variables
    fn render_template_content(
        &self,
        template: &TemplateDocument,
        campaign: &Campaign,
        variables: HashMap<String, JsonValue>,
    ) -> Result<String> {
        // Create Tera context
        let mut context = Context::new();
        
        // Add campaign-level variables
        context.insert("campaign_name", &campaign.name);
        context.insert("campaign_status", &campaign.status);
        context.insert("campaign_directory", &campaign.directory_path);
        
        // Add user-provided variables
        for (key, value) in variables {
            context.insert(&key, &value);
        }
        
        // Render the template
        let mut tera = Tera::default();
        tera.add_raw_template(&template.document_id, &template.document_content)
            .map_err(|e| DbError::InvalidData(format!("Failed to parse template: {}", e)))?;
        
        let rendered = tera.render(&template.document_id, &context)
            .map_err(|e| DbError::InvalidData(format!("Failed to render template: {}", e)))?;
        
        Ok(rendered)
    }
    
    /// Determine where to save a template based on its ID
    fn determine_template_file_path(&self, campaign_dir: &str, template_id: &str) -> String {
        let campaign_path = Path::new(campaign_dir);
        
        let file_path = match template_id {
            // Campaign level documents
            "campaign-bible" => campaign_path.join("campaign_bible.md"),
            "campaign-pitch" => campaign_path.join("pitch.md"),
            "starting-scenario" => campaign_path.join("session_zero/starting_scenario.md"),
            "quick-start-kit" => campaign_path.join("quick_start_kit.md"),
            
            // World building
            "world-overview" => campaign_path.join("world/overview.md"),
            "region-overview" => campaign_path.join("regions/region_overview.md"),
            "faction-template" => campaign_path.join("world/factions/faction.md"),
            
            // Characters and NPCs
            "character-integration" => campaign_path.join("characters/character_integration.md"),
            "major-npc-tracker" => campaign_path.join("npcs/major_npcs.md"),
            "quick-npc-reference" => campaign_path.join("npcs/quick_reference.md"),
            "pc-arc-tracker" => campaign_path.join("characters/pc_arc_tracker.md"),
            
            // Session management
            "session-outline" => campaign_path.join("sessions/session_outline.md"),
            "clue-tracker" => campaign_path.join("sessions/clue_tracker.md"),
            "document-tracker" => campaign_path.join("document_tracker.md"),
            
            // Module templates
            "module-overview" => campaign_path.join("modules/module_overview.md"),
            template if template.starts_with("module-") => {
                campaign_path.join(format!("modules/{}.md", template))
            }
            
            // Default fallback
            _ => campaign_path.join(format!("{}.md", template_id)),
        };
        
        file_path.to_string_lossy().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::connection::establish_connection;
    
    #[test]
    fn test_determine_template_file_path() {
        let mut conn = establish_connection(":memory:").unwrap();
        let service = TemplateService::new(&mut conn);
        
        let path = service.determine_template_file_path(
            "/home/user/campaigns/test",
            "campaign-bible"
        );
        assert_eq!(path, "/home/user/campaigns/test/campaign_bible.md");
        
        let path = service.determine_template_file_path(
            "/home/user/campaigns/test",
            "world-overview"
        );
        assert_eq!(path, "/home/user/campaigns/test/world/overview.md");
        
        let path = service.determine_template_file_path(
            "/home/user/campaigns/test",
            "module-custom"
        );
        assert_eq!(path, "/home/user/campaigns/test/modules/module-custom.md");
    }
}