//! Integration test using real bundle file

use mimir_dm_import::{BundleImporter, Bundle};
use mimir_dm_db::{
    dal::{
        rule_systems::RuleSystemRepository,
        sources::SourceRepository,
        races::RaceRepository,
        items::ItemRepository,
        classes::ClassRepository,
        spells::SpellRepository,
        creatures::CreatureRepository,
        backgrounds::BackgroundRepository,
        feats::FeatRepository,
        traits::AsyncRepository,
    },
};
use std::path::Path;

// Path to the real test bundle
const TEST_BUNDLE_PATH: &str = "/Users/dstorey/Desktop/colliery/mimir/data/rule_sets/dnd5e-2014-core-v1.210.46.tar.gz";

#[tokio::test]
async fn test_real_bundle_extraction() {
    // Skip test if bundle doesn't exist
    if !Path::new(TEST_BUNDLE_PATH).exists() {
        eprintln!("Skipping test: bundle file not found at {}", TEST_BUNDLE_PATH);
        return;
    }

    let bundle = Bundle::from_archive(TEST_BUNDLE_PATH).await.unwrap();
    
    // Verify manifest
    assert_eq!(bundle.manifest.bundle_id, "dnd5e-2014-core");
    assert_eq!(bundle.manifest.bundle_name, "D&D 5e 2014 Core Ruleset");
    assert_eq!(bundle.manifest.rule_system, "dnd5e-2014");
    
    // Verify entity counts from the core bundle
    assert_eq!(bundle.manifest.entity_counts.sources, 3); // PHB, MM, DMG
    assert_eq!(bundle.manifest.entity_counts.races, 38);
    assert_eq!(bundle.manifest.entity_counts.classes, 54);
    assert_eq!(bundle.manifest.entity_counts.items, 778);
    assert_eq!(bundle.manifest.entity_counts.spells, 361);
    assert_eq!(bundle.manifest.entity_counts.creatures, 458);
    assert_eq!(bundle.manifest.entity_counts.backgrounds, 20);
    assert_eq!(bundle.manifest.entity_counts.feats, 42);
    
    // Verify required files exist
    assert!(bundle.has_file("manifest.json"));
    assert!(bundle.has_file("version.json"));
    assert!(bundle.has_file("sources.json"));
    assert!(bundle.has_file("races.json"));
    assert!(bundle.has_file("classes.json"));
    assert!(bundle.has_file("items.json"));
    assert!(bundle.has_file("spells.json"));
    assert!(bundle.has_file("creatures.json"));
    assert!(bundle.has_file("backgrounds.json"));
    assert!(bundle.has_file("feats.json"));
}

#[tokio::test]
async fn test_real_bundle_import() {
    // Skip test if bundle doesn't exist
    if !Path::new(TEST_BUNDLE_PATH).exists() {
        eprintln!("Skipping test: bundle file not found at {}", TEST_BUNDLE_PATH);
        return;
    }

    // Create test database with migrations already applied
    let temp_file = tempfile::NamedTempFile::new().unwrap();
    let db_url = temp_file.path().to_string_lossy().to_string();
    
    // Set up the database using mimir-dm-db test utilities
    {
        use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
        
        const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../mimir-dm-db/migrations");
        
        let mut conn = mimir_dm_db::establish_connection(&db_url).unwrap();
        conn.run_pending_migrations(MIGRATIONS).unwrap();
    }
    
    // Import the bundle
    let importer = BundleImporter::new(db_url.clone());
    let result = importer.import_bundle(TEST_BUNDLE_PATH).await;
    assert!(result.is_ok(), "Import failed: {:?}", result);
    
    // Verify rule system
    let rule_system_repo = RuleSystemRepository::new(db_url.clone());
    let rule_systems = rule_system_repo.list().await.unwrap();
    assert_eq!(rule_systems.len(), 1);
    assert_eq!(rule_systems[0].id, "dnd5e-2014");
    assert_eq!(rule_systems[0].name, "D&D 5e 2014 Core Ruleset");
    
    // Verify sources (PHB, MM, DMG)
    let source_repo = SourceRepository::new(db_url.clone());
    let sources = source_repo.list().await.unwrap();
    assert_eq!(sources.len(), 3);
    
    let source_ids: Vec<&str> = sources.iter().map(|s| s.id.as_str()).collect();
    assert!(source_ids.contains(&"PHB"));
    assert!(source_ids.contains(&"MM"));
    assert!(source_ids.contains(&"DMG"));
    
    // Verify races
    let race_repo = RaceRepository::new(db_url.clone());
    let races = race_repo.list().await.unwrap();
    assert_eq!(races.len(), 38);
    
    // Check for some specific races
    let race_names: Vec<&str> = races.iter().map(|r| r.name.as_str()).collect();
    assert!(race_names.contains(&"Human"));
    assert!(race_names.contains(&"Elf"));
    assert!(race_names.contains(&"Dwarf"));
    assert!(race_names.contains(&"Halfling"));
    
    // Verify classes
    let class_repo = ClassRepository::new(db_url.clone());
    let classes = class_repo.list().await.unwrap();
    assert_eq!(classes.len(), 54);
    
    // Check for base classes
    let class_names: Vec<&str> = classes.iter().map(|c| c.name.as_str()).collect();
    assert!(class_names.contains(&"Fighter"));
    assert!(class_names.contains(&"Wizard"));
    assert!(class_names.contains(&"Cleric"));
    assert!(class_names.contains(&"Rogue"));
    
    // Verify items
    let item_repo = ItemRepository::new(db_url.clone());
    let items = item_repo.list().await.unwrap();
    assert_eq!(items.len(), 778);
    
    // Verify spells
    let spell_repo = SpellRepository::new(db_url.clone());
    let spells = spell_repo.list().await.unwrap();
    assert_eq!(spells.len(), 361);
    
    // Check for some iconic spells
    let spell_names: Vec<&str> = spells.iter().map(|s| s.name.as_str()).collect();
    assert!(spell_names.contains(&"Fireball"));
    assert!(spell_names.contains(&"Magic Missile"));
    assert!(spell_names.contains(&"Cure Wounds"));
    
    // Verify creatures
    let creature_repo = CreatureRepository::new(db_url.clone());
    let creatures = creature_repo.list().await.unwrap();
    assert_eq!(creatures.len(), 458);
    
    // Verify backgrounds
    let background_repo = BackgroundRepository::new(db_url.clone());
    let backgrounds = background_repo.list().await.unwrap();
    assert_eq!(backgrounds.len(), 20);
    
    // Verify feats
    let feat_repo = FeatRepository::new(db_url.clone());
    let feats = feat_repo.list().await.unwrap();
    assert_eq!(feats.len(), 42);
}

