//! Integration tests for feats DAL

use crate::common::{with_test_db, TestDatabase};
use mimir_dm_db::*;
use mimir_dm_db::dal::feats::{FeatRepository, FeatStatistics};
use mimir_dm_db::dal::traits::{AsyncRepository, Repository};
use mimir_dm_db::models::feats::{
    Feat, Prerequisites, AbilityPrereq, AbilityIncreases, AbilityIncrease, 
    AbilityChoice, FeatType
};
use serde_json::json;
use std::collections::HashMap;

// Test fixtures
mod fixtures {
    use super::*;

    pub fn create_rule_system() -> RuleSystem {
        RuleSystem::new(
            "dnd5e-2014".to_string(),
            "D&D 5th Edition (2014)".to_string(),
        )
    }

    pub fn create_rule_system_2024() -> RuleSystem {
        RuleSystem::new(
            "dnd5e-2024".to_string(),
            "D&D 5th Edition (2024)".to_string(),
        )
    }

    pub fn create_source() -> Source {
        Source::new(
            "dnd5e-2014-phb".to_string(),
            "dnd5e-2014".to_string(),
            "Player's Handbook".to_string(),
        )
    }

    pub fn create_source_2024() -> Source {
        Source::new(
            "dnd5e-2024-phb".to_string(),
            "dnd5e-2024".to_string(),
            "Player's Handbook".to_string(),
        )
    }

    pub fn alert() -> Feat {
        Feat::new(
            "alert".to_string(),
            "Alert".to_string(),
            "dnd5e-2014".to_string(),
            "dnd5e-2014-phb".to_string(),
            json!([
                {"type": "entries", "entries": ["Always on the lookout for danger, you gain the following benefits:"]}
            ])
        ).unwrap()
        .with_page(165)
        .with_feat_type(FeatType::General)
    }

    pub fn actor() -> Feat {
        Feat::new(
            "actor".to_string(),
            "Actor".to_string(),
            "dnd5e-2014".to_string(),
            "dnd5e-2014-phb".to_string(),
            json!([
                {"type": "entries", "entries": ["Skilled at mimicry and dramatics, you gain the following benefits:"]}
            ])
        ).unwrap()
        .with_page(165)
        .with_feat_type(FeatType::General)
        .with_ability_increases(AbilityIncreases {
            fixed: Some(vec![
                AbilityIncrease {
                    ability: "cha".to_string(),
                    increase: 1,
                }
            ]),
            choices: None,
        }).unwrap()
    }

    pub fn eldritch_adept() -> Feat {
        Feat::new(
            "eldritch-adept".to_string(),
            "Eldritch Adept".to_string(),
            "dnd5e-2014".to_string(),
            "dnd5e-2014-phb".to_string(),
            json!([
                {"type": "entries", "entries": ["Studying occult lore, you have unlocked eldritch power within yourself."]}
            ])
        ).unwrap()
        .with_page(200)
        .with_feat_type(FeatType::General)
        .with_prerequisites(Prerequisites {
            abilities: None,
            level: None,
            classes: Some(vec!["warlock".to_string()]),
            races: None,
            feats: None,
            spells: Some(vec!["eldritch blast".to_string()]),
            other: None,
        }).unwrap()
    }

    pub fn tough() -> Feat {
        Feat::new(
            "tough".to_string(),
            "Tough".to_string(),
            "dnd5e-2014".to_string(),
            "dnd5e-2014-phb".to_string(),
            json!([
                {"type": "entries", "entries": ["Your hit point maximum increases by an amount equal to twice your level when you gain this feat."]}
            ])
        ).unwrap()
        .with_page(200)
        .with_feat_type(FeatType::General)
    }

    pub fn great_weapon_master() -> Feat {
        Feat::new(
            "great-weapon-master".to_string(),
            "Great Weapon Master".to_string(),
            "dnd5e-2014".to_string(),
            "dnd5e-2014-phb".to_string(),
            json!([
                {"type": "entries", "entries": ["You've learned to put the weight of a weapon to your advantage."]}
            ])
        ).unwrap()
        .with_page(167)
        .with_feat_type(FeatType::General)
        .with_prerequisites(Prerequisites {
            abilities: Some(vec![
                AbilityPrereq {
                    ability: "str".to_string(),
                    score: 13,
                }
            ]),
            level: None,
            classes: None,
            races: None,
            feats: None,
            spells: None,
            other: None,
        }).unwrap()
    }

