//! Template seeder for initial campaign templates

use crate::dal::template_documents::TemplateRepository;
use crate::models::template_documents::{DocumentLevel, NewTemplateDocument, TemplateType};
use crate::models::template_frontmatter::{TemplateFrontmatter, TemplateVariable};
use diesel::prelude::*;
use std::fs;
use std::path::Path;

/// Template metadata for seeding
struct TemplateMetadata {
    file_name: &'static str,
    template_type: TemplateType,
    title: &'static str,
    purpose: &'static str,
    variables: Vec<TemplateVariable>,
}

impl TemplateMetadata {
    /// Get all template metadata
    fn all() -> Vec<Self> {
        vec![
            // Campaign Level Templates
            Self {
                file_name: "campaign-bible.md",
                template_type: TemplateType::CampaignBible,
                title: "Campaign Bible",
                purpose: "Complete reference document for your campaign world and story",
                variables: vec![
                    TemplateVariable {
                        name: "campaign_name".to_string(),
                        var_type: "string".to_string(),
                        description: "The name of your campaign".to_string(),
                        default: serde_json::json!("[Campaign Name]"),
                        required: true,
                    },
                    TemplateVariable {
                        name: "world_name".to_string(),
                        var_type: "string".to_string(),
                        description: "The name of your world".to_string(),
                        default: serde_json::json!("[World Name]"),
                        required: true,
                    },
                ],
            },
            Self {
                file_name: "campaign-pitch.md",
                template_type: TemplateType::CampaignPitch,
                title: "Campaign Pitch",
                purpose: "Create a one-page pitch to excite players about your campaign concept",
                variables: vec![
                    TemplateVariable {
                        name: "campaign_name".to_string(),
                        var_type: "string".to_string(),
                        description: "The name of your campaign".to_string(),
                        default: serde_json::json!("[Campaign Name]"),
                        required: true,
                    },
                    TemplateVariable {
                        name: "genre".to_string(),
                        var_type: "string".to_string(),
                        description: "Primary genre and tone".to_string(),
                        default: serde_json::json!("Fantasy Adventure"),
                        required: true,
                    },
                    TemplateVariable {
                        name: "pillars".to_string(),
                        var_type: "object".to_string(),
                        description: "Campaign pillar ratings (1-5)".to_string(),
                        default: serde_json::json!({
                            "combat": 3,
                            "exploration": 3,
                            "social": 3,
                            "mystery": 3
                        }),
                        required: true,
                    },
                ],
            },
            Self {
                file_name: "starting-scenario.md",
                template_type: TemplateType::StartingScenario,
                title: "Starting Scenario",
                purpose: "Design the opening scenario that brings the party together",
                variables: vec![
                    TemplateVariable {
                        name: "scenario_name".to_string(),
                        var_type: "string".to_string(),
                        description: "Name of the starting scenario".to_string(),
                        default: serde_json::json!("[Scenario Name]"),
                        required: true,
                    },
                    TemplateVariable {
                        name: "location".to_string(),
                        var_type: "string".to_string(),
                        description: "Starting location".to_string(),
                        default: serde_json::json!("[Starting Location]"),
                        required: true,
                    },
                ],
            },
            Self {
                file_name: "quick-start-kit.md",
                template_type: TemplateType::QuickStartKit,
                title: "Quick Start Kit",
                purpose: "Everything needed to start playing immediately",
                variables: vec![
                    TemplateVariable {
                        name: "campaign_name".to_string(),
                        var_type: "string".to_string(),
                        description: "The name of your campaign".to_string(),
                        default: serde_json::json!("[Campaign Name]"),
                        required: true,
                    },
                ],
            },
            
            // Module Templates
            Self {
                file_name: "module-overview.md",
                template_type: TemplateType::ModuleOverview,
                title: "Module Overview",
                purpose: "High-level planning document for a campaign module",
                variables: vec![
                    TemplateVariable {
                        name: "module_name".to_string(),
                        var_type: "string".to_string(),
                        description: "Name of the module".to_string(),
                        default: serde_json::json!("[Module Name]"),
                        required: true,
                    },
                    TemplateVariable {
                        name: "module_number".to_string(),
                        var_type: "number".to_string(),
                        description: "Module number in campaign sequence".to_string(),
                        default: serde_json::json!(1),
                        required: true,
                    },
                ],
            },
            Self {
                file_name: "module-dungeon.md",
                template_type: TemplateType::ModuleDungeon,
                title: "Dungeon Module",
                purpose: "Template for dungeon crawl modules",
                variables: vec![
                    TemplateVariable {
                        name: "dungeon_name".to_string(),
                        var_type: "string".to_string(),
                        description: "Name of the dungeon".to_string(),
                        default: serde_json::json!("[Dungeon Name]"),
                        required: true,
                    },
                ],
            },
            Self {
                file_name: "module-heist.md",
                template_type: TemplateType::ModuleHeist,
                title: "Heist Module",
                purpose: "Template for heist and infiltration modules",
                variables: vec![
                    TemplateVariable {
                        name: "target_name".to_string(),
                        var_type: "string".to_string(),
                        description: "Name of the heist target".to_string(),
                        default: serde_json::json!("[Target Name]"),
                        required: true,
                    },
                ],
            },
            Self {
                file_name: "module-horror.md",
                template_type: TemplateType::ModuleHorror,
                title: "Horror Module",
                purpose: "Template for horror and suspense modules",
                variables: vec![
                    TemplateVariable {
                        name: "horror_name".to_string(),
                        var_type: "string".to_string(),
                        description: "Name of the horror scenario".to_string(),
                        default: serde_json::json!("[Horror Name]"),
                        required: true,
                    },
                ],
            },
            Self {
                file_name: "module-mystery.md",
                template_type: TemplateType::ModuleMystery,
                title: "Mystery Module",
                purpose: "Template for investigation and mystery modules",
                variables: vec![
                    TemplateVariable {
                        name: "mystery_name".to_string(),
                        var_type: "string".to_string(),
                        description: "Name of the mystery".to_string(),
                        default: serde_json::json!("[Mystery Name]"),
                        required: true,
                    },
                ],
            },
            Self {
                file_name: "module-political.md",
                template_type: TemplateType::ModulePolitical,
                title: "Political Module",
                purpose: "Template for political intrigue modules",
                variables: vec![
                    TemplateVariable {
                        name: "conflict_name".to_string(),
                        var_type: "string".to_string(),
                        description: "Name of the political conflict".to_string(),
                        default: serde_json::json!("[Conflict Name]"),
                        required: true,
                    },
                ],
            },
            
            // Character & NPC Templates
            Self {
                file_name: "character-integration.md",
                template_type: TemplateType::CharacterIntegration,
                title: "Character Integration",
                purpose: "Integrate player characters into your campaign world",
                variables: vec![
                    TemplateVariable {
                        name: "character_name".to_string(),
                        var_type: "string".to_string(),
                        description: "Character name".to_string(),
                        default: serde_json::json!("[Character Name]"),
                        required: true,
                    },
                    TemplateVariable {
                        name: "player_name".to_string(),
                        var_type: "string".to_string(),
                        description: "Player name".to_string(),
                        default: serde_json::json!("[Player Name]"),
                        required: true,
                    },
                ],
            },
            Self {
                file_name: "major-npc-tracker.md",
                template_type: TemplateType::MajorNpcTracker,
                title: "Major NPC Tracker",
                purpose: "Track important NPCs and their relationships",
                variables: vec![
                    TemplateVariable {
                        name: "npc_name".to_string(),
                        var_type: "string".to_string(),
                        description: "NPC name".to_string(),
                        default: serde_json::json!("[NPC Name]"),
                        required: true,
                    },
                ],
            },
            Self {
                file_name: "quick-npc-reference.md",
                template_type: TemplateType::QuickNpcReference,
                title: "Quick NPC Reference",
                purpose: "Quick reference sheet for NPCs during play",
                variables: vec![],
            },
            Self {
                file_name: "pc-arc-tracker.md",
                template_type: TemplateType::PcArcTracker,
                title: "PC Arc Tracker",
                purpose: "Track character arcs and personal storylines",
                variables: vec![
                    TemplateVariable {
                        name: "character_name".to_string(),
                        var_type: "string".to_string(),
                        description: "Character name".to_string(),
                        default: serde_json::json!("[Character Name]"),
                        required: true,
                    },
                ],
            },
            
            // World Building Templates
            Self {
                file_name: "world-overview.md",
                template_type: TemplateType::WorldOverview,
                title: "World Overview",
                purpose: "High-level overview of your campaign world",
                variables: vec![
                    TemplateVariable {
                        name: "world_name".to_string(),
                        var_type: "string".to_string(),
                        description: "Name of your world".to_string(),
                        default: serde_json::json!("[World Name]"),
                        required: true,
                    },
                ],
            },
            Self {
                file_name: "region-overview.md",
                template_type: TemplateType::RegionOverview,
                title: "Region Overview",
                purpose: "Detailed overview of a specific region",
                variables: vec![
                    TemplateVariable {
                        name: "region_name".to_string(),
                        var_type: "string".to_string(),
                        description: "Name of the region".to_string(),
                        default: serde_json::json!("[Region Name]"),
                        required: true,
                    },
                ],
            },
            Self {
                file_name: "faction-template.md",
                template_type: TemplateType::FactionTemplate,
                title: "Faction Template",
                purpose: "Define factions and their goals, resources, and relationships",
                variables: vec![
                    TemplateVariable {
                        name: "faction_name".to_string(),
                        var_type: "string".to_string(),
                        description: "Name of the faction".to_string(),
                        default: serde_json::json!("[Faction Name]"),
                        required: true,
                    },
                ],
            },
            
            // Session Management Templates
            Self {
                file_name: "session-outline.md",
                template_type: TemplateType::SessionOutline,
                title: "Session Outline",
                purpose: "Plan and track individual game sessions",
                variables: vec![
                    TemplateVariable {
                        name: "session_number".to_string(),
                        var_type: "number".to_string(),
                        description: "Session number".to_string(),
                        default: serde_json::json!(1),
                        required: true,
                    },
                    TemplateVariable {
                        name: "session_title".to_string(),
                        var_type: "string".to_string(),
                        description: "Session title".to_string(),
                        default: serde_json::json!("[Session Title]"),
                        required: true,
                    },
                ],
            },
            Self {
                file_name: "clue-tracker.md",
                template_type: TemplateType::ClueTracker,
                title: "Clue Tracker",
                purpose: "Track clues and information flow in mystery scenarios",
                variables: vec![],
            },
            Self {
                file_name: "document-tracker.md",
                template_type: TemplateType::DocumentTracker,
                title: "Document Tracker",
                purpose: "Track in-game documents, letters, and handouts",
                variables: vec![],
            },
        ]
    }
}

