//! Spell management for characters
//!
//! Handles spell learning, preparation, casting, and slot management

use crate::error::DbError;
use crate::models::character::data::{CharacterData, SpellSlots};
use crate::models::catalog::Spell;
use crate::connection::DbConnection;
use std::collections::HashMap;

type Result<T> = std::result::Result<T, DbError>;

/// Calculate spell slots for a character based on class levels and multiclassing
///
/// Queries the class's spell slot progression table from the database
/// Returns HashMap of spell level to SpellSlots (max slots)
pub fn calculate_spell_slots(
    conn: &mut DbConnection,
    character: &CharacterData,
) -> Result<HashMap<i32, SpellSlots>> {
    // For now, assume single class - multiclass would require tracking class levels separately
    // TODO: Extend CharacterData to track multiclass levels

    let mut class_service = crate::services::ClassService::new(conn);

    let class = class_service
        .get_class_by_name_and_source(&character.class, "PHB") // TODO: Track class source in CharacterData
        .map_err(|e| DbError::InvalidData(format!("Failed to get class: {}", e)))?
        .ok_or_else(|| {
            DbError::InvalidData(format!(
                "Class '{}' from 'PHB' not found in database. Please import the appropriate rulebook first.",
                character.class
            ))
        })?;

    // Parse spell slot progression from classTableGroups
    if let Some(table_groups) = &class.class_table_groups {
        // Find the spell progression table (has rowsSpellProgression field)
        for group in table_groups {
            if let Some(rows) = group.get("rowsSpellProgression") {
                    if let Some(rows_array) = rows.as_array() {
                        // Character level is 1-indexed, array is 0-indexed
                        let level_index = (character.level - 1).max(0) as usize;

                        if level_index < rows_array.len() {
                            if let Some(slot_row) = rows_array[level_index].as_array() {
                                let mut slots = HashMap::new();

                                // Each column is a spell level (1st, 2nd, 3rd, etc.)
                                for (spell_level, slot_count) in slot_row.iter().enumerate() {
                                    if let Some(count) = slot_count.as_i64() {
                                        if count > 0 {
                                            let spell_level_num = (spell_level + 1) as i32;
                                            slots.insert(spell_level_num, SpellSlots::new(count as i32));
                                        }
                                    }
                                }

                                return Ok(slots);
                            }
                        }
                }
            }
        }
    }

    // No spell progression found - non-spellcasting class
    Ok(HashMap::new())
}

/// Calculate spell save DC for a character
///
/// Formula: 8 + proficiency bonus + spellcasting ability modifier
pub fn calculate_spell_save_dc(character: &CharacterData, spellcasting_ability: &str) -> i32 {
    let prof_bonus = character.proficiency_bonus();
    let ability_mod = get_ability_modifier(character, spellcasting_ability);

    8 + prof_bonus + ability_mod
}

/// Calculate spell attack bonus for a character
///
/// Formula: proficiency bonus + spellcasting ability modifier
pub fn calculate_spell_attack_bonus(character: &CharacterData, spellcasting_ability: &str) -> i32 {
    let prof_bonus = character.proficiency_bonus();
    let ability_mod = get_ability_modifier(character, spellcasting_ability);

    prof_bonus + ability_mod
}

/// Helper to get ability modifier by name
fn get_ability_modifier(character: &CharacterData, ability: &str) -> i32 {
    match ability.to_lowercase().as_str() {
        "strength" | "str" => character.abilities.str_modifier(),
        "dexterity" | "dex" => character.abilities.dex_modifier(),
        "constitution" | "con" => character.abilities.con_modifier(),
        "intelligence" | "int" => character.abilities.int_modifier(),
        "wisdom" | "wis" => character.abilities.wis_modifier(),
        "charisma" | "cha" => character.abilities.cha_modifier(),
        _ => 0, // Unknown ability
    }
}

/// Validate if a spell is on a class's spell list
pub fn validate_spell_for_class(
    _conn: &mut DbConnection,
    spell: &Spell,
    class_name: &str,
) -> Result<bool> {
    // Check if class is in the spell's class list
    if let Some(classes) = &spell.classes {
        if let Some(class_list) = &classes.from_class_list {
            for class_ref in class_list {
                if class_ref.name.eq_ignore_ascii_case(class_name) {
                    return Ok(true);
                }
            }
        }

        // TODO: Check subclass spell lists when we have subclass support
    }

    Ok(false)
}

