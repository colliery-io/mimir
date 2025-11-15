//! Character data structures for YAML serialization
//!
//! These structures represent the complete character state stored in character_data column.

use serde::{Deserialize, Serialize};

/// Ability scores with helper methods for modifiers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AbilityScores {
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub intelligence: i32,
    pub wisdom: i32,
    pub charisma: i32,
}

impl AbilityScores {
    /// Calculate ability modifier from score (uses floor division for negative values)
    pub fn modifier(score: i32) -> i32 {
        let diff = score - 10;
        if diff >= 0 {
            diff / 2
        } else {
            // Floor division for negative numbers
            (diff - 1) / 2
        }
    }

    pub fn str_modifier(&self) -> i32 {
        Self::modifier(self.strength)
    }

    pub fn dex_modifier(&self) -> i32 {
        Self::modifier(self.dexterity)
    }

    pub fn con_modifier(&self) -> i32 {
        Self::modifier(self.constitution)
    }

    pub fn int_modifier(&self) -> i32 {
        Self::modifier(self.intelligence)
    }

    pub fn wis_modifier(&self) -> i32 {
        Self::modifier(self.wisdom)
    }

    pub fn cha_modifier(&self) -> i32 {
        Self::modifier(self.charisma)
    }
}

/// Proficiency tracking for skills, saves, armor, weapons, tools, languages
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Proficiencies {
    #[serde(default)]
    pub skills: Vec<String>,
    #[serde(default)]
    pub saves: Vec<String>,
    #[serde(default)]
    pub armor: Vec<String>,
    #[serde(default)]
    pub weapons: Vec<String>,
    #[serde(default)]
    pub tools: Vec<String>,
    #[serde(default)]
    pub languages: Vec<String>,
}

impl Default for Proficiencies {
    fn default() -> Self {
        Self {
            skills: Vec::new(),
            saves: Vec::new(),
            armor: Vec::new(),
            weapons: Vec::new(),
            tools: Vec::new(),
            languages: Vec::new(),
        }
    }
}

/// Spell slot tracking for a specific spell level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpellSlots {
    pub max: i32,
    pub current: i32,
}

impl SpellSlots {
    pub fn new(max: i32) -> Self {
        Self { max, current: max }
    }

    pub fn expend(&mut self, count: i32) -> bool {
        if self.current >= count {
            self.current -= count;
            true
        } else {
            false
        }
    }

    pub fn recover(&mut self, count: i32) {
        self.current = (self.current + count).min(self.max);
    }

    pub fn recover_all(&mut self) {
        self.current = self.max;
    }
}

/// Spell data for spellcasting characters
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpellData {
    #[serde(default)]
    pub known_spells: Vec<String>,
    #[serde(default)]
    pub prepared_spells: Vec<String>,
    #[serde(default)]
    pub cantrips: Vec<String>,
    #[serde(default)]
    pub spell_slots: std::collections::HashMap<i32, SpellSlots>,
}

impl Default for SpellData {
    fn default() -> Self {
        Self {
            known_spells: Vec::new(),
            prepared_spells: Vec::new(),
            cantrips: Vec::new(),
            spell_slots: std::collections::HashMap::new(),
        }
    }
}

/// Inventory item
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InventoryItem {
    pub name: String,
    pub quantity: i32,
    #[serde(default)]
    pub weight: f64,
    #[serde(default)]
    pub value: f64,
    pub notes: Option<String>,
}

/// Equipped items in specific slots
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EquippedItems {
    pub armor: Option<String>,
    pub shield: Option<String>,
    pub main_hand: Option<String>,
    pub off_hand: Option<String>,
}

impl Default for EquippedItems {
    fn default() -> Self {
        Self {
            armor: None,
            shield: None,
            main_hand: None,
            off_hand: None,
        }
    }
}

/// Character personality traits
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Personality {
    pub traits: Option<String>,
    pub ideals: Option<String>,
    pub bonds: Option<String>,
    pub flaws: Option<String>,
}

