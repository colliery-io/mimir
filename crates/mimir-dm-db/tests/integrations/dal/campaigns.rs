//! Integration tests for campaign DAL

use crate::common::TestDatabase;
use mimir_dm_db::dal::campaigns::CampaignRepository;
use mimir_dm_db::models::campaigns::NewCampaign;

#[test]
fn test_campaign_lifecycle() {
    let test_db = TestDatabase::file_based().unwrap();
    let mut conn = test_db.connection().unwrap();
    
    let mut repo = CampaignRepository::new(&mut conn);
    
    // Create a new campaign
    let new_campaign = NewCampaign {
        name: "The Imprisoned Corruption".to_string(),
        status: "planning".to_string(),
    };
    
    let campaign = repo.create(new_campaign).unwrap();
    assert_eq!(campaign.name, "The Imprisoned Corruption");
    assert_eq!(campaign.status, "planning");
    
    // Test status transitions
    let updated = repo.transition_status(campaign.id, "active").unwrap();
    assert_eq!(updated.status, "active");
    
    let updated = repo.transition_status(campaign.id, "completed").unwrap();
    assert_eq!(updated.status, "completed");
    
    let updated = repo.transition_status(campaign.id, "archived").unwrap();
    assert_eq!(updated.status, "archived");
}

#[test]
fn test_invalid_campaign_transitions() {
    let test_db = TestDatabase::file_based().unwrap();
    let mut conn = test_db.connection().unwrap();
    
    let mut repo = CampaignRepository::new(&mut conn);
    
    // Create a new campaign
    let new_campaign = NewCampaign {
        name: "Test Campaign".to_string(),
        status: "planning".to_string(),
    };
    
    let campaign = repo.create(new_campaign).unwrap();
    
    // Try invalid transition: planning -> completed
    let result = repo.transition_status(campaign.id, "completed");
    assert!(result.is_err());
    
    // Move to active first
    repo.transition_status(campaign.id, "active").unwrap();
    
    // Try invalid transition: active -> planning
    let result = repo.transition_status(campaign.id, "planning");
    assert!(result.is_err());
}

#[test]
fn test_list_active_campaigns() {
    let test_db = TestDatabase::file_based().unwrap();
    let mut conn = test_db.connection().unwrap();
    
    let mut repo = CampaignRepository::new(&mut conn);
    
    // Create multiple campaigns
    repo.create(NewCampaign {
        name: "Campaign 1".to_string(),
        status: "planning".to_string(),
    }).unwrap();
    
    let campaign2 = repo.create(NewCampaign {
        name: "Campaign 2".to_string(),
        status: "planning".to_string(),
    }).unwrap();
    
    repo.create(NewCampaign {
        name: "Campaign 3".to_string(),
        status: "archived".to_string(),
    }).unwrap();
    
    // Make campaign 2 active
    repo.transition_status(campaign2.id, "active").unwrap();
    
    // List active campaigns
    let active = repo.list_active().unwrap();
    assert_eq!(active.len(), 2); // planning and active
    assert!(active.iter().any(|c| c.name == "Campaign 1" && c.status == "planning"));
    assert!(active.iter().any(|c| c.name == "Campaign 2" && c.status == "active"));
}