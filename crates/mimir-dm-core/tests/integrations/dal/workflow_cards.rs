//! Integration tests for workflow card DAL

use crate::common::TestDatabase;
use mimir_dm_core::dal::campaign::campaigns::CampaignRepository;
use mimir_dm_core::dal::campaign::modules::ModuleRepository;
use mimir_dm_core::dal::campaign::sessions::SessionRepository;
use mimir_dm_core::dal::campaign::workflow_cards::WorkflowCardRepository;
use mimir_dm_core::models::campaign::campaigns::NewCampaign;
use mimir_dm_core::models::campaign::modules::NewModule;
use mimir_dm_core::models::campaign::sessions::NewSession;
use mimir_dm_core::models::campaign::workflow_cards::NewWorkflowCard;
use tempfile::TempDir;

#[test]
fn test_campaign_card_workflow() {
    let test_db = TestDatabase::file_based().unwrap();
    let mut conn = test_db.connection().unwrap();
    
    // Create a campaign
    let temp_dir = TempDir::new().unwrap();
    let mut campaign_repo = CampaignRepository::new(&mut conn);
    let campaign = campaign_repo.create(NewCampaign {
        name: "Test Campaign".to_string(),
        status: "concept".to_string(),
        directory_path: temp_dir.path().to_string_lossy().to_string(),
    }).unwrap();
    
    let mut card_repo = WorkflowCardRepository::new(&mut conn);
    
    // Create a campaign card
    let new_card = NewWorkflowCard {
        id: String::new(), // Will be auto-generated
        board_type: "campaign".to_string(),
        title: "The Imprisoned Corruption".to_string(),
        description: Some("An ancient evil threatens to escape".to_string()),
        workflow_state: "concept".to_string(),
        campaign_id: Some(campaign.id),
        module_id: None,
        session_id: None,
        priority: 1,
    };
    
    let card = card_repo.create(new_card).unwrap();
    assert_eq!(card.board_type, "campaign");
    assert_eq!(card.title, "The Imprisoned Corruption");
    assert_eq!(card.workflow_state, "concept");
    
    // Test campaign card state transitions
    let updated = card_repo.move_to_state(&card.id, "session_zero").unwrap();
    assert_eq!(updated.workflow_state, "session_zero");
    
    let updated = card_repo.move_to_state(&card.id, "integration").unwrap();
    assert_eq!(updated.workflow_state, "integration");
    
    let updated = card_repo.move_to_state(&card.id, "active").unwrap();
    assert_eq!(updated.workflow_state, "active");
    
    let updated = card_repo.move_to_state(&card.id, "concluding").unwrap();
    assert_eq!(updated.workflow_state, "concluding");
    
    // Test invalid transition
    let result = card_repo.move_to_state(&card.id, "concept");
    assert!(result.is_err());
}

#[test]
fn test_module_card_workflow() {
    let test_db = TestDatabase::file_based().unwrap();
    let mut conn = test_db.connection().unwrap();
    
    // Create campaign and module
    let temp_dir = TempDir::new().unwrap();
    let mut campaign_repo = CampaignRepository::new(&mut conn);
    let campaign = campaign_repo.create(NewCampaign {
        name: "Test Campaign".to_string(),
        status: "active".to_string(),
        directory_path: temp_dir.path().to_string_lossy().to_string(),
    }).unwrap();
    
    let mut module_repo = ModuleRepository::new(&mut conn);
    let module = module_repo.create(NewModule {
        campaign_id: campaign.id,
        name: "Test Module".to_string(),
        module_number: 1,
        status: "planning".to_string(),
        expected_sessions: 4,
    }).unwrap();
    
    let mut card_repo = WorkflowCardRepository::new(&mut conn);
    
    // Create a module card
    let card = card_repo.create(NewWorkflowCard {
        id: String::new(),
        board_type: "module".to_string(),
        title: "The Brittle Steel Mystery".to_string(),
        description: None,
        workflow_state: "planning".to_string(),
        campaign_id: None,
        module_id: Some(module.id),
        session_id: None,
        priority: 0,
    }).unwrap();
    
    // Test module card state transitions
    card_repo.move_to_state(&card.id, "development").unwrap();
    card_repo.move_to_state(&card.id, "ready").unwrap();
    card_repo.move_to_state(&card.id, "active").unwrap();
    
    let updated = card_repo.move_to_state(&card.id, "completed").unwrap();
    assert_eq!(updated.workflow_state, "completed");
}

#[test]
fn test_session_card_workflow() {
    let test_db = TestDatabase::file_based().unwrap();
    let mut conn = test_db.connection().unwrap();
    
    // Create campaign and session
    let temp_dir = TempDir::new().unwrap();
    let mut campaign_repo = CampaignRepository::new(&mut conn);
    let campaign = campaign_repo.create(NewCampaign {
        name: "Test Campaign".to_string(),
        status: "active".to_string(),
        directory_path: temp_dir.path().to_string_lossy().to_string(),
    }).unwrap();
    
    let mut session_repo = SessionRepository::new(&mut conn);
    let session = session_repo.create(NewSession {
        campaign_id: campaign.id,
        module_id: None,
        session_number: 1,
        status: "next_week".to_string(),
        scheduled_date: None,
    }).unwrap();
    
    let mut card_repo = WorkflowCardRepository::new(&mut conn);
    
    // Create a session card
    let card = card_repo.create(NewWorkflowCard {
        id: String::new(),
        board_type: "session".to_string(),
        title: "Session 1: The Gathering Storm".to_string(),
        description: None,
        workflow_state: "next_week".to_string(),
        campaign_id: None,
        module_id: None,
        session_id: Some(session.id),
        priority: 0,
    }).unwrap();
    
    // Test session card state transitions
    card_repo.move_to_state(&card.id, "prep_needed").unwrap();
    card_repo.move_to_state(&card.id, "in_prep").unwrap();
    card_repo.move_to_state(&card.id, "ready").unwrap();
    
    let updated = card_repo.move_to_state(&card.id, "complete").unwrap();
    assert_eq!(updated.workflow_state, "complete");
}

