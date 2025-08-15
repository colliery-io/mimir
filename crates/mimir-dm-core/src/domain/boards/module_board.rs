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
            "planning" => vec!["module_overview"],
            "development" => vec!["quick_npc_reference"],
            "ready" => vec!["session_outline"],
            "active" => vec!["document_tracker"],
            _ => vec![],
        }
    }
    
    fn optional_documents(&self, stage: &str) -> Vec<&str> {
        match stage {
            "planning" => vec![],
            "development" => vec!["major_npc_tracker", "faction_template"],
            "ready" => vec!["clue_tracker", "region_overview"],
            "active" => vec![],
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
        match stage {
            "backlog" => StageMetadata {
                display_name: "Backlog".to_string(),
                description: "Module ideas waiting to be developed".to_string(),
                completion_message: None,
                transition_prompt: Some(
                    "Ready to start planning this module? You'll begin outlining the core concept and structure."
                        .to_string()
                ),
                help_text: Some(
                    "The backlog holds module ideas that haven't been started yet. When you're ready to develop a new module, move it to planning."
                        .to_string()
                ),
            },
            "planning" => StageMetadata {
                display_name: "Planning".to_string(),
                description: "Developing the module concept and structure".to_string(),
                completion_message: Some(
                    "Module concept is solid! Time to develop the details."
                        .to_string()
                ),
                transition_prompt: Some(
                    "Have you completed the module outline? Moving to development will begin creating encounters and NPCs."
                        .to_string()
                ),
                help_text: Some(
                    "During planning, focus on the module's core concept, stakes, hook, and overall structure. This is your blueprint."
                        .to_string()
                ),
            },
            "development" => StageMetadata {
                display_name: "Development".to_string(),
                description: "Creating encounters, NPCs, and locations".to_string(),
                completion_message: Some(
                    "Module content is complete! Now finalize everything for play."
                        .to_string()
                ),
                transition_prompt: Some(
                    "Is all content created? Moving to ready means the module is fully prepared for play."
                        .to_string()
                ),
                help_text: Some(
                    "Develop your NPCs, encounters, locations, and clues. Build out the module's content based on your plan."
                        .to_string()
                ),
            },
            "ready" => StageMetadata {
                display_name: "Ready".to_string(),
                description: "Module is complete and ready to run".to_string(),
                completion_message: Some(
                    "Module is ready to run! You can start it whenever you're ready."
                        .to_string()
                ),
                transition_prompt: Some(
                    "Ready to run this module? Moving to active means you'll begin playing it in your next session."
                        .to_string()
                ),
                help_text: Some(
                    "The module is fully prepared. Review your materials and make any final adjustments before running it."
                        .to_string()
                ),
            },
            "active" => StageMetadata {
                display_name: "Active".to_string(),
                description: "Module is currently being played".to_string(),
                completion_message: None,
                transition_prompt: Some(
                    "Has the module concluded? Mark it complete to archive it and move on to the next module."
                        .to_string()
                ),
                help_text: Some(
                    "This module is currently being played. Track your sessions and take notes as you go."
                        .to_string()
                ),
            },
            "completed" => StageMetadata {
                display_name: "Completed".to_string(),
                description: "Module has been played and completed".to_string(),
                completion_message: None,
                transition_prompt: None,
                help_text: Some(
                    "This module has been completed. Session notes and outcomes are preserved for reference."
                        .to_string()
                ),
            },
            _ => StageMetadata {
                display_name: stage.to_string(),
                description: format!("Module in {} stage", stage),
                completion_message: None,
                transition_prompt: None,
                help_text: None,
            },
        }
    }
    
    fn no_completion_required_documents(&self, stage: &str) -> Vec<&str> {
        match stage {
            "active" => vec!["document_tracker"],
            _ => vec![],
        }
    }
}