//! Unit tests for bundle extraction and parsing

mod common;

use mimir_dm_import::{Bundle, ImportError};

#[tokio::test]
async fn test_bundle_extraction() {
    let bundle_file = common::create_test_bundle();
    let bundle = Bundle::from_archive(bundle_file.path()).await.unwrap();
    
    // Check manifest was parsed correctly
    assert_eq!(bundle.manifest.bundle_id, "test-bundle");
    assert_eq!(bundle.manifest.bundle_name, "Test Bundle");
    assert_eq!(bundle.manifest.rule_system, "test-system");
    assert_eq!(bundle.manifest.entity_counts.races, 2);
    assert_eq!(bundle.manifest.entity_counts.items, 1);
    
    // Check root directory was detected
    assert_eq!(bundle.root_dir, "test-bundle");
    
    // Check files were extracted
    assert!(bundle.has_file("manifest.json"));
    assert!(bundle.has_file("version.json"));
    assert!(bundle.has_file("sources.json"));
    assert!(bundle.has_file("races.json"));
    assert!(bundle.has_file("items.json"));
}

#[tokio::test]
async fn test_bundle_file_access() {
    let bundle_file = common::create_test_bundle();
    let bundle = Bundle::from_archive(bundle_file.path()).await.unwrap();
    
    // Test get_file
    let manifest_bytes = bundle.get_file("manifest.json").unwrap();
    assert!(manifest_bytes.len() > 0);
    
    // Test get_file_string
    let manifest_str = bundle.get_file_string("manifest.json").unwrap().unwrap();
    assert!(manifest_str.contains("test-bundle"));
    
    // Test parse_json_file
    let sources: serde_json::Value = bundle.parse_json_file("sources.json").unwrap();
    assert_eq!(sources["sources"][0]["id"], "TST");
}

#[tokio::test]
async fn test_missing_manifest() {
    let bundle_file = common::create_invalid_bundle();
    let result = Bundle::from_archive(bundle_file.path()).await;
    
    match result {
        Err(ImportError::MissingFile { filename }) => {
            assert_eq!(filename, "manifest.json");
        }
        _ => panic!("Expected MissingFile error for manifest.json"),
    }
}

#[tokio::test]
async fn test_bundle_not_found() {
    let result = Bundle::from_archive("/nonexistent/bundle.tar.gz").await;
    
    match result {
        Err(ImportError::BundleNotFound { .. }) => {
            // Expected
        }
        _ => panic!("Expected BundleNotFound error"),
    }
}

#[tokio::test]
async fn test_parse_entity_files() {
    let bundle_file = common::create_test_bundle();
    let bundle = Bundle::from_archive(bundle_file.path()).await.unwrap();
    
    // Parse races
    let races_data: serde_json::Value = bundle.parse_json_file("races.json").unwrap();
    let races = &races_data["races"];
    assert_eq!(races.as_array().unwrap().len(), 2);
    assert_eq!(races[0]["id"], "test-human");
    assert_eq!(races[1]["id"], "test-elf");
    
    // Parse items
    let items_data: serde_json::Value = bundle.parse_json_file("items.json").unwrap();
    let items = &items_data["items"];
    assert_eq!(items.as_array().unwrap().len(), 1);
    assert_eq!(items[0]["id"], "test-sword");
}

#[tokio::test]
async fn test_list_files() {
    let bundle_file = common::create_test_bundle();
    let bundle = Bundle::from_archive(bundle_file.path()).await.unwrap();
    
    let files = bundle.list_files();
    assert!(files.contains(&"manifest.json"));
    assert!(files.contains(&"version.json"));
    assert!(files.contains(&"sources.json"));
    assert!(files.contains(&"races.json"));
    assert!(files.contains(&"items.json"));
    assert_eq!(files.len(), 5);
}