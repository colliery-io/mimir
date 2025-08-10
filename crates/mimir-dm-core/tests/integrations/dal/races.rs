//! Integration tests for races DAL

use crate::common::{with_test_db, TestDatabase};
use mimir_dm_db::*;
use mimir_dm_db::dal::races::RaceRepository;
use mimir_dm_db::dal::traits::{AsyncRepository, Repository};
use mimir_dm_db::models::races::{AbilityScores, Speed};
use serde_json::json;

// Test fixtures
mod fixtures {
    use super::*;

    pub fn create_rule_system() -> RuleSystem {
        RuleSystem::new(
            "dnd5e-2014".to_string(),
            "D&D 5th Edition (2014)".to_string(),
        )
    }

    pub fn create_source() -> Source {
        Source::new(
            "dnd5e-2014-phb".to_string(),
            "dnd5e-2014".to_string(),
            "Player's Handbook".to_string(),
        )
    }

    pub fn human_base() -> Race {
        Race::new(
            "human".to_string(),
            "Human".to_string(),
            "dnd5e-2014".to_string(),
            "dnd5e-2014-phb".to_string(),
            "race".to_string(),
            json!([
                {"type": "entries", "name": "Age", "entries": ["Humans reach adulthood in their late teens and live less than a century."]},
                {"type": "entries", "name": "Size", "entries": ["Humans vary widely in height and build, from barely 5 feet to well over 6 feet tall. Your size is Medium."]}
            ])
        ).unwrap()
        .with_page(29)
        .with_size("M".to_string())
        .with_speed(Speed { walk: Some(30), fly: None, swim: None, climb: None, burrow: None }).unwrap()
        .with_ability_scores(AbilityScores {
            strength: Some(1),
            dexterity: Some(1),
            constitution: Some(1),
            intelligence: Some(1),
            wisdom: Some(1),
            charisma: Some(1),
            choose: None,
        }).unwrap()
        .with_language_proficiencies(vec!["Common", "one of your choice"]).unwrap()
    }

    pub fn elf_base() -> Race {
        Race::new(
            "elf".to_string(),
            "Elf".to_string(),
            "dnd5e-2014".to_string(),
            "dnd5e-2014-phb".to_string(),
            "race".to_string(),
            json!([
                {"type": "entries", "name": "Darkvision", "entries": ["You can see in dim light within 60 feet of you as if it were bright light."]},
                {"type": "entries", "name": "Keen Senses", "entries": ["You have proficiency in the Perception skill."]}
            ])
        ).unwrap()
        .with_page(21)
        .with_size("M".to_string())
        .with_speed(Speed { walk: Some(30), fly: None, swim: None, climb: None, burrow: None }).unwrap()
        .with_ability_scores(AbilityScores {
            strength: None,
            dexterity: Some(2),
            constitution: None,
            intelligence: None,
            wisdom: None,
            charisma: None,
            choose: None,
        }).unwrap()
        .with_trait_tags(vec!["Darkvision".to_string(), "Keen Senses".to_string()]).unwrap()
    }

    pub fn high_elf_subrace() -> Race {
        Race::new(
            "high-elf".to_string(),
            "High Elf".to_string(),
            "dnd5e-2014".to_string(),
            "dnd5e-2014-phb".to_string(),
            "subrace".to_string(),
            json!([
                {"type": "entries", "name": "Cantrip", "entries": ["You know one cantrip of your choice from the wizard spell list."]}
            ])
        ).unwrap()
        .with_parent_race("elf".to_string())
        .with_page(23)
        .with_size("M".to_string())
        .with_speed(Speed { walk: Some(30), fly: None, swim: None, climb: None, burrow: None }).unwrap()
        .with_ability_scores(AbilityScores {
            strength: None,
            dexterity: Some(2),
            constitution: None,
            intelligence: Some(1),
            wisdom: None,
            charisma: None,
            choose: None,
        }).unwrap()
    }

    pub fn wood_elf_subrace() -> Race {
        Race::new(
            "wood-elf".to_string(),
            "Wood Elf".to_string(),
            "dnd5e-2014".to_string(),
            "dnd5e-2014-phb".to_string(),
            "subrace".to_string(),
            json!([
                {"type": "entries", "name": "Fleet of Foot", "entries": ["Your base walking speed increases to 35 feet."]}
            ])
        ).unwrap()
        .with_parent_race("elf".to_string())
        .with_page(24)
        .with_size("M".to_string())
        .with_speed(Speed { walk: Some(35), fly: None, swim: None, climb: None, burrow: None }).unwrap()
        .with_ability_scores(AbilityScores {
            strength: None,
            dexterity: Some(2),
            constitution: None,
            intelligence: None,
            wisdom: Some(1),
            charisma: None,
            choose: None,
        }).unwrap()
    }

    pub fn halfling_base() -> Race {
        Race::new(
            "halfling".to_string(),
            "Halfling".to_string(),
            "dnd5e-2014".to_string(),
            "dnd5e-2014-phb".to_string(),
            "race".to_string(),
            json!([
                {"type": "entries", "name": "Lucky", "entries": ["When you roll a 1 on an attack roll, ability check, or saving throw, you can reroll the die."]}
            ])
        ).unwrap()
        .with_page(26)
        .with_size("S".to_string())
        .with_speed(Speed { walk: Some(25), fly: None, swim: None, climb: None, burrow: None }).unwrap()
    }
}

