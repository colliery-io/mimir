//! Integration tests for creatures DAL

use crate::common::{with_test_db, TestDatabase};
use mimir_dm_db::*;
use mimir_dm_db::dal::creatures::CreatureRepository;
use mimir_dm_db::dal::traits::{AsyncRepository, Repository};
use mimir_dm_db::models::creatures::{
    Creature, HitPoints, Speed, AbilityScores, ArmorClass, CreatureAction, 
    LegendaryActions, CreatureSize
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
            "dnd5e-2014-mm".to_string(),
            "dnd5e-2014".to_string(),
            "Monster Manual".to_string(),
        )
    }

    pub fn goblin() -> Creature {
        Creature::new(
            "goblin".to_string(),
            "Goblin".to_string(),
            "dnd5e-2014".to_string(),
            "dnd5e-2014-mm".to_string(),
        ).unwrap()
        .with_page(166)
        .with_size(CreatureSize::Small)
        .with_type("humanoid".to_string())
        .with_type_tags(vec!["goblinoid".to_string()]).unwrap()
        .with_alignment(vec!["N".to_string(), "E".to_string()]).unwrap()
        .with_armor_class(vec![ArmorClass {
            ac: 15,
            from: Some(vec!["leather armor".to_string(), "shield".to_string()]),
            condition: None,
        }]).unwrap()
        .with_hit_points(HitPoints {
            average: 7,
            formula: "2d6".to_string(),
        }).unwrap()
        .with_speed(Speed {
            walk: Some(30),
            fly: None,
            swim: None,
            climb: None,
            burrow: None,
            hover: None,
        }).unwrap()
        .with_ability_scores(AbilityScores {
            str: 8,
            dex: 14,
            con: 10,
            int: 10,
            wis: 8,
            cha: 8,
        }).unwrap()
        .with_challenge_rating("1/4".to_string())
        .with_proficiency_bonus(2)
        .with_actions(vec![
            CreatureAction {
                name: "Scimitar".to_string(),
                entries: vec![json!("Melee Weapon Attack: +4 to hit, reach 5 ft., one target. Hit: 5 (1d6 + 2) slashing damage.")],
            },
            CreatureAction {
                name: "Shortbow".to_string(),
                entries: vec![json!("Ranged Weapon Attack: +4 to hit, range 80/320 ft., one target. Hit: 5 (1d6 + 2) piercing damage.")],
            },
        ]).unwrap()
        .with_entries(vec![json!("Small humanoid (goblinoid), neutral evil")])
        .unwrap()
    }

    pub fn ancient_red_dragon() -> Creature {
        Creature::new(
            "ancient-red-dragon".to_string(),
            "Ancient Red Dragon".to_string(),
            "dnd5e-2014".to_string(),
            "dnd5e-2014-mm".to_string(),
        ).unwrap()
        .with_page(98)
        .with_size(CreatureSize::Gargantuan)
        .with_type("dragon".to_string())
        .with_alignment(vec!["C".to_string(), "E".to_string()]).unwrap()
        .with_armor_class(vec![ArmorClass {
            ac: 22,
            from: Some(vec!["natural armor".to_string()]),
            condition: None,
        }]).unwrap()
        .with_hit_points(HitPoints {
            average: 546,
            formula: "28d20 + 252".to_string(),
        }).unwrap()
        .with_speed(Speed {
            walk: Some(40),
            fly: Some(80),
            swim: None,
            climb: Some(40),
            burrow: None,
            hover: None,
        }).unwrap()
        .with_ability_scores(AbilityScores {
            str: 30,
            dex: 10,
            con: 29,
            int: 18,
            wis: 15,
            cha: 23,
        }).unwrap()
        .with_challenge_rating("24".to_string())
        .with_proficiency_bonus(7)
        .with_legendary_actions(LegendaryActions {
            intro: Some(vec![json!("The dragon can take 3 legendary actions, choosing from the options below.")]),
            actions: vec![
                CreatureAction {
                    name: "Detect".to_string(),
                    entries: vec![json!("The dragon makes a Wisdom (Perception) check.")],
                },
                CreatureAction {
                    name: "Tail Attack".to_string(),
                    entries: vec![json!("The dragon makes a tail attack.")],
                },
            ],
        }).unwrap()
        .with_entries(vec![json!("Gargantuan dragon, chaotic evil")])
        .unwrap()
    }
}

// Basic CRUD operation tests (only Create and Read)
mod crud_operations {
    use super::*;

    #[test]
    fn test_create_creature() -> Result<()> {
        with_test_db(|conn| {
            // First create dependencies
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let creature = fixtures::goblin();
            let created = conn.create(creature)?;
            
            assert_eq!(created.id, "goblin");
            assert_eq!(created.name, "Goblin");
            assert_eq!(created.size, Some("S".to_string()));
            assert_eq!(created.creature_type, Some("humanoid".to_string()));
            assert_eq!(created.challenge_rating, Some("1/4".to_string()));
            
            Ok(())
        })
    }

    #[test]
    fn test_find_by_id() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let creature = fixtures::ancient_red_dragon();
            conn.create(creature)?;
            
            let found: Option<Creature> = conn.find_by_id("ancient-red-dragon")?;
            assert!(found.is_some());
            let c = found.unwrap();
            assert_eq!(c.name, "Ancient Red Dragon");
            assert_eq!(c.challenge_rating, Some("24".to_string()));
            assert!(c.has_legendary_actions());
            
            let not_found: Option<Creature> = conn.find_by_id("nonexistent")?;
            assert!(not_found.is_none());
            
