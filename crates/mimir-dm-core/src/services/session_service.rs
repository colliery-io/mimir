//! Session service for managing sessions within modules

use crate::dal::campaign::sessions::SessionRepository;
use crate::dal::campaign::template_documents::TemplateRepository;
use crate::domain::BoardRegistry;
use crate::error::Result;
use crate::models::campaign::sessions::{NewSession, Session};
use crate::models::campaign::template_documents::TemplateDocument;
use diesel::prelude::*;
use std::path::PathBuf;
use std::fs;
use tera::Tera;
use serde_json::json;

pub struct SessionService;

impl SessionService {
    /// Create a new session for a module
    pub fn create_session(
        conn: &mut SqliteConnection,
        module_id: i32,
        campaign_id: i32,
        campaign_directory: &str,
        module_number: i32,
    ) -> Result<Session> {
        // Get the next session number
        let mut session_repo = SessionRepository::new(conn);
        let session_number = session_repo.get_next_session_number(campaign_id)?;
        
        // Get the initial status from the session board
        let board_registry = BoardRegistry::new();
        let session_board = board_registry.get("session")
            .ok_or_else(|| diesel::result::Error::NotFound)?;
        
        // Get the first stage from the session board
        let stages = session_board.stages();
        let initial_status = stages
            .first()
            .ok_or_else(|| diesel::result::Error::QueryBuilderError(
                "Session board has no stages defined".into()
            ))?;
        
        // Create the session record
        let new_session = NewSession {
            campaign_id,
            module_id: Some(module_id),
            session_number,
            status: initial_status.to_string(),
            scheduled_date: None,
        };
        
        let session = session_repo.create(new_session)?;
        
        // Create session folder
        let session_dir = PathBuf::from(campaign_directory)
            .join("modules")
            .join(format!("module_{:02}", module_number))
            .join(format!("session_{:03}", session_number));
        
        fs::create_dir_all(&session_dir)?;
        
        // Create session documents from templates
        Self::create_session_documents(conn, &session_dir, session_number, module_number)?;
        
        Ok(session)
    }
    
    /// Create session documents from templates
    fn create_session_documents(
        conn: &mut SqliteConnection,
        session_dir: &PathBuf,
        session_number: i32,
        module_number: i32,
    ) -> Result<()> {
        // Create session notes from template
        let notes_template = TemplateRepository::get_latest(conn, "session_notes")?;
        let notes_content = Self::render_template(
            &notes_template,
            session_number,
            module_number,
        )?;
        
        let notes_path = session_dir.join("session-notes.md");
        fs::write(&notes_path, notes_content)?;
        
        // Create session outline from template
        let outline_template = TemplateRepository::get_latest(conn, "session_outline")?;
        let outline_content = Self::render_template(
            &outline_template,
            session_number,
            module_number,
        )?;
        
        let outline_path = session_dir.join("session-outline.md");
        fs::write(&outline_path, outline_content)?;
        
        Ok(())
    }
    
    /// Render a template with session context
    fn render_template(
        template: &TemplateDocument,
        session_number: i32,
        module_number: i32,
    ) -> Result<String> {
        let mut tera = Tera::default();
        tera.add_raw_template("template", &template.document_content)
            .map_err(|e| diesel::result::Error::QueryBuilderError(format!("Template error: {}", e).into()))?;
        
        // Create context with defaults from template
        let mut context = template.create_context();
        
        // Add session-specific values
        context.insert("session_number", &json!(session_number));
        context.insert("module_number", &json!(module_number));
        
        let rendered = tera.render("template", &context)
            .map_err(|e| diesel::result::Error::QueryBuilderError(format!("Render error: {}", e).into()))?;
        Ok(rendered)
    }
    
    /// Copy session outline to session folder for prep
    pub fn copy_outline_to_session(
        campaign_directory: &str,
        module_number: i32,
        session_number: i32,
    ) -> Result<()> {
        let module_dir = PathBuf::from(campaign_directory)
            .join("modules")
            .join(format!("module_{:02}", module_number));
        
        let session_dir = module_dir.join(format!("session_{:03}", session_number));
        
        // Check if outline exists in module root
        let source_outline = module_dir.join("session-outline.md");
        if source_outline.exists() {
            let dest_outline = session_dir.join("session-outline.md");
            fs::copy(&source_outline, &dest_outline)?;
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::establish_connection;
    use crate::seed::template_seeder::seed_templates;
    
    #[test]
    fn test_create_session() {
        let mut conn = establish_connection(":memory:").unwrap();
        crate::run_migrations(&mut conn).unwrap();
        seed_templates(&mut conn).unwrap();
        
        // Create test campaign and module data
        use crate::schema::{campaigns, modules};
        use diesel::prelude::*;
        
        diesel::insert_into(campaigns::table)
            .values((
                campaigns::name.eq("Test Campaign"),
                campaigns::status.eq("active"),
                campaigns::created_at.eq(chrono::Utc::now().to_rfc3339()),
            ))
            .execute(&mut conn)
            .unwrap();
            
        diesel::insert_into(modules::table)
            .values((
                modules::campaign_id.eq(1),
                modules::name.eq("Test Module"),
                modules::module_type.eq("mystery"),
                modules::module_number.eq(1),
                modules::status.eq("active"),
                modules::created_at.eq(chrono::Utc::now().to_rfc3339()),
            ))
            .execute(&mut conn)
            .unwrap();
        
        // Create temp directory for test
        let temp_dir = tempfile::tempdir().unwrap();
        let campaign_dir = temp_dir.path().to_str().unwrap();
        
        // Create session
        let session = SessionService::create_session(
            &mut conn,
            1,  // module_id
            1,  // campaign_id
            campaign_dir,
            1,  // module_number
        ).unwrap();
        
        assert_eq!(session.session_number, 1);
        assert_eq!(session.status, "planning");
        assert_eq!(session.module_id, Some(1));
        
        // Check session folder exists
        let session_dir = PathBuf::from(campaign_dir)
            .join("modules")
            .join("module_01")
            .join("session_001");
        assert!(session_dir.exists());
        
        // Check session documents exist
        assert!(session_dir.join("session-notes.md").exists());
        assert!(session_dir.join("session-outline.md").exists());
    }
}