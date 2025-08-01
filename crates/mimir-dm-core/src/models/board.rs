//! Board and workflow management types
//!
//! Implements the Three-Board System for campaign workflow management.

use super::ids::{CampaignId, ModuleId, SessionId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// The three board types in the system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoardType {
    Campaign,
    Module,
    Session,
}

/// Campaign board workflow states
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CampaignWorkflowState {
    /// Ideas and possibilities
    Concept,
    /// Player-facing preparation
    SessionZero,
    /// Weaving into active play
    Integration,
    /// Currently affecting play
    Active,
    /// Wrapping up threads
    Concluding,
}

/// Module board workflow states
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModuleWorkflowState {
    /// Future possibilities
    Backlog,
    /// Active development
    Planning,
    /// Detailed creation
    Development,
    /// Prepared components
    Ready,
    /// Currently running
    Active,
    /// Finished modules
    Completed,
}

/// Session board workflow states
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionWorkflowState {
    /// Upcoming session
    NextWeek,
    /// Required materials
    PrepNeeded,
    /// Active preparation
    InPrep,
    /// Table-ready materials
    Ready,
    /// Finished sessions
    Complete,
}

/// A card on any of the three boards
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowCard {
    /// Unique identifier for this card
    pub id: String,
    
    /// Which board this card belongs to
    pub board_type: BoardType,
    
    /// Brief title for the card
    pub title: String,
    
    /// Optional longer description
    pub description: Option<String>,
    
    /// When this card was created
    pub created_at: DateTime<Utc>,
    
    /// When this card last moved columns
    pub last_moved_at: DateTime<Utc>,
    
    /// Current workflow state (stored as string for flexibility)
    pub workflow_state: String,
    
    /// Associated entity ID based on board type
    pub entity_id: Option<EntityReference>,
    
    /// Tags for filtering and organization
    pub tags: Vec<String>,
    
    /// Cards that must complete before this one can progress
    pub blocked_by: Vec<String>,
    
    /// Priority within the column (lower = higher priority)
    pub priority: i32,
}

/// Reference to an entity (Campaign, Module, or Session)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntityReference {
    Campaign(CampaignId),
    Module(ModuleId),
    Session(SessionId),
}

impl WorkflowCard {
    /// Create a new workflow card
    pub fn new(board_type: BoardType, title: String, workflow_state: String) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            board_type,
            title,
            description: None,
            created_at: now,
            last_moved_at: now,
            workflow_state,
            entity_id: None,
            tags: Vec::new(),
            blocked_by: Vec::new(),
            priority: 0,
        }
    }
    
    /// Move card to a new workflow state
    pub fn move_to(&mut self, new_state: String) {
        self.workflow_state = new_state;
        self.last_moved_at = Utc::now();
    }
    
    /// Check if this card is blocked by other cards
    pub fn is_blocked(&self) -> bool {
        !self.blocked_by.is_empty()
    }
    
    /// Add a tag to this card
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }
    
    /// Remove a tag from this card
    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.retain(|t| t != tag);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_workflow_card_creation() {
        let card = WorkflowCard::new(
            BoardType::Campaign,
            "The Imprisoned Corruption".to_string(),
            "concept".to_string()
        );
        
        assert_eq!(card.board_type, BoardType::Campaign);
        assert_eq!(card.title, "The Imprisoned Corruption");
        assert_eq!(card.workflow_state, "concept");
        assert!(!card.is_blocked());
    }
    
    #[test]
    fn test_workflow_card_movement() {
        let mut card = WorkflowCard::new(
            BoardType::Module,
            "The Brittle Steel Mystery".to_string(),
            "backlog".to_string()
        );
        
        let original_time = card.last_moved_at;
        
        // Sleep briefly to ensure time difference
        std::thread::sleep(std::time::Duration::from_millis(10));
        
        card.move_to("planning".to_string());
        
        assert_eq!(card.workflow_state, "planning");
        assert!(card.last_moved_at > original_time);
    }
    
    #[test]
    fn test_card_blocking() {
        let mut card = WorkflowCard::new(
            BoardType::Session,
            "Session 4: The Mountain Road".to_string(),
            "next_week".to_string()
        );
        
        assert!(!card.is_blocked());
        
        card.blocked_by.push("session-3-prep".to_string());
        assert!(card.is_blocked());
    }
}