/// Rest type for spell slot restoration
#[derive(Debug, Clone, PartialEq)]
pub enum RestType {
    Short,
    Long,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::character::data::*;

    fn setup_test_db() -> crate::connection::DbConnection {
        let mut conn = crate::establish_connection(":memory:").unwrap();
        crate::run_migrations(&mut conn).unwrap();

        // Insert test Wizard class with spell progression
        let wizard_json = r#"{
            "name": "Wizard",
            "source": "PHB",
            "hd": {"number": 1, "faces": 6},
            "casterProgression": "full",
            "spellcastingAbility": "int",
            "classTableGroups": [
                {
                    "colLabels": ["1st", "2nd", "3rd", "4th", "5th", "6th", "7th", "8th", "9th"],
                    "rowsSpellProgression": [
                        [2, 0, 0, 0, 0, 0, 0, 0, 0],
                        [3, 0, 0, 0, 0, 0, 0, 0, 0],
                        [4, 2, 0, 0, 0, 0, 0, 0, 0],
                        [4, 3, 0, 0, 0, 0, 0, 0, 0],
                        [4, 3, 2, 0, 0, 0, 0, 0, 0],
                        [4, 3, 3, 0, 0, 0, 0, 0, 0],
                        [4, 3, 3, 1, 0, 0, 0, 0, 0],
                        [4, 3, 3, 2, 0, 0, 0, 0, 0],
                        [4, 3, 3, 3, 1, 0, 0, 0, 0],
                        [4, 3, 3, 3, 2, 0, 0, 0, 0],
                        [4, 3, 3, 3, 2, 1, 0, 0, 0],
                        [4, 3, 3, 3, 2, 1, 0, 0, 0],
                        [4, 3, 3, 3, 2, 1, 1, 0, 0],
                        [4, 3, 3, 3, 2, 1, 1, 0, 0],
                        [4, 3, 3, 3, 2, 1, 1, 1, 0],
                        [4, 3, 3, 3, 2, 1, 1, 1, 0],
                        [4, 3, 3, 3, 2, 1, 1, 1, 1],
                        [4, 3, 3, 3, 3, 1, 1, 1, 1],
                        [4, 3, 3, 3, 3, 2, 1, 1, 1],
                        [4, 3, 3, 3, 3, 2, 2, 1, 1]
                    ]
                }
            ]
        }"#;

        use diesel::prelude::*;
        diesel::insert_into(crate::schema::catalog_classes::table)
            .values((
                crate::schema::catalog_classes::name.eq("Wizard"),
                crate::schema::catalog_classes::source.eq("PHB"),
                crate::schema::catalog_classes::hit_dice.eq("d6"),
                crate::schema::catalog_classes::full_class_json.eq(wizard_json),
            ))
            .execute(&mut conn)
            .expect("Failed to insert Wizard class");

