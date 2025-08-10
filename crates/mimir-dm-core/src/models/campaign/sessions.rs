//! Session database models and operations

use crate::schema::sessions;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// Database model for sessions
#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize, Associations)]
#[diesel(table_name = sessions)]
#[diesel(belongs_to(crate::models::campaign::campaigns::Campaign))]
#[diesel(belongs_to(crate::models::campaign::modules::Module))]
pub struct Session {
    pub id: i32,
    pub campaign_id: i32,
    pub module_id: Option<i32>,
    pub session_number: i32,
    pub status: String,
    pub scheduled_date: Option<String>,
    pub prep_started_at: Option<String>,
    pub prep_completed_at: Option<String>,
    pub completed_at: Option<String>,
    pub created_at: String,
}

/// New session for insertion
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = sessions)]
pub struct NewSession {
    pub campaign_id: i32,
    pub module_id: Option<i32>,
    pub session_number: i32,
    pub status: String,
    pub scheduled_date: Option<String>,
}

/// Session update structure
#[derive(Debug, Clone, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = sessions)]
pub struct UpdateSession {
    pub status: Option<String>,
    pub scheduled_date: Option<Option<String>>,
    pub prep_started_at: Option<Option<String>>,
    pub prep_completed_at: Option<Option<String>>,
    pub completed_at: Option<Option<String>>,
}

impl Session {
    /// Check if session can transition to the given status
    pub fn can_transition_to(&self, new_status: &str) -> bool {
        match (self.status.as_str(), new_status) {
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
    
    /// Get prep duration in minutes
    pub fn prep_duration_minutes(&self) -> Option<i64> {
        match (&self.prep_started_at, &self.prep_completed_at) {
            (Some(_start), Some(_end)) => {
                // Parse ISO datetime strings and calculate duration
                // For now, return None - would need chrono for proper implementation
                None
            }
            _ => None,
        }
    }
}