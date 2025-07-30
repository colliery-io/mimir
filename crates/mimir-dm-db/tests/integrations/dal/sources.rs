//! Integration tests for sources DAL

use crate::common::{with_test_db, TestDatabase};
use mimir_dm_db::*;
use mimir_dm_db::dal::sources::SourceRepository;
use mimir_dm_db::dal::traits::{AsyncRepository, Repository};
use serde::{Deserialize, Serialize};

// Test fixtures
mod fixtures {
    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub struct SourceMetadata {
        pub isbn: Option<String>,
        pub pages: Option<u32>,
        pub language: String,
    }

    pub fn dnd5e_2014_phb() -> Source {
        Source::new(
            "dnd5e-2014-phb".to_string(),
            "dnd5e-2014".to_string(),
            "Player's Handbook".to_string(),
        )
        .with_abbreviation("PHB".to_string())
        .with_publisher("Wizards of the Coast".to_string())
        .with_publish_date("2014-08-19".to_string())
        .with_official(true)
        .with_srd(false)
    }

    pub fn dnd5e_2014_dmg() -> Source {
        Source::new(
            "dnd5e-2014-dmg".to_string(),
            "dnd5e-2014".to_string(),
            "Dungeon Master's Guide".to_string(),
        )
        .with_abbreviation("DMG".to_string())
        .with_publisher("Wizards of the Coast".to_string())
        .with_publish_date("2014-12-09".to_string())
        .with_official(true)
        .with_srd(false)
    }

    pub fn dnd5e_2014_mm() -> Source {
        Source::new(
            "dnd5e-2014-mm".to_string(),
            "dnd5e-2014".to_string(),
            "Monster Manual".to_string(),
        )
        .with_abbreviation("MM".to_string())
        .with_publisher("Wizards of the Coast".to_string())
        .with_publish_date("2014-09-30".to_string())
        .with_official(true)
        .with_srd(false)
    }

    pub fn dnd5e_2014_srd() -> Source {
        Source::new(
            "dnd5e-2014-srd".to_string(),
            "dnd5e-2014".to_string(),
            "System Reference Document 5.1".to_string(),
        )
        .with_abbreviation("SRD".to_string())
        .with_publisher("Wizards of the Coast".to_string())
        .with_publish_date("2016-01-12".to_string())
        .with_official(true)
        .with_srd(true)
    }

    pub fn third_party_source() -> Source {
        Source::new(
            "third-party-adventure".to_string(),
            "dnd5e-2014".to_string(),
            "Adventures in the Forgotten Kingdom".to_string(),
        )
        .with_abbreviation("AFK".to_string())
        .with_publisher("Third Party Games".to_string())
        .with_publish_date("2023-06-15".to_string())
        .with_official(false)
        .with_srd(false)
    }

    pub fn create_rule_system() -> RuleSystem {
        RuleSystem::new(
            "dnd5e-2014".to_string(),
            "D&D 5th Edition (2014)".to_string(),
        )
    }
}

// CRUD operation tests
mod crud_operations {
    use super::*;

    #[test]
    fn test_create_source() -> Result<()> {
        with_test_db(|conn| {
            // First create the rule system
            conn.create(fixtures::create_rule_system())?;
            
            let source = fixtures::dnd5e_2014_phb();
            let created = conn.create(source)?;
            
            assert_eq!(created.id, "dnd5e-2014-phb");
            assert_eq!(created.name, "Player's Handbook");
            assert_eq!(created.abbreviation, Some("PHB".to_string()));
            assert_eq!(created.publisher, Some("Wizards of the Coast".to_string()));
            assert_eq!(created.publish_date, Some("2014-08-19".to_string()));
            assert!(created.is_official);
            assert!(!created.is_srd);
            
            Ok(())
        })
    }

    #[test]
    fn test_find_by_id() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            
            let source = fixtures::dnd5e_2014_dmg();
            conn.create(source)?;
            
            let found: Option<Source> = conn.find_by_id("dnd5e-2014-dmg")?;
            assert!(found.is_some());
            assert_eq!(found.unwrap().name, "Dungeon Master's Guide");
            
            let not_found: Option<Source> = conn.find_by_id("nonexistent")?;
            assert!(not_found.is_none());
            
