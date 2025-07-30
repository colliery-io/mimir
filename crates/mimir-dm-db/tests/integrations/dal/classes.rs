//! Integration tests for classes DAL

use crate::common::{with_test_db, TestDatabase};
use mimir_dm_db::*;
use mimir_dm_db::dal::classes::ClassRepository;
use mimir_dm_db::dal::traits::{AsyncRepository, Repository};
use mimir_dm_db::models::classes::{ClassFeature, StartingEquipment, StartingProficiencies, SkillChoice};
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

    pub fn fighter_class() -> Class {
        Class::new(
            "fighter".to_string(),
            "Fighter".to_string(),
            "dnd5e-2014".to_string(),
            "dnd5e-2014-phb".to_string(),
            "class".to_string(),
            json!([
                {"type": "entries", "name": "Fighting Style", "entries": ["You adopt a particular style of fighting as your specialty."]},
                {"type": "entries", "name": "Second Wind", "entries": ["You have a limited well of stamina that you can draw on to protect yourself from harm."]}
            ])
        ).unwrap()
        .with_page(70)
        .with_hit_die(10)
        .with_primary_abilities(vec!["str".to_string(), "dex".to_string()]).unwrap()
        .with_saving_throws(vec!["str".to_string(), "con".to_string()]).unwrap()
        .with_skill_proficiency_count(2)
        .with_skill_proficiency_choices(vec!["Acrobatics".to_string(), "Animal Handling".to_string(), "Athletics".to_string()]).unwrap()
        .with_starting_proficiencies(StartingProficiencies {
            armor: Some(vec!["light armor".to_string(), "medium armor".to_string(), "heavy armor".to_string(), "shields".to_string()]),
            weapons: Some(vec!["simple weapons".to_string(), "martial weapons".to_string()]),
            tools: None,
            saving_throws: Some(vec!["Strength".to_string(), "Constitution".to_string()]),
            skills: Some(SkillChoice {
                choose: Some(2),
                from: vec!["Acrobatics".to_string(), "Animal Handling".to_string(), "Athletics".to_string()],
            }),
        }).unwrap()
    }

    pub fn wizard_class() -> Class {
        Class::new(
            "wizard".to_string(),
            "Wizard".to_string(),
            "dnd5e-2014".to_string(),
            "dnd5e-2014-phb".to_string(),
            "class".to_string(),
            json!([
                {"type": "entries", "name": "Spellcasting", "entries": ["As a student of arcane magic, you have a spellbook containing spells that show the first glimmerings of your true power."]},
                {"type": "entries", "name": "Arcane Recovery", "entries": ["You have learned to regain some of your magical energy by studying your spellbook."]}
            ])
        ).unwrap()
        .with_page(106)
        .with_hit_die(6)
        .with_primary_abilities(vec!["int".to_string()]).unwrap()
        .with_saving_throws(vec!["int".to_string(), "wis".to_string()]).unwrap()
        .with_skill_proficiency_count(2)
        .with_spell_ability("int".to_string())
        .with_caster_progression("full".to_string())
    }

    pub fn champion_subclass() -> Class {
        Class::new(
            "fighter-champion".to_string(),
            "Champion".to_string(),
            "dnd5e-2014".to_string(),
            "dnd5e-2014-phb".to_string(),
            "subclass".to_string(),
            json!([
                {"type": "entries", "name": "Improved Critical", "entries": ["Beginning when you choose this archetype at 3rd level, your weapon attacks score a critical hit on a roll of 19 or 20."]}
            ])
        ).unwrap()
        .with_parent_class("fighter".to_string())
        .with_page(72)
        .with_hit_die(10)  // Inherited from parent
        .with_subclass_title("Martial Archetype".to_string())
        .with_subclass_level(3)
        .with_features(vec![
            ClassFeature {
                name: "Improved Critical".to_string(),
                level: 3,
                entries: vec![json!("Beginning when you choose this archetype at 3rd level, your weapon attacks score a critical hit on a roll of 19 or 20.")],
            }
        ]).unwrap()
    }

    pub fn evoker_subclass() -> Class {
        Class::new(
            "wizard-evoker".to_string(),
            "School of Evocation".to_string(),
            "dnd5e-2014".to_string(),
            "dnd5e-2014-phb".to_string(),
            "subclass".to_string(),
            json!([
                {"type": "entries", "name": "Evocation Savant", "entries": ["Beginning when you select this school at 2nd level, the gold and time you must spend to copy an evocation spell into your spellbook is halved."]}
            ])
        ).unwrap()
        .with_parent_class("wizard".to_string())
        .with_page(117)
        .with_hit_die(6)  // Inherited from parent
        .with_spell_ability("int".to_string())  // Inherited from parent
        .with_caster_progression("full".to_string())  // Inherited from parent
        .with_subclass_title("Arcane Tradition".to_string())
        .with_subclass_level(2)
    }

    pub fn warlock_class() -> Class {
        Class::new(
            "warlock".to_string(),
            "Warlock".to_string(),
            "dnd5e-2014".to_string(),
            "dnd5e-2014-phb".to_string(),
            "class".to_string(),
            json!([
                {"type": "entries", "name": "Otherworldly Patron", "entries": ["At 1st level, you have struck a pact with an otherworldly being of your choice."]}
            ])
        ).unwrap()
        .with_page(105)
        .with_hit_die(8)
        .with_primary_abilities(vec!["cha".to_string()]).unwrap()
        .with_saving_throws(vec!["wis".to_string(), "cha".to_string()]).unwrap()
        .with_spell_ability("cha".to_string())
        .with_caster_progression("pact".to_string())
    }
}

