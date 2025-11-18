//! Integration tests for character LLM tools
//!
//! Tests the tool layer that wraps CharacterService

#[cfg(test)]
mod tests {
    use crate::services::tools::character_tools::*;
    use crate::services::tools::character_write_tools::*;
    use mimir_dm_core::{DatabaseService, run_migrations};
    use mimir_dm_core::services::{CharacterService, PlayerService};
    use mimir_dm_core::services::character::creation::{CharacterBuilder, AbilityScoreMethod};
    use mimir_dm_llm::ToolTrait;
    use serde_json::json;
    use std::sync::Arc;
    use tempfile::TempDir;

    fn setup_test_db() -> (Arc<DatabaseService>, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db_service = DatabaseService::new(db_path.to_str().unwrap(), false)
            .expect("Failed to create database service");

        // Run migrations
        let mut conn = db_service.get_connection().unwrap();
        run_migrations(&mut *conn).unwrap();

        // Seed test data
        seed_test_catalog_data(&mut conn);

        (Arc::new(db_service), temp_dir)
    }

    fn seed_test_catalog_data(conn: &mut diesel::SqliteConnection) {
        use diesel::prelude::*;

        // Insert Wizard class
        diesel::sql_query(
            "INSERT INTO catalog_classes (name, source, hit_dice, full_class_json) VALUES (?, ?, ?, ?)"
        )
        .bind::<diesel::sql_types::Text, _>("Wizard")
        .bind::<diesel::sql_types::Text, _>("PHB")
        .bind::<diesel::sql_types::Text, _>("d6")
        .bind::<diesel::sql_types::Text, _>(r#"{"name":"Wizard","source":"PHB","hd":{"number":1,"faces":6},"casterProgression":"full","spellcastingAbility":"int","classTableGroups":[{"colLabels":["1st","2nd","3rd"],"rowsSpellProgression":[[2,0,0],[3,0,0],[4,2,0]]}]}"#)
        .execute(conn)
        .ok();

        // Insert Human race
        diesel::sql_query(
            "INSERT INTO catalog_races (name, source, size, speed, full_race_json) VALUES (?, ?, ?, ?, ?)"
        )
        .bind::<diesel::sql_types::Text, _>("Human")
        .bind::<diesel::sql_types::Text, _>("PHB")
        .bind::<diesel::sql_types::Text, _>("M")
        .bind::<diesel::sql_types::Integer, _>(30)
        .bind::<diesel::sql_types::Text, _>(r#"{"name":"Human","source":"PHB","size":["M"],"speed":30,"ability":[{"str":1,"dex":1,"con":1,"int":1,"wis":1,"cha":1}]}"#)
        .execute(conn)
        .ok();

        // Insert Sage background
        diesel::sql_query(
            "INSERT INTO catalog_backgrounds (name, skills, languages, tools, feature, source, full_background_json) VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind::<diesel::sql_types::Text, _>("Sage")
        .bind::<diesel::sql_types::Text, _>("Arcana, History")
        .bind::<diesel::sql_types::Text, _>("")
        .bind::<diesel::sql_types::Text, _>("")
        .bind::<diesel::sql_types::Text, _>("Researcher")
        .bind::<diesel::sql_types::Text, _>("PHB")
        .bind::<diesel::sql_types::Text, _>(r#"{"name":"Sage","source":"PHB","skillProficiencies":["Arcana","History"]}"#)
        .execute(conn)
        .ok();
    }

    fn create_test_campaign(db_service: &Arc<DatabaseService>, temp_dir: &TempDir) -> i32 {
        use mimir_dm_core::dal::campaign::campaigns::CampaignRepository;
        use mimir_dm_core::models::campaign::NewCampaign;

        let mut conn = db_service.get_connection().unwrap();
        let mut campaign_repo = CampaignRepository::new(&mut conn);
        let campaign = campaign_repo
            .create(NewCampaign {
                name: "Test Campaign".to_string(),
                directory_path: temp_dir.path().to_str().unwrap().to_string(),
                status: "concept".to_string(),
            })
            .unwrap();

        campaign.id
    }

    fn create_test_player(db_service: &Arc<DatabaseService>) -> i32 {
        let mut conn = db_service.get_connection().unwrap();
        let mut player_service = PlayerService::new(&mut conn);
        let player = player_service
            .create_player("Test Player", None, None)
            .unwrap();
        player.id
    }

    fn create_test_character(db_service: &Arc<DatabaseService>, campaign_id: i32, player_id: i32, temp_dir: &TempDir) -> i32 {
        let mut conn = db_service.get_connection().unwrap();

        let mut char_data = CharacterBuilder::new(&mut conn)
            .set_identity("Gandalf".to_string(), player_id)
            .set_race("Human", "PHB", None).unwrap()
            .set_class("Wizard", "PHB", None).unwrap()
            .set_background("Sage", "PHB").unwrap()
            .set_ability_scores(AbilityScoreMethod::Manual {
                strength: 10,
                dexterity: 12,
                constitution: 14,
                intelligence: 16,
                wisdom: 13,
                charisma: 8,
            }).unwrap()
            .build().unwrap();

        // Calculate and set spell slots for the Wizard
        use mimir_dm_core::services::character::calculate_spell_slots;
        let spell_slots = calculate_spell_slots(&mut conn, &char_data).unwrap();
        char_data.spells.spell_slots = spell_slots;

        let mut char_service = CharacterService::new(&mut conn);
        let character = char_service
            .create_character(campaign_id, player_id, temp_dir.path().to_str().unwrap(), char_data)
            .unwrap();

        character.id
    }

    #[tokio::test]
    async fn test_get_character_tool() {
        let (db_service, temp_dir) = setup_test_db();
        let campaign_id = create_test_campaign(&db_service, &temp_dir);
        let player_id = create_test_player(&db_service);
        let character_id = create_test_character(&db_service, campaign_id, player_id, &temp_dir);

        let tool = GetCharacterTool::new(Arc::clone(&db_service));

        // Test successful retrieval
        let arguments = json!({
            "character_id": character_id
        });

        let result = tool.execute(arguments).await;
        assert!(result.is_ok());

        let response: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(response["character_name"], "Gandalf");
        assert_eq!(response["level"], 1);
        assert_eq!(response["race"], "Human");
        assert_eq!(response["class"], "Wizard");
        assert_eq!(response["abilities"]["intelligence"]["score"], 17); // 16 + 1 from Human

        // Test missing character
        let bad_arguments = json!({
            "character_id": 99999
        });

        let result = tool.execute(bad_arguments).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_campaign_characters_tool() {
        let (db_service, temp_dir) = setup_test_db();
        let campaign_id = create_test_campaign(&db_service, &temp_dir);
        let player_id = create_test_player(&db_service);

        // Create multiple characters
        for name in &["Frodo", "Sam", "Merry"] {
            let mut conn = db_service.get_connection().unwrap();
            let char_data = CharacterBuilder::new(&mut conn)
                .set_identity(name.to_string(), player_id)
                .set_race("Human", "PHB", None).unwrap()
                .set_class("Wizard", "PHB", None).unwrap()
                .set_background("Sage", "PHB").unwrap()
                .set_ability_scores(AbilityScoreMethod::Manual {
                    strength: 10, dexterity: 12, constitution: 14,
                    intelligence: 10, wisdom: 12, charisma: 10,
                }).unwrap()
                .build().unwrap();

            let mut char_service = CharacterService::new(&mut conn);
            char_service.create_character(campaign_id, player_id, temp_dir.path().to_str().unwrap(), char_data).unwrap();
        }

        let tool = ListCampaignCharactersTool::new(Arc::clone(&db_service));

        let arguments = json!({
            "campaign_id": campaign_id
        });

        let result = tool.execute(arguments).await;
        assert!(result.is_ok());

        let response: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(response["character_count"], 3);
        assert!(response["characters"].is_array());
        assert_eq!(response["characters"].as_array().unwrap().len(), 3);
    }

    #[tokio::test]
    async fn test_get_character_stats_tool() {
        let (db_service, temp_dir) = setup_test_db();
        let campaign_id = create_test_campaign(&db_service, &temp_dir);
        let player_id = create_test_player(&db_service);
        let character_id = create_test_character(&db_service, campaign_id, player_id, &temp_dir);

        let tool = GetCharacterStatsTool::new(Arc::clone(&db_service));

        let arguments = json!({
            "character_id": character_id
        });

        let result = tool.execute(arguments).await;
        assert!(result.is_ok());

        let response: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(response["character_name"], "Gandalf");
        assert_eq!(response["level"], 1);
        assert_eq!(response["abilities"]["intelligence"]["modifier"], 3); // (17-10)/2 = 3
        assert_eq!(response["combat"]["proficiency_bonus"], 2);
    }

    #[tokio::test]
    async fn test_check_spell_slots_tool() {
        let (db_service, temp_dir) = setup_test_db();
        let campaign_id = create_test_campaign(&db_service, &temp_dir);
        let player_id = create_test_player(&db_service);
        let character_id = create_test_character(&db_service, campaign_id, player_id, &temp_dir);

        let tool = CheckSpellSlotsTool::new(Arc::clone(&db_service));

        let arguments = json!({
            "character_id": character_id
        });

        let result = tool.execute(arguments).await;
        assert!(result.is_ok());

        let response: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(response["character_name"], "Gandalf");
        assert_eq!(response["class"], "Wizard");
        assert_eq!(response["is_spellcaster"], true);

        // Wizard level 1 should have 2 first level slots
        let spell_slots = response["spell_slots"].as_object().unwrap();
        assert!(spell_slots.contains_key("1"));
    }

    #[tokio::test]
    async fn test_update_character_hp_tool() {
        let (db_service, temp_dir) = setup_test_db();
        let campaign_id = create_test_campaign(&db_service, &temp_dir);
        let player_id = create_test_player(&db_service);
        let character_id = create_test_character(&db_service, campaign_id, player_id, &temp_dir);

        let tool = UpdateCharacterHpTool::new(Arc::clone(&db_service));

        // Test requires_confirmation
        assert!(tool.requires_confirmation());

        // Test describe_action
        let arguments = json!({
            "character_id": character_id,
            "new_hp": 5,
            "reason": "Took damage from goblin"
        });

        let description = tool.describe_action(&arguments);
        assert!(description.is_some());
        let desc = description.unwrap();
        assert_eq!(desc.title, "Update Character HP");
        assert!(desc.description.contains("Gandalf"));

        // Test execution
        let result = tool.execute(arguments).await;
        assert!(result.is_ok());

        let response: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(response["success"], true);
        assert_eq!(response["new_hp"], 5);
        assert_eq!(response["character_name"], "Gandalf");

        // Verify HP was actually updated
        let mut conn = db_service.get_connection().unwrap();
        let mut char_service = CharacterService::new(&mut conn);
        let (_, char_data) = char_service.get_character(character_id).unwrap();
        assert_eq!(char_data.current_hp, 5);
    }

    #[tokio::test]
    async fn test_add_inventory_item_tool() {
        let (db_service, temp_dir) = setup_test_db();
        let campaign_id = create_test_campaign(&db_service, &temp_dir);
        let player_id = create_test_player(&db_service);
        let character_id = create_test_character(&db_service, campaign_id, player_id, &temp_dir);

        let tool = AddInventoryItemTool::new(Arc::clone(&db_service));

        assert!(tool.requires_confirmation());

        let arguments = json!({
            "character_id": character_id,
            "item_name": "Healing Potion",
            "quantity": 3,
            "weight": 0.5,
            "value": 50.0,
            "notes": "Heals 2d4+2 HP"
        });

        let result = tool.execute(arguments).await;
        assert!(result.is_ok());

        let response: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(response["success"], true);
        assert_eq!(response["item_added"], "Healing Potion");
        assert_eq!(response["quantity"], 3);

        // Verify item was added
        let mut conn = db_service.get_connection().unwrap();
        let mut char_service = CharacterService::new(&mut conn);
        let (_, char_data) = char_service.get_character(character_id).unwrap();
        assert_eq!(char_data.inventory.len(), 1);
        assert_eq!(char_data.inventory[0].name, "Healing Potion");
        assert_eq!(char_data.inventory[0].quantity, 3);
    }

    #[tokio::test]
    async fn test_cast_spell_tool() {
        let (db_service, temp_dir) = setup_test_db();
        let campaign_id = create_test_campaign(&db_service, &temp_dir);
        let player_id = create_test_player(&db_service);
        let character_id = create_test_character(&db_service, campaign_id, player_id, &temp_dir);

        let tool = CastSpellTool::new(Arc::clone(&db_service));

        assert!(tool.requires_confirmation());

        // Test cantrip (no slot consumed)
        let cantrip_args = json!({
            "character_id": character_id,
            "spell_name": "Fire Bolt",
            "spell_level": 0
        });

        let result = tool.execute(cantrip_args).await;
        assert!(result.is_ok());
        let response: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert!(response["message"].as_str().unwrap().contains("cantrip"));

        // Test level 1 spell
        let spell_args = json!({
            "character_id": character_id,
            "spell_name": "Magic Missile",
            "spell_level": 1
        });

        let result = tool.execute(spell_args).await;
        assert!(result.is_ok());

        let response: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(response["success"], true);
        assert_eq!(response["spell_cast"], "Magic Missile");
        assert_eq!(response["spell_level"], 1);
        assert_eq!(response["slots_remaining"], 1); // Started with 2, now 1

        // Verify spell slot was consumed
        let mut conn = db_service.get_connection().unwrap();
        let mut char_service = CharacterService::new(&mut conn);
        let (_, char_data) = char_service.get_character(character_id).unwrap();
        let slots = char_data.spells.spell_slots.get(&1).unwrap();
        assert_eq!(slots.current, 1);
        assert_eq!(slots.max, 2);
    }

    #[tokio::test]
    async fn test_cast_spell_insufficient_slots() {
        let (db_service, temp_dir) = setup_test_db();
        let campaign_id = create_test_campaign(&db_service, &temp_dir);
        let player_id = create_test_player(&db_service);
        let character_id = create_test_character(&db_service, campaign_id, player_id, &temp_dir);

        // Consume all slots
        let mut conn = db_service.get_connection().unwrap();
        let mut char_service = CharacterService::new(&mut conn);
        let (_, mut char_data) = char_service.get_character(character_id).unwrap();
        char_data.spells.spell_slots.get_mut(&1).unwrap().current = 0;
        char_service.update_character(character_id, char_data, Some("Used all slots".to_string())).unwrap();

        let tool = CastSpellTool::new(Arc::clone(&db_service));

        let spell_args = json!({
            "character_id": character_id,
            "spell_name": "Magic Missile",
            "spell_level": 1
        });

        let result = tool.execute(spell_args).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("No level 1 spell slots remaining"));
    }
}
