//! Common test utilities

use std::io::Write;
use std::path::Path;
use tar::Builder;
use flate2::write::GzEncoder;
use flate2::Compression;
use tempfile::NamedTempFile;

/// Create a minimal test bundle with basic structure
pub fn create_test_bundle() -> NamedTempFile {
    let temp_file = NamedTempFile::new().unwrap();
    let gz = GzEncoder::new(&temp_file, Compression::default());
    let mut tar = Builder::new(gz);

    // Add manifest.json
    let manifest = r#"{
        "format_version": "1.0.0",
        "bundle_id": "test-bundle",
        "bundle_name": "Test Bundle",
        "bundle_version": "1.0.0",
        "rule_system": "test-system",
        "description": "A test bundle for unit tests",
        "sources_included": ["TST"],
        "entity_counts": {
            "sources": 1,
            "races": 2,
            "classes": 0,
            "items": 1,
            "spells": 0,
            "creatures": 0,
            "backgrounds": 0,
            "feats": 0
        }
    }"#;
    
    add_file_to_tar(&mut tar, "test-bundle/manifest.json", manifest.as_bytes());

    // Add version.json
    let version = r#"{
        "bundle_version": "1.0.0",
        "source_data_version": "test-v1.0.0",
        "parser_version": "1.0.0",
        "created_at": "2024-01-01T00:00:00Z"
    }"#;
    
    add_file_to_tar(&mut tar, "test-bundle/version.json", version.as_bytes());

    // Add sources.json
    let sources = r#"{
        "sources": [
            {
                "id": "TST",
                "full_name": "Test Source Book",
                "abbreviation": "TST",
                "authors": ["Test Author"],
                "published_date": "2024-01-01",
                "version": "1.0",
                "is_official": true,
                "book_type": "core"
            }
        ]
    }"#;
    
    add_file_to_tar(&mut tar, "test-bundle/sources.json", sources.as_bytes());

    // Add minimal races.json
    let races = r#"{
        "races": [
            {
                "id": "test-human",
                "name": "Test Human",
                "source": "TST",
                "page": 1,
                "race_type": "race",
                "parent_race_id": null,
                "size": "M",
                "speed": {"walk": 30},
                "ability_scores": {"str": 1, "dex": 1},
                "entries": ["A test race for testing."]
            },
            {
                "id": "test-elf",
                "name": "Test Elf",
                "source": "TST",
                "page": 2,
                "race_type": "race",
                "parent_race_id": null,
                "size": "M",
                "speed": {"walk": 30},
                "darkvision": 60,
                "entries": ["Another test race."]
            }
        ]
    }"#;
    
    add_file_to_tar(&mut tar, "test-bundle/races.json", races.as_bytes());

    // Add minimal items.json
    let items = r#"{
        "items": [
            {
                "id": "test-sword",
                "name": "Test Sword",
                "source": "TST",
                "page": 10,
                "type": "M",
                "weight_lb": 3.0,
                "value_cp": 1500,
                "damage": {"dice": "1d8", "type": "slashing"},
                "properties": ["versatile"],
                "rarity": "common"
            }
        ]
    }"#;
    
    add_file_to_tar(&mut tar, "test-bundle/items.json", items.as_bytes());

    tar.finish().unwrap();
    temp_file
}

/// Create an invalid test bundle (missing manifest)
pub fn create_invalid_bundle() -> NamedTempFile {
    let temp_file = NamedTempFile::new().unwrap();
    let gz = GzEncoder::new(&temp_file, Compression::default());
    let mut tar = Builder::new(gz);

    // Only add version.json, missing manifest.json
    let version = r#"{"bundle_version": "1.0.0"}"#;
    add_file_to_tar(&mut tar, "test-bundle/version.json", version.as_bytes());

    tar.finish().unwrap();
    temp_file
}

/// Helper to add a file to tar archive
fn add_file_to_tar<W: Write>(tar: &mut Builder<W>, path: &str, data: &[u8]) {
    let mut header = tar::Header::new_gnu();
    header.set_path(path).unwrap();
    header.set_size(data.len() as u64);
    header.set_mode(0o644);
    header.set_cksum();
    
    tar.append(&header, data).unwrap();
}

/// Create a test database URL for isolated testing
pub fn test_db_url() -> String {
    let temp_dir = tempfile::tempdir().unwrap();
    let db_path = temp_dir.path().join("test.db");
    format!("{}", db_path.display())
}

/// Initialize a test database with schema
pub async fn init_test_db(db_url: &str) {
    use mimir_dm_db::establish_connection;
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    
    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../../migrations");
    
    let mut conn = establish_connection(db_url).unwrap();
    conn.run_pending_migrations(MIGRATIONS).unwrap();
}