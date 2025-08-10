//! Integration tests for backgrounds DAL

use crate::common::{with_test_db, TestDatabase};
use mimir_dm_db::*;
use mimir_dm_db::dal::backgrounds::BackgroundRepository;
use mimir_dm_db::dal::traits::{AsyncRepository, Repository};
use mimir_dm_db::models::backgrounds::{
    Background, SkillProficiency, LanguageProficiency, 
    ToolProficiency, StartingEquipment, EquipmentItem, CurrencyAmount
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

    pub fn acolyte() -> Background {
        Background::new(
            "acolyte".to_string(),
            "Acolyte".to_string(),
            "dnd5e-2014".to_string(),
            "dnd5e-2014-phb".to_string(),
            json!([
                {"type": "entries", "entries": ["You have spent your life in the service of a temple to a specific god or pantheon of gods."]}
            ])
        ).unwrap()
        .with_page(127)
        .with_skill_proficiencies(vec![
            SkillProficiency {
                skill: "Insight".to_string(),
                choice: None,
            },
            SkillProficiency {
                skill: "Religion".to_string(),
                choice: None,
            },
        ]).unwrap()
        .with_language_proficiencies(vec![
            LanguageProficiency {
                language: None,
                choice: Some(mimir_dm_db::models::backgrounds::LanguageChoice {
                    from: vec!["Any".to_string()],
                    count: 2,
                }),
            },
        ]).unwrap()
        .with_feature("Shelter of the Faithful".to_string(), 
                     "You and your companions can expect to receive free healing and care at a temple, shrine, or other established presence of your faith.".to_string())
    }

    pub fn criminal() -> Background {
        Background::new(
            "criminal".to_string(),
            "Criminal".to_string(),
            "dnd5e-2014".to_string(),
            "dnd5e-2014-phb".to_string(),
            json!([
                {"type": "entries", "entries": ["You are an experienced criminal with a history of breaking the law."]}
            ])
        ).unwrap()
        .with_page(129)
        .with_skill_proficiencies(vec![
            SkillProficiency {
                skill: "Deception".to_string(),
                choice: None,
            },
            SkillProficiency {
                skill: "Stealth".to_string(),
                choice: None,
            },
        ]).unwrap()
        .with_tool_proficiencies(vec![
            ToolProficiency {
                tool: Some("Thieves' Tools".to_string()),
                choice: None,
            },
            ToolProficiency {
                tool: None,
                choice: Some(mimir_dm_db::models::backgrounds::ToolChoice {
                    from: vec!["Gaming Set".to_string()],
                    count: 1,
                }),
            },
        ]).unwrap()
        .with_feature("Criminal Contact".to_string(),
                     "You have a reliable and trustworthy contact who acts as your liaison to a network of other criminals.".to_string())
    }

    pub fn folk_hero() -> Background {
        Background::new(
            "folk-hero".to_string(),
            "Folk Hero".to_string(),
            "dnd5e-2014".to_string(),
            "dnd5e-2014-phb".to_string(),
            json!([
                {"type": "entries", "entries": ["You come from a humble social rank, but you are destined for so much more."]}
            ])
        ).unwrap()
        .with_page(131)
        .with_skill_proficiencies(vec![
            SkillProficiency {
                skill: "Animal Handling".to_string(),
                choice: None,
            },
            SkillProficiency {
                skill: "Survival".to_string(),
                choice: None,
            },
        ]).unwrap()
        .with_tool_proficiencies(vec![
            ToolProficiency {
                tool: None,
                choice: Some(mimir_dm_db::models::backgrounds::ToolChoice {
                    from: vec!["Artisan's Tools".to_string()],
                    count: 1,
                }),
            },
        ]).unwrap()
        .with_starting_equipment(StartingEquipment {
            items: vec![
                EquipmentItem {
                    item: "Smith's Tools".to_string(),
                    quantity: 1,
                },
                EquipmentItem {
                    item: "Set of Artisan's Clothes".to_string(),
                    quantity: 1,
                },
            ],
            choices: None,
            gold: Some(CurrencyAmount {
                amount: 10,
                unit: "gp".to_string(),
            }),
        }).unwrap()
        .with_feature("Rustic Hospitality".to_string(),
                     "Since you come from the ranks of the common folk, you fit in among them with ease.".to_string())
    }

    pub fn scholar() -> Background {
        Background::new(
            "scholar".to_string(),
            "Scholar".to_string(),
            "dnd5e-2014".to_string(),
            "dnd5e-2014-phb".to_string(),
            json!([
                {"type": "entries", "entries": ["You spent years learning the lore of the multiverse."]}
            ])
        ).unwrap()
        .with_page(137)
        .with_skill_proficiencies(vec![
            SkillProficiency {
                skill: "Arcana".to_string(),
                choice: None,
            },
            SkillProficiency {
                skill: "History".to_string(),
                choice: None,
            },
        ]).unwrap()
        .with_language_proficiencies(vec![
            LanguageProficiency {
                language: None,
                choice: Some(mimir_dm_db::models::backgrounds::LanguageChoice {
                    from: vec!["Any".to_string()],
                    count: 2,
                }),
            },
        ]).unwrap()
        .with_feature("Researcher".to_string(),
                     "When you attempt to learn or recall a piece of lore, if you do not know that information, you often know where and from whom you can obtain it.".to_string())
    }
}