impl Default for Personality {
    fn default() -> Self {
        Self {
            traits: None,
            ideals: None,
            bonds: None,
            flaws: None,
        }
    }
}

/// Complete character data structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CharacterData {
    // Metadata
    pub character_name: String,
    pub player_id: i32,
    pub level: i32,
    #[serde(default)]
    pub experience_points: i32,
    pub version: i32,
    pub snapshot_reason: Option<String>,
    pub created_at: String,

    // Core Identity
    pub race: String,
    pub subrace: Option<String>,
    pub class: String,
    pub subclass: Option<String>,
    pub background: String,
    pub alignment: Option<String>,

    // Abilities
    pub abilities: AbilityScores,

    // HP and Resources
    pub max_hp: i32,
    pub current_hp: i32,
    pub hit_dice_remaining: i32,
    pub hit_dice_type: String,

    // Proficiencies
    pub proficiencies: Proficiencies,

    // Class Features
    #[serde(default)]
    pub class_features: Vec<String>,

    // Feats
    #[serde(default)]
    pub feats: Vec<String>,

    // Spells
    #[serde(default)]
    pub spells: SpellData,

    // Inventory
    #[serde(default)]
    pub inventory: Vec<InventoryItem>,

    // Equipment
    #[serde(default)]
    pub equipped: EquippedItems,

    // Personality
    #[serde(default)]
    pub personality: Personality,
}

impl CharacterData {
    /// Calculate proficiency bonus based on level
    pub fn proficiency_bonus(&self) -> i32 {
        match self.level {
            1..=4 => 2,
            5..=8 => 3,
            9..=12 => 4,
            13..=16 => 5,
            17..=20 => 6,
            _ => 2, // fallback
        }
    }

    /// Check if character is proficient in a skill
    pub fn is_proficient_in_skill(&self, skill: &str) -> bool {
        self.proficiencies.skills.iter().any(|s| s == skill)
    }

