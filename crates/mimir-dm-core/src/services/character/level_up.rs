//! Character level up logic and progression
//!
//! Handles level advancement including HP calculation, ASI, feats, and multiclassing.

use crate::error::{DbError, Result};
use crate::models::character::data::{AbilityScores, SpellSlots};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Options for leveling up a character
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelUpOptions {
    /// Class to level up in (allows multiclassing)
    pub class_name: String,

    /// HP gain method
    pub hp_method: HpGainMethod,

    /// Ability score improvement or feat selection (if applicable at this level)
    pub asi_or_feat: Option<AsiOrFeat>,

    /// Subclass choice (if this is the level where subclass is chosen)
    pub subclass_choice: Option<String>,

    /// Reason for this level up (e.g., "Leveled up after defeating dragon")
    pub snapshot_reason: Option<String>,
}

/// Method for gaining HP on level up
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HpGainMethod {
    /// Roll the hit die (value is the roll result)
    Roll(i32),

    /// Take the average (rounded up)
    Average,
}

/// Ability Score Improvement or Feat selection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AsiOrFeat {
    /// Improve two ability scores by 1 each, or one by 2
    AbilityScoreImprovement {
        ability1: String,
        increase1: i32,
        ability2: Option<String>,
        increase2: Option<i32>,
    },

    /// Take a feat instead of ASI
    Feat(String),
}

/// Class information for level progression
#[derive(Debug, Clone)]
pub struct ClassInfo {
    pub name: String,
    pub hit_die: String,
    pub hit_die_value: i32,
    pub spellcasting_type: Option<SpellcastingType>,
    pub asi_levels: Vec<i32>,
}

/// Type of spellcasting progression
#[derive(Debug, Clone, PartialEq)]
pub enum SpellcastingType {
    Full,      // Wizard, Cleric, etc.
    Half,      // Paladin, Ranger
    Third,     // Eldritch Knight, Arcane Trickster
    Warlock,   // Unique progression
}

/// Multiclassing prerequisites (minimum ability scores)
pub struct MulticlassPrerequisites {
    pub class_name: String,
    pub required_abilities: Vec<(String, i32)>,
}

impl LevelUpOptions {
    /// Validate HP gain for a given hit die
    pub fn validate_hp_gain(&self, hit_die_value: i32) -> Result<()> {
        match &self.hp_method {
            HpGainMethod::Roll(value) => {
                if *value < 1 || *value > hit_die_value {
                    return Err(DbError::InvalidData(format!(
                        "HP roll {} is invalid for hit die d{}",
                        value, hit_die_value
                    )));
                }
            }
            HpGainMethod::Average => {}
        }
        Ok(())
    }

    /// Validate ASI/Feat choice
    pub fn validate_asi_or_feat(&self) -> Result<()> {
        if let Some(choice) = &self.asi_or_feat {
            match choice {
                AsiOrFeat::AbilityScoreImprovement {
                    ability1,
                    increase1,
                    ability2,
                    increase2,
                } => {
                    // Validate ability names
                    if !Self::is_valid_ability(ability1) {
                        return Err(DbError::InvalidData(format!(
                            "Invalid ability score: {}",
                            ability1
                        )));
                    }

                    // Validate increases
                    if *increase1 < 1 || *increase1 > 2 {
                        return Err(DbError::InvalidData(
                            "Ability score increase must be 1 or 2".to_string(),
                        ));
                    }

                    if let (Some(ability), Some(increase)) = (ability2, increase2) {
                        if !Self::is_valid_ability(ability) {
                            return Err(DbError::InvalidData(format!(
                                "Invalid ability score: {}",
                                ability
                            )));
                        }

                        if *increase < 1 || *increase > 2 {
                            return Err(DbError::InvalidData(
                                "Ability score increase must be 1 or 2".to_string(),
                            ));
                        }

                        // Total increase must be 2
                        if increase1 + increase > 2 {
                            return Err(DbError::InvalidData(
                                "Total ability score increase must be exactly 2".to_string(),
                            ));
                        }
                    }
                }
                AsiOrFeat::Feat(feat_name) => {
                    if feat_name.trim().is_empty() {
                        return Err(DbError::InvalidData("Feat name cannot be empty".to_string()));
                    }
                }
            }
        }
        Ok(())
    }

    fn is_valid_ability(name: &str) -> bool {
        matches!(
            name.to_lowercase().as_str(),
            "strength" | "dexterity" | "constitution" | "intelligence" | "wisdom" | "charisma"
        )
    }
}