// CRUD operation tests
mod crud_operations {
    use super::*;

    #[test]
    fn test_create_background() -> Result<()> {
        with_test_db(|conn| {
            // First create dependencies
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let background = fixtures::acolyte();
            let created = conn.create(background)?;
            
            assert_eq!(created.id, "acolyte");
            assert_eq!(created.name, "Acolyte");
            assert_eq!(created.page, Some(127));
            assert!(created.has_feature());
            assert!(created.grants_skill_proficiencies());
            assert!(created.grants_language_proficiencies());
            
            Ok(())
        })
    }

    #[test]
    fn test_find_by_id() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let background = fixtures::criminal();
            conn.create(background)?;
            
            let found: Option<Background> = conn.find_by_id("criminal")?;
            assert!(found.is_some());
            let bg = found.unwrap();
            assert_eq!(bg.name, "Criminal");
            assert!(bg.grants_skill_proficiencies());
            assert!(bg.grants_tool_proficiencies());
            
            let not_found: Option<Background> = conn.find_by_id("nonexistent")?;
            assert!(not_found.is_none());
            
            Ok(())
        })
    }

    #[test]
    fn test_update_background() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let background = fixtures::scholar();
            conn.create(background)?;
            
            let mut updated = fixtures::scholar();
            updated.name = "Scholar (Updated)".to_string();
            
            let result = conn.update("scholar", updated)?;
            assert_eq!(result.name, "Scholar (Updated)");
            
            Ok(())
        })
    }

    #[test]
    fn test_delete_background() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let background = Background::new(
                "test-background".to_string(),
                "Test Background".to_string(),
                "dnd5e-2014".to_string(),
                "dnd5e-2014-phb".to_string(),
                json!([])
            )?;
            conn.create(background)?;
            
            Repository::<Background>::delete(conn, "test-background")?;
            
            let found: Option<Background> = conn.find_by_id("test-background")?;
            assert!(found.is_none());
            
            // Try to delete non-existent
            let result = Repository::<Background>::delete(conn, "nonexistent");
            assert!(matches!(result, Err(DbError::NotFound { .. })));
            
            Ok(())
        })
    }

    #[test]
    fn test_list_backgrounds() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            conn.create(fixtures::acolyte())?;
            conn.create(fixtures::criminal())?;
            conn.create(fixtures::folk_hero())?;
            
            let list: Vec<Background> = conn.list()?;
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
        
        let repo = BackgroundRepository::new(test_db.url.clone());
        
        // Create
        let background = fixtures::acolyte();
        let created = repo.create(background).await?;
        assert_eq!(created.id, "acolyte");
        
        // Find
        let found = repo.find_by_id("acolyte").await?;
        assert!(found.is_some());
        
        // Update
        let mut updated = found.unwrap();
        updated.name = "Acolyte (Updated)".to_string();
        let result = repo.update("acolyte", updated).await?;
        assert_eq!(result.name, "Acolyte (Updated)");
        
        // List
        let list = repo.list().await?;
        assert_eq!(list.len(), 1);
        
        // Delete
        repo.delete("acolyte").await?;
        let deleted = repo.find_by_id("acolyte").await?;
        assert!(deleted.is_none());
        
        Ok(())
    }

    #[tokio::test]
    async fn test_find_with_proficiencies() -> Result<()> {
        let test_db = TestDatabase::file_based()?;
        
        // Create dependencies
        let rule_repo = dal::rule_systems::RuleSystemRepository::new(test_db.url.clone());
        rule_repo.create(fixtures::create_rule_system()).await?;
        
        let source_repo = dal::sources::SourceRepository::new(test_db.url.clone());
        source_repo.create(fixtures::create_source()).await?;
        
        let repo = BackgroundRepository::new(test_db.url.clone());
        
        // Create backgrounds with different proficiencies
        repo.create(fixtures::acolyte()).await?; // Skills + Languages
        repo.create(fixtures::criminal()).await?; // Skills + Tools
        repo.create(fixtures::folk_hero()).await?; // Skills + Tools + Equipment
        repo.create(fixtures::scholar()).await?; // Skills + Languages
        
        // Find by skill proficiencies
        let with_skills = repo.find_with_skill_proficiencies().await?;
        assert_eq!(with_skills.len(), 4);
        
        // Find by language proficiencies
        let with_languages = repo.find_with_language_proficiencies().await?;
        assert_eq!(with_languages.len(), 2); // acolyte, scholar
        
        // Find by tool proficiencies
        let with_tools = repo.find_with_tool_proficiencies().await?;
        assert_eq!(with_tools.len(), 2); // criminal, folk_hero
        
        // Find with starting equipment
        let with_equipment = repo.find_with_starting_equipment().await?;
        assert_eq!(with_equipment.len(), 1); // folk_hero
        
        Ok(())
    }

    #[tokio::test]
    async fn test_search_functionality() -> Result<()> {
        let test_db = TestDatabase::file_based()?;
        
        // Create dependencies
        let rule_repo = dal::rule_systems::RuleSystemRepository::new(test_db.url.clone());
        rule_repo.create(fixtures::create_rule_system()).await?;
        
        let source_repo = dal::sources::SourceRepository::new(test_db.url.clone());
        source_repo.create(fixtures::create_source()).await?;
        
        let repo = BackgroundRepository::new(test_db.url.clone());
        
        // Create backgrounds
        repo.create(fixtures::acolyte()).await?;
        repo.create(fixtures::criminal()).await?;
        repo.create(fixtures::folk_hero()).await?;
        repo.create(fixtures::scholar()).await?;
        
        // Search by name
        let search_results = repo.search_by_name("aco").await?;
        assert_eq!(search_results.len(), 1);
        assert_eq!(search_results[0].name, "Acolyte");
        
        // Search by feature text
        let contact_search = repo.search_by_feature_text("contact").await?;
        assert_eq!(contact_search.len(), 1);
        assert_eq!(contact_search[0].name, "Criminal");
        
        // Find by feature name
        let researcher = repo.find_by_feature_name("Researcher").await?;
        assert_eq!(researcher.len(), 1);
        assert_eq!(researcher[0].name, "Scholar");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_find_by_rule_system_and_source() -> Result<()> {
        let test_db = TestDatabase::file_based()?;
        
        // Create dependencies
        let rule_repo = dal::rule_systems::RuleSystemRepository::new(test_db.url.clone());
        rule_repo.create(fixtures::create_rule_system()).await?;
        
        let source_repo = dal::sources::SourceRepository::new(test_db.url.clone());
        source_repo.create(fixtures::create_source()).await?;
        
        let repo = BackgroundRepository::new(test_db.url.clone());
        
        // Create backgrounds
        repo.create(fixtures::acolyte()).await?;
        repo.create(fixtures::criminal()).await?;
        
        // Find by rule system
        let by_rule_system = repo.find_by_rule_system("dnd5e-2014").await?;
        assert_eq!(by_rule_system.len(), 2);
        
        // Find by source
        let by_source = repo.find_by_source("dnd5e-2014-phb").await?;
        assert_eq!(by_source.len(), 2);
        
        Ok(())
    }
}

