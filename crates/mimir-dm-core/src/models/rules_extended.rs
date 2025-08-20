//! Extended D&D 5e Rules Reference Models for Classes, Races, Feats, and Backgrounds

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A D&D 5e character class (or subclass)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterClass {
    pub name: String,
    pub source: String,
    pub page: Option<u32>,
    pub class_name: Option<String>,  // If present, this is a subclass
    pub class_source: Option<String>,  // Source of the parent class
    pub subclass_short_name: Option<String>,  // Short name for the subclass
    pub hd: Option<HitDice>,
    pub proficiency: Option<Vec<String>>,
    pub starting_proficiencies: Option<serde_json::Value>,
    pub starting_equipment: Option<serde_json::Value>,
    pub multiclassing: Option<serde_json::Value>,
    pub caster_progression: Option<String>,
    pub cantrip_progression: Option<Vec<u8>>,
    pub spell_slots_progression: Option<serde_json::Value>,
    pub spells_known_progression: Option<Vec<u8>>,
    pub spellcasting_ability: Option<String>,  // For subclasses that grant spellcasting
    pub class_features: Option<Vec<serde_json::Value>>,  // Can be strings or objects with classFeature field
    pub subclass_features: Option<Vec<String>>,  // Features specific to this subclass
    pub additional_spells: Option<Vec<serde_json::Value>>,  // Domain/oath spells, etc.
    pub subclass_title: Option<String>,
    pub srd: Option<serde_json::Value>,
    pub basic_rules: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HitDice {
    pub faces: u8,
    pub number: u8,
}

/// Container for class data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassData {
    pub class: Vec<CharacterClass>,
}

/// Simplified class for search results
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassSummary {
    pub name: String,
    pub source: String,
    pub hit_die: String,
    pub primary_ability: String,
    pub saves: String,
    pub spellcaster: bool,
    pub description: String,
    // Additional subclass-specific information
    pub subclass_features: Option<Vec<String>>,
    pub additional_spells: Option<Vec<String>>,  // Simplified spell list
    pub spellcasting_ability: Option<String>,
    pub cantrip_progression: Option<Vec<u8>>,
}

impl From<&CharacterClass> for ClassSummary {
    fn from(class: &CharacterClass) -> Self {
        // Check if this is a subclass
        let is_subclass = class.class_name.is_some();
        
        // For subclasses, format the name as "Parent: Subclass"
        let display_name = if let Some(parent) = &class.class_name {
            format!("{}: {}", parent, class.name)
        } else {
            class.name.clone()
        };
        
        let hit_die = if is_subclass {
            "—".to_string()  // Subclasses use parent's hit die
        } else {
            class.hd.as_ref()
                .map(|hd| format!("1d{}", hd.faces))
                .unwrap_or_else(|| "Unknown".to_string())
        };
        
        let spellcaster = class.caster_progression.is_some();
        let caster_type = class.caster_progression.as_deref().unwrap_or("");
        
        // Use the parent class name for determining abilities if it's a subclass
        let class_for_abilities = class.class_name.as_deref().unwrap_or(&class.name);
        
        let primary_ability = if is_subclass {
            "—".to_string()
        } else {
            match class_for_abilities {
            "Barbarian" => "Strength",
            "Bard" => "Charisma",
            "Cleric" => "Wisdom",
            "Druid" => "Wisdom",
            "Fighter" => "Strength or Dexterity",
            "Monk" => "Dexterity & Wisdom",
            "Paladin" => "Strength & Charisma",
            "Ranger" => "Dexterity & Wisdom",
            "Rogue" => "Dexterity",
            "Sorcerer" => "Charisma",
            "Warlock" => "Charisma",
            "Wizard" => "Intelligence",
            _ => "Varies",
        }.to_string()
        };
        
        let saves = if is_subclass {
            "—".to_string()
        } else {
            match class_for_abilities {
            "Barbarian" => "Strength, Constitution",
            "Bard" => "Dexterity, Charisma",
            "Cleric" => "Wisdom, Charisma",
            "Druid" => "Intelligence, Wisdom",
            "Fighter" => "Strength, Constitution",
            "Monk" => "Strength, Dexterity",
            "Paladin" => "Wisdom, Charisma",
            "Ranger" => "Strength, Dexterity",
            "Rogue" => "Dexterity, Intelligence",
            "Sorcerer" => "Constitution, Charisma",
            "Warlock" => "Wisdom, Charisma",
            "Wizard" => "Intelligence, Wisdom",
            _ => "Unknown",
        }.to_string()
        };
        
        let description = if is_subclass {
            format!("Subclass of {}", class.class_name.as_deref().unwrap_or("Unknown"))
        } else if spellcaster {
            format!("{} spellcaster", caster_type)
        } else {
            "Martial class".to_string()
        };
        
        // Extract simplified spell list from additional_spells
        let additional_spells = class.additional_spells.as_ref().map(|spells| {
            spells.iter()
                .filter_map(|spell| {
                    // Handle various spell formats in the JSON
                    if let Some(str_val) = spell.as_str() {
                        Some(str_val.to_string())
                    } else if let Some(obj) = spell.as_object() {
                        // Try to extract spell name from object format
                        obj.get("spell")
                            .or_else(|| obj.get("name"))
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string())
                    } else {
                        None
                    }
                })
                .collect()
        });
        
