//! Module board definition

use super::{BoardDefinition, StageMetadata};

pub struct ModuleBoard;

impl ModuleBoard {
    pub fn new() -> Self {
        Self
    }
}

impl BoardDefinition for ModuleBoard {
    fn board_type(&self) -> &str {
        "module"
    }
    
    fn stages(&self) -> Vec<&str> {
        vec!["backlog", "planning", "development", "ready", "active", "completed"]
    }
    
    fn can_transition(&self, from: &str, to: &str) -> bool {
        match (from, to) {
            // Forward progression
            ("backlog", "planning") => true,
            ("planning", "development") => true,
            ("development", "ready") => true,
            ("ready", "active") => true,
            ("active", "completed") => true,
            
            // Allow moving back
            ("planning", "backlog") => true,
            ("development", "planning") => true,
            ("ready", "development") => true,
            
            _ => false,
        }
    }
    
    fn required_documents(&self, stage: &str) -> Vec<&str> {
        match stage {
            "planning" => vec!["module_outline"],
            "development" => vec!["module_overview", "encounter_list"],
            "ready" => vec!["module_guide", "npc_roster", "location_gazetteer"],
            _ => vec![],
        }
    }
    
    fn optional_documents(&self, stage: &str) -> Vec<&str> {
        match stage {
            "planning" => vec!["inspiration_notes"],
            "development" => vec!["custom_monsters", "treasure_list"],
            "ready" => vec!["player_handouts", "battle_maps"],
            "active" => vec!["session_notes"],
            _ => vec![],
        }
    }
    
    fn next_stage(&self, current: &str) -> Option<&str> {
        match current {
            "backlog" => Some("planning"),
            "planning" => Some("development"),
            "development" => Some("ready"),
            "ready" => Some("active"),
            "active" => Some("completed"),
            _ => None,
        }
    }
    
    fn stage_metadata(&self, stage: &str) -> StageMetadata {
        StageMetadata {
            display_name: stage.to_string(),
            description: format!("Module in {} stage", stage),
            completion_message: None,
            transition_prompt: None,
            help_text: None,
        }
    }
}