// Metadata and special features tests
mod special_features {
    use super::*;

    #[test]
    fn test_skill_proficiencies_handling() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let background = fixtures::acolyte();
            let created = conn.create(background)?;
            
            let skills = created.skill_proficiencies_typed()?.unwrap();
            assert_eq!(skills.len(), 2);
            assert_eq!(skills[0].skill, "Insight");
            assert_eq!(skills[1].skill, "Religion");
            
            Ok(())
        })
    }

    #[test]
    fn test_tool_proficiencies_handling() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let background = fixtures::criminal();
            let created = conn.create(background)?;
            
            let tools = created.tool_proficiencies_typed()?.unwrap();
            assert_eq!(tools.len(), 2);
            assert_eq!(tools[0].tool, Some("Thieves' Tools".to_string()));
            assert!(tools[1].choice.is_some());
            
            Ok(())
        })
    }

    #[test]
    fn test_starting_equipment_handling() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let background = fixtures::folk_hero();
            let created = conn.create(background)?;
            
            let equipment = created.starting_equipment_typed()?.unwrap();
            assert_eq!(equipment.items.len(), 2);
            assert_eq!(equipment.items[0].item, "Smith's Tools");
            assert_eq!(equipment.gold.as_ref().unwrap().amount, 10);
            
            Ok(())
        })
    }

    #[test]
    fn test_feature_handling() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let background = fixtures::scholar();
            let created = conn.create(background)?;
            
            assert!(created.has_feature());
            let (name, text) = created.feature().unwrap();
            assert_eq!(name, "Researcher");
            assert!(text.contains("recall a piece of lore"));
            
            Ok(())
        })
    }

    #[test]
    fn test_proficiency_helpers() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let acolyte = fixtures::acolyte();
            let created_acolyte = conn.create(acolyte)?;
            assert!(created_acolyte.grants_skill_proficiencies());
            assert!(created_acolyte.grants_language_proficiencies());
            assert!(!created_acolyte.grants_tool_proficiencies());
            assert!(!created_acolyte.provides_starting_equipment());
            
            let folk_hero = fixtures::folk_hero();
            let created_folk_hero = conn.create(folk_hero)?;
            assert!(created_folk_hero.grants_skill_proficiencies());
            assert!(!created_folk_hero.grants_language_proficiencies());
            assert!(created_folk_hero.grants_tool_proficiencies());
            assert!(created_folk_hero.provides_starting_equipment());
            
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
            
            let background = fixtures::acolyte();
            conn.create(background.clone())?;
            
            // Try to create duplicate
            let result = conn.create(background);
            assert!(result.is_err());
            assert!(result.unwrap_err().is_unique_violation());
            
            Ok(())
        })
    }

    #[test]
    fn test_foreign_key_constraints() -> Result<()> {
        with_test_db(|conn| {
            // Try to create background without rule system
            let background = Background::new(
                "orphan-background".to_string(),
                "Orphan Background".to_string(),
                "nonexistent-rule-system".to_string(),
                "nonexistent-source".to_string(),
                json!([])
            )?;
            
            let result = conn.create(background);
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
            let background = fixtures::acolyte();
            let result = conn.update("nonexistent", background);
            assert!(matches!(result, Err(DbError::NotFound { .. })));
            
            // Delete non-existent
            let result = Repository::<Background>::delete(conn, "nonexistent");
            assert!(matches!(result, Err(DbError::NotFound { .. })));
            
            Ok(())
        })
    }
}