            Ok(())
        })
    }

    #[test]
    fn test_update_source() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            
            let source = fixtures::dnd5e_2014_mm();
            conn.create(source)?;
            
            let mut updated = Source::new(
                "dnd5e-2014-mm".to_string(),
                "dnd5e-2014".to_string(),
                "Monster Manual (Revised)".to_string(),
            );
            updated.is_official = false;
            
            let result = conn.update("dnd5e-2014-mm", updated)?;
            assert_eq!(result.name, "Monster Manual (Revised)");
            assert!(!result.is_official);
            
            Ok(())
        })
    }

    #[test]
    fn test_delete_source() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            
            let source = Source::new(
                "test-source".to_string(),
                "dnd5e-2014".to_string(),
                "Test Source".to_string(),
            );
            conn.create(source)?;
            
            Repository::<Source>::delete(conn, "test-source")?;
            
            let found: Option<Source> = conn.find_by_id("test-source")?;
            assert!(found.is_none());
            
            // Try to delete non-existent
            let result = Repository::<Source>::delete(conn, "nonexistent");
            assert!(matches!(result, Err(DbError::NotFound { .. })));
            
            Ok(())
        })
    }

    #[test]
    fn test_list_sources() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            
            conn.create(fixtures::dnd5e_2014_phb())?;
            conn.create(fixtures::dnd5e_2014_dmg())?;
            conn.create(fixtures::dnd5e_2014_mm())?;
            
            let list: Vec<Source> = conn.list()?;
            assert_eq!(list.len(), 3);
            
            Ok(())
        })
    }
}

// Async repository tests
mod async_operations {
    use super::*;

    #[tokio::test]
    async fn test_async_crud_lifecycle() -> Result<()> {
        // Create an isolated test database
        let test_db = TestDatabase::file_based()?;
        
        // First create the rule system
        let rule_repo = dal::rule_systems::RuleSystemRepository::new(test_db.url.clone());
        rule_repo.create(fixtures::create_rule_system()).await?;
        
        let repo = SourceRepository::new(test_db.url.clone());
        
        // Create
        let source = fixtures::dnd5e_2014_phb();
        let created = repo.create(source).await?;
        assert_eq!(created.id, "dnd5e-2014-phb");
        
        // Find
        let found = repo.find_by_id("dnd5e-2014-phb").await?;
        assert!(found.is_some());
        
        // Update
        let mut updated = found.unwrap();
        updated.name = "Player's Handbook (Updated)".to_string();
        let result = repo.update("dnd5e-2014-phb", updated).await?;
        assert_eq!(result.name, "Player's Handbook (Updated)");
        
        // List
        let list = repo.list().await?;
        assert_eq!(list.len(), 1);
        
        // Delete
        repo.delete("dnd5e-2014-phb").await?;
        let deleted = repo.find_by_id("dnd5e-2014-phb").await?;
        assert!(deleted.is_none());
        
        Ok(())
    }