impl ClassInfo {
    /// Get class information by name
    pub fn get(class_name: &str) -> Option<Self> {
        match class_name.to_lowercase().as_str() {
            "barbarian" => Some(ClassInfo {
                name: "Barbarian".to_string(),
                hit_die: "d12".to_string(),
                hit_die_value: 12,
                spellcasting_type: None,
                asi_levels: vec![4, 8, 12, 16, 19],
            }),
            "bard" => Some(ClassInfo {
                name: "Bard".to_string(),
                hit_die: "d8".to_string(),
                hit_die_value: 8,
                spellcasting_type: Some(SpellcastingType::Full),
                asi_levels: vec![4, 8, 12, 16, 19],
            }),
            "cleric" => Some(ClassInfo {
                name: "Cleric".to_string(),
                hit_die: "d8".to_string(),
                hit_die_value: 8,
                spellcasting_type: Some(SpellcastingType::Full),
                asi_levels: vec![4, 8, 12, 16, 19],
            }),
            "druid" => Some(ClassInfo {
                name: "Druid".to_string(),
                hit_die: "d8".to_string(),
                hit_die_value: 8,
                spellcasting_type: Some(SpellcastingType::Full),
                asi_levels: vec![4, 8, 12, 16, 19],
            }),
            "fighter" => Some(ClassInfo {
                name: "Fighter".to_string(),
                hit_die: "d10".to_string(),
                hit_die_value: 10,
                spellcasting_type: None,
                asi_levels: vec![4, 6, 8, 12, 14, 16, 19],
            }),
            "monk" => Some(ClassInfo {
                name: "Monk".to_string(),
                hit_die: "d8".to_string(),
                hit_die_value: 8,
                spellcasting_type: None,
                asi_levels: vec![4, 8, 12, 16, 19],
            }),
            "paladin" => Some(ClassInfo {
                name: "Paladin".to_string(),
                hit_die: "d10".to_string(),
                hit_die_value: 10,
                spellcasting_type: Some(SpellcastingType::Half),
                asi_levels: vec![4, 8, 12, 16, 19],
            }),
            "ranger" => Some(ClassInfo {
                name: "Ranger".to_string(),
                hit_die: "d10".to_string(),
                hit_die_value: 10,
                spellcasting_type: Some(SpellcastingType::Half),
                asi_levels: vec![4, 8, 12, 16, 19],
            }),
            "rogue" => Some(ClassInfo {
                name: "Rogue".to_string(),
                hit_die: "d8".to_string(),
                hit_die_value: 8,
                spellcasting_type: None,
                asi_levels: vec![4, 8, 10, 12, 16, 19],
            }),
            "sorcerer" => Some(ClassInfo {
                name: "Sorcerer".to_string(),
                hit_die: "d6".to_string(),
                hit_die_value: 6,
                spellcasting_type: Some(SpellcastingType::Full),
                asi_levels: vec![4, 8, 12, 16, 19],
            }),
            "warlock" => Some(ClassInfo {
                name: "Warlock".to_string(),
                hit_die: "d8".to_string(),
                hit_die_value: 8,
                spellcasting_type: Some(SpellcastingType::Warlock),
                asi_levels: vec![4, 8, 12, 16, 19],
            }),
            "wizard" => Some(ClassInfo {
                name: "Wizard".to_string(),
                hit_die: "d6".to_string(),
                hit_die_value: 6,
                spellcasting_type: Some(SpellcastingType::Full),
                asi_levels: vec![4, 8, 12, 16, 19],
            }),
            _ => None,
        }
    }

    /// Calculate average HP gain for this class
    pub fn average_hp_gain(&self) -> i32 {
        (self.hit_die_value / 2) + 1
    }
}

