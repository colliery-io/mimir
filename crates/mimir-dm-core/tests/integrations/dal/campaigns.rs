//! Integration tests for campaign DAL

use crate::common::TestDatabase;
use mimir_dm_db::dal::campaigns::CampaignRepository;
use mimir_dm_db::models::campaigns::NewCampaign;
use tempfile::TempDir;

#[test]
fn test_campaign_lifecycle() {
    let test_db = TestDatabase::file_based().unwrap();
    let mut conn = test_db.connection().unwrap();
    
    let mut repo = CampaignRepository::new(&mut conn);
    
    // Create a temporary directory for the campaign
    let temp_dir = TempDir::new().unwrap();
    
    // Create a new campaign
    let new_campaign = NewCampaign {
        name: "The Imprisoned Corruption".to_string(),
        status: "concept".to_string(),
        directory_path: temp_dir.path().to_string_lossy().to_string(),
    };
    
    let campaign = repo.create(new_campaign).unwrap();
    assert_eq!(campaign.name, "The Imprisoned Corruption");
    assert_eq!(campaign.status, "concept");
    
    // Test status transitions through workflow stages
    let updated = repo.transition_status(campaign.id, "session_zero").unwrap();
    assert_eq!(updated.status, "session_zero");
    
    let updated = repo.transition_status(campaign.id, "integration").unwrap();
    assert_eq!(updated.status, "integration");
    
    let updated = repo.transition_status(campaign.id, "active").unwrap();
    assert_eq!(updated.status, "active");
    
    let updated = repo.transition_status(campaign.id, "concluding").unwrap();
    assert_eq!(updated.status, "concluding");
}

#[test]
fn test_invalid_campaign_transitions() {
    let test_db = TestDatabase::file_based().unwrap();
    let mut conn = test_db.connection().unwrap();
    
    let mut repo = CampaignRepository::new(&mut conn);
    
    // Create a temporary directory for the campaign
    let temp_dir = TempDir::new().unwrap();
    
    // Create a new campaign
    let new_campaign = NewCampaign {
        name: "Test Campaign".to_string(),
        status: "concept".to_string(),
        directory_path: temp_dir.path().to_string_lossy().to_string(),
    };
    
    let campaign = repo.create(new_campaign).unwrap();
    
    // Try invalid transition: concept -> active (skipping stages)
    let result = repo.transition_status(campaign.id, "active");
    assert!(result.is_err());
    
    // Move through valid stages first
    repo.transition_status(campaign.id, "session_zero").unwrap();
    repo.transition_status(campaign.id, "integration").unwrap();
    repo.transition_status(campaign.id, "active").unwrap();
    
    // Try invalid transition: active -> concept (going backwards)
    let result = repo.transition_status(campaign.id, "concept");
    assert!(result.is_err());
}

#[test]
fn test_list_active_campaigns() {
    let test_db = TestDatabase::file_based().unwrap();
    let mut conn = test_db.connection().unwrap();
    
    let mut repo = CampaignRepository::new(&mut conn);
    
    // Create temporary directories for campaigns
    let temp_dir1 = TempDir::new().unwrap();
    let temp_dir2 = TempDir::new().unwrap();
    let temp_dir3 = TempDir::new().unwrap();
    
    // Create multiple campaigns
    repo.create(NewCampaign {
        name: "Campaign 1".to_string(),
        status: "concept".to_string(),
        directory_path: temp_dir1.path().to_string_lossy().to_string(),
    }).unwrap();
    
    let campaign2 = repo.create(NewCampaign {
        name: "Campaign 2".to_string(),
        status: "concept".to_string(),
        directory_path: temp_dir2.path().to_string_lossy().to_string(),
    }).unwrap();
    
    repo.create(NewCampaign {
        name: "Campaign 3".to_string(),
        status: "concluding".to_string(),
        directory_path: temp_dir3.path().to_string_lossy().to_string(),
    }).unwrap();
    
    // Make campaign 2 active through valid stages
    repo.transition_status(campaign2.id, "session_zero").unwrap();
    repo.transition_status(campaign2.id, "integration").unwrap();
    repo.transition_status(campaign2.id, "active").unwrap();
    
    // List active campaigns
    let active = repo.list_active().unwrap();
    assert_eq!(active.len(), 2); // concept and active (not concluding)
    assert!(active.iter().any(|c| c.name == "Campaign 1" && c.status == "concept"));
    assert!(active.iter().any(|c| c.name == "Campaign 2" && c.status == "active"));
}