    #[tokio::test]
    async fn test_find_by_rule_system() -> Result<()> {
        let test_db = TestDatabase::file_based()?;
        
        // Create rule system
        let rule_repo = dal::rule_systems::RuleSystemRepository::new(test_db.url.clone());
        rule_repo.create(fixtures::create_rule_system()).await?;
        
        let repo = SourceRepository::new(test_db.url.clone());
        
        // Create sources
        repo.create(fixtures::dnd5e_2014_phb()).await?;
        repo.create(fixtures::dnd5e_2014_dmg()).await?;
        repo.create(fixtures::dnd5e_2014_mm()).await?;
        
        // Find by rule system
        let sources = repo.find_by_rule_system("dnd5e-2014").await?;
        assert_eq!(sources.len(), 3);
        
        // Non-existent rule system
        let sources = repo.find_by_rule_system("nonexistent").await?;
        assert_eq!(sources.len(), 0);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_find_official_sources() -> Result<()> {
        let test_db = TestDatabase::file_based()?;
        
        // Create rule system
        let rule_repo = dal::rule_systems::RuleSystemRepository::new(test_db.url.clone());
        rule_repo.create(fixtures::create_rule_system()).await?;
        
        let repo = SourceRepository::new(test_db.url.clone());
        
        // Create mix of official and third-party sources
        repo.create(fixtures::dnd5e_2014_phb()).await?;
        repo.create(fixtures::dnd5e_2014_dmg()).await?;
        repo.create(fixtures::third_party_source()).await?;
        
        // Find official sources
        let official = repo.find_official().await?;
        assert_eq!(official.len(), 2);
        assert!(official.iter().all(|s| s.is_official));
        
        Ok(())
    }

    #[tokio::test]
    async fn test_find_srd_sources() -> Result<()> {
        let test_db = TestDatabase::file_based()?;
        
        // Create rule system
        let rule_repo = dal::rule_systems::RuleSystemRepository::new(test_db.url.clone());
        rule_repo.create(fixtures::create_rule_system()).await?;
        
        let repo = SourceRepository::new(test_db.url.clone());
        
        // Create mix of SRD and non-SRD sources
        repo.create(fixtures::dnd5e_2014_phb()).await?;
        repo.create(fixtures::dnd5e_2014_srd()).await?;
        
        // Find SRD sources
        let srd = repo.find_srd().await?;
        assert_eq!(srd.len(), 1);
        assert!(srd[0].is_srd);
        assert_eq!(srd[0].id, "dnd5e-2014-srd");
        
        Ok(())
    }
}

// Metadata and special features tests
mod special_features {
    use super::*;

    #[test]
    fn test_metadata_handling() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            
            let metadata = fixtures::SourceMetadata {
                isbn: Some("978-0-7869-6560-1".to_string()),
                pages: Some(320),
                language: "English".to_string(),
            };
            
            let source = Source::new(
                "dnd5e-phb-deluxe".to_string(),
                "dnd5e-2014".to_string(),
                "Player's Handbook Deluxe Edition".to_string(),
            )
            .with_metadata(metadata.clone())?;
            
            let created = conn.create(source)?;
            
            let retrieved_metadata: fixtures::SourceMetadata = 
                created.metadata_as()?.unwrap();
            assert_eq!(retrieved_metadata, metadata);
            
            Ok(())
        })
    }

    #[test]
    fn test_builder_pattern() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            
            let source = Source::new("test".to_string(), "dnd5e-2014".to_string(), "Test Source".to_string())
                .with_abbreviation("TST".to_string())
                .with_publisher("Test Publisher".to_string())
                .with_publish_date("2024-01-01".to_string())
                .with_official(false)
                .with_srd(true);
            
            let created = conn.create(source)?;
            
            assert_eq!(created.abbreviation, Some("TST".to_string()));
            assert_eq!(created.publisher, Some("Test Publisher".to_string()));
            assert_eq!(created.publish_date, Some("2024-01-01".to_string()));
            assert!(!created.is_official);
            assert!(created.is_srd);
            
            Ok(())
        })
    }
}

// Constraint and error handling tests
mod constraint_tests {
    use super::*;

    #[test]
    fn test_unique_constraint() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            
            let source = fixtures::dnd5e_2014_phb();
            conn.create(source.clone())?;
            
            // Try to create duplicate
            let result = conn.create(source);
            assert!(result.is_err());
            assert!(result.unwrap_err().is_unique_violation());
            
            Ok(())
        })
    }

    #[test]
    fn test_foreign_key_constraint() -> Result<()> {
        with_test_db(|conn| {
            // Try to create source without rule system
            let source = Source::new(
                "orphan-source".to_string(),
                "nonexistent-rule-system".to_string(),
                "Orphan Source".to_string(),
            );
            
            let result = conn.create(source);
            assert!(result.is_err());
            assert!(result.unwrap_err().is_foreign_key_violation());
            
            Ok(())
        })
    }

    #[test]
    fn test_not_found_errors() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            
            // Update non-existent
            let source = fixtures::dnd5e_2014_phb();
            let result = conn.update("nonexistent", source);
            assert!(matches!(result, Err(DbError::NotFound { .. })));
            
            // Delete non-existent
            let result = Repository::<Source>::delete(conn, "nonexistent");
            assert!(matches!(result, Err(DbError::NotFound { .. })));
            
            Ok(())
        })
    }
}