#[tokio::test]
async fn test_specific_entities() {
    // Skip test if bundle doesn't exist
    if !Path::new(TEST_BUNDLE_PATH).exists() {
        eprintln!("Skipping test: bundle file not found at {}", TEST_BUNDLE_PATH);
        return;
    }

    // Extract bundle to check specific entities
    let bundle = Bundle::from_archive(TEST_BUNDLE_PATH).await.unwrap();
    
    // Check a specific spell (Fireball)
    let spells_data: serde_json::Value = bundle.parse_json_file("spells.json").unwrap();
    let spells = spells_data["spells"].as_array().unwrap();
    
    let fireball = spells.iter()
        .find(|s| s["name"] == "Fireball")
        .expect("Fireball spell not found");
    
    assert_eq!(fireball["id"], "fireball");
    assert_eq!(fireball["level"], 3);
    assert_eq!(fireball["school"], "V"); // Evocation
    
    // Classes might be in different formats depending on the data version
    if let Some(classes) = fireball.get("classes") {
        // Check if it's an array of strings
        if let Some(classes_array) = classes.as_array() {
            assert!(classes_array.iter().any(|c| c.as_str() == Some("sorcerer") || 
                                                  (c.is_object() && c["name"] == "sorcerer")));
            assert!(classes_array.iter().any(|c| c.as_str() == Some("wizard") || 
                                                  (c.is_object() && c["name"] == "wizard")));
        }
        // Or it might be an object with fromClassList
        else if let Some(from_class_list) = classes.get("fromClassList") {
            if let Some(class_list) = from_class_list.as_array() {
                assert!(class_list.iter().any(|c| c.as_str() == Some("sorcerer") || 
                                                      (c.is_object() && c["name"] == "Sorcerer")));
                assert!(class_list.iter().any(|c| c.as_str() == Some("wizard") || 
                                                      (c.is_object() && c["name"] == "Wizard")));
            }
        }
    }
    
    // Check a specific creature (Goblin)
    let creatures_data: serde_json::Value = bundle.parse_json_file("creatures.json").unwrap();
    let creatures = creatures_data["creatures"].as_array().unwrap();
    
    let goblin = creatures.iter()
        .find(|c| c["name"] == "Goblin")
        .expect("Goblin creature not found");
    
    assert_eq!(goblin["id"], "goblin");
    
    // Size might be a string or an array
    if let Some(size_str) = goblin["size"].as_str() {
        assert_eq!(size_str, "S");
    } else if let Some(size_arr) = goblin["size"].as_array() {
        assert!(size_arr.contains(&serde_json::Value::String("S".to_string())));
    }
    
    // Challenge rating might be a string or object
    if let Some(cr) = goblin.get("challenge_rating").and_then(|v| v.as_str()) {
        assert_eq!(cr, "1/4");
    } else if let Some(cr) = goblin.get("cr").and_then(|v| v.as_str()) {
        assert_eq!(cr, "1/4");
    }
    
    // Traits and actions might not always be present or might have different names
    if let Some(traits) = goblin.get("traits").and_then(|v| v.as_array()) {
        assert!(traits.len() > 0);
    } else if let Some(trait_val) = goblin.get("trait").and_then(|v| v.as_array()) {
        assert!(trait_val.len() > 0);
    }
    
    if let Some(actions) = goblin.get("actions").and_then(|v| v.as_array()) {
        assert!(actions.len() > 0);
    } else if let Some(action) = goblin.get("action").and_then(|v| v.as_array()) {
        assert!(action.len() > 0);
    }
}