use std::sync::Arc;
use tauri::State;
use mimir_dm_db::{
    models::{
        documents::{NewDocument},
        campaigns::Campaign,
    },
    dal::{
        documents::DocumentRepository,
        campaigns::CampaignRepository,
    },
    DbConnection,
};
use crate::DatabaseService;
use crate::types::ApiResponse;

/// Stage document templates mapping
fn get_stage_documents(stage: &str) -> Vec<(&'static str, &'static str)> {
    match stage {
        "concept" => vec![
            ("campaign-sparks", "Campaign Sparks"),
            ("campaign-pitch", "Campaign Pitch"), 
            ("big-three", "Big Three"),
            ("first-adventure", "First Adventure Outline"),
        ],
        "session_zero" => vec![
            ("starting-scenario", "Starting Scenario"),
            ("world-primer", "World Primer"),
            ("character-guidelines", "Character Guidelines"),
            ("table-expectations", "Table Expectations"),
            ("character-integration", "Character Integration Forms"),
            ("session-zero-packet", "Session Zero Packet"),
        ],
        "integration" => vec![
            ("campaign-bible", "Campaign Bible"),
            ("character-integration-notes", "Character Integration Notes"),
            ("major-npcs", "Major NPCs"),
            ("world-timeline", "World Events Timeline"),
        ],
        _ => vec![],
    }
}

/// Create documents for a campaign stage
fn create_stage_documents(
    conn: &mut DbConnection,
    campaign: &Campaign,
    stage: &str,
) -> Result<Vec<String>, anyhow::Error> {
    let documents = get_stage_documents(stage);
    let mut created = Vec::new();
    
    for (template_id, title) in documents {
        // Check if document already exists
        let existing = DocumentRepository::find_by_campaign(conn, campaign.id)?;
        let exists = existing.iter().any(|d| d.template_id == template_id);
        
        if !exists {
            let new_doc = NewDocument {
                campaign_id: campaign.id,
                module_id: None,
                session_id: None,
                template_id: template_id.to_string(),
                document_type: template_id.replace('-', "_"),
                title: title.to_string(),
                file_path: format!("{}/{}.md", campaign.directory_path, template_id),
            };
            
            DocumentRepository::create(conn, new_doc)?;
            created.push(title.to_string());
        }
    }
    
    Ok(created)
}

/// Transition campaign to a new stage
#[tauri::command]
pub async fn transition_campaign_stage(
    campaign_id: i32,
    new_stage: String,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<ApiResponse<Campaign>, String> {
    let mut pooled_conn = db_service.get_connection().map_err(|e| e.to_string())?;
    let conn = &mut *pooled_conn;
    
    // Get the campaign
    let mut campaign_repo = CampaignRepository::new(conn);
    let campaign = match campaign_repo.find_by_id(campaign_id) {
        Ok(Some(c)) => c,
        Ok(None) => return Ok(ApiResponse::error("Campaign not found".to_string())),
        Err(e) => return Ok(ApiResponse::error(format!("Database error: {}", e)))
    };
    
    // Validate transition
    if !campaign.can_transition_to(&new_stage) {
        return Ok(ApiResponse::error(format!(
            "Cannot transition from {} to {}",
            campaign.status, new_stage
        )));
    }
    
    // Transition the campaign
    let updated_campaign = campaign_repo.transition_status(campaign_id, &new_stage)
        .map_err(|e| format!("Failed to transition campaign: {}", e))?;
    
    // Create documents for the new stage
    let created_docs = create_stage_documents(conn, &updated_campaign, &new_stage)
        .map_err(|e| format!("Failed to create stage documents: {}", e))?;
    
    if !created_docs.is_empty() {
        println!("Created {} documents for {} stage", created_docs.len(), new_stage);
    }
    
    Ok(ApiResponse::success(updated_campaign))
}

/// Initialize documents for current campaign stage
#[tauri::command]
pub async fn initialize_stage_documents(
    campaign_id: i32,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<ApiResponse<Vec<String>>, String> {
    let mut pooled_conn = db_service.get_connection().map_err(|e| e.to_string())?;
    let conn = &mut *pooled_conn;
    
    // Get the campaign
    let mut campaign_repo = CampaignRepository::new(conn);
    let campaign = match campaign_repo.find_by_id(campaign_id) {
        Ok(Some(c)) => c,
        Ok(None) => return Ok(ApiResponse::error("Campaign not found".to_string())),
        Err(e) => return Ok(ApiResponse::error(format!("Database error: {}", e)))
    };
    
    // Create documents for the current stage
    let created_docs = create_stage_documents(conn, &campaign, &campaign.status)
        .map_err(|e| format!("Failed to create stage documents: {}", e))?;
    
    Ok(ApiResponse::success(created_docs))
}