#[test]
fn test_card_tags() {
    let test_db = TestDatabase::file_based().unwrap();
    let mut conn = test_db.connection().unwrap();
    
    // Create a campaign first
    let temp_dir = TempDir::new().unwrap();
    let mut campaign_repo = CampaignRepository::new(&mut conn);
    let campaign = campaign_repo.create(NewCampaign {
        name: "Test Campaign".to_string(),
        status: "concept".to_string(),
        directory_path: temp_dir.path().to_string_lossy().to_string(),
    }).unwrap();
    
    let mut card_repo = WorkflowCardRepository::new(&mut conn);
    
    // Create a card with valid campaign reference
    let card = card_repo.create(NewWorkflowCard {
        id: String::new(),
        board_type: "campaign".to_string(),
        title: "Test Card".to_string(),
        description: None,
        workflow_state: "concept".to_string(),
        campaign_id: Some(campaign.id),
        module_id: None,
        session_id: None,
        priority: 0,
    }).unwrap();
    
    // Add tags
    card_repo.add_tag(&card.id, "urgent").unwrap();
    card_repo.add_tag(&card.id, "player-backstory").unwrap();
    card_repo.add_tag(&card.id, "main-quest").unwrap();
    
    // Get tags
    let tags = card_repo.get_tags(&card.id).unwrap();
    assert_eq!(tags.len(), 3);
    assert!(tags.contains(&"urgent".to_string()));
    assert!(tags.contains(&"player-backstory".to_string()));
    assert!(tags.contains(&"main-quest".to_string()));
    
    // Remove a tag
    card_repo.remove_tag(&card.id, "urgent").unwrap();
    let tags = card_repo.get_tags(&card.id).unwrap();
    assert_eq!(tags.len(), 2);
    assert!(!tags.contains(&"urgent".to_string()));
    
    // Find cards by tag
    let cards = card_repo.find_by_tag("main-quest").unwrap();
    assert_eq!(cards.len(), 1);
    assert_eq!(cards[0].id, card.id);
}

#[test]
fn test_list_cards_by_board() {
    let test_db = TestDatabase::file_based().unwrap();
    let mut conn = test_db.connection().unwrap();
    
    // Create campaign and module first
    let temp_dir = TempDir::new().unwrap();
    let mut campaign_repo = CampaignRepository::new(&mut conn);
    let campaign = campaign_repo.create(NewCampaign {
        name: "Test Campaign".to_string(),
        status: "active".to_string(),
        directory_path: temp_dir.path().to_string_lossy().to_string(),
    }).unwrap();
    
    let mut module_repo = ModuleRepository::new(&mut conn);
    let module = module_repo.create(NewModule {
        campaign_id: campaign.id,
        name: "Test Module".to_string(),
        module_number: 1,
        status: "planning".to_string(),
        expected_sessions: 4,
    }).unwrap();
    
    let mut card_repo = WorkflowCardRepository::new(&mut conn);
    
    // Create multiple cards on different boards
    card_repo.create(NewWorkflowCard {
        id: "1".to_string(),
        board_type: "campaign".to_string(),
        title: "Campaign Card 1".to_string(),
        description: None,
        workflow_state: "concept".to_string(),
        campaign_id: Some(campaign.id),
        module_id: None,
        session_id: None,
        priority: 1,
    }).unwrap();
    
    card_repo.create(NewWorkflowCard {
        id: "2".to_string(),
        board_type: "campaign".to_string(),
        title: "Campaign Card 2".to_string(),
        description: None,
        workflow_state: "active".to_string(),
        campaign_id: Some(campaign.id),
        module_id: None,
        session_id: None,
        priority: 0,
    }).unwrap();
    
    card_repo.create(NewWorkflowCard {
        id: "3".to_string(),
        board_type: "module".to_string(),
        title: "Module Card 1".to_string(),
        description: None,
        workflow_state: "planning".to_string(),
        campaign_id: None,
        module_id: Some(module.id),
        session_id: None,
        priority: 0,
    }).unwrap();
    
    // List campaign cards
    let campaign_cards = card_repo.list_by_board("campaign").unwrap();
    assert_eq!(campaign_cards.len(), 2);
    
    // List by board and state
    let active_campaign_cards = card_repo.list_by_board_and_state("campaign", "active").unwrap();
    assert_eq!(active_campaign_cards.len(), 1);
    assert_eq!(active_campaign_cards[0].title, "Campaign Card 2");
    
    // Check priority ordering (lower priority first)
    let all_campaign_cards = card_repo.list_by_board("campaign").unwrap();
    assert_eq!(all_campaign_cards[0].priority, 0); // Card 2
    assert_eq!(all_campaign_cards[1].priority, 1); // Card 1
}