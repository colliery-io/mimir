//! Integration tests for session DAL

use crate::common::TestDatabase;
use mimir_dm_core::dal::campaign::campaigns::CampaignRepository;
use mimir_dm_core::dal::campaign::sessions::SessionRepository;
use mimir_dm_core::models::campaign::campaigns::NewCampaign;
use mimir_dm_core::models::campaign::sessions::NewSession;
use chrono::{Utc, Duration};
use tempfile::TempDir;

#[test]
fn test_session_lifecycle() {
    let test_db = TestDatabase::file_based().unwrap();
    let mut conn = test_db.connection().unwrap();
    
    // Create a campaign first
    let temp_dir = TempDir::new().unwrap();
    let mut campaign_repo = CampaignRepository::new(&mut conn);
    let campaign = campaign_repo.create(NewCampaign {
        name: "Test Campaign".to_string(),
        status: "active".to_string(),
        directory_path: temp_dir.path().to_string_lossy().to_string(),
    }).unwrap();
    
    let mut session_repo = SessionRepository::new(&mut conn);
    
    // Create a new session
    let new_session = NewSession {
        campaign_id: campaign.id,
        module_id: None,
        session_number: 1,
        status: "next_week".to_string(),
        scheduled_date: Some((Utc::now() + Duration::days(7)).date_naive().to_string()),
    };
    
    let session = session_repo.create(new_session).unwrap();
    assert_eq!(session.session_number, 1);
    assert_eq!(session.status, "next_week");
    assert!(session.scheduled_date.is_some());
    
    // Test status transitions
    let updated = session_repo.transition_status(session.id, "prep_needed").unwrap();
    assert_eq!(updated.status, "prep_needed");
    
    let updated = session_repo.transition_status(session.id, "in_prep").unwrap();
    assert_eq!(updated.status, "in_prep");
    assert!(updated.prep_started_at.is_some());
    
    let updated = session_repo.transition_status(session.id, "ready").unwrap();
    assert_eq!(updated.status, "ready");
    assert!(updated.prep_completed_at.is_some());
    
    let updated = session_repo.transition_status(session.id, "complete").unwrap();
    assert_eq!(updated.status, "complete");
    assert!(updated.completed_at.is_some());
}

#[test]
fn test_invalid_session_transitions() {
    let test_db = TestDatabase::file_based().unwrap();
    let mut conn = test_db.connection().unwrap();
    
    // Create a campaign
    let temp_dir = TempDir::new().unwrap();
    let mut campaign_repo = CampaignRepository::new(&mut conn);
    let campaign = campaign_repo.create(NewCampaign {
        name: "Test Campaign".to_string(),
        status: "active".to_string(),
        directory_path: temp_dir.path().to_string_lossy().to_string(),
    }).unwrap();
    
    let mut session_repo = SessionRepository::new(&mut conn);
    
    // Create a session
    let session = session_repo.create(NewSession {
        campaign_id: campaign.id,
        module_id: None,
        session_number: 1,
        status: "next_week".to_string(),
        scheduled_date: None,
    }).unwrap();
    
    // Try invalid transition: next_week -> ready
    let result = session_repo.transition_status(session.id, "ready");
    assert!(result.is_err());
    
    // Try invalid transition: next_week -> complete
    let result = session_repo.transition_status(session.id, "complete");
    assert!(result.is_err());
}

#[test]
fn test_find_sessions_needing_prep() {
    let test_db = TestDatabase::file_based().unwrap();
    let mut conn = test_db.connection().unwrap();
    
    // Create a campaign
    let temp_dir = TempDir::new().unwrap();
    let mut campaign_repo = CampaignRepository::new(&mut conn);
    let campaign = campaign_repo.create(NewCampaign {
        name: "Test Campaign".to_string(),
        status: "active".to_string(),
        directory_path: temp_dir.path().to_string_lossy().to_string(),
    }).unwrap();
    
    let mut session_repo = SessionRepository::new(&mut conn);
    
    // Create sessions with different scheduled dates
    session_repo.create(NewSession {
        campaign_id: campaign.id,
        module_id: None,
        session_number: 1,
        status: "next_week".to_string(),
        scheduled_date: Some((Utc::now() + Duration::days(2)).date_naive().to_string()),
    }).unwrap();
    
    session_repo.create(NewSession {
        campaign_id: campaign.id,
        module_id: None,
        session_number: 2,
        status: "next_week".to_string(),
        scheduled_date: Some((Utc::now() + Duration::days(4)).date_naive().to_string()),
    }).unwrap();
    
    session_repo.create(NewSession {
        campaign_id: campaign.id,
        module_id: None,
        session_number: 3,
        status: "next_week".to_string(),
        scheduled_date: Some((Utc::now() + Duration::days(10)).date_naive().to_string()),
    }).unwrap();
    
    // Already in prep
    session_repo.create(NewSession {
        campaign_id: campaign.id,
        module_id: None,
        session_number: 4,
        status: "in_prep".to_string(),
        scheduled_date: Some((Utc::now() + Duration::days(1)).date_naive().to_string()),
    }).unwrap();
    
    // Find sessions needing prep (T-3)
    let sessions = session_repo.find_sessions_needing_prep(campaign.id, 3).unwrap();
    assert_eq!(sessions.len(), 1); // Only session 1 is within 3 days and status "next_week"
    assert_eq!(sessions[0].session_number, 1);
    
    // Find sessions needing prep (T-5)
    let sessions = session_repo.find_sessions_needing_prep(campaign.id, 5).unwrap();
    assert_eq!(sessions.len(), 2); // Sessions 1 and 2
}

#[test]
fn test_session_numbering() {
    let test_db = TestDatabase::file_based().unwrap();
    let mut conn = test_db.connection().unwrap();
    
    // Create a campaign
    let temp_dir = TempDir::new().unwrap();
    let mut campaign_repo = CampaignRepository::new(&mut conn);
    let campaign = campaign_repo.create(NewCampaign {
        name: "Test Campaign".to_string(),
        status: "active".to_string(),
        directory_path: temp_dir.path().to_string_lossy().to_string(),
    }).unwrap();
    
    let mut session_repo = SessionRepository::new(&mut conn);
    
    // Get next session number for empty campaign
    let next_num = session_repo.get_next_session_number(campaign.id).unwrap();
    assert_eq!(next_num, 1);
    
    // Create some sessions
    session_repo.create(NewSession {
        campaign_id: campaign.id,
        module_id: None,
        session_number: 1,
        status: "complete".to_string(),
        scheduled_date: None,
    }).unwrap();
    
    session_repo.create(NewSession {
        campaign_id: campaign.id,
        module_id: None,
        session_number: 2,
        status: "complete".to_string(),
        scheduled_date: None,
    }).unwrap();
    
    // Get next session number
    let next_num = session_repo.get_next_session_number(campaign.id).unwrap();
    assert_eq!(next_num, 3);
}