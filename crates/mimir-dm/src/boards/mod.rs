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