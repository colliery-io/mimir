//! Spell management for characters
//!
//! Handles spell learning, preparation, casting, and slot management

use crate::connection::DbConnection;
use crate::error::DbError;
use crate::models::catalog::Spell;
use crate::models::character::data::{CharacterData, SpellSlots};
use std::collections::HashMap;

type Result<T> = std::result::Result<T, DbError>;

/// Calculate spell slots for a character based on class levels and multiclassing
///
/// Uses PHB multiclass spellcaster rules to calculate total caster level
/// Returns HashMap of spell level to SpellSlots (max slots)
pub fn calculate_spell_slots(
    _conn: &mut DbConnection,
    character: &CharacterData,
) -> Result<HashMap<i32, SpellSlots>> {
    // Calculate total caster level using multiclass rules
    let mut caster_level = 0;

    for class in &character.classes {
        let class_name = class.class_name.to_lowercase();
        let level = class.level;

        // Full casters: contribute full level
        if matches!(
            class_name.as_str(),
            "bard" | "cleric" | "druid" | "sorcerer" | "wizard"
        ) {
            caster_level += level;
        }
        // Half casters: contribute level / 2 (round down)
        else if matches!(class_name.as_str(), "paladin" | "ranger") {
            caster_level += level / 2;
        }
        // Third casters (subclass dependent, but check by subclass name patterns)
        else if class_name == "fighter" || class_name == "rogue" {
            // Eldritch Knight (Fighter) and Arcane Trickster (Rogue) are 1/3 casters
            // Check subclass
            if let Some(subclass) = &class.subclass {
                let sub_lower = subclass.to_lowercase();
                if sub_lower.contains("eldritch knight") || sub_lower.contains("arcane trickster") {
                    caster_level += level / 3;
                }
            }
        }
        // Warlock uses pact magic (separate system), doesn't contribute to multiclass slots
        // Artificer would be half caster but round up - not implemented here
    }

    if caster_level == 0 {
        return Ok(HashMap::new());
    }

    // Multiclass Spellcaster table (same as full caster progression)
    // Format: [1st, 2nd, 3rd, 4th, 5th, 6th, 7th, 8th, 9th]
    let slot_table: Vec<Vec<i32>> = vec![
        vec![2, 0, 0, 0, 0, 0, 0, 0, 0], // Level 1
        vec![3, 0, 0, 0, 0, 0, 0, 0, 0], // Level 2
        vec![4, 2, 0, 0, 0, 0, 0, 0, 0], // Level 3
        vec![4, 3, 0, 0, 0, 0, 0, 0, 0], // Level 4
        vec![4, 3, 2, 0, 0, 0, 0, 0, 0], // Level 5
        vec![4, 3, 3, 0, 0, 0, 0, 0, 0], // Level 6
        vec![4, 3, 3, 1, 0, 0, 0, 0, 0], // Level 7
        vec![4, 3, 3, 2, 0, 0, 0, 0, 0], // Level 8
        vec![4, 3, 3, 3, 1, 0, 0, 0, 0], // Level 9
        vec![4, 3, 3, 3, 2, 0, 0, 0, 0], // Level 10
        vec![4, 3, 3, 3, 2, 1, 0, 0, 0], // Level 11
        vec![4, 3, 3, 3, 2, 1, 0, 0, 0], // Level 12
        vec![4, 3, 3, 3, 2, 1, 1, 0, 0], // Level 13
        vec![4, 3, 3, 3, 2, 1, 1, 0, 0], // Level 14
        vec![4, 3, 3, 3, 2, 1, 1, 1, 0], // Level 15
        vec![4, 3, 3, 3, 2, 1, 1, 1, 0], // Level 16
        vec![4, 3, 3, 3, 2, 1, 1, 1, 1], // Level 17
        vec![4, 3, 3, 3, 3, 1, 1, 1, 1], // Level 18
        vec![4, 3, 3, 3, 3, 2, 1, 1, 1], // Level 19
        vec![4, 3, 3, 3, 3, 2, 2, 1, 1], // Level 20
    ];

    let level_index = (caster_level - 1).min(19) as usize;
    let slot_row = &slot_table[level_index];

    let mut slots = HashMap::new();
    for (i, &count) in slot_row.iter().enumerate() {
        if count > 0 {
            let spell_level = (i + 1) as i32;
            slots.insert(spell_level, SpellSlots::new(count));
        }
    }

    Ok(slots)
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
    /// Short rest - typically 1 hour
    Short,
    /// Long rest - typically 8 hours
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

    fn create_test_wizard(level: i32) -> CharacterData {
        CharacterData {
            character_name: "Test Wizard".to_string(),
            player_id: 1,
            level,
            experience_points: 0,
            version: 1,
            snapshot_reason: None,
            created_at: "2025-01-01".to_string(),
            race: "Human".to_string(),
            subrace: None,
            classes: vec![ClassLevel {
                class_name: "Wizard".to_string(),
                level,
                subclass: None,
                hit_dice_type: "d6".to_string(),
                hit_dice_remaining: level,
            }],
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
            max_hp: 6 + (level - 1) * 4,
            current_hp: 6 + (level - 1) * 4,
            proficiencies: Proficiencies::default(),
            class_features: Vec::new(),
            feats: Vec::new(),
            spells: SpellData::default(),
            inventory: Vec::new(),
            currency: Currency::default(),
            speed: 30, // Human speed
            equipped: EquippedItems::default(),
            personality: Personality::default(),
        }
    }

    #[test]
    fn test_spell_slot_calculation_level_1() {
        let mut conn = setup_test_db();
        let character = create_test_wizard(1);

        let slots = calculate_spell_slots(&mut conn, &character).unwrap();
        assert_eq!(slots.len(), 1);
        assert_eq!(slots.get(&1).unwrap().max, 2);
    }

    #[test]
    fn test_spell_slot_calculation_level_5() {
        let mut conn = setup_test_db();
        let character = create_test_wizard(5);

        let slots = calculate_spell_slots(&mut conn, &character).unwrap();
        assert_eq!(slots.len(), 3);
        assert_eq!(slots.get(&1).unwrap().max, 4);
        assert_eq!(slots.get(&2).unwrap().max, 3);
        assert_eq!(slots.get(&3).unwrap().max, 2);
    }

    #[test]
    fn test_spell_save_dc_calculation() {
        let character = create_test_wizard(5);

        let dc = calculate_spell_save_dc(&character, "intelligence");
        // Level 5 = +3 proficiency, Int 16 = +3 modifier
        // 8 + 3 + 3 = 14
        assert_eq!(dc, 14);
    }

    #[test]
    fn test_spell_attack_bonus_calculation() {
        let character = create_test_wizard(5);

        let bonus = calculate_spell_attack_bonus(&character, "intelligence");
        // Level 5 = +3 proficiency, Int 16 = +3 modifier
        // 3 + 3 = 6
        assert_eq!(bonus, 6);
    }
}
