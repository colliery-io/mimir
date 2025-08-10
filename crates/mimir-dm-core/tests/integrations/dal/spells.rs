//! Integration tests for spells DAL

use crate::common::{with_test_db, TestDatabase};
use mimir_dm_db::*;
use mimir_dm_db::dal::spells::SpellRepository;
use mimir_dm_db::dal::traits::{AsyncRepository, Repository};
use mimir_dm_db::models::spells::{
    Spell, CastingTime, Range, Distance, Components, Duration,
    UpcastInfo, SpellSchool
};
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

    pub fn fire_bolt() -> Spell {
        Spell::new(
            "fire-bolt".to_string(),
            "Fire Bolt".to_string(),
            "dnd5e-2014".to_string(),
            "dnd5e-2014-phb".to_string(),
            json!([
                {"type": "entries", "entries": ["You hurl a mote of fire at a creature or object within range."]}
            ])
        ).unwrap()
        .with_page(242)
        .with_level(0)
        .with_school(SpellSchool::Evocation)
        .with_casting_time(CastingTime {
            number: 1,
            unit: "action".to_string(),
            condition: None,
        }).unwrap()
        .with_range(Range {
            range_type: "point".to_string(),
            distance: Some(Distance {
                distance_type: "feet".to_string(),
                amount: Some(120),
            }),
        }).unwrap()
        .with_components(Components {
            v: Some(true),
            s: Some(true),
            m: None,
        }).unwrap()
        .with_duration(Duration {
            duration_type: "instant".to_string(),
            duration: None,
            concentration: Some(false),
            ends: None,
        }).unwrap()
        .with_damage_type(vec!["fire".to_string()]).unwrap()
        .with_classes(vec!["sorcerer".to_string(), "wizard".to_string()]).unwrap()
    }

    pub fn healing_word() -> Spell {
        Spell::new(
            "healing-word".to_string(),
            "Healing Word".to_string(),
            "dnd5e-2014".to_string(),
            "dnd5e-2014-phb".to_string(),
            json!([
                {"type": "entries", "entries": ["A creature of your choice that you can see within range regains hit points."]}
            ])
        ).unwrap()
        .with_page(250)
        .with_level(1)
        .with_school(SpellSchool::Evocation)
        .with_casting_time(CastingTime {
            number: 1,
            unit: "bonus action".to_string(),
            condition: None,
        }).unwrap()
        .with_range(Range {
            range_type: "point".to_string(),
            distance: Some(Distance {
                distance_type: "feet".to_string(),
                amount: Some(60),
            }),
        }).unwrap()
        .with_components(Components {
            v: Some(true),
            s: None,
            m: None,
        }).unwrap()
        .with_duration(Duration {
            duration_type: "instant".to_string(),
            duration: None,
            concentration: Some(false),
            ends: None,
        }).unwrap()
        .with_upcast_info(UpcastInfo {
            entries: vec![json!({"type": "entries", "entries": ["When you cast this spell using a spell slot of 2nd level or higher, the healing increases by 1d4 for each slot level above 1st."]})],
        }).unwrap()
        .with_classes(vec!["bard".to_string(), "cleric".to_string(), "druid".to_string()]).unwrap()
    }
}

// Basic CRUD operation tests (only Create and Read)
mod crud_operations {
    use super::*;

    #[test]
    fn test_create_spell() -> Result<()> {
        with_test_db(|conn| {
            // First create dependencies
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let spell = fixtures::fire_bolt();
            let created = conn.create(spell)?;
            
            assert_eq!(created.id, "fire-bolt");
            assert_eq!(created.name, "Fire Bolt");
            assert_eq!(created.level, Some(0));
            assert_eq!(created.school, Some("V".to_string()));
            assert!(created.is_cantrip());
            assert!(!created.can_be_ritual());
            assert!(!created.requires_concentration());
            
            Ok(())
        })
    }

    #[test]
    fn test_find_by_id() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let spell = fixtures::healing_word();
            conn.create(spell)?;
            
            let found: Option<Spell> = conn.find_by_id("healing-word")?;
            assert!(found.is_some());
            let s = found.unwrap();
            assert_eq!(s.name, "Healing Word");
            assert_eq!(s.level, Some(1));
            assert!(s.has_upcast_effects());
            
