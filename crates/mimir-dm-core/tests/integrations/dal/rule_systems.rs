//! Integration tests for rule_systems DAL

use crate::common::with_test_db;
use mimir_dm_db::*;
use mimir_dm_db::dal::rule_systems::RuleSystemRepository;
use mimir_dm_db::dal::traits::{AsyncRepository, Repository};
use serde::{Deserialize, Serialize};

// Test fixtures
mod fixtures {
    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub struct RuleSystemMetadata {
        pub setting: String,
        pub genre: Vec<String>,
    }

    pub fn dnd5e_2014() -> RuleSystem {
        RuleSystem::new(
            "dnd5e-2014".to_string(),
            "D&D 5th Edition (2014)".to_string(),
        )
        .with_short_name("D&D 5e 2014".to_string())
        .with_publisher("Wizards of the Coast".to_string())
        .with_version("2014".to_string())
    }

    pub fn dnd5e_2024() -> RuleSystem {
        RuleSystem::new(
            "dnd5e-2024".to_string(),
            "D&D 5th Edition (2024)".to_string(),
        )
        .with_short_name("D&D 5e 2024".to_string())
        .with_publisher("Wizards of the Coast".to_string())
        .with_version("2024".to_string())
    }

    pub fn pathfinder_2e() -> RuleSystem {
        RuleSystem::new(
            "pf2e".to_string(),
            "Pathfinder 2nd Edition".to_string(),
        )
        .with_short_name("PF2e".to_string())
        .with_publisher("Paizo".to_string())
        .with_version("2.0".to_string())
    }
}

// CRUD operation tests
mod crud_operations {
    use super::*;

    #[test]
    fn test_create_rule_system() -> Result<()> {
        with_test_db(|conn| {
            let rule_system = fixtures::dnd5e_2014();
            let created = conn.create(rule_system)?;
            
            assert_eq!(created.id, "dnd5e-2014");
            assert_eq!(created.name, "D&D 5th Edition (2014)");
            assert_eq!(created.short_name, Some("D&D 5e 2014".to_string()));
            assert_eq!(created.publisher, Some("Wizards of the Coast".to_string()));
            assert_eq!(created.version, Some("2014".to_string()));
            assert!(created.is_active);
            
            Ok(())
        })
    }

    #[test]
    fn test_find_by_id() -> Result<()> {
        with_test_db(|conn| {
            let rule_system = fixtures::pathfinder_2e();
            conn.create(rule_system)?;
            
            let found: Option<RuleSystem> = conn.find_by_id("pf2e")?;
            assert!(found.is_some());
            assert_eq!(found.unwrap().name, "Pathfinder 2nd Edition");
            
            let not_found: Option<RuleSystem> = conn.find_by_id("nonexistent")?;
            assert!(not_found.is_none());
            
            Ok(())
        })
    }

    #[test]
    fn test_update_rule_system() -> Result<()> {
        with_test_db(|conn| {
            let rule_system = fixtures::dnd5e_2024();
            conn.create(rule_system)?;
            
            let mut updated = RuleSystem::new(
                "dnd5e-2024".to_string(),
                "D&D 5th Edition (2024) - Updated".to_string(),
            );
            updated.is_active = false;
            
            let result = conn.update("dnd5e-2024", updated)?;
            assert_eq!(result.name, "D&D 5th Edition (2024) - Updated");
            assert!(!result.is_active);
            
            Ok(())
        })
    }

    #[test]
    fn test_delete_rule_system() -> Result<()> {
        with_test_db(|conn| {
            let rule_system = RuleSystem::new(
                "test-system".to_string(),
                "Test System".to_string(),
            );
            conn.create(rule_system)?;
            
            Repository::<RuleSystem>::delete(conn, "test-system")?;
            
            let found: Option<RuleSystem> = conn.find_by_id("test-system")?;
            assert!(found.is_none());
            
            // Try to delete non-existent
            let result = Repository::<RuleSystem>::delete(conn, "nonexistent");
            assert!(matches!(result, Err(DbError::NotFound { .. })));
            
            Ok(())
        })
    }

    #[test]
    fn test_list_rule_systems() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::dnd5e_2014())?;
            conn.create(fixtures::dnd5e_2024())?;
            conn.create(fixtures::pathfinder_2e())?;
            
            let list: Vec<RuleSystem> = conn.list()?;
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
        let test_db = crate::common::TestDatabase::file_based()?;
        let repo = RuleSystemRepository::new(test_db.url.clone());
        
        // Create
        let rule_system = fixtures::dnd5e_2014();
        let created = repo.create(rule_system).await?;
        assert_eq!(created.id, "dnd5e-2014");
        
        // Find
        let found = repo.find_by_id("dnd5e-2014").await?;
        assert!(found.is_some());
        
        // Update
        let mut updated = found.unwrap();
        updated.name = "Updated Name".to_string();
        let result = repo.update("dnd5e-2014", updated).await?;
        assert_eq!(result.name, "Updated Name");
        
        // List
        let list = repo.list().await?;
        assert_eq!(list.len(), 1);
        
        // Delete
        repo.delete("dnd5e-2014").await?;
        let deleted = repo.find_by_id("dnd5e-2014").await?;
        assert!(deleted.is_none());
        
        Ok(())
    }

}

// Metadata and special features tests
mod special_features {
    use super::*;

    #[test]
    fn test_metadata_handling() -> Result<()> {
        with_test_db(|conn| {
            let metadata = fixtures::RuleSystemMetadata {
                setting: "Forgotten Realms".to_string(),
                genre: vec!["fantasy".to_string(), "high magic".to_string()],
            };
            
            let rule_system = RuleSystem::new(
                "dnd5e-fr".to_string(),
                "D&D 5e Forgotten Realms".to_string(),
            )
            .with_metadata(metadata.clone())?;
            
            let created = conn.create(rule_system)?;
            
            let retrieved_metadata: fixtures::RuleSystemMetadata = 
                created.metadata_as()?.unwrap();
            assert_eq!(retrieved_metadata, metadata);
            
            Ok(())
        })
    }

    #[test]
    fn test_builder_pattern() -> Result<()> {
        with_test_db(|conn| {
            let system = RuleSystem::new("test".to_string(), "Test System".to_string())
                .with_short_name("Test".to_string())
                .with_publisher("Test Publisher".to_string())
                .with_version("1.0".to_string());
            
            let created = conn.create(system)?;
            
            assert_eq!(created.short_name, Some("Test".to_string()));
            assert_eq!(created.publisher, Some("Test Publisher".to_string()));
            assert_eq!(created.version, Some("1.0".to_string()));
            
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
            let system = fixtures::dnd5e_2014();
            conn.create(system.clone())?;
            
            // Try to create duplicate
            let result = conn.create(system);
            assert!(result.is_err());
            assert!(result.unwrap_err().is_unique_violation());
            
            Ok(())
        })
    }

    #[test]
    fn test_not_found_errors() -> Result<()> {
        with_test_db(|conn| {
            // Update non-existent
            let system = fixtures::dnd5e_2014();
            let result = conn.update("nonexistent", system);
            assert!(matches!(result, Err(DbError::NotFound { .. })));
            
            // Delete non-existent
            let result = Repository::<RuleSystem>::delete(conn, "nonexistent");
            assert!(matches!(result, Err(DbError::NotFound { .. })));
            
            Ok(())
        })
    }
}