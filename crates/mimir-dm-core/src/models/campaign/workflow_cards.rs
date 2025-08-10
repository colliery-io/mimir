//! Workflow card database models and operations

use crate::schema::{workflow_cards, workflow_card_tags};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// Database model for workflow cards
#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize, Associations)]
#[diesel(table_name = workflow_cards)]
#[diesel(belongs_to(crate::models::campaign::campaigns::Campaign))]
#[diesel(belongs_to(crate::models::campaign::modules::Module))]
#[diesel(belongs_to(crate::models::campaign::sessions::Session))]
pub struct WorkflowCard {
    pub id: String,
    pub board_type: String,
    pub title: String,
    pub description: Option<String>,
    pub created_at: String,
    pub last_moved_at: String,
    pub workflow_state: String,
    pub campaign_id: Option<i32>,
    pub module_id: Option<i32>,
    pub session_id: Option<i32>,
    pub priority: i32,
}

/// New workflow card for insertion
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = workflow_cards)]
pub struct NewWorkflowCard {
    pub id: String,
    pub board_type: String,
    pub title: String,
    pub description: Option<String>,
    pub workflow_state: String,
    pub campaign_id: Option<i32>,
    pub module_id: Option<i32>,
    pub session_id: Option<i32>,
    pub priority: i32,
}

/// Workflow card update structure
#[derive(Debug, Clone, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = workflow_cards)]
pub struct UpdateWorkflowCard {
    pub title: Option<String>,
    pub description: Option<Option<String>>,
    pub workflow_state: Option<String>,
    pub last_moved_at: Option<String>,
    pub priority: Option<i32>,
}

/// Database model for workflow card tags
#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize, Associations)]
#[diesel(table_name = workflow_card_tags)]
#[diesel(belongs_to(WorkflowCard, foreign_key = card_id))]
#[diesel(primary_key(card_id, tag))]
pub struct WorkflowCardTag {
    pub card_id: String,
    pub tag: String,
}

/// New tag for insertion
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = workflow_card_tags)]
pub struct NewWorkflowCardTag {
    pub card_id: String,
    pub tag: String,
}

impl WorkflowCard {
    /// Check if card can transition to the given state based on board type
    pub fn can_transition_to(&self, new_state: &str) -> bool {
        match self.board_type.as_str() {
            "campaign" => self.can_transition_campaign_state(new_state),
            "module" => self.can_transition_module_state(new_state),
            "session" => self.can_transition_session_state(new_state),
            _ => false,
        }
    }
    
    fn can_transition_campaign_state(&self, new_state: &str) -> bool {
        match (self.workflow_state.as_str(), new_state) {
            ("concept", "session_zero") => true,
            ("concept", "archived") => true, // Can abandon concepts
            ("session_zero", "integration") => true,
            ("session_zero", "concept") => true, // Can move back
            ("integration", "active") => true,
            ("integration", "session_zero") => true, // Can move back
            ("active", "concluding") => true,
            ("concluding", "completed") => true,
            _ => false,
        }
    }
    
    fn can_transition_module_state(&self, new_state: &str) -> bool {
        match (self.workflow_state.as_str(), new_state) {
            ("backlog", "planning") => true,
            ("planning", "development") => true,
            ("planning", "backlog") => true, // Can move back
            ("development", "ready") => true,
            ("development", "planning") => true, // Can move back
            ("ready", "active") => true,
            ("ready", "development") => true, // Can move back
            ("active", "completed") => true,
            _ => false,
        }
    }
    
    fn can_transition_session_state(&self, new_state: &str) -> bool {
        match (self.workflow_state.as_str(), new_state) {
            ("next_week", "prep_needed") => true,
            ("prep_needed", "in_prep") => true,
            ("prep_needed", "next_week") => true, // Can defer
            ("in_prep", "ready") => true,
            ("in_prep", "prep_needed") => true, // Can move back
            ("ready", "complete") => true,
            ("ready", "in_prep") => true, // Can move back for more prep
            _ => false,
        }
    }
}