    /// Check if character is proficient in a saving throw
    pub fn is_proficient_in_save(&self, save: &str) -> bool {
        self.proficiencies.saves.iter().any(|s| s == save)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ability_modifier_calculation() {
        assert_eq!(AbilityScores::modifier(10), 0);
        assert_eq!(AbilityScores::modifier(11), 0);
        assert_eq!(AbilityScores::modifier(12), 1);
        assert_eq!(AbilityScores::modifier(8), -1);
        assert_eq!(AbilityScores::modifier(20), 5);
        assert_eq!(AbilityScores::modifier(3), -4);
    }

    #[test]
    fn test_ability_scores_modifiers() {
        let abilities = AbilityScores {
            strength: 16,
            dexterity: 12,
            constitution: 14,
            intelligence: 10,
            wisdom: 13,
            charisma: 8,
        };

        assert_eq!(abilities.str_modifier(), 3);
        assert_eq!(abilities.dex_modifier(), 1);
        assert_eq!(abilities.con_modifier(), 2);
        assert_eq!(abilities.int_modifier(), 0);
        assert_eq!(abilities.wis_modifier(), 1);
        assert_eq!(abilities.cha_modifier(), -1);
    }

    #[test]
    fn test_proficiency_bonus_by_level() {
        let mut character = CharacterData {
            character_name: "Test".to_string(),
            player_id: 1,
            level: 1,
            experience_points: 0,
            version: 1,
            snapshot_reason: None,
            created_at: "2025-01-01".to_string(),
            race: "Human".to_string(),
            subrace: None,
            class: "Fighter".to_string(),
            subclass: None,
            background: "Soldier".to_string(),
            alignment: Some("Neutral".to_string()),
            abilities: AbilityScores {
                strength: 15,
                dexterity: 14,
                constitution: 13,
                intelligence: 12,
                wisdom: 10,
                charisma: 8,
            },
            max_hp: 12,
            current_hp: 12,
            hit_dice_remaining: 1,
            hit_dice_type: "d10".to_string(),
            proficiencies: Proficiencies::default(),
            class_features: Vec::new(),
            feats: Vec::new(),
            spells: SpellData::default(),
            inventory: Vec::new(),
            equipped: EquippedItems::default(),
            personality: Personality::default(),
        };

        character.level = 1;
        assert_eq!(character.proficiency_bonus(), 2);

        character.level = 5;
        assert_eq!(character.proficiency_bonus(), 3);

        character.level = 9;
        assert_eq!(character.proficiency_bonus(), 4);

        character.level = 13;
        assert_eq!(character.proficiency_bonus(), 5);

        character.level = 17;
        assert_eq!(character.proficiency_bonus(), 6);
    }

    #[test]
    fn test_spell_slot_management() {
        let mut slots = SpellSlots::new(4);
        assert_eq!(slots.current, 4);
        assert_eq!(slots.max, 4);

        assert!(slots.expend(2));
        assert_eq!(slots.current, 2);

        assert!(!slots.expend(3)); // not enough slots
        assert_eq!(slots.current, 2);

        slots.recover(1);
        assert_eq!(slots.current, 3);

        slots.recover(5); // should cap at max
        assert_eq!(slots.current, 4);

        slots.expend(4);
        slots.recover_all();
        assert_eq!(slots.current, 4);
    }

    #[test]
    fn test_yaml_serialization() {
        let character = CharacterData {
            character_name: "Thorin".to_string(),
            player_id: 1,
            level: 3,
            experience_points: 900,
            version: 1,
            snapshot_reason: Some("Initial creation".to_string()),
            created_at: "2025-01-15T10:30:00Z".to_string(),
            race: "Dwarf".to_string(),
            subrace: Some("Mountain".to_string()),
            class: "Fighter".to_string(),
            subclass: Some("Champion".to_string()),
            background: "Soldier".to_string(),
            alignment: Some("Lawful Good".to_string()),
            abilities: AbilityScores {
                strength: 16,
                dexterity: 12,
                constitution: 16,
                intelligence: 10,
                wisdom: 13,
                charisma: 8,
            },
            max_hp: 28,
            current_hp: 28,
            hit_dice_remaining: 3,
            hit_dice_type: "d10".to_string(),
            proficiencies: Proficiencies {
                skills: vec!["Athletics".to_string(), "Intimidation".to_string()],
                saves: vec!["Strength".to_string(), "Constitution".to_string()],
                armor: vec!["All armor".to_string(), "Shields".to_string()],
                weapons: vec!["Simple weapons".to_string(), "Martial weapons".to_string()],
                tools: vec!["Smith's tools".to_string()],
                languages: vec!["Common".to_string(), "Dwarvish".to_string()],
            },
            class_features: vec![
                "Fighting Style (Defense)".to_string(),
                "Second Wind".to_string(),
            ],
            feats: Vec::new(),
            spells: SpellData::default(),
            inventory: vec![InventoryItem {
                name: "Rations".to_string(),
                quantity: 10,
                weight: 20.0,
                value: 5.0,
                notes: None,
            }],
            equipped: EquippedItems {
                armor: Some("Chain Mail".to_string()),
                shield: Some("Shield".to_string()),
                main_hand: Some("Warhammer".to_string()),
                off_hand: None,
            },
            personality: Personality {
                traits: Some("I'm always polite and respectful.".to_string()),
                ideals: Some("Responsibility.".to_string()),
                bonds: Some("I would still lay down my life for the people I served with.".to_string()),
                flaws: Some("I obey authority without question.".to_string()),
            },
        };

        // Test YAML serialization
        let yaml = serde_yaml::to_string(&character).expect("Failed to serialize");
        assert!(yaml.contains("character_name: Thorin"));
        assert!(yaml.contains("race: Dwarf"));
        assert!(yaml.contains("strength: 16"));

        // Test round-trip
        let deserialized: CharacterData =
            serde_yaml::from_str(&yaml).expect("Failed to deserialize");
        assert_eq!(character, deserialized);
    }
}