/// Seed the database with initial templates
pub fn seed_templates(conn: &mut SqliteConnection) -> Result<usize, diesel::result::Error> {
    let template_dir = Path::new("/Users/dstorey/Desktop/colliery/mimir/docs/src/campaign-framework/06-templates/templates");
    let mut count = 0;
    
    for metadata in TemplateMetadata::all() {
        let file_path = template_dir.join(metadata.file_name);
        
        // Read the template content
        let content = fs::read_to_string(&file_path)
            .map_err(|e| diesel::result::Error::QueryBuilderError(
                format!("Failed to read template file {}: {}", metadata.file_name, e).into()
            ))?;
        
        // Create frontmatter
        let frontmatter = TemplateFrontmatter {
            id: metadata.template_type.as_str().replace('_', "-"),
            title: metadata.title.to_string(),
            template_type: metadata.template_type.as_str().to_string(),
            level: metadata.template_type.document_level().as_str().to_string(),
            purpose: metadata.purpose.to_string(),
            variables: metadata.variables,
            author: "Mimir Team".to_string(),
        };
        
        // Create the template document
        let new_template = NewTemplateDocument {
            document_id: frontmatter.id.clone(),
            version_number: Some(1),
            document_content: content,
            document_type: Some(frontmatter.template_type.clone()),
            document_level: Some(frontmatter.level.clone()),
            purpose: Some(frontmatter.purpose.clone()),
            variables_schema: Some(frontmatter.variables_schema()?),
            default_values: Some(serde_json::to_string(&frontmatter.defaults_map())?),
            is_active: Some(true),
            metadata: Some(frontmatter.to_json()?),
        };
        
        // Check if template already exists
        let exists = TemplateRepository::get_latest(conn, &new_template.document_id).is_ok();
        
        if !exists {
            TemplateRepository::create(conn, new_template)?;
            count += 1;
        }
    }
    
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::establish_connection;
    
    #[test]
    fn test_seed_templates() {
        let mut conn = establish_connection(":memory:").unwrap();
        
        // Run migrations
        crate::run_migrations(&mut conn).unwrap();
        
        // Seed templates
        let count = seed_templates(&mut conn).unwrap();
        assert_eq!(count, 21); // We have 21 templates
        
        // Verify a few templates exist
        let campaign_pitch = TemplateRepository::get_latest(&mut conn, "campaign-pitch").unwrap();
        assert_eq!(campaign_pitch.document_type.unwrap(), "campaign_pitch");
        assert_eq!(campaign_pitch.document_level.unwrap(), "campaign");
        
        let module_overview = TemplateRepository::get_latest(&mut conn, "module-overview").unwrap();
        assert_eq!(module_overview.document_type.unwrap(), "module_overview");
        assert_eq!(module_overview.document_level.unwrap(), "module");
    }
}