        ClassSummary {
            name: display_name,
            source: class.source.clone(),
            hit_die,
            primary_ability,
            saves,
            spellcaster,
            description,
            subclass_features: class.subclass_features.clone(),
            additional_spells,
            spellcasting_ability: class.spellcasting_ability.clone(),
            cantrip_progression: class.cantrip_progression.clone(),
        }
    }
}

/// A D&D 5e race
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Race {
    pub name: String,
    pub source: String,
    pub page: Option<u32>,
    pub size: Option<Vec<String>>,
    pub speed: Option<serde_json::Value>,
    pub ability: Option<Vec<AbilityBonus>>,
    pub entries: Option<Vec<serde_json::Value>>,
    pub language_proficiencies: Option<Vec<serde_json::Value>>,
    pub resist: Option<Vec<String>>,
    pub immune: Option<Vec<String>>,
    pub vulnerable: Option<Vec<String>>,
    pub darkvision: Option<u16>,
    pub trait_tags: Option<Vec<String>>,
    pub srd: Option<serde_json::Value>,
    pub basic_rules: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbilityBonus {
    #[serde(flatten)]
    pub scores: HashMap<String, serde_json::Value>,  // Can be i8 or object with "choose" field
}

/// Container for race data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaceData {
    pub race: Vec<Race>,
    pub subrace: Option<Vec<serde_json::Value>>,
}

/// Simplified race for search results
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RaceSummary {
    pub name: String,
    pub source: String,
    pub size: String,
    pub speed: u16,
    pub ability_bonuses: String,
    pub traits: Vec<String>,
    pub description: String,
}