            Ok(())
        })
    }

    #[test]
    fn test_list_creatures() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            conn.create(fixtures::goblin())?;
            conn.create(fixtures::ancient_red_dragon())?;
            
            let list: Vec<Creature> = conn.list()?;
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
        
        let repo = CreatureRepository::new(test_db.url.clone());
        
        // Create
        let creature = fixtures::goblin();
        let created = repo.create(creature).await?;
        assert_eq!(created.id, "goblin");
        
        // Find
        let found = repo.find_by_id("goblin").await?;
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
            
            let creature = fixtures::goblin();
            conn.create(creature.clone())?;
            
            // Try to create duplicate
            let result = conn.create(creature);
            assert!(result.is_err());
            assert!(result.unwrap_err().is_unique_violation());
            
            Ok(())
        })
    }

    #[test]
    fn test_foreign_key_constraints() -> Result<()> {
        with_test_db(|conn| {
            // Try to create creature without rule system
            let creature = Creature::new(
                "orphan-creature".to_string(),
                "Orphan Creature".to_string(),
                "nonexistent-rule-system".to_string(),
                "nonexistent-source".to_string(),
            )?;
            
            let result = conn.create(creature);
            assert!(result.is_err());
            assert!(result.unwrap_err().is_foreign_key_violation());
            
            Ok(())
        })
    }
}

// Creature model feature tests
mod model_features {
    use super::*;

    #[test]
    fn test_creature_properties() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let goblin = fixtures::goblin();
            let created_goblin = conn.create(goblin)?;
            
            assert_eq!(created_goblin.size_enum(), Some(CreatureSize::Small));
            assert_eq!(created_goblin.challenge_rating_numeric(), Some(0.25));
            assert!(!created_goblin.has_legendary_actions());
            assert!(!created_goblin.has_lair_actions());
            
            let dragon = fixtures::ancient_red_dragon();
            let created_dragon = conn.create(dragon)?;
            
            assert_eq!(created_dragon.size_enum(), Some(CreatureSize::Gargantuan));
            assert_eq!(created_dragon.challenge_rating_numeric(), Some(24.0));
            assert!(created_dragon.has_legendary_actions());
            assert!(!created_dragon.has_lair_actions());
            
            Ok(())
        })
    }

    #[test]
    fn test_json_field_parsing() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let goblin = fixtures::goblin();
            let created = conn.create(goblin)?;
            
            // Test ability scores parsing
            let ability_scores = created.ability_scores_typed()?.unwrap();
            assert_eq!(ability_scores.str, 8);
            assert_eq!(ability_scores.dex, 14);
            assert_eq!(ability_scores.con, 10);
            
            // Test hit points parsing
            let hp = created.hit_points_typed()?.unwrap();
            assert_eq!(hp.average, 7);
            assert_eq!(hp.formula, "2d6");
            
            // Test speed parsing
            let speed = created.speed_typed()?.unwrap();
            assert_eq!(speed.walk, Some(30));
            assert_eq!(speed.fly, None);
            
            // Test armor class parsing
            let ac = created.armor_class_typed()?.unwrap();
            assert_eq!(ac[0].ac, 15);
            assert_eq!(ac[0].from, Some(vec!["leather armor".to_string(), "shield".to_string()]));
            
            // Test actions parsing
            let actions = created.actions_typed()?.unwrap();
            assert_eq!(actions.len(), 2);
            assert_eq!(actions[0].name, "Scimitar");
            assert_eq!(actions[1].name, "Shortbow");
            
            // Test type tags parsing
            let type_tags = created.type_tags_vec()?.unwrap();
            assert_eq!(type_tags, vec!["goblinoid"]);
            
            // Test alignment parsing
            let alignment = created.alignment_vec()?.unwrap();
            assert_eq!(alignment, vec!["N", "E"]);
            
            Ok(())
        })
    }

    #[test]
    fn test_ability_modifier_calculation() -> Result<()> {
        // Test standard D&D ability modifier calculation
        assert_eq!(Creature::ability_modifier(8), -1);   // 8 -> -1
        assert_eq!(Creature::ability_modifier(10), 0);   // 10 -> 0  
        assert_eq!(Creature::ability_modifier(14), 2);   // 14 -> +2
        assert_eq!(Creature::ability_modifier(20), 5);   // 20 -> +5
        assert_eq!(Creature::ability_modifier(30), 10);  // 30 -> +10
        
        Ok(())
    }

    #[test]
    fn test_size_enum_conversion() -> Result<()> {
        assert_eq!(CreatureSize::Tiny.to_string(), "T");
        assert_eq!(CreatureSize::Small.to_string(), "S");
        assert_eq!(CreatureSize::Medium.to_string(), "M");
        assert_eq!(CreatureSize::Large.to_string(), "L");
        assert_eq!(CreatureSize::Huge.to_string(), "H");
        assert_eq!(CreatureSize::Gargantuan.to_string(), "G");
        
        assert_eq!("S".parse::<CreatureSize>().unwrap(), CreatureSize::Small);
        assert_eq!("LARGE".parse::<CreatureSize>().unwrap(), CreatureSize::Large);
        
        Ok(())
    }

    #[test] 
    fn test_challenge_rating_parsing() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            // Test fractional CR
            let goblin = fixtures::goblin(); // CR 1/4
            let created_goblin = conn.create(goblin)?;
            assert_eq!(created_goblin.challenge_rating_numeric(), Some(0.25));
            
            // Test whole number CR
            let dragon = fixtures::ancient_red_dragon(); // CR 24
            let created_dragon = conn.create(dragon)?;
            assert_eq!(created_dragon.challenge_rating_numeric(), Some(24.0));
            
            Ok(())
        })
    }
}