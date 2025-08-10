//! Integration tests for items DAL

use crate::common::{with_test_db, TestDatabase};
use mimir_dm_db::*;
use mimir_dm_db::dal::items::ItemRepository;
use mimir_dm_db::dal::traits::{AsyncRepository, Repository};
use mimir_dm_db::models::items::{AttunementPrereq, Damage, NewItem};
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

    pub fn longsword() -> Item {
        // Create NewItem first, then convert to Item for the tests
        let new_item = NewItem::new(
            "longsword".to_string(),
            "Longsword".to_string(),
            "dnd5e-2014".to_string(),
            "dnd5e-2014-phb".to_string(),
            json!([
                {"type": "entries", "entries": ["A longsword is a versatile, one-handed melee weapon in the sword category."]}
            ])
        ).unwrap()
        .with_page(149)
        .with_type("martial weapon".to_string())
        .with_weight(3.0)
        .with_value(1500) // 15 gp in copper
        .with_damage(Damage {
            dice: "1d8".to_string(),
            damage_type: "slashing".to_string(),
        }).unwrap()
        .with_properties(vec!["versatile".to_string()]).unwrap();
        
        // Convert NewItem to Item for test usage
        Item {
            id: new_item.id,
            name: new_item.name,
            rule_system_id: new_item.rule_system_id,
            source_id: new_item.source_id,
            page: new_item.page,
            base_item_id: new_item.base_item_id,
            item_type: new_item.item_type,
            weight_lb: new_item.weight_lb,
            value_cp: new_item.value_cp,
            armor_class: new_item.armor_class,
            damage: new_item.damage,
            properties: new_item.properties,
            rarity: new_item.rarity.clone(),
            requires_attunement: new_item.requires_attunement,
            attunement_prereq: new_item.attunement_prereq,
            magic_bonus: new_item.magic_bonus,
            additional_properties: new_item.additional_properties,
            entries: new_item.entries,
            is_magic: new_item.rarity.is_some(), // Generated column value
            created_at: new_item.created_at,
            updated_at: new_item.updated_at,
        }
    }

    pub fn chain_mail() -> Item {
        let new_item = NewItem::new(
            "chain-mail".to_string(),
            "Chain Mail".to_string(),
            "dnd5e-2014".to_string(),
            "dnd5e-2014-phb".to_string(),
            json!([
                {"type": "entries", "entries": ["Made of interlocking metal rings, chain mail includes a layer of quilted fabric worn underneath the mail to prevent chafing."]}
            ])
        ).unwrap()
        .with_page(144)
        .with_type("heavy armor".to_string())
        .with_weight(55.0)
        .with_value(7500) // 75 gp in copper
        .with_armor_class(16);
        
        // Convert NewItem to Item for test usage
        Item {
            id: new_item.id,
            name: new_item.name,
            rule_system_id: new_item.rule_system_id,
            source_id: new_item.source_id,
            page: new_item.page,
            base_item_id: new_item.base_item_id,
            item_type: new_item.item_type,
            weight_lb: new_item.weight_lb,
            value_cp: new_item.value_cp,
            armor_class: new_item.armor_class,
            damage: new_item.damage,
            properties: new_item.properties,
            rarity: new_item.rarity.clone(),
            requires_attunement: new_item.requires_attunement,
            attunement_prereq: new_item.attunement_prereq,
            magic_bonus: new_item.magic_bonus,
            additional_properties: new_item.additional_properties,
            entries: new_item.entries,
            is_magic: new_item.rarity.is_some(), // Generated column value
            created_at: new_item.created_at,
            updated_at: new_item.updated_at,
        }
    }

    pub fn flame_tongue_sword() -> Item {
        let new_item = NewItem::new(
            "flame-tongue-longsword".to_string(),
            "Flame Tongue Longsword".to_string(),
            "dnd5e-2014".to_string(),
            "dnd5e-2014-phb".to_string(),
            json!([
                {"type": "entries", "entries": ["You can use a bonus action to speak this magic sword's command word, causing flames to erupt from the blade."]}
            ])
        ).unwrap()
        .with_base_item("longsword".to_string())
        .with_page(170)
        .with_type("martial weapon".to_string())
        .with_weight(3.0)
        .with_damage(Damage {
            dice: "1d8".to_string(),
            damage_type: "slashing".to_string(),
        }).unwrap()
        .with_properties(vec!["versatile".to_string()]).unwrap()
        .with_rarity("rare".to_string())
        .with_attunement(true)
        .with_magic_bonus(1);
        
        // Convert NewItem to Item for test usage
        Item {
            id: new_item.id,
            name: new_item.name,
            rule_system_id: new_item.rule_system_id,
            source_id: new_item.source_id,
            page: new_item.page,
            base_item_id: new_item.base_item_id,
            item_type: new_item.item_type,
            weight_lb: new_item.weight_lb,
            value_cp: new_item.value_cp,
            armor_class: new_item.armor_class,
            damage: new_item.damage,
            properties: new_item.properties,
            rarity: new_item.rarity.clone(),
            requires_attunement: new_item.requires_attunement,
            attunement_prereq: new_item.attunement_prereq,
            magic_bonus: new_item.magic_bonus,
            additional_properties: new_item.additional_properties,
            entries: new_item.entries,
            is_magic: new_item.rarity.is_some(), // Generated column value
            created_at: new_item.created_at,
            updated_at: new_item.updated_at,
        }
    }

    pub fn cloak_of_protection() -> Item {
        let new_item = NewItem::new(
            "cloak-of-protection".to_string(),
            "Cloak of Protection".to_string(),
            "dnd5e-2014".to_string(),
            "dnd5e-2014-phb".to_string(),
            json!([
                {"type": "entries", "entries": ["You gain a +1 bonus to AC and saving throws while you wear this cloak."]}
            ])
        ).unwrap()
        .with_page(159)
        .with_type("wondrous item".to_string())
        .with_weight(1.0)
        .with_rarity("uncommon".to_string())
        .with_attunement(true);
        
        // Convert NewItem to Item for test usage
        Item {
            id: new_item.id,
            name: new_item.name,
            rule_system_id: new_item.rule_system_id,
            source_id: new_item.source_id,
            page: new_item.page,
            base_item_id: new_item.base_item_id,
            item_type: new_item.item_type,
            weight_lb: new_item.weight_lb,
            value_cp: new_item.value_cp,
            armor_class: new_item.armor_class,
            damage: new_item.damage,
            properties: new_item.properties,
            rarity: new_item.rarity.clone(),
            requires_attunement: new_item.requires_attunement,
            attunement_prereq: new_item.attunement_prereq,
            magic_bonus: new_item.magic_bonus,
            additional_properties: new_item.additional_properties,
            entries: new_item.entries,
            is_magic: new_item.rarity.is_some(), // Generated column value
            created_at: new_item.created_at,
            updated_at: new_item.updated_at,
        }
    }

    pub fn staff_of_power() -> Item {
        let new_item = NewItem::new(
            "staff-of-power".to_string(),
            "Staff of Power".to_string(),
            "dnd5e-2014".to_string(),
            "dnd5e-2014-phb".to_string(),
            json!([
                {"type": "entries", "entries": ["This staff can be wielded as a magic quarterstaff that grants a +2 bonus to attack and damage rolls made with it."]}
            ])
        ).unwrap()
        .with_page(202)
        .with_type("staff".to_string())
        .with_weight(4.0)
        .with_rarity("very rare".to_string())
        .with_attunement(true)
        .with_attunement_prereq(AttunementPrereq {
            class: Some("sorcerer, warlock, or wizard".to_string()),
            race: None,
            alignment: None,
            other: None,
        }).unwrap()
        .with_magic_bonus(2);
        
        // Convert NewItem to Item for test usage
        Item {
            id: new_item.id,
            name: new_item.name,
            rule_system_id: new_item.rule_system_id,
            source_id: new_item.source_id,
            page: new_item.page,
            base_item_id: new_item.base_item_id,
            item_type: new_item.item_type,
            weight_lb: new_item.weight_lb,
            value_cp: new_item.value_cp,
            armor_class: new_item.armor_class,
            damage: new_item.damage,
            properties: new_item.properties,
            rarity: new_item.rarity.clone(),
            requires_attunement: new_item.requires_attunement,
            attunement_prereq: new_item.attunement_prereq,
            magic_bonus: new_item.magic_bonus,
            additional_properties: new_item.additional_properties,
            entries: new_item.entries,
            is_magic: new_item.rarity.is_some(), // Generated column value
            created_at: new_item.created_at,
            updated_at: new_item.updated_at,
        }
    }
}

