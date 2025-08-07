//! Integration tests for the full import pipeline

mod common;

use mimir_dm_import::{BundleImporter, import_bundle};
use std::io::Write;
use mimir_dm_db::{
    dal::{
        rule_systems::RuleSystemRepository,
        sources::SourceRepository,
        races::RaceRepository,
        items::ItemRepository,
        traits::AsyncRepository,
    },
};

#[tokio::test]
async fn test_full_import_pipeline() {
    // Create test bundle and database
    let bundle_file = common::create_test_bundle();
    let db_url = common::test_db_url();
    common::init_test_db(&db_url).await;
    
    // Import the bundle
    let importer = BundleImporter::new(db_url.clone());
    let result = importer.import_bundle(bundle_file.path()).await;
    assert!(result.is_ok());
    
    // Verify rule system was imported
    let rule_system_repo = RuleSystemRepository::new(db_url.clone());
    let rule_systems = rule_system_repo.list().await.unwrap();
    assert_eq!(rule_systems.len(), 1);
    assert_eq!(rule_systems[0].id, "test-system");
    assert_eq!(rule_systems[0].name, "Test Bundle");
    
    // Verify source was imported
    let source_repo = SourceRepository::new(db_url.clone());
    let sources = source_repo.list().await.unwrap();
    assert_eq!(sources.len(), 1);
    assert_eq!(sources[0].id, "TST");
    assert_eq!(sources[0].full_name, "Test Source Book");
    
    // Verify races were imported
    let race_repo = RaceRepository::new(db_url.clone());
    let races = race_repo.list().await.unwrap();
    assert_eq!(races.len(), 2);
    
    let human = races.iter().find(|r| r.id == "test-human").unwrap();
    assert_eq!(human.name, "Test Human");
    assert_eq!(human.source_id, "TST");
    
    let elf = races.iter().find(|r| r.id == "test-elf").unwrap();
    assert_eq!(elf.name, "Test Elf");
    // Note: darkvision is stored in entries or traits, not as a separate field
    
    // Verify items were imported
    let item_repo = ItemRepository::new(db_url.clone());
    let items = item_repo.list().await.unwrap();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].id, "test-sword");
    assert_eq!(items[0].name, "Test Sword");
}

#[tokio::test]
async fn test_import_bundle_function() {
    // Create test bundle and database
    let bundle_file = common::create_test_bundle();
    let db_url = common::test_db_url();
    common::init_test_db(&db_url).await;
    
    // Use the convenience function
    let result = import_bundle(&db_url, bundle_file.path()).await;
    assert!(result.is_ok());
    
    // Verify data was imported
    let rule_system_repo = RuleSystemRepository::new(db_url.clone());
    let rule_systems = rule_system_repo.list().await.unwrap();
    assert_eq!(rule_systems.len(), 1);
}

#[tokio::test]
async fn test_duplicate_import_fails() {
    // Create test bundle and database
    let bundle_file = common::create_test_bundle();
    let db_url = common::test_db_url();
    common::init_test_db(&db_url).await;
    
    let importer = BundleImporter::new(db_url.clone());
    
    // First import should succeed
    let result1 = importer.import_bundle(bundle_file.path()).await;
    assert!(result1.is_ok());
    
    // Second import should fail due to duplicate rule system
    let result2 = importer.import_bundle(bundle_file.path()).await;
    assert!(result2.is_err());
    
    // Should be a database constraint violation
    match result2 {
        Err(mimir_dm_import::ImportError::Database(_)) => {
            // Expected
        }
        _ => panic!("Expected Database error for duplicate import"),
    }
}

#[tokio::test]
async fn test_partial_bundle_import() {
    // Create a test bundle with only some entity types
    let mut temp_file = tempfile::NamedTempFile::new().unwrap();
    let buffer = Vec::new();
    let gz = flate2::write::GzEncoder::new(buffer, flate2::Compression::default());
    let mut tar = tar::Builder::new(gz);
    
    // Add minimal files
    let manifest = r#"{
        "format_version": "1.0.0",
        "bundle_id": "partial-bundle",
        "bundle_name": "Partial Bundle",
        "bundle_version": "1.0.0",
        "rule_system": "partial-system",
        "description": "A partial bundle",
        "entity_counts": {
            "sources": 1,
            "races": 1,
            "classes": 0,
            "items": 0,
            "spells": 0,
            "creatures": 0,
            "backgrounds": 0,
            "feats": 0
        }
    }"#;
    
    add_file(&mut tar, "partial-bundle/manifest.json", manifest.as_bytes());
    add_file(&mut tar, "partial-bundle/version.json", r#"{"bundle_version": "1.0.0"}"#.as_bytes());
    
    // Add sources.json since race references it
    let sources = r#"{
        "sources": [{
            "id": "PRT",
            "full_name": "Partial Test",
            "abbreviation": "PRT",
            "authors": ["Test"],
            "published_date": "2024-01-01",
            "version": "1.0",
            "is_official": false,
            "book_type": "core"
        }]
    }"#;
    add_file(&mut tar, "partial-bundle/sources.json", sources.as_bytes());
    
    // Add races.json
    let races = r#"{
        "races": [{
            "id": "partial-race",
            "name": "Partial Race",
            "source": "PRT",
            "race_type": "race",
            "size": "M",
            "speed": {"walk": 30}
        }]
    }"#;
    add_file(&mut tar, "partial-bundle/races.json", races.as_bytes());
    
    // Finish the tar and write to file
    let gz = tar.into_inner().unwrap();
    let compressed_data = gz.finish().unwrap();
    temp_file.write_all(&compressed_data).unwrap();
    temp_file.flush().unwrap();
    
    // Import the partial bundle
    let db_url = common::test_db_url();
    common::init_test_db(&db_url).await;
    
    let importer = BundleImporter::new(db_url.clone());
    let result = importer.import_bundle(temp_file.path()).await;
    assert!(result.is_ok());
    
    // Verify only races were imported
    let race_repo = RaceRepository::new(db_url.clone());
    let races = race_repo.list().await.unwrap();
    assert_eq!(races.len(), 1);
    assert_eq!(races[0].id, "partial-race");
}

// Helper function to add files to tar
fn add_file<W: std::io::Write>(tar: &mut tar::Builder<W>, path: &str, data: &[u8]) {
    let mut header = tar::Header::new_gnu();
    header.set_path(path).unwrap();
    header.set_size(data.len() as u64);
    header.set_mode(0o644);
    header.set_cksum();
    tar.append(&header, data).unwrap();
}