    pub fn fighting_style_archery() -> Feat {
        Feat::new(
            "fighting-style-archery".to_string(),
            "Fighting Style: Archery".to_string(),
            "dnd5e-2024".to_string(),
            "dnd5e-2024-phb".to_string(),
            json!([
                {"type": "entries", "entries": ["You gain a +2 bonus to attack rolls you make with ranged weapons."]}
            ])
        ).unwrap()
        .with_page(190)
        .with_feat_type(FeatType::FightingStyle)
    }

    pub fn origin_feat() -> Feat {
        Feat::new(
            "background-feat".to_string(),
            "Background Feat".to_string(),
            "dnd5e-2024".to_string(),
            "dnd5e-2024-phb".to_string(),
            json!([
                {"type": "entries", "entries": ["This feat represents training from your background."]}
            ])
        ).unwrap()
        .with_page(50)
        .with_feat_type(FeatType::Origin)
        .with_ability_increases(AbilityIncreases {
            fixed: None,
            choices: Some(vec![
                AbilityChoice {
                    from: vec!["str".to_string(), "dex".to_string(), "con".to_string(), "int".to_string(), "wis".to_string(), "cha".to_string()],
                    count: 1,
                    increase: 1,
                }
            ]),
        }).unwrap()
    }

    pub fn epic_boon() -> Feat {
        Feat::new(
            "epic-boon-spell-recall".to_string(),
            "Epic Boon: Spell Recall".to_string(),
            "dnd5e-2024".to_string(),
            "dnd5e-2024-phb".to_string(),
            json!([
                {"type": "entries", "entries": ["You can cast any spell you know or have prepared without expending a spell slot."]}
            ])
        ).unwrap()
        .with_page(230)
        .with_feat_type(FeatType::Epic)
        .with_prerequisites(Prerequisites {
            abilities: None,
            level: Some(20),
            classes: None,
            races: None,
            feats: None,
            spells: None,
            other: Some(vec!["Epic level character".to_string()]),
        }).unwrap()
    }
}

// CRUD operation tests
mod crud_operations {
    use super::*;

    #[test]
    fn test_create_feat() -> Result<()> {
        with_test_db(|conn| {
            // First create dependencies
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let feat = fixtures::alert();
            let created = conn.create(feat)?;
            
            assert_eq!(created.id, "alert");
            assert_eq!(created.name, "Alert");
            assert_eq!(created.page, Some(165));
            assert_eq!(created.feat_type, Some("general".to_string()));
            assert!(!created.has_prerequisites());
            assert!(!created.provides_ability_increases());
            
            Ok(())
        })
    }

    #[test]
    fn test_find_by_id() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let feat = fixtures::actor();
            conn.create(feat)?;
            
            let found: Option<Feat> = conn.find_by_id("actor")?;
            assert!(found.is_some());
            let f = found.unwrap();
            assert_eq!(f.name, "Actor");
            assert!(f.provides_ability_increases());
            
            let not_found: Option<Feat> = conn.find_by_id("nonexistent")?;
            assert!(not_found.is_none());
            
