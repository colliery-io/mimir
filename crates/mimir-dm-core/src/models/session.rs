//! Simplified session entity focused on workflow management

use super::ids::{CampaignId, ModuleId, SessionId};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// Minimal session entity for workflow tracking
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Session {
    /// Unique identifier
    pub id: SessionId,
    
    /// Which campaign this belongs to
    pub campaign_id: CampaignId,
    
    /// Which module this belongs to (optional for non-module sessions)
    pub module_id: Option<ModuleId>,
    
    /// Sequential session number within campaign
    pub session_number: u32,
    
    /// When this session is scheduled (T-0)
    pub scheduled_date: Option<NaiveDate>,
    
    /// When prep was started
    pub prep_started_at: Option<DateTime<Utc>>,
    
    /// When prep was completed
    pub prep_completed_at: Option<DateTime<Utc>>,
    
    /// When session was actually run
    pub completed_at: Option<DateTime<Utc>>,
    
    /// When this session entity was created
    pub created_at: DateTime<Utc>,
}

impl Session {
    /// Create a new session
    pub fn new(campaign_id: CampaignId, session_number: u32) -> Self {
        Self {
            id: SessionId::new(uuid::Uuid::new_v4().as_u128() as i32),
            campaign_id,
            module_id: None,
            session_number,
            scheduled_date: None,
            prep_started_at: None,
            prep_completed_at: None,
            completed_at: None,
            created_at: Utc::now(),
        }
    }
    
    /// Create a new session for a module
    pub fn new_for_module(campaign_id: CampaignId, module_id: ModuleId, session_number: u32) -> Self {
        let mut session = Self::new(campaign_id, session_number);
        session.module_id = Some(module_id);
        session
    }
    
    /// Start session prep
    pub fn start_prep(&mut self) {
        if self.prep_started_at.is_none() {
            self.prep_started_at = Some(Utc::now());
        }
    }
    
    /// Complete session prep
    pub fn complete_prep(&mut self) {
        self.prep_completed_at = Some(Utc::now());
        
        // Auto-start prep if not already started
        if self.prep_started_at.is_none() {
            self.prep_started_at = Some(Utc::now());
        }
    }
    
    /// Mark session as run
    pub fn complete(&mut self) {
        self.completed_at = Some(Utc::now());
    }
    
    /// Check if prep has been started
    pub fn is_prep_started(&self) -> bool {
        self.prep_started_at.is_some()
    }
    
    /// Check if prep has been completed
    pub fn is_prep_complete(&self) -> bool {
        self.prep_completed_at.is_some()
    }
    
    /// Check if session has been run
    pub fn is_complete(&self) -> bool {
        self.completed_at.is_some()
    }
    
    /// Get prep duration in minutes
    pub fn prep_duration_minutes(&self) -> Option<i64> {
        match (self.prep_started_at, self.prep_completed_at) {
            (Some(start), Some(end)) => {
                let duration = end.signed_duration_since(start);
                Some(duration.num_minutes())
            }
            _ => None,
        }
    }
    
    /// Check if this session is scheduled
    pub fn is_scheduled(&self) -> bool {
        self.scheduled_date.is_some()
    }
    
    /// Get days until session (negative if in past)
    pub fn days_until_session(&self) -> Option<i64> {
        self.scheduled_date.map(|date| {
            let today = chrono::Local::now().date_naive();
            date.signed_duration_since(today).num_days()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_session_creation() {
        let campaign_id = CampaignId::new(1);
        let session = Session::new(campaign_id, 1);
        
        assert_eq!(session.session_number, 1);
        assert!(!session.is_prep_started());
        assert!(!session.is_prep_complete());
        assert!(!session.is_complete());
        assert!(!session.is_scheduled());
    }
    
    #[test]
    fn test_session_workflow() {
        let mut session = Session::new(CampaignId::new(1), 1);
        
        // Start prep
        session.start_prep();
        assert!(session.is_prep_started());
        assert!(!session.is_prep_complete());
        
        // Complete prep
        session.complete_prep();
        assert!(session.is_prep_complete());
        
        // Run session
        session.complete();
        assert!(session.is_complete());
    }
    
    #[test]
    fn test_prep_duration() {
        let mut session = Session::new(CampaignId::new(1), 1);
        
        assert_eq!(session.prep_duration_minutes(), None);
        
        let start = Utc::now();
        session.prep_started_at = Some(start);
        session.prep_completed_at = Some(start + chrono::Duration::minutes(45));
        
        assert_eq!(session.prep_duration_minutes(), Some(45));
    }
    
    #[test]
    fn test_session_scheduling() {
        let mut session = Session::new(CampaignId::new(1), 1);
        
        let in_three_days = chrono::Local::now().date_naive() + chrono::Duration::days(3);
        session.scheduled_date = Some(in_three_days);
        
        assert!(session.is_scheduled());
        assert_eq!(session.days_until_session(), Some(3));
    }
}