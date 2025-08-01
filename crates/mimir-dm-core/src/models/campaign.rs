//! Simplified campaign entity focused on workflow management

use super::ids::CampaignId;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// Minimal campaign entity for workflow tracking
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Campaign {
    /// Unique identifier
    pub id: CampaignId,
    
    /// Campaign name
    pub name: String,
    
    /// Overall lifecycle status
    pub status: CampaignStatus,
    
    /// When the campaign was created
    pub created_at: DateTime<Utc>,
    
    /// Session Zero date (for workflow triggers)
    pub session_zero_date: Option<NaiveDate>,
    
    /// First actual play session date (for workflow triggers)
    pub first_session_date: Option<NaiveDate>,
    
    /// When the campaign was last active
    pub last_activity_at: DateTime<Utc>,
}

/// Campaign lifecycle status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CampaignStatus {
    /// Being planned
    Planning,
    /// Actively running
    Active,
    /// Finished
    Completed,
    /// Archived/Abandoned
    Archived,
}

impl Campaign {
    /// Create a new campaign
    pub fn new(name: String) -> Self {
        let now = Utc::now();
        Self {
            id: CampaignId::new(uuid::Uuid::new_v4().as_u128() as i32),
            name,
            status: CampaignStatus::Planning,
            created_at: now,
            session_zero_date: None,
            first_session_date: None,
            last_activity_at: now,
        }
    }
    
    /// Update the last activity timestamp
    pub fn touch(&mut self) {
        self.last_activity_at = Utc::now();
    }
    
    /// Check if campaign has had Session Zero
    pub fn has_session_zero(&self) -> bool {
        self.session_zero_date.is_some()
    }
    
    /// Check if campaign has started actual play
    pub fn has_started(&self) -> bool {
        self.first_session_date.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_campaign_creation() {
        let campaign = Campaign::new("The Imprisoned Corruption".to_string());
        
        assert_eq!(campaign.name, "The Imprisoned Corruption");
        assert_eq!(campaign.status, CampaignStatus::Planning);
        assert!(!campaign.has_session_zero());
        assert!(!campaign.has_started());
    }
    
    #[test]
    fn test_campaign_activity_tracking() {
        let mut campaign = Campaign::new("Test Campaign".to_string());
        let original_time = campaign.last_activity_at;
        
        std::thread::sleep(std::time::Duration::from_millis(10));
        campaign.touch();
        
        assert!(campaign.last_activity_at > original_time);
    }
}