impl From<&Race> for RaceSummary {
    fn from(race: &Race) -> Self {
        let size = race.size.as_ref()
            .and_then(|s| s.first())
            .map(|s| match s.as_str() {
                "S" => "Small",
                "M" => "Medium",
                "L" => "Large",
                _ => s.as_str(),
            })
            .unwrap_or("Medium")
            .to_string();
        
        let speed = match &race.speed {
            Some(serde_json::Value::Number(n)) => n.as_u64().unwrap_or(30) as u16,
            Some(serde_json::Value::Object(obj)) => {
                obj.get("walk")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(30) as u16
            },
            _ => 30,
        };
        
        let ability_bonuses = race.ability.as_ref()
            .map(|abilities| {
                abilities.iter()
                    .flat_map(|a| {
                        a.scores.iter().filter_map(|(stat, bonus)| {
                            // Handle both i8 values and objects
                            if let Some(num) = bonus.as_i64() {
                                Some(format!("{} {:+}", stat.to_uppercase(), num))
                            } else if bonus.is_object() {
                                // Handle "choose" ability scores
                                Some(format!("{} (choice)", stat.to_uppercase()))
                            } else {
                                None
                            }
                        })
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            })
            .unwrap_or_else(|| "None".to_string());
        
        let mut traits = vec![];
        if race.darkvision.is_some() {
            traits.push(format!("Darkvision {}ft", race.darkvision.unwrap()));
        }
        if let Some(resists) = &race.resist {
            for resist in resists {
                traits.push(format!("Resistance to {}", resist));
            }
        }
        
        let description = format!("{} humanoid", size);
        
        RaceSummary {
            name: race.name.clone(),
            source: race.source.clone(),
            size,
            speed,
            ability_bonuses,
            traits,
            description,
        }
    }
}

/// A D&D 5e feat
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Feat {
    pub name: String,
    pub source: String,
    pub page: Option<u32>,
    pub entries: Vec<serde_json::Value>,
    pub prerequisite: Option<Vec<serde_json::Value>>,
    pub ability: Option<Vec<AbilityBonus>>,
    pub srd: Option<serde_json::Value>,
    pub basic_rules: Option<bool>,
}

/// Container for feat data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatData {
    pub feat: Vec<Feat>,
}

/// Simplified feat for search results
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatSummary {
    pub name: String,
    pub source: String,
    pub prerequisite: String,
    pub description: String,
}

impl From<&Feat> for FeatSummary {
    fn from(feat: &Feat) -> Self {
        let prerequisite = feat.prerequisite.as_ref()
            .and_then(|p| p.first())
            .and_then(|p| p.as_object())
            .and_then(|o| o.values().next())
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "None".to_string());
        
        let description = feat.entries.first()
            .and_then(|e| e.as_str())
            .map(|s| {
                if s.len() > 200 {
                    format!("{}...", &s[..197])
                } else {
                    s.to_string()
                }
            })
            .unwrap_or_else(|| "No description available".to_string());
        
        FeatSummary {
            name: feat.name.clone(),
            source: feat.source.clone(),
            prerequisite,
            description,
        }
    }
}

/// A D&D 5e background
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Background {
    pub name: String,
    pub source: String,
    pub page: Option<u32>,
    pub entries: Option<Vec<serde_json::Value>>,  // Optional, some backgrounds don't have entries
    pub skill_proficiencies: Option<Vec<serde_json::Value>>,
    pub language_proficiencies: Option<Vec<serde_json::Value>>,
    pub tool_proficiencies: Option<Vec<serde_json::Value>>,
    pub starting_equipment: Option<serde_json::Value>,
    pub feature: Option<serde_json::Value>,
    pub srd: Option<serde_json::Value>,
    pub basic_rules: Option<bool>,
}

/// Container for background data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundData {
    pub background: Vec<Background>,
}

/// Simplified background for search results
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackgroundSummary {
    pub name: String,
    pub source: String,
    pub skills: String,
    pub languages: String,
    pub tools: String,
    pub description: String,
}

impl From<&Background> for BackgroundSummary {
    fn from(background: &Background) -> Self {
        let skills = background.skill_proficiencies.as_ref()
            .map(|s| format!("{} skills", s.len()))
            .unwrap_or_else(|| "None".to_string());
        
        let languages = background.language_proficiencies.as_ref()
            .map(|l| format!("{} languages", l.len()))
            .unwrap_or_else(|| "None".to_string());
        
        let tools = background.tool_proficiencies.as_ref()
            .map(|t| format!("{} tools", t.len()))
            .unwrap_or_else(|| "None".to_string());
        
        let description = background.entries.as_ref()
            .and_then(|entries| entries.first())
            .and_then(|e| e.as_str())
            .map(|s| {
                if s.len() > 200 {
                    format!("{}...", &s[..197])
                } else {
                    s.to_string()
                }
            })
            .unwrap_or_else(|| "No description available".to_string());
        
        BackgroundSummary {
            name: background.name.clone(),
            source: background.source.clone(),
            skills,
            languages,
            tools,
            description,
        }
    }
}