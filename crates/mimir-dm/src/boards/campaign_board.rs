//! Campaign board definition

use super::{BoardDefinition, StageMetadata};

pub struct CampaignBoard;

impl CampaignBoard {
    pub fn new() -> Self {
        Self
    }
}

impl BoardDefinition for CampaignBoard {
    fn board_type(&self) -> &str {
        "campaign"
    }
    
    fn stages(&self) -> Vec<&str> {
        vec!["concept", "session_zero", "integration", "active", "concluding", "completed"]
    }
    
    fn can_transition(&self, from: &str, to: &str) -> bool {
        match (from, to) {
            // Forward progression
            ("concept", "session_zero") => true,
            ("session_zero", "integration") => true,
            ("integration", "active") => true,
            ("active", "concluding") => true,
            ("concluding", "completed") => true,
                        
            _ => false,
        }
    }
    
    fn required_documents(&self, stage: &str) -> Vec<&str> {
        match stage {
            "concept" => vec!["campaign_pitch"],
            "session_zero" => vec![
                "starting_scenario",
                "world_primer",
                "character_guidelines",
                "table_expectations",
                "character_integration_forms",
            ],
            "integration" => vec![
                "campaign_bible",
                "character_integration_notes",
                "major_npcs",
                "world_events_timeline",
            ],
            "active" => vec![], // No required documents
            "concluding" => vec![],
            "completed" => vec![],
            _ => vec![],
        }
    }
    
    fn optional_documents(&self, stage: &str) -> Vec<&str> {
        match stage {
            "concept" => vec!["campaign_notes", "inspiration_board"],
            "session_zero" => vec!["safety_tools", "house_rules"],
            "integration" => vec!["player_secrets", "faction_overview"],
            "active" => vec!["session_notes", "player_handouts"],
            _ => vec![],
        }
    }
    
    fn next_stage(&self, current: &str) -> Option<&str> {
        match current {
            "concept" => Some("session_zero"),
            "session_zero" => Some("integration"),
            "integration" => Some("active"),
            "active" => Some("concluding"),
            "concluding" => Some("completed"),
            _ => None,
        }
    }
    
    fn stage_metadata(&self, stage: &str) -> StageMetadata {
        match stage {
            "concept" => StageMetadata {
                display_name: "Concept".to_string(),
                description: "Initial campaign planning and pitch development".to_string(),
                completion_message: Some(
                    "Great! Your campaign pitch is complete. Next, you'll prepare materials for Session Zero."
                        .to_string()
                ),
                transition_prompt: Some(
                    "You can always edit this document later, but make sure your players have a chance to read the pitch and provide initial feedback before Session Zero."
                        .to_string()
                ),
                help_text: Some(
                    "The Concept stage is where you develop your initial campaign idea and create a compelling pitch to attract players."
                        .to_string()
                ),
            },
            "session_zero" => StageMetadata {
                display_name: "Session Zero".to_string(),
                description: "Preparing materials for the collaborative session zero".to_string(),
                completion_message: Some(
                    "Excellent! Your Session Zero materials are ready. After your Session Zero, you'll move to the Integration stage."
                        .to_string()
                ),
                transition_prompt: Some(
                    "Remember to share these documents with your players before Session Zero. Take notes during the session as you'll need them for the Integration stage."
                        .to_string()
                ),
                help_text: Some(
                    "Session Zero is a collaborative session where you and your players establish expectations, create characters, and build the campaign world together."
                        .to_string()
                ),
            },
            "integration" => StageMetadata {
                display_name: "Integration".to_string(),
                description: "Integrating player feedback and characters into the campaign".to_string(),
                completion_message: Some(
                    "Perfect! Your campaign is fully integrated and ready to begin. Time to start your adventure!"
                        .to_string()
                ),
                transition_prompt: Some(
                    "These documents will be your reference throughout the campaign. Make sure everything from Session Zero has been incorporated."
                        .to_string()
                ),
                help_text: Some(
                    "The Integration stage is where you take all the collaborative elements from Session Zero and weave them into your campaign world."
                        .to_string()
                ),
            },
            "active" => StageMetadata {
                display_name: "Active".to_string(),
                description: "Campaign is actively being played".to_string(),
                completion_message: None,
                transition_prompt: None,
                help_text: Some(
                    "Your campaign is now active! Use the session boards to manage individual game sessions."
                        .to_string()
                ),
            },
            _ => StageMetadata {
                display_name: stage.to_string(),
                description: format!("The {} stage", stage),
                completion_message: None,
                transition_prompt: None,
                help_text: None,
            },
        }
    }
}