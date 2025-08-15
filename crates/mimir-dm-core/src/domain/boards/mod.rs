//! Board configuration and workflow management
//! 
//! This module defines the structure and behavior of different board types
//! (campaign, module, session) including their stages, transitions, and requirements.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod campaign_board;
pub mod module_board;
pub mod session_board;

/// Trait for defining board behavior
pub trait BoardDefinition {
    /// Get the board type identifier
    fn board_type(&self) -> &str;
    
    /// Get all valid stages for this board
    fn stages(&self) -> Vec<&str>;
    
    /// Check if a transition from one stage to another is valid
    fn can_transition(&self, from: &str, to: &str) -> bool;
    
    /// Get required document types for a specific stage
    fn required_documents(&self, stage: &str) -> Vec<&str>;
    
    /// Get optional document types for a specific stage
    fn optional_documents(&self, stage: &str) -> Vec<&str>;
    
    /// Get the next stage in the normal workflow progression
    fn next_stage(&self, current: &str) -> Option<&str>;
    
    /// Get stage-specific metadata (e.g., prompts, help text)
    fn stage_metadata(&self, stage: &str) -> StageMetadata;
    
    /// Get documents that don't require completion for stage progression
    /// (e.g., tracking documents that are ongoing)
    fn no_completion_required_documents(&self, _stage: &str) -> Vec<&str> {
        vec![]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageMetadata {
    pub display_name: String,
    pub description: String,
    pub completion_message: Option<String>,
    pub transition_prompt: Option<String>,
    pub help_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardCompletionStatus {
    pub board_type: String,
    pub current_stage: String,
    pub total_required_documents: usize,
    pub completed_required_documents: usize,
    pub total_optional_documents: usize,
    pub completed_optional_documents: usize,
    pub missing_required_documents: Vec<String>,
    pub is_stage_complete: bool,
    pub can_progress: bool,
    pub next_stage: Option<String>,
    pub stage_metadata: StageMetadata,
}

/// Registry of all board definitions
pub struct BoardRegistry {
    boards: HashMap<String, Box<dyn BoardDefinition + Send + Sync>>,
}

impl BoardRegistry {
    pub fn new() -> Self {
        let mut boards = HashMap::new();
        
        // Register all board types
        boards.insert(
            "campaign".to_string(), 
            Box::new(campaign_board::CampaignBoard::new()) as Box<dyn BoardDefinition + Send + Sync>
        );
        boards.insert(
            "module".to_string(), 
            Box::new(module_board::ModuleBoard::new()) as Box<dyn BoardDefinition + Send + Sync>
        );
        boards.insert(
            "session".to_string(), 
            Box::new(session_board::SessionBoard::new()) as Box<dyn BoardDefinition + Send + Sync>
        );
        
        Self { boards }
    }
    
    pub fn get(&self, board_type: &str) -> Option<&Box<dyn BoardDefinition + Send + Sync>> {
        self.boards.get(board_type)
    }
}

impl Default for BoardRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_registry_contains_all_boards() {
        let registry = BoardRegistry::new();
        
        // Test that all expected board types are registered
        assert!(registry.get("campaign").is_some());
        assert!(registry.get("module").is_some());
        assert!(registry.get("session").is_some());
    }

    #[test]
    fn test_board_registry_returns_none_for_invalid_type() {
        let registry = BoardRegistry::new();
        
        assert!(registry.get("invalid").is_none());
        assert!(registry.get("").is_none());
        assert!(registry.get("Campaign").is_none()); // Case sensitive
    }

    #[test]
    fn test_board_registry_returns_correct_board_types() {
        let registry = BoardRegistry::new();
        
        let campaign_board = registry.get("campaign").unwrap();
        assert_eq!(campaign_board.board_type(), "campaign");
        
        let module_board = registry.get("module").unwrap();
        assert_eq!(module_board.board_type(), "module");
        
        let session_board = registry.get("session").unwrap();
        assert_eq!(session_board.board_type(), "session");
    }

    #[test]
    fn test_board_registry_default_impl() {
        let registry = BoardRegistry::default();
        
        // Should have same behavior as new()
        assert!(registry.get("campaign").is_some());
        assert!(registry.get("module").is_some());
        assert!(registry.get("session").is_some());
    }

    #[test]
    fn test_all_boards_have_stages() {
        let registry = BoardRegistry::new();
        
        for board_type in ["campaign", "module", "session"] {
            let board = registry.get(board_type).expect(&format!("{} board should exist", board_type));
            let stages = board.stages();
            assert!(!stages.is_empty(), "{} board should have at least one stage", board_type);
        }
    }

    #[test]
    fn test_all_boards_have_valid_progressions() {
        let registry = BoardRegistry::new();
        
        for board_type in ["campaign", "module", "session"] {
            let board = registry.get(board_type).expect(&format!("{} board should exist", board_type));
            let stages = board.stages();
            
            // For each stage (except the last), there should be a valid next stage
            for i in 0..stages.len() - 1 {
                let current = stages[i];
                let next = board.next_stage(current);
                assert!(next.is_some(), "{} board: stage {} should have a next stage", board_type, current);
                
                // And transition should be allowed
                if let Some(next_stage) = next {
                    assert!(
                        board.can_transition(current, next_stage),
                        "{} board: should allow transition from {} to {}",
                        board_type, current, next_stage
                    );
                }
            }
        }
    }

    #[test]
    fn test_board_completion_status_serialization() {
        // Test that BoardCompletionStatus can be serialized/deserialized
        let status = BoardCompletionStatus {
            board_type: "campaign".to_string(),
            current_stage: "concept".to_string(),
            total_required_documents: 5,
            completed_required_documents: 3,
            total_optional_documents: 2,
            completed_optional_documents: 1,
            missing_required_documents: vec!["doc1".to_string(), "doc2".to_string()],
            is_stage_complete: false,
            can_progress: false,
            next_stage: Some("session_zero".to_string()),
            stage_metadata: StageMetadata {
                display_name: "Concept".to_string(),
                description: "Initial planning".to_string(),
                completion_message: Some("Great!".to_string()),
                transition_prompt: Some("Ready?".to_string()),
                help_text: Some("Help".to_string()),
            },
        };
        
        // Serialize to JSON
        let json = serde_json::to_string(&status).expect("Should serialize");
        
        // Deserialize back
        let deserialized: BoardCompletionStatus = serde_json::from_str(&json).expect("Should deserialize");
        
        assert_eq!(deserialized.board_type, status.board_type);
        assert_eq!(deserialized.current_stage, status.current_stage);
        assert_eq!(deserialized.total_required_documents, status.total_required_documents);
        assert_eq!(deserialized.missing_required_documents, status.missing_required_documents);
    }
}