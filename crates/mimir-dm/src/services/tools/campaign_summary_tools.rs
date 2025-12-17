//! Campaign Summary tools for LLM interactions
//!
//! Tools for managing AI-generated campaign story summaries

use async_trait::async_trait;
use mimir_dm_core::services::{
    format_source_for_llm, CampaignService, CampaignSummary, CampaignSummaryService,
};
use mimir_dm_core::DatabaseService;
use mimir_dm_llm::traits::{ActionDescription, ChangeDetail};
use mimir_dm_llm::{LlmProvider, Message, ToolTrait};
use serde_json::{json, Value};
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, info};

use crate::services::llm::Provider;

/// Tool for regenerating campaign summary using LLM
pub struct RegenerateCampaignSummaryTool {
    db_service: Arc<DatabaseService>,
    llm_provider: Arc<Mutex<Option<Provider>>>,
}

impl RegenerateCampaignSummaryTool {
    pub fn new(db_service: Arc<DatabaseService>, llm_provider: Arc<Mutex<Option<Provider>>>) -> Self {
        Self {
            db_service,
            llm_provider,
        }
    }
}

#[async_trait]
impl ToolTrait for RegenerateCampaignSummaryTool {
    fn name(&self) -> &str {
        "regenerate_campaign_summary"
    }

    fn description(&self) -> &str {
        "Regenerate the AI-generated campaign story summary.

Usage:
- Call when the campaign summary needs to be updated
- Gathers all session notes and module information
- Uses AI to generate a cohesive story summary
- Caches the result for future use

When to use:
- After completing sessions with significant story progress
- When the cached summary is out of date
- When explicitly asked to refresh/regenerate the summary

Output:
- Generated summary text
- Confirmation of cache update"
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "campaign_id": {
                    "type": "integer",
                    "description": "ID of the campaign to summarize"
                }
            },
            "required": ["campaign_id"]
        })
    }

    fn requires_confirmation(&self) -> bool {
        true // LLM call can take time/cost money
    }

    fn describe_action(&self, arguments: &Value) -> Option<ActionDescription> {
        let campaign_id = arguments.get("campaign_id")?.as_i64()?;

        Some(ActionDescription {
            title: "Regenerate Campaign Summary".to_string(),
            description: format!(
                "Generate new AI summary for campaign {} from session notes",
                campaign_id
            ),
            changes: ChangeDetail::Generic {
                items: vec![
                    "Gather session notes and module info".to_string(),
                    "Generate summary using AI".to_string(),
                    "Update cached summary".to_string(),
                ],
            },
        })
    }

    async fn execute(&self, arguments: Value) -> Result<String, Box<dyn Error + Send + Sync>> {
        let campaign_id = arguments
            .get("campaign_id")
            .and_then(|v| v.as_i64())
            .ok_or("campaign_id is required")? as i32;

        debug!("Regenerating campaign summary for campaign {}", campaign_id);

        // Get campaign directory
        let campaign_dir = {
            let mut conn = self.db_service.get_connection()?;
            let mut service = CampaignService::new(&mut conn);
            service
                .get_campaign(campaign_id)?
                .map(|c| c.directory_path)
                .ok_or("Campaign not found")?
        };

        // Gather source materials
        let source = {
            let mut conn = self.db_service.get_connection()?;
            let mut service = CampaignSummaryService::new(&mut conn);
            service.gather_source_materials(campaign_id, &campaign_dir)?
        };

        // Check if we have any content to summarize
        if source.session_notes.is_empty() && source.modules.is_empty() {
            return Ok(serde_json::to_string_pretty(&json!({
                "success": false,
                "error": "No session notes or modules found to summarize",
                "campaign_id": campaign_id
            }))?);
        }

        // Format prompt for LLM
        let prompt = format_source_for_llm(&source);

        // Get LLM provider
        let provider_guard = self.llm_provider.lock().await;
        let provider = provider_guard
            .as_ref()
            .ok_or("LLM provider not available")?;

        // Call LLM to generate summary
        info!("Calling LLM to generate campaign summary...");
        let messages = vec![Message {
            role: "user".to_string(),
            content: prompt,
            tool_call_id: None,
        }];

        let response = provider
            .chat(messages, None, None, None, None, None, None, None)
            .await?;
        let summary_text = response.content;

        // Calculate source hash and save
        let source_hash = CampaignSummaryService::calculate_source_hash(&source);
        let summary = CampaignSummary {
            summary: summary_text.clone(),
            generated_at: chrono::Utc::now().to_rfc3339(),
            source_hash,
            campaign_id,
        };

        // Save to cache
        {
            let mut conn = self.db_service.get_connection()?;
            let service = CampaignSummaryService::new(&mut conn);
            service.save_summary(&campaign_dir, &summary)?;
        }

        info!("Campaign summary regenerated and cached");

        Ok(serde_json::to_string_pretty(&json!({
            "success": true,
            "campaign_id": campaign_id,
            "summary": summary_text,
            "generated_at": summary.generated_at,
            "source_notes_count": source.session_notes.len(),
            "source_modules_count": source.modules.len()
        }))?)
    }
}

/// Tool for invalidating (clearing) the cached campaign summary
pub struct InvalidateCampaignSummaryTool {
    db_service: Arc<DatabaseService>,
}