impl MulticlassPrerequisites {
    /// Get multiclass prerequisites for a class
    pub fn get(class_name: &str) -> Option<Self> {
        match class_name.to_lowercase().as_str() {
            "barbarian" => Some(Self {
                class_name: "Barbarian".to_string(),
                required_abilities: vec![("Strength".to_string(), 13)],
            }),
            "bard" => Some(Self {
                class_name: "Bard".to_string(),
                required_abilities: vec![("Charisma".to_string(), 13)],
            }),
            "cleric" => Some(Self {
                class_name: "Cleric".to_string(),
                required_abilities: vec![("Wisdom".to_string(), 13)],
            }),
            "druid" => Some(Self {
                class_name: "Druid".to_string(),
                required_abilities: vec![("Wisdom".to_string(), 13)],
            }),
            "fighter" => Some(Self {
                class_name: "Fighter".to_string(),
                required_abilities: vec![("Strength".to_string(), 13), ("Dexterity".to_string(), 13)],
            }),
            "monk" => Some(Self {
                class_name: "Monk".to_string(),
                required_abilities: vec![("Dexterity".to_string(), 13), ("Wisdom".to_string(), 13)],
            }),
            "paladin" => Some(Self {
                class_name: "Paladin".to_string(),
                required_abilities: vec![("Strength".to_string(), 13), ("Charisma".to_string(), 13)],
            }),
            "ranger" => Some(Self {
                class_name: "Ranger".to_string(),
                required_abilities: vec![("Dexterity".to_string(), 13), ("Wisdom".to_string(), 13)],
            }),
            "rogue" => Some(Self {
                class_name: "Rogue".to_string(),
                required_abilities: vec![("Dexterity".to_string(), 13)],
            }),
            "sorcerer" => Some(Self {
                class_name: "Sorcerer".to_string(),
                required_abilities: vec![("Charisma".to_string(), 13)],
            }),
            "warlock" => Some(Self {
                class_name: "Warlock".to_string(),
                required_abilities: vec![("Charisma".to_string(), 13)],
            }),
            "wizard" => Some(Self {
                class_name: "Wizard".to_string(),
                required_abilities: vec![("Intelligence".to_string(), 13)],
            }),
            _ => None,
        }
    }

    /// Check if character meets prerequisites for this class
    pub fn check(&self, abilities: &AbilityScores) -> Result<()> {
        for (ability_name, min_score) in &self.required_abilities {
            let score = match ability_name.to_lowercase().as_str() {
                "strength" => abilities.strength,
                "dexterity" => abilities.dexterity,
                "constitution" => abilities.constitution,
                "intelligence" => abilities.intelligence,
                "wisdom" => abilities.wisdom,
                "charisma" => abilities.charisma,
                _ => {
                    return Err(DbError::InvalidData(format!(
                        "Unknown ability: {}",
                        ability_name
                    )))
                }
            };

            if score < *min_score {
                return Err(DbError::InvalidData(format!(
                    "Multiclass prerequisite not met: {} requires {} {} (character has {})",
                    self.class_name, ability_name, min_score, score
                )));
            }
        }
        Ok(())
    }
}