// CRUD operation tests
mod crud_operations {
    use super::*;

    #[test]
    fn test_create_race() -> Result<()> {
        with_test_db(|conn| {
            // First create dependencies
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let race = fixtures::human_base();
            let created = conn.create(race)?;
            
            assert_eq!(created.id, "human");
            assert_eq!(created.name, "Human");
            assert_eq!(created.race_type, "race");
            assert!(created.parent_race_id.is_none());
            assert_eq!(created.size, Some("M".to_string()));
            
            Ok(())
        })
    }

    #[test]
    fn test_find_by_id() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let race = fixtures::elf_base();
            conn.create(race)?;
            
            let found: Option<Race> = conn.find_by_id("elf")?;
            assert!(found.is_some());
            assert_eq!(found.unwrap().name, "Elf");
            
            let not_found: Option<Race> = conn.find_by_id("nonexistent")?;
            assert!(not_found.is_none());
            
            Ok(())
        })
    }

    #[test]
    fn test_update_race() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let race = fixtures::halfling_base();
            conn.create(race)?;
            
            let mut updated = fixtures::halfling_base();
            updated.name = "Halfling (Updated)".to_string();
            
            let result = conn.update("halfling", updated)?;
            assert_eq!(result.name, "Halfling (Updated)");
            
            Ok(())
        })
    }

    #[test]
    fn test_delete_race() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let race = Race::new(
                "test-race".to_string(),
                "Test Race".to_string(),
                "dnd5e-2014".to_string(),
                "dnd5e-2014-phb".to_string(),
                "race".to_string(),
                json!([])
            )?;
            conn.create(race)?;
            
            Repository::<Race>::delete(conn, "test-race")?;
            
            let found: Option<Race> = conn.find_by_id("test-race")?;
            assert!(found.is_none());
            
            // Try to delete non-existent
            let result = Repository::<Race>::delete(conn, "nonexistent");
            assert!(matches!(result, Err(DbError::NotFound { .. })));
            
            Ok(())
        })
    }

    #[test]
    fn test_list_races() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            conn.create(fixtures::human_base())?;
            conn.create(fixtures::elf_base())?;
            conn.create(fixtures::halfling_base())?;
            
            let list: Vec<Race> = conn.list()?;
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
        
        // First create dependencies
        let rule_repo = dal::rule_systems::RuleSystemRepository::new(test_db.url.clone());
        rule_repo.create(fixtures::create_rule_system()).await?;
        
        let source_repo = dal::sources::SourceRepository::new(test_db.url.clone());
        source_repo.create(fixtures::create_source()).await?;
        
        let repo = RaceRepository::new(test_db.url.clone());
        
        // Create
        let race = fixtures::human_base();
        let created = repo.create(race).await?;
        assert_eq!(created.id, "human");
        
        // Find
        let found = repo.find_by_id("human").await?;
        assert!(found.is_some());
        
        // Update
        let mut updated = found.unwrap();
        updated.name = "Human (Updated)".to_string();
        let result = repo.update("human", updated).await?;
        assert_eq!(result.name, "Human (Updated)");
        
        // List
        let list = repo.list().await?;
        assert_eq!(list.len(), 1);
        
        // Delete
        repo.delete("human").await?;
        let deleted = repo.find_by_id("human").await?;
        assert!(deleted.is_none());
        
        Ok(())
    }

    #[tokio::test]
    async fn test_find_base_races_and_subraces() -> Result<()> {
        let test_db = TestDatabase::file_based()?;
        
        // Create dependencies
        let rule_repo = dal::rule_systems::RuleSystemRepository::new(test_db.url.clone());
        rule_repo.create(fixtures::create_rule_system()).await?;
        
        let source_repo = dal::sources::SourceRepository::new(test_db.url.clone());
        source_repo.create(fixtures::create_source()).await?;
        
        let repo = RaceRepository::new(test_db.url.clone());
        
        // Create base race and subraces
        repo.create(fixtures::elf_base()).await?;
        repo.create(fixtures::high_elf_subrace()).await?;
        repo.create(fixtures::wood_elf_subrace()).await?;
        repo.create(fixtures::human_base()).await?;
        
        // Find base races
        let base_races = repo.find_base_races().await?;
        assert_eq!(base_races.len(), 2);
        assert!(base_races.iter().all(|r| r.is_base_race()));
        
        // Find subraces
        let subraces = repo.find_subraces("elf").await?;
        assert_eq!(subraces.len(), 2);
        assert!(subraces.iter().all(|r| r.is_subrace()));
        
        Ok(())
    }

    #[tokio::test]
    async fn test_find_by_type() -> Result<()> {
        let test_db = TestDatabase::file_based()?;
        
        // Create dependencies
        let rule_repo = dal::rule_systems::RuleSystemRepository::new(test_db.url.clone());
        rule_repo.create(fixtures::create_rule_system()).await?;
        
        let source_repo = dal::sources::SourceRepository::new(test_db.url.clone());
        source_repo.create(fixtures::create_source()).await?;
        
        let repo = RaceRepository::new(test_db.url.clone());
        
        // Create races and subraces
        repo.create(fixtures::elf_base()).await?;
        repo.create(fixtures::high_elf_subrace()).await?;
        repo.create(fixtures::human_base()).await?;
        
        // Find by type
        let races = repo.find_by_type("race").await?;
        assert_eq!(races.len(), 2);
        
        let subraces = repo.find_by_type("subrace").await?;
        assert_eq!(subraces.len(), 1);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_find_by_size() -> Result<()> {
        let test_db = TestDatabase::file_based()?;
        
        // Create dependencies
        let rule_repo = dal::rule_systems::RuleSystemRepository::new(test_db.url.clone());
        rule_repo.create(fixtures::create_rule_system()).await?;
        
        let source_repo = dal::sources::SourceRepository::new(test_db.url.clone());
        source_repo.create(fixtures::create_source()).await?;
        
        let repo = RaceRepository::new(test_db.url.clone());
        
        // Create races with different sizes
        repo.create(fixtures::human_base()).await?;
        repo.create(fixtures::elf_base()).await?;
        repo.create(fixtures::halfling_base()).await?;
        
        // Find by size
        let medium = repo.find_by_size("M").await?;
        assert_eq!(medium.len(), 2);
        
        let small = repo.find_by_size("S").await?;
        assert_eq!(small.len(), 1);
        assert_eq!(small[0].id, "halfling");
        
        Ok(())
    }
}