impl InvalidateCampaignSummaryTool {
    pub fn new(db_service: Arc<DatabaseService>) -> Self {
        Self { db_service }
    }
}

#[async_trait]
impl ToolTrait for InvalidateCampaignSummaryTool {
    fn name(&self) -> &str {
        "invalidate_campaign_summary"
    }

    fn description(&self) -> &str {
        "Clear the cached campaign summary, forcing regeneration on next use.

Usage:
- Call to invalidate stale summary cache
- Next time summary is needed, it will be regenerated

When to use:
- When you know the summary is outdated
- After major story changes
- Before regenerating with fresh data"
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "campaign_id": {
                    "type": "integer",
                    "description": "ID of the campaign"
                }
            },
            "required": ["campaign_id"]
        })
    }

    fn requires_confirmation(&self) -> bool {
        false // Just deleting cache, harmless
    }

    fn describe_action(&self, arguments: &Value) -> Option<ActionDescription> {
        let campaign_id = arguments.get("campaign_id")?.as_i64()?;

        Some(ActionDescription {
            title: "Invalidate Campaign Summary".to_string(),
            description: format!("Clear cached summary for campaign {}", campaign_id),
            changes: ChangeDetail::Generic {
                items: vec!["Delete cached summary file".to_string()],
            },
        })
    }

    async fn execute(&self, arguments: Value) -> Result<String, Box<dyn Error + Send + Sync>> {
        let campaign_id = arguments
            .get("campaign_id")
            .and_then(|v| v.as_i64())
            .ok_or("campaign_id is required")? as i32;

        // Get campaign directory
        let campaign_dir = {
            let mut conn = self.db_service.get_connection()?;
            let mut service = CampaignService::new(&mut conn);
            service
                .get_campaign(campaign_id)?
                .map(|c| c.directory_path)
                .ok_or("Campaign not found")?
        };

        // Invalidate cache
        {
            let mut conn = self.db_service.get_connection()?;
            let service = CampaignSummaryService::new(&mut conn);
            service.invalidate_cache(&campaign_dir)?;
        }

        info!(
            "Campaign summary cache invalidated for campaign {}",
            campaign_id
        );

        Ok(serde_json::to_string_pretty(&json!({
            "success": true,
            "campaign_id": campaign_id,
            "message": "Summary cache cleared. Next summary request will regenerate."
        }))?)
    }
}

/// Tool for getting the current campaign summary (from cache or indicates if stale)
pub struct GetCampaignSummaryTool {
    db_service: Arc<DatabaseService>,
}

impl GetCampaignSummaryTool {
    pub fn new(db_service: Arc<DatabaseService>) -> Self {
        Self { db_service }
    }
}

#[async_trait]
impl ToolTrait for GetCampaignSummaryTool {
    fn name(&self) -> &str {
        "get_campaign_summary"
    }

    fn description(&self) -> &str {
        "Get the current campaign story summary.

Usage:
- Returns cached summary if available and fresh
- Indicates if summary is stale and needs regeneration

Output:
- Summary text (if available)
- Status: fresh, stale, or missing
- Recommendation for regeneration if needed"
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "campaign_id": {
                    "type": "integer",
                    "description": "ID of the campaign"
                }
            },
            "required": ["campaign_id"]
        })
    }

    fn requires_confirmation(&self) -> bool {
        false // Read-only
    }

    fn describe_action(&self, _arguments: &Value) -> Option<ActionDescription> {
        None // Read-only, no action description needed
    }

    async fn execute(&self, arguments: Value) -> Result<String, Box<dyn Error + Send + Sync>> {
        let campaign_id = arguments
            .get("campaign_id")
            .and_then(|v| v.as_i64())
            .ok_or("campaign_id is required")? as i32;

        // Get campaign directory
        let campaign_dir = {
            let mut conn = self.db_service.get_connection()?;
            let mut service = CampaignService::new(&mut conn);
            service
                .get_campaign(campaign_id)?
                .map(|c| c.directory_path)
                .ok_or("Campaign not found")?
        };

        // Check cache status
        let mut conn = self.db_service.get_connection()?;
        let mut service = CampaignSummaryService::new(&mut conn);

        let (maybe_summary, is_stale, _source) =
            service.get_summary_with_staleness_check(campaign_id, &campaign_dir)?;

        let result = match maybe_summary {
            Some(summary) if !is_stale => {
                json!({
                    "status": "fresh",
                    "campaign_id": campaign_id,
                    "summary": summary.summary,
                    "generated_at": summary.generated_at,
                    "needs_regeneration": false
                })
            }
            Some(summary) => {
                json!({
                    "status": "stale",
                    "campaign_id": campaign_id,
                    "summary": summary.summary,
                    "generated_at": summary.generated_at,
                    "needs_regeneration": true,
                    "message": "Summary is out of date. Consider calling regenerate_campaign_summary."
                })
            }
            None => {
                json!({
                    "status": "missing",
                    "campaign_id": campaign_id,
                    "summary": null,
                    "needs_regeneration": true,
                    "message": "No summary exists. Call regenerate_campaign_summary to create one."
                })
            }
        };

        Ok(serde_json::to_string_pretty(&result)?)
    }
}