            Ok(())
        })
    }

    #[test]
    fn test_update_feat() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let feat = fixtures::tough();
            conn.create(feat)?;
            
            let mut updated = fixtures::tough();
            updated.name = "Tough (Updated)".to_string();
            
            let result = conn.update("tough", updated)?;
            assert_eq!(result.name, "Tough (Updated)");
            
            Ok(())
        })
    }

    #[test]
    fn test_delete_feat() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let feat = Feat::new(
                "test-feat".to_string(),
                "Test Feat".to_string(),
                "dnd5e-2014".to_string(),
                "dnd5e-2014-phb".to_string(),
                json!([])
            )?;
            conn.create(feat)?;
            
            Repository::<Feat>::delete(conn, "test-feat")?;
            
            let found: Option<Feat> = conn.find_by_id("test-feat")?;
            assert!(found.is_none());
            
            // Try to delete non-existent
            let result = Repository::<Feat>::delete(conn, "nonexistent");
            assert!(matches!(result, Err(DbError::NotFound { .. })));
            
            Ok(())
        })
    }

    #[test]
    fn test_list_feats() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            conn.create(fixtures::alert())?;
            conn.create(fixtures::actor())?;
            conn.create(fixtures::tough())?;
            
            let list: Vec<Feat> = conn.list()?;
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
        
        let repo = FeatRepository::new(test_db.url.clone());
        
        // Create
        let feat = fixtures::alert();
        let created = repo.create(feat).await?;
        assert_eq!(created.id, "alert");
        
        // Find
        let found = repo.find_by_id("alert").await?;
        assert!(found.is_some());
        
        // Update
        let mut updated = found.unwrap();
        updated.name = "Alert (Updated)".to_string();
        let result = repo.update("alert", updated).await?;
        assert_eq!(result.name, "Alert (Updated)");
        
        // List
        let list = repo.list().await?;
        assert_eq!(list.len(), 1);
        
        // Delete
        repo.delete("alert").await?;
        let deleted = repo.find_by_id("alert").await?;
        assert!(deleted.is_none());
        
        Ok(())
    }

    #[tokio::test]
    async fn test_find_by_feat_type() -> Result<()> {
        let test_db = TestDatabase::file_based()?;
        
        // Create dependencies
        let rule_repo = dal::rule_systems::RuleSystemRepository::new(test_db.url.clone());
        rule_repo.create(fixtures::create_rule_system()).await?;
        rule_repo.create(fixtures::create_rule_system_2024()).await?;
        
        let source_repo = dal::sources::SourceRepository::new(test_db.url.clone());
        source_repo.create(fixtures::create_source()).await?;
        source_repo.create(fixtures::create_source_2024()).await?;
        
        let repo = FeatRepository::new(test_db.url.clone());
        
        // Create feats of different types
        repo.create(fixtures::alert()).await?; // General
        repo.create(fixtures::actor()).await?; // General
        repo.create(fixtures::origin_feat()).await?; // Origin
        repo.create(fixtures::fighting_style_archery()).await?; // Fighting Style
        repo.create(fixtures::epic_boon()).await?; // Epic
        
        // Find by type
        let general = repo.find_general_feats().await?;
        assert_eq!(general.len(), 2);
        
        let origin = repo.find_origin_feats().await?;
        assert_eq!(origin.len(), 1);
        
        let fighting_style = repo.find_fighting_style_feats().await?;
        assert_eq!(fighting_style.len(), 1);
        
        let epic = repo.find_epic_feats().await?;
        assert_eq!(epic.len(), 1);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_prerequisites_filtering() -> Result<()> {
        let test_db = TestDatabase::file_based()?;
        
        // Create dependencies
        let rule_repo = dal::rule_systems::RuleSystemRepository::new(test_db.url.clone());
        rule_repo.create(fixtures::create_rule_system()).await?;
        rule_repo.create(fixtures::create_rule_system_2024()).await?;
        
        let source_repo = dal::sources::SourceRepository::new(test_db.url.clone());
        source_repo.create(fixtures::create_source()).await?;
        source_repo.create(fixtures::create_source_2024()).await?;
        
        let repo = FeatRepository::new(test_db.url.clone());
        
        // Create feats with and without prerequisites
        repo.create(fixtures::alert()).await?; // No prerequisites
        repo.create(fixtures::great_weapon_master()).await?; // Strength prerequisite
        repo.create(fixtures::eldritch_adept()).await?; // Class/spell prerequisites
        repo.create(fixtures::epic_boon()).await?; // Level prerequisite
        
        // Find with/without prerequisites
        let with_prereqs = repo.find_with_prerequisites().await?;
        assert_eq!(with_prereqs.len(), 3);
        
        let without_prereqs = repo.find_without_prerequisites().await?;
        assert_eq!(without_prereqs.len(), 1);
        assert_eq!(without_prereqs[0].name, "Alert");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_ability_increases_filtering() -> Result<()> {
        let test_db = TestDatabase::file_based()?;
        
        // Create dependencies
        let rule_repo = dal::rule_systems::RuleSystemRepository::new(test_db.url.clone());
        rule_repo.create(fixtures::create_rule_system()).await?;
        rule_repo.create(fixtures::create_rule_system_2024()).await?;
        
        let source_repo = dal::sources::SourceRepository::new(test_db.url.clone());
        source_repo.create(fixtures::create_source()).await?;
        source_repo.create(fixtures::create_source_2024()).await?;
        
        let repo = FeatRepository::new(test_db.url.clone());
        
        // Create feats with and without ability increases
        repo.create(fixtures::alert()).await?; // No ability increases
        repo.create(fixtures::actor()).await?; // +1 Charisma
        repo.create(fixtures::origin_feat()).await?; // Choice of +1 to any
        
        // Find with ability increases
        let with_increases = repo.find_with_ability_increases().await?;
        assert_eq!(with_increases.len(), 2);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_search_and_filtering() -> Result<()> {
        let test_db = TestDatabase::file_based()?;
        
        // Create dependencies
        let rule_repo = dal::rule_systems::RuleSystemRepository::new(test_db.url.clone());
        rule_repo.create(fixtures::create_rule_system()).await?;
        rule_repo.create(fixtures::create_rule_system_2024()).await?;
        
        let source_repo = dal::sources::SourceRepository::new(test_db.url.clone());
        source_repo.create(fixtures::create_source()).await?;
        source_repo.create(fixtures::create_source_2024()).await?;
        
        let repo = FeatRepository::new(test_db.url.clone());
        
        // Create various feats
        repo.create(fixtures::alert()).await?;
        repo.create(fixtures::actor()).await?;
        repo.create(fixtures::great_weapon_master()).await?;
        
        // Search by name
        let search_results = repo.search_by_name("act").await?;
        assert_eq!(search_results.len(), 1);
        assert_eq!(search_results[0].name, "Actor");
        
        // Search by rule system
        let by_rule_system = repo.find_by_rule_system("dnd5e-2014").await?;
        assert_eq!(by_rule_system.len(), 3);
        
        // Find multiclass compatible (simplified test)
        let multiclass = repo.find_multiclass_compatible().await?;
        // Should find feats without class prerequisites
        assert!(multiclass.len() >= 2); // alert, actor, great_weapon_master
        
        Ok(())
    }

    #[tokio::test]
    async fn test_feat_statistics() -> Result<()> {
        let test_db = TestDatabase::file_based()?;
        
        // Create dependencies
        let rule_repo = dal::rule_systems::RuleSystemRepository::new(test_db.url.clone());
        rule_repo.create(fixtures::create_rule_system()).await?;
        rule_repo.create(fixtures::create_rule_system_2024()).await?;
        
        let source_repo = dal::sources::SourceRepository::new(test_db.url.clone());
        source_repo.create(fixtures::create_source()).await?;
        source_repo.create(fixtures::create_source_2024()).await?;
        
        let repo = FeatRepository::new(test_db.url.clone());
        
        // Create a variety of feats
        repo.create(fixtures::alert()).await?; // General, no prereqs, no ASI
        repo.create(fixtures::actor()).await?; // General, no prereqs, +1 CHA
        repo.create(fixtures::great_weapon_master()).await?; // General, STR prereq
        repo.create(fixtures::origin_feat()).await?; // Origin, choice ASI
        repo.create(fixtures::fighting_style_archery()).await?; // Fighting Style
        repo.create(fixtures::epic_boon()).await?; // Epic, level prereq
        
        // Get statistics
        let stats = repo.get_feat_statistics("dnd5e-2014").await?;
        assert_eq!(stats.total, 3); // Only 2014 feats
        assert_eq!(stats.general, 3);
        assert_eq!(stats.origin, 0); // Origin feat is 2024
        assert_eq!(stats.fighting_style, 0); // Fighting style is 2024
        assert_eq!(stats.epic, 0); // Epic is 2024
        assert_eq!(stats.with_prerequisites, 1); // Great Weapon Master
        assert_eq!(stats.with_ability_increases, 1); // Actor
        
        // Test 2024 stats
        let stats_2024 = repo.get_feat_statistics("dnd5e-2024").await?;
        assert_eq!(stats_2024.total, 3); // Origin, Fighting Style, Epic
        assert_eq!(stats_2024.origin, 1);
        assert_eq!(stats_2024.fighting_style, 1);
        assert_eq!(stats_2024.epic, 1);
        
        Ok(())
    }
}

// Prerequisite checking tests
mod prerequisite_tests {
    use super::*;

    #[test]
    fn test_ability_prerequisite_checking() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let feat = fixtures::great_weapon_master();
            let created = conn.create(feat)?;
            
            // Test with sufficient strength
            let mut abilities = HashMap::new();
            abilities.insert("str".to_string(), 15);
            abilities.insert("dex".to_string(), 12);
            assert!(created.meets_ability_prerequisites(&abilities)?);
            
            // Test with insufficient strength
            abilities.insert("str".to_string(), 10);
            assert!(!created.meets_ability_prerequisites(&abilities)?);
            
            Ok(())
        })
    }

    #[test]
    fn test_level_prerequisite_checking() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_rule_system_2024())?;
            conn.create(fixtures::create_source())?;
            conn.create(fixtures::create_source_2024())?;
            
            let feat = fixtures::epic_boon();
            let created = conn.create(feat)?;
            
            // Test with sufficient level
            assert!(created.meets_level_prerequisite(20)?);
            
            // Test with insufficient level
            assert!(!created.meets_level_prerequisite(19)?);
            
            Ok(())
        })
    }

    #[test]
    fn test_class_prerequisite_checking() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let feat = fixtures::eldritch_adept();
            let created = conn.create(feat)?;
            
            // Test with warlock class
            let character_classes = vec!["fighter".to_string(), "warlock".to_string()];
            assert!(created.meets_class_prerequisites(&character_classes)?);
            
            // Test without warlock class
            let character_classes = vec!["fighter".to_string(), "wizard".to_string()];
            assert!(!created.meets_class_prerequisites(&character_classes)?);
            
            Ok(())
        })
    }

    #[test]
    fn test_ability_increase_calculation() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_rule_system_2024())?;
            conn.create(fixtures::create_source())?;
            conn.create(fixtures::create_source_2024())?;
            
            // Test fixed increase
            let actor = fixtures::actor();
            let created_actor = conn.create(actor)?;
            assert_eq!(created_actor.total_ability_increase_points()?, 1);
            
            // Test choice increase
            let origin_feat = fixtures::origin_feat();
            let created_origin = conn.create(origin_feat)?;
            assert_eq!(created_origin.total_ability_increase_points()?, 1);
            
            // Test no increase
            let alert = fixtures::alert();
            let created_alert = conn.create(alert)?;
            assert_eq!(created_alert.total_ability_increase_points()?, 0);
            
            Ok(())
        })
    }
}

