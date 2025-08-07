//! Session board definition

use super::{BoardDefinition, StageMetadata};

pub struct SessionBoard;

impl SessionBoard {
    pub fn new() -> Self {
        Self
    }
}

impl BoardDefinition for SessionBoard {
    fn board_type(&self) -> &str {
        "session"
    }
    
    fn stages(&self) -> Vec<&str> {
        vec!["next_week", "prep_needed", "in_prep", "ready", "complete"]
    }
    
    fn can_transition(&self, from: &str, to: &str) -> bool {
        match (from, to) {
            // Forward progression
            ("next_week", "prep_needed") => true,
            ("prep_needed", "in_prep") => true,
            ("in_prep", "ready") => true,
            ("ready", "complete") => true,
            
            // Allow deferring
            ("prep_needed", "next_week") => true,
            
            // Allow moving back for more prep
            ("in_prep", "prep_needed") => true,
            ("ready", "in_prep") => true,
            
            _ => false,
        }
    }
    
    fn required_documents(&self, stage: &str) -> Vec<&str> {
        match stage {
            "in_prep" => vec!["session_outline"],
            "ready" => vec!["session_plan", "encounter_notes"],
            _ => vec![],
        }
    }
    
    fn optional_documents(&self, stage: &str) -> Vec<&str> {
        match stage {
            "prep_needed" => vec!["prep_checklist"],
            "in_prep" => vec!["npc_dialogue", "improv_prompts"],
            "ready" => vec!["initiative_tracker", "loot_rolls"],
            "complete" => vec!["session_recap", "xp_log"],
            _ => vec![],
        }
    }
    
    fn next_stage(&self, current: &str) -> Option<&str> {
        match current {
            "next_week" => Some("prep_needed"),
            "prep_needed" => Some("in_prep"),
            "in_prep" => Some("ready"),
            "ready" => Some("complete"),
            _ => None,
        }
    }
    
    fn stage_metadata(&self, stage: &str) -> StageMetadata {
        StageMetadata {
            display_name: stage.to_string(),
            description: format!("Session in {} stage", stage),
            completion_message: None,
            transition_prompt: None,
            help_text: None,
        }
    }
}