// CRUD operation tests
mod crud_operations {
    use super::*;

    #[test]
    fn test_create_item() -> Result<()> {
        with_test_db(|conn| {
            // First create dependencies
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let item = fixtures::longsword();
            let created = conn.create(item)?;
            
            assert_eq!(created.id, "longsword");
            assert_eq!(created.name, "Longsword");
            assert_eq!(created.item_type, Some("martial weapon".to_string()));
            assert_eq!(created.weight_lb, Some(3.0));
            assert_eq!(created.value_cp, Some(1500));
            assert!(!created.is_magic);
            assert!(created.is_base_item());
            
            Ok(())
        })
    }

    #[test]
    fn test_find_by_id() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let item = fixtures::chain_mail();
            conn.create(item)?;
            
            let found: Option<Item> = conn.find_by_id("chain-mail")?;
            assert!(found.is_some());
            let armor = found.unwrap();
            assert_eq!(armor.name, "Chain Mail");
            assert!(armor.is_armor());
            assert_eq!(armor.armor_class, Some(16));
            
            let not_found: Option<Item> = conn.find_by_id("nonexistent")?;
            assert!(not_found.is_none());
            
            Ok(())
        })
    }

    #[test]
    fn test_update_item() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let item = fixtures::cloak_of_protection();
            conn.create(item)?;
            
            let mut updated = fixtures::cloak_of_protection();
            updated.name = "Cloak of Protection (Updated)".to_string();
            
            let result = conn.update("cloak-of-protection", updated)?;
            assert_eq!(result.name, "Cloak of Protection (Updated)");
            
            Ok(())
        })
    }

    #[test]
    fn test_delete_item() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let new_item = NewItem::new(
                "test-item".to_string(),
                "Test Item".to_string(),
                "dnd5e-2014".to_string(),
                "dnd5e-2014-phb".to_string(),
                json!([])
            )?;
            let item = Item {
                id: new_item.id.clone(),
                name: new_item.name.clone(),
                rule_system_id: new_item.rule_system_id.clone(),
                source_id: new_item.source_id.clone(),
                page: new_item.page,
                base_item_id: new_item.base_item_id.clone(),
                item_type: new_item.item_type.clone(),
                weight_lb: new_item.weight_lb,
                value_cp: new_item.value_cp,
                armor_class: new_item.armor_class,
                damage: new_item.damage.clone(),
                properties: new_item.properties.clone(),
                rarity: new_item.rarity.clone(),
                requires_attunement: new_item.requires_attunement,
                attunement_prereq: new_item.attunement_prereq.clone(),
                magic_bonus: new_item.magic_bonus,
                additional_properties: new_item.additional_properties.clone(),
                entries: new_item.entries.clone(),
                is_magic: new_item.rarity.clone().is_some(),
                created_at: new_item.created_at,
                updated_at: new_item.updated_at,
            };
            conn.create(item)?;
            
            Repository::<Item>::delete(conn, "test-item")?;
            
            let found: Option<Item> = conn.find_by_id("test-item")?;
            assert!(found.is_none());
            
            // Try to delete non-existent
            let result = Repository::<Item>::delete(conn, "nonexistent");
            assert!(matches!(result, Err(DbError::NotFound { .. })));
            
            Ok(())
        })
    }

    #[test]
    fn test_list_items() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            conn.create(fixtures::longsword())?;
            conn.create(fixtures::chain_mail())?;
            conn.create(fixtures::cloak_of_protection())?;
            
            let list: Vec<Item> = conn.list()?;
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
        
        let repo = ItemRepository::new(test_db.url.clone());
        
        // Create
        let item = fixtures::longsword();
        let created = repo.create(item).await?;
        assert_eq!(created.id, "longsword");
        
        // Find
        let found = repo.find_by_id("longsword").await?;
        assert!(found.is_some());
        
        // Update
        let mut updated = found.unwrap();
        updated.name = "Longsword (Updated)".to_string();
        let result = repo.update("longsword", updated).await?;
        assert_eq!(result.name, "Longsword (Updated)");
        
        // List
        let list = repo.list().await?;
        assert_eq!(list.len(), 1);
        
        // Delete
        repo.delete("longsword").await?;
        let deleted = repo.find_by_id("longsword").await?;
        assert!(deleted.is_none());
        
        Ok(())
    }

    #[tokio::test]
    async fn test_find_magic_vs_mundane() -> Result<()> {
        let test_db = TestDatabase::file_based()?;
        
        // Create dependencies
        let rule_repo = dal::rule_systems::RuleSystemRepository::new(test_db.url.clone());
        rule_repo.create(fixtures::create_rule_system()).await?;
        
        let source_repo = dal::sources::SourceRepository::new(test_db.url.clone());
        source_repo.create(fixtures::create_source()).await?;
        
        let repo = ItemRepository::new(test_db.url.clone());
        
        // Create mix of magic and mundane items
        repo.create(fixtures::longsword()).await?; // Mundane
        repo.create(fixtures::chain_mail()).await?; // Mundane
        repo.create(fixtures::cloak_of_protection()).await?; // Magic
        repo.create(fixtures::staff_of_power()).await?; // Magic
        
        // Find magic items
        let magic = repo.find_magic_items().await?;
        assert_eq!(magic.len(), 2);
        assert!(magic.iter().all(|i| i.is_magic));
        
        // Find mundane items
        let mundane = repo.find_mundane_items().await?;
        assert_eq!(mundane.len(), 2);
        assert!(mundane.iter().all(|i| !i.is_magic));
        
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
        
        let repo = ItemRepository::new(test_db.url.clone());
        
        // Create items of different types
        repo.create(fixtures::longsword()).await?; // martial weapon
        repo.create(fixtures::chain_mail()).await?; // heavy armor
        repo.create(fixtures::cloak_of_protection()).await?; // wondrous item
        
        // Find by type
        let weapons = repo.find_by_type("martial weapon").await?;
        assert_eq!(weapons.len(), 1);
        assert_eq!(weapons[0].id, "longsword");
        
        let armor = repo.find_by_type("heavy armor").await?;
        assert_eq!(armor.len(), 1);
        assert_eq!(armor[0].id, "chain-mail");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_find_attunement_items() -> Result<()> {
        let test_db = TestDatabase::file_based()?;
        
        // Create dependencies
        let rule_repo = dal::rule_systems::RuleSystemRepository::new(test_db.url.clone());
        rule_repo.create(fixtures::create_rule_system()).await?;
        
        let source_repo = dal::sources::SourceRepository::new(test_db.url.clone());
        source_repo.create(fixtures::create_source()).await?;
        
        let repo = ItemRepository::new(test_db.url.clone());
        
        // Create mix of attunement and non-attunement items
        repo.create(fixtures::longsword()).await?; // No attunement
        repo.create(fixtures::cloak_of_protection()).await?; // Requires attunement
        repo.create(fixtures::staff_of_power()).await?; // Requires attunement
        
        // Find attunement items
        let attunement = repo.find_attunement_items().await?;
        assert_eq!(attunement.len(), 2);
        assert!(attunement.iter().all(|i| i.needs_attunement()));
        
        Ok(())
    }

    #[tokio::test]
    async fn test_find_base_items_and_variants() -> Result<()> {
        let test_db = TestDatabase::file_based()?;
        
        // Create dependencies
        let rule_repo = dal::rule_systems::RuleSystemRepository::new(test_db.url.clone());
        rule_repo.create(fixtures::create_rule_system()).await?;
        
        let source_repo = dal::sources::SourceRepository::new(test_db.url.clone());
        source_repo.create(fixtures::create_source()).await?;
        
        let repo = ItemRepository::new(test_db.url.clone());
        
        // Create base item and variant
        repo.create(fixtures::longsword()).await?; // Base item
        repo.create(fixtures::flame_tongue_sword()).await?; // Variant
        repo.create(fixtures::chain_mail()).await?; // Another base item
        
        // Find base items
        let base_items = repo.find_base_items().await?;
        assert_eq!(base_items.len(), 2);
        assert!(base_items.iter().all(|i| i.is_base_item()));
        
        // Find variants of longsword
        let variants = repo.find_variants("longsword").await?;
        assert_eq!(variants.len(), 1);
        assert!(variants.iter().all(|i| i.is_variant()));
        assert_eq!(variants[0].id, "flame-tongue-longsword");
        
        Ok(())
    }
}