// CRUD operation tests
mod crud_operations {
    use super::*;

    #[test]
    fn test_create_class() -> Result<()> {
        with_test_db(|conn| {
            // First create dependencies
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let class = fixtures::fighter_class();
            let created = conn.create(class)?;
            
            assert_eq!(created.id, "fighter");
            assert_eq!(created.name, "Fighter");
            assert_eq!(created.class_type, "class");
            assert!(created.parent_class_id.is_none());
            assert_eq!(created.hit_die, Some(10));
            assert!(created.is_base_class());
            assert!(!created.is_spellcaster());
            
            Ok(())
        })
    }

    #[test]
    fn test_find_by_id() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let class = fixtures::wizard_class();
            conn.create(class)?;
            
            let found: Option<Class> = conn.find_by_id("wizard")?;
            assert!(found.is_some());
            let wizard = found.unwrap();
            assert_eq!(wizard.name, "Wizard");
            assert!(wizard.is_spellcaster());
            assert_eq!(wizard.caster_type(), "full");
            
            let not_found: Option<Class> = conn.find_by_id("nonexistent")?;
            assert!(not_found.is_none());
            
            Ok(())
        })
    }

    #[test]
    fn test_update_class() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let class = fixtures::warlock_class();
            conn.create(class)?;
            
            let mut updated = fixtures::warlock_class();
            updated.name = "Warlock (Updated)".to_string();
            
            let result = conn.update("warlock", updated)?;
            assert_eq!(result.name, "Warlock (Updated)");
            
            Ok(())
        })
    }

    #[test]
    fn test_delete_class() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let class = Class::new(
                "test-class".to_string(),
                "Test Class".to_string(),
                "dnd5e-2014".to_string(),
                "dnd5e-2014-phb".to_string(),
                "class".to_string(),
                json!([])
            )?;
            conn.create(class)?;
            
            Repository::<Class>::delete(conn, "test-class")?;
            
            let found: Option<Class> = conn.find_by_id("test-class")?;
            assert!(found.is_none());
            
            // Try to delete non-existent
            let result = Repository::<Class>::delete(conn, "nonexistent");
            assert!(matches!(result, Err(DbError::NotFound { .. })));
            
            Ok(())
        })
    }

    #[test]
    fn test_list_classes() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            conn.create(fixtures::fighter_class())?;
            conn.create(fixtures::wizard_class())?;
            conn.create(fixtures::warlock_class())?;
            
            let list: Vec<Class> = conn.list()?;
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
        
        let repo = ClassRepository::new(test_db.url.clone());
        
        // Create
        let class = fixtures::fighter_class();
        let created = repo.create(class).await?;
        assert_eq!(created.id, "fighter");
        
        // Find
        let found = repo.find_by_id("fighter").await?;
        assert!(found.is_some());
        
        // Update
        let mut updated = found.unwrap();
        updated.name = "Fighter (Updated)".to_string();
        let result = repo.update("fighter", updated).await?;
        assert_eq!(result.name, "Fighter (Updated)");
        
        // List
        let list = repo.list().await?;
        assert_eq!(list.len(), 1);
        
        // Delete
        repo.delete("fighter").await?;
        let deleted = repo.find_by_id("fighter").await?;
        assert!(deleted.is_none());
        
        Ok(())
    }

    #[tokio::test]
    async fn test_find_base_classes_and_subclasses() -> Result<()> {
        let test_db = TestDatabase::file_based()?;
        
        // Create dependencies
        let rule_repo = dal::rule_systems::RuleSystemRepository::new(test_db.url.clone());
        rule_repo.create(fixtures::create_rule_system()).await?;
        
        let source_repo = dal::sources::SourceRepository::new(test_db.url.clone());
        source_repo.create(fixtures::create_source()).await?;
        
        let repo = ClassRepository::new(test_db.url.clone());
        
        // Create base classes and subclasses
        repo.create(fixtures::fighter_class()).await?;
        repo.create(fixtures::champion_subclass()).await?;
        repo.create(fixtures::wizard_class()).await?;
        repo.create(fixtures::evoker_subclass()).await?;
        
        // Find base classes
        let base_classes = repo.find_base_classes().await?;
        assert_eq!(base_classes.len(), 2);
        assert!(base_classes.iter().all(|c| c.is_base_class()));
        
        // Find subclasses
        let fighter_subclasses = repo.find_subclasses("fighter").await?;
        assert_eq!(fighter_subclasses.len(), 1);
        assert!(fighter_subclasses.iter().all(|c| c.is_subclass()));
        
        let wizard_subclasses = repo.find_subclasses("wizard").await?;
        assert_eq!(wizard_subclasses.len(), 1);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_find_spellcasters() -> Result<()> {
        let test_db = TestDatabase::file_based()?;
        
        // Create dependencies
        let rule_repo = dal::rule_systems::RuleSystemRepository::new(test_db.url.clone());
        rule_repo.create(fixtures::create_rule_system()).await?;
        
        let source_repo = dal::sources::SourceRepository::new(test_db.url.clone());
        source_repo.create(fixtures::create_source()).await?;
        
        let repo = ClassRepository::new(test_db.url.clone());
        
        // Create mix of spellcasters and non-spellcasters
        repo.create(fixtures::fighter_class()).await?; // Non-spellcaster
        repo.create(fixtures::wizard_class()).await?;  // Full caster
        repo.create(fixtures::warlock_class()).await?; // Pact caster
        
        // Find spellcasters
        let spellcasters = repo.find_spellcasters().await?;
        assert_eq!(spellcasters.len(), 2);
        assert!(spellcasters.iter().all(|c| c.is_spellcaster()));
        
        Ok(())
    }

    #[tokio::test]
    async fn test_find_by_caster_progression() -> Result<()> {
        let test_db = TestDatabase::file_based()?;
        
        // Create dependencies
        let rule_repo = dal::rule_systems::RuleSystemRepository::new(test_db.url.clone());
        rule_repo.create(fixtures::create_rule_system()).await?;
        
        let source_repo = dal::sources::SourceRepository::new(test_db.url.clone());
        source_repo.create(fixtures::create_source()).await?;
        
        let repo = ClassRepository::new(test_db.url.clone());
        
        // Create classes with different caster progressions
        repo.create(fixtures::wizard_class()).await?;  // Full caster
        repo.create(fixtures::warlock_class()).await?; // Pact caster
        
        // Find by caster progression
        let full_casters = repo.find_by_caster_progression("full").await?;
        assert_eq!(full_casters.len(), 1);
        assert_eq!(full_casters[0].id, "wizard");
        
        let pact_casters = repo.find_by_caster_progression("pact").await?;
        assert_eq!(pact_casters.len(), 1);
        assert_eq!(pact_casters[0].id, "warlock");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_find_by_hit_die() -> Result<()> {
        let test_db = TestDatabase::file_based()?;
        
        // Create dependencies
        let rule_repo = dal::rule_systems::RuleSystemRepository::new(test_db.url.clone());
        rule_repo.create(fixtures::create_rule_system()).await?;
        
        let source_repo = dal::sources::SourceRepository::new(test_db.url.clone());
        source_repo.create(fixtures::create_source()).await?;
        
        let repo = ClassRepository::new(test_db.url.clone());
        
        // Create classes with different hit dice
        repo.create(fixtures::fighter_class()).await?; // d10
        repo.create(fixtures::wizard_class()).await?;  // d6
        repo.create(fixtures::warlock_class()).await?; // d8
        
        // Find by hit die
        let d10_classes = repo.find_by_hit_die(10).await?;
        assert_eq!(d10_classes.len(), 1);
        assert_eq!(d10_classes[0].id, "fighter");
        
        let d6_classes = repo.find_by_hit_die(6).await?;
        assert_eq!(d6_classes.len(), 1);
        assert_eq!(d6_classes[0].id, "wizard");
        
        Ok(())
    }
}

// Metadata and special features tests
mod special_features {
    use super::*;

    #[test]
    fn test_primary_abilities_handling() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let class = fixtures::fighter_class();
            let created = conn.create(class)?;
            
            let abilities = created.primary_abilities_vec()?.unwrap();
            assert_eq!(abilities.len(), 2);
            assert!(abilities.contains(&"str".to_string()));
            assert!(abilities.contains(&"dex".to_string()));
            
            Ok(())
        })
    }

    #[test]
    fn test_saving_throws_handling() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let class = fixtures::wizard_class();
            let created = conn.create(class)?;
            
            let saves = created.saving_throws_vec()?.unwrap();
            assert_eq!(saves.len(), 2);
            assert!(saves.contains(&"int".to_string()));
            assert!(saves.contains(&"wis".to_string()));
            
            Ok(())
        })
    }

    #[test]
    fn test_starting_proficiencies_handling() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let class = fixtures::fighter_class();
            let created = conn.create(class)?;
            
            let profs = created.starting_proficiencies_typed()?.unwrap();
            assert!(profs.armor.is_some());
            assert!(profs.weapons.is_some());
            assert!(profs.skills.is_some());
            
            let skill_choice = profs.skills.unwrap();
            assert_eq!(skill_choice.choose, Some(2));
            assert_eq!(skill_choice.from.len(), 3);
            
            Ok(())
        })
    }

    #[test]
    fn test_features_handling() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            // First create the parent class
            conn.create(fixtures::fighter_class())?;
            
            let class = fixtures::champion_subclass();
            let created = conn.create(class)?;
            
            let features = created.features_vec()?.unwrap();
            assert_eq!(features.len(), 1);
            assert_eq!(features[0].name, "Improved Critical");
            assert_eq!(features[0].level, 3);
            
            Ok(())
        })
    }

    #[test]
    fn test_entries_handling() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let class = fixtures::fighter_class();
            let created = conn.create(class)?;
            
            let entries = created.entries_value()?;
            assert!(entries.is_array());
            assert_eq!(entries.as_array().unwrap().len(), 2);
            
            Ok(())
        })
    }

    #[test]
    fn test_class_type_helpers() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let base_class = fixtures::fighter_class();
            let created_base = conn.create(base_class)?;
            assert!(created_base.is_base_class());
            assert!(!created_base.is_subclass());
            
            let subclass = fixtures::champion_subclass();
            let created_sub = conn.create(subclass)?;
            assert!(!created_sub.is_base_class());
            assert!(created_sub.is_subclass());
            
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
            
            let class = fixtures::fighter_class();
            conn.create(class.clone())?;
            
            // Try to create duplicate
            let result = conn.create(class);
            assert!(result.is_err());
            assert!(result.unwrap_err().is_unique_violation());
            
            Ok(())
        })
    }

    #[test]
    fn test_foreign_key_constraints() -> Result<()> {
        with_test_db(|conn| {
            // Try to create class without rule system
            let class = Class::new(
                "orphan-class".to_string(),
                "Orphan Class".to_string(),
                "nonexistent-rule-system".to_string(),
                "nonexistent-source".to_string(),
                "class".to_string(),
                json!([])
            )?;
            
            let result = conn.create(class);
            assert!(result.is_err());
            assert!(result.unwrap_err().is_foreign_key_violation());
            
            Ok(())
        })
    }

    #[test]
    fn test_parent_class_constraint() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            // Try to create subclass with non-existent parent
            let subclass = Class::new(
                "orphan-subclass".to_string(),
                "Orphan Subclass".to_string(),
                "dnd5e-2014".to_string(),
                "dnd5e-2014-phb".to_string(),
                "subclass".to_string(),
                json!([])
            )?
            .with_parent_class("nonexistent-parent".to_string());
            
            let result = conn.create(subclass);
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
            let class = fixtures::fighter_class();
            let result = conn.update("nonexistent", class);
            assert!(matches!(result, Err(DbError::NotFound { .. })));
            
            // Delete non-existent
            let result = Repository::<Class>::delete(conn, "nonexistent");
            assert!(matches!(result, Err(DbError::NotFound { .. })));
            
            Ok(())
        })
    }
}