// Special features and type tests
mod special_features {
    use super::*;

    #[test]
    fn test_feat_type_helpers() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_rule_system_2024())?;
            conn.create(fixtures::create_source())?;
            conn.create(fixtures::create_source_2024())?;
            
            let origin = fixtures::origin_feat();
            let created_origin = conn.create(origin)?;
            assert!(created_origin.is_origin_feat());
            assert!(!created_origin.is_fighting_style());
            assert!(!created_origin.is_epic_feat());
            
            let fighting_style = fixtures::fighting_style_archery();
            let created_fighting_style = conn.create(fighting_style)?;
            assert!(!created_fighting_style.is_origin_feat());
            assert!(created_fighting_style.is_fighting_style());
            assert!(!created_fighting_style.is_epic_feat());
            
            let epic = fixtures::epic_boon();
            let created_epic = conn.create(epic)?;
            assert!(!created_epic.is_origin_feat());
            assert!(!created_epic.is_fighting_style());
            assert!(created_epic.is_epic_feat());
            
            Ok(())
        })
    }

    #[test]
    fn test_feat_type_enum_conversion() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let feat = fixtures::alert();
            let created = conn.create(feat)?;
            
            assert_eq!(created.feat_type_enum(), Some(FeatType::General));
            
            Ok(())
        })
    }

    #[test]
    fn test_prerequisites_and_increases_handling() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let feat = fixtures::great_weapon_master();
            let created = conn.create(feat)?;
            
            let prereqs = created.prerequisites_typed()?.unwrap();
            assert!(prereqs.abilities.is_some());
            assert_eq!(prereqs.abilities.unwrap().len(), 1);
            
            Ok(())
        })
    }

    #[test]
    fn test_ability_increases_handling() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let feat = fixtures::actor();
            let created = conn.create(feat)?;
            
            let increases = created.ability_increases_typed()?.unwrap();
            assert!(increases.fixed.is_some());
            assert_eq!(increases.fixed.unwrap().len(), 1);
            
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
            
            let feat = fixtures::alert();
            conn.create(feat.clone())?;
            
            // Try to create duplicate
            let result = conn.create(feat);
            assert!(result.is_err());
            assert!(result.unwrap_err().is_unique_violation());
            
            Ok(())
        })
    }

    #[test]
    fn test_foreign_key_constraints() -> Result<()> {
        with_test_db(|conn| {
            // Try to create feat without rule system
            let feat = Feat::new(
                "orphan-feat".to_string(),
                "Orphan Feat".to_string(),
                "nonexistent-rule-system".to_string(),
                "nonexistent-source".to_string(),
                json!([])
            )?;
            
            let result = conn.create(feat);
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
            let feat = fixtures::alert();
            let result = conn.update("nonexistent", feat);
            assert!(matches!(result, Err(DbError::NotFound { .. })));
            
            // Delete non-existent
            let result = Repository::<Feat>::delete(conn, "nonexistent");
            assert!(matches!(result, Err(DbError::NotFound { .. })));
            
            Ok(())
        })
    }
}