// Metadata and special features tests
mod special_features {
    use super::*;

    #[test]
    fn test_damage_handling() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let item = fixtures::longsword();
            let created = conn.create(item)?;
            
            let damage = created.damage_typed()?.unwrap();
            assert_eq!(damage.dice, "1d8");
            assert_eq!(damage.damage_type, "slashing");
            
            Ok(())
        })
    }

    #[test]
    fn test_properties_handling() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let item = fixtures::longsword();
            let created = conn.create(item)?;
            
            let properties = created.properties_vec()?.unwrap();
            assert_eq!(properties.len(), 1);
            assert!(properties.contains(&"versatile".to_string()));
            
            Ok(())
        })
    }

    #[test]
    fn test_attunement_prereq_handling() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let item = fixtures::staff_of_power();
            let created = conn.create(item)?;
            
            let prereq = created.attunement_prereq_typed()?.unwrap();
            assert_eq!(prereq.class, Some("sorcerer, warlock, or wizard".to_string()));
            assert_eq!(prereq.race, None);
            
            Ok(())
        })
    }

    #[test]
    fn test_item_type_helpers() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let weapon = fixtures::longsword();
            let created_weapon = conn.create(weapon)?;
            assert!(created_weapon.is_weapon());
            assert!(!created_weapon.is_armor());
            
            let armor = fixtures::chain_mail();
            let created_armor = conn.create(armor)?;
            assert!(!created_armor.is_weapon());
            assert!(created_armor.is_armor());
            
            Ok(())
        })
    }

    #[test]
    fn test_value_conversion() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            let item = fixtures::longsword();
            let created = conn.create(item)?;
            
            // 1500 cp = 15 gp
            assert_eq!(created.value_gp(), Some(15.0));
            
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
            
            let item = fixtures::longsword();
            conn.create(item.clone())?;
            
            // Try to create duplicate
            let result = conn.create(item);
            assert!(result.is_err());
            assert!(result.unwrap_err().is_unique_violation());
            
            Ok(())
        })
    }

    #[test]
    fn test_foreign_key_constraints() -> Result<()> {
        with_test_db(|conn| {
            // Try to create item without rule system
            let new_item = NewItem::new(
                "orphan-item".to_string(),
                "Orphan Item".to_string(),
                "nonexistent-rule-system".to_string(),
                "nonexistent-source".to_string(),
                json!([])
            )?;
            let item = Item {
                id: new_item.id.clone(),
                name: new_item.name.clone(),
                rule_system_id: new_item.rule_system_id.clone(),
                source_id: new_item.source_id.clone(),
                page: new_item.page,
                base_item_id: new_item.base_item_id.clone(),
                item_type: new_item.item_type.clone(),
                weight_lb: new_item.weight_lb,
                value_cp: new_item.value_cp,
                armor_class: new_item.armor_class,
                damage: new_item.damage.clone(),
                properties: new_item.properties.clone(),
                rarity: new_item.rarity.clone(),
                requires_attunement: new_item.requires_attunement,
                attunement_prereq: new_item.attunement_prereq.clone(),
                magic_bonus: new_item.magic_bonus,
                additional_properties: new_item.additional_properties.clone(),
                entries: new_item.entries.clone(),
                is_magic: new_item.rarity.clone().is_some(),
                created_at: new_item.created_at,
                updated_at: new_item.updated_at,
            };
            
            let result = conn.create(item);
            assert!(result.is_err());
            assert!(result.unwrap_err().is_foreign_key_violation());
            
            Ok(())
        })
    }

    #[test]
    fn test_base_item_constraint() -> Result<()> {
        with_test_db(|conn| {
            conn.create(fixtures::create_rule_system())?;
            conn.create(fixtures::create_source())?;
            
            // Try to create variant without base item
            let new_variant = NewItem::new(
                "orphan-variant".to_string(),
                "Orphan Variant".to_string(),
                "dnd5e-2014".to_string(),
                "dnd5e-2014-phb".to_string(),
                json!([])
            )?
            .with_base_item("nonexistent-base".to_string());
            let variant = Item {
                id: new_variant.id.clone(),
                name: new_variant.name.clone(),
                rule_system_id: new_variant.rule_system_id.clone(),
                source_id: new_variant.source_id.clone(),
                page: new_variant.page,
                base_item_id: new_variant.base_item_id.clone(),
                item_type: new_variant.item_type.clone(),
                weight_lb: new_variant.weight_lb,
                value_cp: new_variant.value_cp,
                armor_class: new_variant.armor_class,
                damage: new_variant.damage.clone(),
                properties: new_variant.properties.clone(),
                rarity: new_variant.rarity.clone(),
                requires_attunement: new_variant.requires_attunement,
                attunement_prereq: new_variant.attunement_prereq.clone(),
                magic_bonus: new_variant.magic_bonus,
                additional_properties: new_variant.additional_properties.clone(),
                entries: new_variant.entries.clone(),
                is_magic: new_variant.rarity.clone().is_some(),
                created_at: new_variant.created_at,
                updated_at: new_variant.updated_at,
            };
            
            let result = conn.create(variant);
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
            let item = fixtures::longsword();
            let result = conn.update("nonexistent", item);
            assert!(matches!(result, Err(DbError::NotFound { .. })));
            
            // Delete non-existent
            let result = Repository::<Item>::delete(conn, "nonexistent");
            assert!(matches!(result, Err(DbError::NotFound { .. })));
            
            Ok(())
        })
    }
}