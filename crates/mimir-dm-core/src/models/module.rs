//! Simplified module entity focused on workflow management

use super::ids::{CampaignId, ModuleId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Minimal module entity for workflow tracking
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Module {
    /// Unique identifier
    pub id: ModuleId,
    
    /// Which campaign this belongs to
    pub campaign_id: CampaignId,
    
    /// Module name
    pub name: String,
    
    /// Sequential number within campaign
    pub module_number: u32,
    
    /// Expected number of sessions (for 60% completion trigger)
    pub expected_sessions: u32,
    
    /// Actual sessions run so far
    pub actual_sessions: u32,
    
    /// When this module was created
    pub created_at: DateTime<Utc>,
    
    /// When this module started running
    pub started_at: Option<DateTime<Utc>>,
    
    /// When this module was completed
    pub completed_at: Option<DateTime<Utc>>,
}

impl Module {
    /// Create a new module
    pub fn new(campaign_id: CampaignId, name: String, module_number: u32, expected_sessions: u32) -> Self {
        Self {
            id: ModuleId::new(uuid::Uuid::new_v4().as_u128() as i32),
            campaign_id,
            name,
            module_number,
            expected_sessions,
            actual_sessions: 0,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
        }
    }
    
    /// Calculate completion percentage
    pub fn completion_percentage(&self) -> f32 {
        if self.expected_sessions == 0 {
            return 0.0;
        }
        (self.actual_sessions as f32 / self.expected_sessions as f32) * 100.0
    }
    
    /// Check if module has reached 60% completion (trigger for next module planning)
    pub fn should_trigger_next_module(&self) -> bool {
        self.completion_percentage() >= 60.0
    }
    
    /// Mark module as started
    pub fn start(&mut self) {
        if self.started_at.is_none() {
            self.started_at = Some(Utc::now());
        }
    }
    
    /// Increment session count
    pub fn increment_sessions(&mut self) {
        self.actual_sessions += 1;
        
        // Auto-start if this is the first session
        if self.actual_sessions == 1 {
            self.start();
        }
    }
    
    /// Mark module as completed
    pub fn complete(&mut self) {
        self.completed_at = Some(Utc::now());
    }
    
    /// Check if module is currently active
    pub fn is_active(&self) -> bool {
        self.started_at.is_some() && self.completed_at.is_none()
    }
    
    /// Check if module is completed
    pub fn is_completed(&self) -> bool {
        self.completed_at.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_module_creation() {
        let campaign_id = CampaignId::new(1);
        let module = Module::new(
            campaign_id,
            "The Brittle Steel Mystery".to_string(),
            1,
            4
        );
        
        assert_eq!(module.name, "The Brittle Steel Mystery");
        assert_eq!(module.module_number, 1);
        assert_eq!(module.expected_sessions, 4);
        assert_eq!(module.actual_sessions, 0);
        assert!(!module.is_active());
        assert!(!module.is_completed());
    }
    
    #[test]
    fn test_completion_percentage() {
        let mut module = Module::new(
            CampaignId::new(1),
            "Test Module".to_string(),
            1,
            4
        );
        
        assert_eq!(module.completion_percentage(), 0.0);
        assert!(!module.should_trigger_next_module());
        
        module.increment_sessions();
        module.increment_sessions();
        assert_eq!(module.completion_percentage(), 50.0);
        assert!(!module.should_trigger_next_module());
        
        module.increment_sessions();
        assert_eq!(module.completion_percentage(), 75.0);
        assert!(module.should_trigger_next_module());
    }
    
    #[test]
    fn test_module_lifecycle() {
        let mut module = Module::new(
            CampaignId::new(1),
            "Test Module".to_string(),
            1,
            3
        );
        
        assert!(!module.is_active());
        assert!(module.started_at.is_none());
        
        module.increment_sessions();
        assert!(module.is_active());
        assert!(module.started_at.is_some());
        
        module.complete();
        assert!(!module.is_active());
        assert!(module.is_completed());
    }
}