        conn
    }

    #[test]
    fn test_spell_slot_calculation_level_1() {
        let mut conn = setup_test_db();
        let character = CharacterData {
            character_name: "Test Wizard".to_string(),
            player_id: 1,
            level: 1,
            experience_points: 0,
            version: 1,
            snapshot_reason: None,
            created_at: "2025-01-01".to_string(),
            race: "Human".to_string(),
            subrace: None,
            class: "Wizard".to_string(),
            subclass: None,
            background: "Sage".to_string(),
            alignment: Some("Neutral Good".to_string()),
            abilities: AbilityScores {
                strength: 8,
                dexterity: 14,
                constitution: 12,
                intelligence: 16,
                wisdom: 13,
                charisma: 10,
            },
            max_hp: 8,
            current_hp: 8,
            hit_dice_remaining: 1,
            hit_dice_type: "d6".to_string(),
            proficiencies: Proficiencies::default(),
            class_features: Vec::new(),
            feats: Vec::new(),
            spells: SpellData::default(),
            inventory: Vec::new(),
            currency: Currency::default(),
            equipped: EquippedItems::default(),
            personality: Personality::default(),
        };

        let slots = calculate_spell_slots(&mut conn, &character).unwrap();
        assert_eq!(slots.len(), 1);
        assert_eq!(slots.get(&1).unwrap().max, 2);
    }

    #[test]
    fn test_spell_slot_calculation_level_5() {
        let mut conn = setup_test_db();
        let character = CharacterData {
            character_name: "Test Wizard".to_string(),
            player_id: 1,
            level: 5,
            experience_points: 6500,
            version: 1,
            snapshot_reason: None,
            created_at: "2025-01-01".to_string(),
            race: "Human".to_string(),
            subrace: None,
            class: "Wizard".to_string(),
            subclass: None,
            background: "Sage".to_string(),
            alignment: Some("Neutral Good".to_string()),
            abilities: AbilityScores {
                strength: 8,
                dexterity: 14,
                constitution: 12,
                intelligence: 16,
                wisdom: 13,
                charisma: 10,
            },
            max_hp: 22,
            current_hp: 22,
            hit_dice_remaining: 5,
            hit_dice_type: "d6".to_string(),
            proficiencies: Proficiencies::default(),
            class_features: Vec::new(),
            feats: Vec::new(),
            spells: SpellData::default(),
            inventory: Vec::new(),
            currency: Currency::default(),
            equipped: EquippedItems::default(),
            personality: Personality::default(),
        };

        let slots = calculate_spell_slots(&mut conn, &character).unwrap();
        assert_eq!(slots.len(), 3);
        assert_eq!(slots.get(&1).unwrap().max, 4);
        assert_eq!(slots.get(&2).unwrap().max, 3);
        assert_eq!(slots.get(&3).unwrap().max, 2);
    }

    #[test]
    fn test_spell_save_dc_calculation() {
        let character = CharacterData {
            character_name: "Test Wizard".to_string(),
            player_id: 1,
            level: 5,
            experience_points: 6500,
            version: 1,
            snapshot_reason: None,
            created_at: "2025-01-01".to_string(),
            race: "Human".to_string(),
            subrace: None,
            class: "Wizard".to_string(),
            subclass: None,
            background: "Sage".to_string(),
            alignment: Some("Neutral Good".to_string()),
            abilities: AbilityScores {
                strength: 8,
                dexterity: 14,
                constitution: 12,
                intelligence: 16,
                wisdom: 13,
                charisma: 10,
            },
            max_hp: 22,
            current_hp: 22,
            hit_dice_remaining: 5,
            hit_dice_type: "d6".to_string(),
            proficiencies: Proficiencies::default(),
            class_features: Vec::new(),
            feats: Vec::new(),
            spells: SpellData::default(),
            inventory: Vec::new(),
            currency: Currency::default(),
            equipped: EquippedItems::default(),
            personality: Personality::default(),
        };

        let dc = calculate_spell_save_dc(&character, "intelligence");
        // Level 5 = +3 proficiency, Int 16 = +3 modifier
        // 8 + 3 + 3 = 14
        assert_eq!(dc, 14);
    }

    #[test]
    fn test_spell_attack_bonus_calculation() {
        let character = CharacterData {
            character_name: "Test Wizard".to_string(),
            player_id: 1,
            level: 5,
            experience_points: 6500,
            version: 1,
            snapshot_reason: None,
            created_at: "2025-01-01".to_string(),
            race: "Human".to_string(),
            subrace: None,
            class: "Wizard".to_string(),
            subclass: None,
            background: "Sage".to_string(),
            alignment: Some("Neutral Good".to_string()),
            abilities: AbilityScores {
                strength: 8,
                dexterity: 14,
                constitution: 12,
                intelligence: 16,
                wisdom: 13,
                charisma: 10,
            },
            max_hp: 22,
            current_hp: 22,
            hit_dice_remaining: 5,
            hit_dice_type: "d6".to_string(),
            proficiencies: Proficiencies::default(),
            class_features: Vec::new(),
            feats: Vec::new(),
            spells: SpellData::default(),
            inventory: Vec::new(),
            currency: Currency::default(),
            equipped: EquippedItems::default(),
            personality: Personality::default(),
        };

        let bonus = calculate_spell_attack_bonus(&character, "intelligence");
        // Level 5 = +3 proficiency, Int 16 = +3 modifier
        // 3 + 3 = 6
        assert_eq!(bonus, 6);
    }
}
