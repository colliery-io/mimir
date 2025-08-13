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
                "character_integration",
            ],
            "integration" => vec![
                "campaign_bible",
                "major_npc_tracker",
            ],
            "active" => vec![], // No required documents
            "concluding" => vec![],
            "completed" => vec![],
            _ => vec![],
        }
    }
    
    fn optional_documents(&self, stage: &str) -> Vec<&str> {
        match stage {
            "concept" => vec![],  // No optional documents - notes and inspiration are working tools, not artifacts
            "session_zero" => vec!["safety_tools", "house_rules"],
            "integration" => vec!["player_secrets", "faction_overview"],
            "active" => vec![],  // No documents in active stage - managed through session boards
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_type() {
        let board = CampaignBoard::new();
        assert_eq!(board.board_type(), "campaign");
    }

    #[test]
    fn test_stages_order() {
        let board = CampaignBoard::new();
        let stages = board.stages();
        
        assert_eq!(stages.len(), 6);
        assert_eq!(stages[0], "concept");
        assert_eq!(stages[1], "session_zero");
        assert_eq!(stages[2], "integration");
        assert_eq!(stages[3], "active");
        assert_eq!(stages[4], "concluding");
        assert_eq!(stages[5], "completed");
    }

    #[test]
    fn test_valid_forward_transitions() {
        let board = CampaignBoard::new();
        
        // Test all valid forward transitions
        assert!(board.can_transition("concept", "session_zero"));
        assert!(board.can_transition("session_zero", "integration"));
        assert!(board.can_transition("integration", "active"));
        assert!(board.can_transition("active", "concluding"));
        assert!(board.can_transition("concluding", "completed"));
    }

    #[test]
    fn test_invalid_transitions() {
        let board = CampaignBoard::new();
        
        // Test backward transitions (not allowed)
        assert!(!board.can_transition("session_zero", "concept"));
        assert!(!board.can_transition("integration", "session_zero"));
        assert!(!board.can_transition("active", "integration"));
        
        // Test skip transitions (not allowed)
        assert!(!board.can_transition("concept", "integration"));
        assert!(!board.can_transition("concept", "active"));
        assert!(!board.can_transition("session_zero", "active"));
        
        // Test self-transitions (not allowed)
        assert!(!board.can_transition("concept", "concept"));
        assert!(!board.can_transition("active", "active"));
        
        // Test from completed (no transitions allowed)
        assert!(!board.can_transition("completed", "concept"));
        assert!(!board.can_transition("completed", "active"));
        
        // Test invalid stage names
        assert!(!board.can_transition("invalid", "concept"));
        assert!(!board.can_transition("concept", "invalid"));
    }

    #[test]
    fn test_required_documents_per_stage() {
        let board = CampaignBoard::new();
        
        // Concept stage
        let concept_docs = board.required_documents("concept");
        assert_eq!(concept_docs.len(), 1);
        assert_eq!(concept_docs[0], "campaign_pitch");
        
        // Session Zero stage
        let session_zero_docs = board.required_documents("session_zero");
        assert_eq!(session_zero_docs.len(), 5);
        assert!(session_zero_docs.contains(&"starting_scenario"));
        assert!(session_zero_docs.contains(&"world_primer"));
        assert!(session_zero_docs.contains(&"character_guidelines"));
        assert!(session_zero_docs.contains(&"table_expectations"));
        assert!(session_zero_docs.contains(&"character_integration"));
        
        // Integration stage
        let integration_docs = board.required_documents("integration");
        assert_eq!(integration_docs.len(), 2);
        assert!(integration_docs.contains(&"campaign_bible"));
        assert!(integration_docs.contains(&"major_npc_tracker"));
        
        // Active stage (no required documents)
        assert_eq!(board.required_documents("active").len(), 0);
        assert_eq!(board.required_documents("concluding").len(), 0);
        assert_eq!(board.required_documents("completed").len(), 0);
        
        // Invalid stage
        assert_eq!(board.required_documents("invalid").len(), 0);
    }

    #[test]
    fn test_optional_documents_per_stage() {
        let board = CampaignBoard::new();
        
        // Concept stage - no optional documents (notes and inspiration are tools, not artifacts)
        let concept_optional = board.optional_documents("concept");
        assert_eq!(concept_optional.len(), 0);
        
        // Session Zero stage
        let session_zero_optional = board.optional_documents("session_zero");
        assert_eq!(session_zero_optional.len(), 2);
        assert!(session_zero_optional.contains(&"safety_tools"));
        assert!(session_zero_optional.contains(&"house_rules"));
        
        // Integration stage
        let integration_optional = board.optional_documents("integration");
        assert_eq!(integration_optional.len(), 2);
        assert!(integration_optional.contains(&"player_secrets"));
        assert!(integration_optional.contains(&"faction_overview"));
        
        // Active stage - no documents (managed through session boards)
        let active_optional = board.optional_documents("active");
        assert_eq!(active_optional.len(), 0);
        
        // Stages with no optional documents
        assert_eq!(board.optional_documents("concluding").len(), 0);
        assert_eq!(board.optional_documents("completed").len(), 0);
        assert_eq!(board.optional_documents("invalid").len(), 0);
    }

    #[test]
    fn test_next_stage_progression() {
        let board = CampaignBoard::new();
        
        assert_eq!(board.next_stage("concept"), Some("session_zero"));
        assert_eq!(board.next_stage("session_zero"), Some("integration"));
        assert_eq!(board.next_stage("integration"), Some("active"));
        assert_eq!(board.next_stage("active"), Some("concluding"));
        assert_eq!(board.next_stage("concluding"), Some("completed"));
        assert_eq!(board.next_stage("completed"), None);
        assert_eq!(board.next_stage("invalid"), None);
    }

    #[test]
    fn test_stage_metadata_completeness() {
        let board = CampaignBoard::new();
        
        // Test that all stages have metadata
        for stage in board.stages() {
            let metadata = board.stage_metadata(stage);
            assert!(!metadata.display_name.is_empty());
            assert!(!metadata.description.is_empty());
        }
        
        // Test specific metadata for concept stage
        let concept_meta = board.stage_metadata("concept");
        assert_eq!(concept_meta.display_name, "Concept");
        assert!(concept_meta.description.contains("planning"));
        assert!(concept_meta.completion_message.is_some());
        assert!(concept_meta.transition_prompt.is_some());
        assert!(concept_meta.help_text.is_some());
        
        // Test specific metadata for session_zero stage
        let session_zero_meta = board.stage_metadata("session_zero");
        assert_eq!(session_zero_meta.display_name, "Session Zero");
        assert!(session_zero_meta.description.contains("collaborative"));
        assert!(session_zero_meta.completion_message.is_some());
        assert!(session_zero_meta.transition_prompt.is_some());
        assert!(session_zero_meta.help_text.is_some());
        
        // Test specific metadata for integration stage
        let integration_meta = board.stage_metadata("integration");
        assert_eq!(integration_meta.display_name, "Integration");
        assert!(integration_meta.description.contains("player feedback"));
        assert!(integration_meta.completion_message.is_some());
        assert!(integration_meta.transition_prompt.is_some());
        assert!(integration_meta.help_text.is_some());
        
        // Test specific metadata for active stage
        let active_meta = board.stage_metadata("active");
        assert_eq!(active_meta.display_name, "Active");
        assert!(active_meta.description.contains("actively being played"));
        assert!(active_meta.completion_message.is_none());
        assert!(active_meta.transition_prompt.is_none());
        assert!(active_meta.help_text.is_some());
        
        // Test fallback metadata for unknown stage
        let unknown_meta = board.stage_metadata("unknown");
        assert_eq!(unknown_meta.display_name, "unknown");
        assert_eq!(unknown_meta.description, "The unknown stage");
        assert!(unknown_meta.completion_message.is_none());
        assert!(unknown_meta.transition_prompt.is_none());
        assert!(unknown_meta.help_text.is_none());
    }

    #[test]
    fn test_stage_progression_completeness() {
        let board = CampaignBoard::new();
        let stages = board.stages();
        
        // Verify that each stage (except the last) has a next stage
        for i in 0..stages.len() - 1 {
            let current = stages[i];
            let expected_next = stages[i + 1];
            assert_eq!(board.next_stage(current), Some(expected_next));
        }
        
        // Verify the last stage has no next stage
        assert_eq!(board.next_stage(stages[stages.len() - 1]), None);
    }

    #[test]
    fn test_transition_consistency_with_next_stage() {
        let board = CampaignBoard::new();
        
        // For each stage that has a next stage, verify can_transition agrees
        for stage in board.stages() {
            if let Some(next) = board.next_stage(stage) {
                assert!(
                    board.can_transition(stage, next),
                    "Stage {} should be able to transition to next stage {}",
                    stage,
                    next
                );
            }
        }
    }

    #[test]
    fn test_no_orphaned_transitions() {
        let board = CampaignBoard::new();
        let valid_stages: Vec<&str> = board.stages();
        
        // Test that can_transition only returns true for valid stage pairs
        for from in &valid_stages {
            for to in &valid_stages {
                if board.can_transition(from, to) {
                    // If transition is allowed, verify it matches next_stage
                    assert_eq!(
                        board.next_stage(from),
                        Some(*to),
                        "Transition from {} to {} is allowed but doesn't match next_stage",
                        from,
                        to
                    );
                }
            }
        }
    }
}