            let not_found: Option<Spell> = conn.find_by_id("nonexistent")?;
            assert!(not_found.is_none());
            
            Ok(())
        })
    }

    #[test]
    fn test_list_spells() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            conn.create(fixtures::fire_bolt())?;
            conn.create(fixtures::healing_word())?;
            
            let list: Vec<Spell> = conn.list()?;
            assert_eq!(list.len(), 2);
            
            Ok(())
        })
    }
}

// Async repository tests (only Create and Read)
mod async_operations {
    use super::*;

    #[tokio::test]
    async fn test_async_create_and_read() -> Result<()> {
        // Create an isolated test database
        let test_db = TestDatabase::file_based()?;
        
        // First create dependencies
        let rule_repo = dal::rule_systems::RuleSystemRepository::new(test_db.url.clone());
        rule_repo.create(fixtures::create_rule_system()).await?;
        
        let source_repo = dal::sources::SourceRepository::new(test_db.url.clone());
        source_repo.create(fixtures::create_source()).await?;
        
        let repo = SpellRepository::new(test_db.url.clone());
        
        // Create
        let spell = fixtures::fire_bolt();
        let created = repo.create(spell).await?;
        assert_eq!(created.id, "fire-bolt");
        
        // Find
        let found = repo.find_by_id("fire-bolt").await?;
        assert!(found.is_some());
        
        // List
        let list = repo.list().await?;
        assert_eq!(list.len(), 1);
        
        Ok(())
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
            
            let spell = fixtures::fire_bolt();
            conn.create(spell.clone())?;
            
            // Try to create duplicate
            let result = conn.create(spell);
            assert!(result.is_err());
            assert!(result.unwrap_err().is_unique_violation());
            
            Ok(())
        })
    }

    #[test]
    fn test_foreign_key_constraints() -> Result<()> {
        with_test_db(|conn| {
            // Try to create spell without rule system
            let spell = Spell::new(
                "orphan-spell".to_string(),
                "Orphan Spell".to_string(),
                "nonexistent-rule-system".to_string(),
                "nonexistent-source".to_string(),
                json!([])
            )?;
            
            let result = conn.create(spell);
            assert!(result.is_err());
            assert!(result.unwrap_err().is_foreign_key_violation());
            
            Ok(())
        })
    }
}

// Spell model feature tests
mod model_features {
    use super::*;

    #[test]
    fn test_spell_properties() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let fire_bolt = fixtures::fire_bolt();
            let created_fire_bolt = conn.create(fire_bolt)?;
            
            assert!(created_fire_bolt.is_cantrip());
            assert!(!created_fire_bolt.can_be_ritual());
            assert!(!created_fire_bolt.requires_concentration());
            assert!(created_fire_bolt.deals_damage());
            assert!(!created_fire_bolt.allows_saving_throw());
            assert!(!created_fire_bolt.has_upcast_effects());
            
            let healing_word = fixtures::healing_word();
            let created_healing_word = conn.create(healing_word)?;
            
            assert!(!created_healing_word.is_cantrip());
            assert!(!created_healing_word.can_be_ritual());
            assert!(!created_healing_word.requires_concentration());
            assert!(!created_healing_word.deals_damage());
            assert!(!created_healing_word.allows_saving_throw());
            assert!(created_healing_word.has_upcast_effects());
            
            Ok(())
        })
    }

    #[test]
    fn test_class_compatibility() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let fire_bolt = fixtures::fire_bolt();
            let created = conn.create(fire_bolt)?;
            
            assert!(created.can_be_cast_by_class("wizard")?);
            assert!(created.can_be_cast_by_class("sorcerer")?);
            assert!(!created.can_be_cast_by_class("cleric")?);
            assert!(!created.can_be_cast_by_class("bard")?);
            
            Ok(())
        })
    }

    #[test]
    fn test_spell_school_handling() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let fire_bolt = fixtures::fire_bolt();
            let created = conn.create(fire_bolt)?;
            
            assert_eq!(created.school_enum(), Some(SpellSchool::Evocation));
            assert_eq!(created.school, Some("V".to_string()));
            
            Ok(())
        })
    }
}