/// Calculate spell slots for multiclass spellcasters
pub fn calculate_spell_slots(class_levels: &HashMap<String, i32>) -> HashMap<i32, SpellSlots> {
    let mut caster_level = 0;

    for (class_name, level) in class_levels {
        if let Some(class_info) = ClassInfo::get(class_name) {
            match class_info.spellcasting_type {
                Some(SpellcastingType::Full) => caster_level += level,
                Some(SpellcastingType::Half) => caster_level += level / 2,
                Some(SpellcastingType::Third) => caster_level += level / 3,
                Some(SpellcastingType::Warlock) => {
                    // Warlock uses unique pact magic - handle separately
                    continue;
                }
                None => {}
            }
        }
    }

    // Standard spell slot progression table
    let slots = match caster_level {
        1 => vec![(1, 2)],
        2 => vec![(1, 3)],
        3 => vec![(1, 4), (2, 2)],
        4 => vec![(1, 4), (2, 3)],
        5 => vec![(1, 4), (2, 3), (3, 2)],
        6 => vec![(1, 4), (2, 3), (3, 3)],
        7 => vec![(1, 4), (2, 3), (3, 3), (4, 1)],
        8 => vec![(1, 4), (2, 3), (3, 3), (4, 2)],
        9 => vec![(1, 4), (2, 3), (3, 3), (4, 3), (5, 1)],
        10 => vec![(1, 4), (2, 3), (3, 3), (4, 3), (5, 2)],
        11 => vec![(1, 4), (2, 3), (3, 3), (4, 3), (5, 2), (6, 1)],
        12 => vec![(1, 4), (2, 3), (3, 3), (4, 3), (5, 2), (6, 1)],
        13 => vec![(1, 4), (2, 3), (3, 3), (4, 3), (5, 2), (6, 1), (7, 1)],
        14 => vec![(1, 4), (2, 3), (3, 3), (4, 3), (5, 2), (6, 1), (7, 1)],
        15 => vec![(1, 4), (2, 3), (3, 3), (4, 3), (5, 2), (6, 1), (7, 1), (8, 1)],
        16 => vec![(1, 4), (2, 3), (3, 3), (4, 3), (5, 2), (6, 1), (7, 1), (8, 1)],
        17 => vec![(1, 4), (2, 3), (3, 3), (4, 3), (5, 2), (6, 1), (7, 1), (8, 1), (9, 1)],
        18 => vec![(1, 4), (2, 3), (3, 3), (4, 3), (5, 3), (6, 1), (7, 1), (8, 1), (9, 1)],
        19 => vec![(1, 4), (2, 3), (3, 3), (4, 3), (5, 3), (6, 2), (7, 1), (8, 1), (9, 1)],
        20 => vec![(1, 4), (2, 3), (3, 3), (4, 3), (5, 3), (6, 2), (7, 2), (8, 1), (9, 1)],
        _ => vec![],
    };

    slots
        .into_iter()
        .map(|(level, max)| (level, SpellSlots::new(max)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hp_gain_validation() {
        let options = LevelUpOptions {
            class_name: "Fighter".to_string(),
            hp_method: HpGainMethod::Roll(10),
            asi_or_feat: None,
            subclass_choice: None,
            snapshot_reason: None,
        };

        assert!(options.validate_hp_gain(10).is_ok());
        assert!(options.validate_hp_gain(12).is_ok());

        let bad_options = LevelUpOptions {
            class_name: "Fighter".to_string(),
            hp_method: HpGainMethod::Roll(11),
            asi_or_feat: None,
            subclass_choice: None,
            snapshot_reason: None,
        };

        assert!(bad_options.validate_hp_gain(10).is_err());
    }

    #[test]
    fn test_asi_validation() {
        let options = LevelUpOptions {
            class_name: "Fighter".to_string(),
            hp_method: HpGainMethod::Average,
            asi_or_feat: Some(AsiOrFeat::AbilityScoreImprovement {
                ability1: "Strength".to_string(),
                increase1: 2,
                ability2: None,
                increase2: None,
            }),
            subclass_choice: None,
            snapshot_reason: None,
        };

        assert!(options.validate_asi_or_feat().is_ok());

        let bad_options = LevelUpOptions {
            class_name: "Fighter".to_string(),
            hp_method: HpGainMethod::Average,
            asi_or_feat: Some(AsiOrFeat::AbilityScoreImprovement {
                ability1: "Strength".to_string(),
                increase1: 2,
                ability2: Some("Dexterity".to_string()),
                increase2: Some(1),
            }),
            subclass_choice: None,
            snapshot_reason: None,
        };

        assert!(bad_options.validate_asi_or_feat().is_err());
    }

    #[test]
    fn test_multiclass_prerequisites() {
        let abilities = AbilityScores {
            strength: 15,
            dexterity: 12,
            constitution: 14,
            intelligence: 10,
            wisdom: 13,
            charisma: 8,
        };

        let barbarian_prereqs = MulticlassPrerequisites::get("Barbarian").unwrap();
        assert!(barbarian_prereqs.check(&abilities).is_ok());

        let monk_prereqs = MulticlassPrerequisites::get("Monk").unwrap();
        assert!(monk_prereqs.check(&abilities).is_err()); // Needs DEX 13
    }

    #[test]
    fn test_spell_slot_calculation() {
        let mut class_levels = HashMap::new();
        class_levels.insert("Wizard".to_string(), 5);

        let slots = calculate_spell_slots(&class_levels);
        assert_eq!(slots.get(&1).unwrap().max, 4);
        assert_eq!(slots.get(&2).unwrap().max, 3);
        assert_eq!(slots.get(&3).unwrap().max, 2);

        // Multiclass: Wizard 3 / Cleric 2
        let mut multiclass_levels = HashMap::new();
        multiclass_levels.insert("Wizard".to_string(), 3);
        multiclass_levels.insert("Cleric".to_string(), 2);

        let multiclass_slots = calculate_spell_slots(&multiclass_levels);
        assert_eq!(multiclass_slots.get(&1).unwrap().max, 4);
        assert_eq!(multiclass_slots.get(&2).unwrap().max, 3);
        assert_eq!(multiclass_slots.get(&3).unwrap().max, 2);
    }

    #[test]
    fn test_class_info() {
        let fighter = ClassInfo::get("Fighter").unwrap();
        assert_eq!(fighter.hit_die_value, 10);
        assert_eq!(fighter.average_hp_gain(), 6);
        assert!(fighter.asi_levels.contains(&4));
        assert!(fighter.asi_levels.contains(&6)); // Fighter gets extra ASI

        let wizard = ClassInfo::get("Wizard").unwrap();
        assert_eq!(wizard.hit_die_value, 6);
        assert_eq!(wizard.average_hp_gain(), 4);
        assert_eq!(wizard.spellcasting_type, Some(SpellcastingType::Full));
    }
}