// Metadata and special features tests
mod special_features {
    use super::*;

    #[test]
    fn test_speed_handling() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let race = fixtures::elf_base();
            let created = conn.create(race)?;
            
            let speed = created.speed_typed()?.unwrap();
            assert_eq!(speed.walk, Some(30));
            assert_eq!(speed.fly, None);
            
            Ok(())
        })
    }

    #[test]
    fn test_ability_scores_handling() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let race = fixtures::human_base();
            let created = conn.create(race)?;
            
            let scores = created.ability_scores_typed()?.unwrap();
            assert_eq!(scores.strength, Some(1));
            assert_eq!(scores.dexterity, Some(1));
            assert_eq!(scores.constitution, Some(1));
            assert_eq!(scores.intelligence, Some(1));
            assert_eq!(scores.wisdom, Some(1));
            assert_eq!(scores.charisma, Some(1));
            
            Ok(())
        })
    }

    #[test]
    fn test_trait_tags_handling() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let race = fixtures::elf_base();
            let created = conn.create(race)?;
            
            let tags = created.trait_tags_vec()?.unwrap();
            assert_eq!(tags.len(), 2);
            assert!(tags.contains(&"Darkvision".to_string()));
            assert!(tags.contains(&"Keen Senses".to_string()));
            
            Ok(())
        })
    }

    #[test]
    fn test_entries_handling() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let race = fixtures::human_base();
            let created = conn.create(race)?;
            
            let entries = created.entries_value()?;
            assert!(entries.is_array());
            assert_eq!(entries.as_array().unwrap().len(), 2);
            
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
            conn.create(fixtures::create_source())?;
            
            let race = fixtures::human_base();
            conn.create(race.clone())?;
            
            // Try to create duplicate
            let result = conn.create(race);
            assert!(result.is_err());
            assert!(result.unwrap_err().is_unique_violation());
            
            Ok(())
        })
    }

    #[test]
    fn test_foreign_key_constraints() -> Result<()> {
        with_test_db(|conn| {
            // Try to create race without rule system
            let race = Race::new(
                "orphan-race".to_string(),
                "Orphan Race".to_string(),
                "nonexistent-rule-system".to_string(),
                "nonexistent-source".to_string(),
                "race".to_string(),
                json!([])
            )?;
            
            let result = conn.create(race);
            assert!(result.is_err());
            assert!(result.unwrap_err().is_foreign_key_violation());
            
            Ok(())
        })
    }

    #[test]
    fn test_parent_race_constraint() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            // Try to create subrace with non-existent parent
            let subrace = Race::new(
                "orphan-subrace".to_string(),
                "Orphan Subrace".to_string(),
                "dnd5e-2014".to_string(),
                "dnd5e-2014-phb".to_string(),
                "subrace".to_string(),
                json!([])
            )?
            .with_parent_race("nonexistent-parent".to_string());
            
            let result = conn.create(subrace);
            assert!(result.is_err());
            assert!(result.unwrap_err().is_foreign_key_violation());
            
            Ok(())
        })
    }

    #[test]
    fn test_not_found_errors() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            // Update non-existent
            let race = fixtures::human_base();
            let result = conn.update("nonexistent", race);
            assert!(matches!(result, Err(DbError::NotFound { .. })));
            
            // Delete non-existent
            let result = Repository::<Race>::delete(conn, "nonexistent");
            assert!(matches!(result, Err(DbError::NotFound { .. })));
            
            Ok(())
        })
    }
}