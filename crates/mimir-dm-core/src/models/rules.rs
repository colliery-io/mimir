//! D&D 5e Rules Reference Models
//! 
//! Data structures for static game content like spells, items, creatures, etc.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A D&D 5e spell
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Spell {
    pub name: String,
    pub source: String,
    pub page: Option<u32>,
    pub level: u8,
    pub school: SpellSchool,
    pub time: Vec<CastingTime>,
    pub range: SpellRange,
    pub components: Components,
    pub duration: Vec<Duration>,
    pub entries: Vec<serde_json::Value>,  // Can be strings or objects
    #[serde(default)]
    pub classes: Option<Classes>,
    #[serde(default)]
    pub scaling_level_dice: Option<ScalingLevelDice>,
    #[serde(default)]
    pub damage_inflict: Option<Vec<String>>,
    #[serde(default)]
    pub saving_throw: Option<Vec<String>>,
    #[serde(default)]
    pub meta: Option<SpellMeta>,
}

/// Spell metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpellMeta {
    #[serde(default)]
    pub ritual: bool,
}

/// Spell school codes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SpellSchool {
    #[serde(rename = "A")]
    Abjuration,
    #[serde(rename = "C")]
    Conjuration,
    #[serde(rename = "D")]
    Divination,
    #[serde(rename = "E")]
    Enchantment,
    #[serde(rename = "V")]
    Evocation,
    #[serde(rename = "I")]
    Illusion,
    #[serde(rename = "N")]
    Necromancy,
    #[serde(rename = "T")]
    Transmutation,
}

impl SpellSchool {
    pub fn as_str(&self) -> &'static str {
        match self {
            SpellSchool::Abjuration => "Abjuration",
            SpellSchool::Conjuration => "Conjuration",
            SpellSchool::Divination => "Divination",
            SpellSchool::Enchantment => "Enchantment",
            SpellSchool::Evocation => "Evocation",
            SpellSchool::Illusion => "Illusion",
            SpellSchool::Necromancy => "Necromancy",
            SpellSchool::Transmutation => "Transmutation",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CastingTime {
    pub number: u32,
    pub unit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SpellRange {
    #[serde(rename_all = "camelCase")]
    Point {
        #[serde(rename = "type")]
        range_type: String,
        distance: Distance,
    },
    #[serde(rename_all = "camelCase")]
    Special {
        #[serde(rename = "type")]
        range_type: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Distance {
    #[serde(rename = "type")]
    pub distance_type: String,
    pub amount: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Components {
    #[serde(default)]
    pub v: bool,
    #[serde(default)]
    pub s: bool,
    #[serde(default)]
    pub m: Option<serde_json::Value>,  // Can be bool, string, or object
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Duration {
    #[serde(rename = "type")]
    pub duration_type: String,
    #[serde(default)]
    pub duration: Option<DurationTime>,
    #[serde(default)]
    pub concentration: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DurationTime {
    #[serde(rename = "type")]
    pub time_type: String,
    pub amount: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Classes {
    #[serde(rename = "fromClassList")]
    pub from_class_list: Vec<ClassEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassEntry {
    pub name: String,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalingLevelDice {
    pub label: String,
    pub scaling: HashMap<String, String>,
}

/// Container for spell data from 5etools JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpellData {
    pub spell: Vec<Spell>,
}

/// A D&D 5e item
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub name: String,
    pub source: String,
    pub page: Option<u32>,
    #[serde(rename = "type")]
    pub item_type: Option<String>,  // Some items don't have a type
    pub rarity: Option<String>,
    pub value: Option<f64>, // Value in copper pieces (can be fractional)
    pub weight: Option<f64>,  // Use f64 to handle decimal weights
    pub entries: Option<Vec<serde_json::Value>>,
    pub additional_entries: Option<Vec<serde_json::Value>>,
    
    // Weapon properties
    pub weapon_category: Option<String>,
    pub property: Option<Vec<String>>,
    pub dmg1: Option<String>,
    pub dmg2: Option<String>,
    pub dmg_type: Option<String>,
    pub range: Option<String>,
    pub weapon: Option<bool>,
    
    // Armor properties
    pub ac: Option<u8>,
    pub strength: Option<String>,
    pub stealth: Option<bool>,
    pub armor: Option<bool>,
    
    // Magic item properties
    pub req_attune: Option<serde_json::Value>,  // Can be bool or string
    pub tier: Option<String>,
    pub curse: Option<bool>,
    
    // Other flags
    pub srd: Option<serde_json::Value>,  // Can be bool or string
    pub basic_rules: Option<bool>,
    pub misc_tags: Option<Vec<String>>,
}

/// Container for item data from 5etools JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemData {
    pub item: Vec<Item>,
}

/// Simplified item for search results
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemSummary {
    pub name: String,
    pub item_type: String,
    pub type_name: String, // Human-readable type
    pub source: String,
    pub rarity: String,
    pub value: Option<f64>,
    pub weight: Option<f64>,
    pub ac: Option<u8>,
    pub damage: Option<String>,
    pub req_attune: Option<String>,
    pub description: String,
}

impl From<&Item> for ItemSummary {
    fn from(item: &Item) -> Self {
        // Map type codes to readable names
        let item_type = item.item_type.clone().unwrap_or_else(|| "?".to_string());
        let type_name = match item_type.as_str() {
            // Weapons
            "S" => "Simple Weapon",
            "M" => "Martial Weapon",
            "R" => "Ranged Weapon",
            "A" => "Ammunition",
            // Armor
            "LA" => "Light Armor",
            "MA" => "Medium Armor",
            "HA" => "Heavy Armor",
            // Equipment
            "G" => "Adventuring Gear",
            "AT" => "Artisan's Tools",
            "T" => "Tools",
            "GS" => "Gaming Set",
            "SCF" => "Spellcasting Focus",
            "INS" => "Instrument",
            // Transport
            "MNT" => "Mount",
            "TAH" => "Tack and Harness",
            "VEH" => "Vehicle",
            // Other
            "FD" => "Food & Drink",
            "TG" => "Trade Good",
            "$C" => "Treasure",
            _ => "Other",
        }.to_string();
        
        // Get damage string for weapons
        let damage = if item.weapon.unwrap_or(false) {
            item.dmg1.clone()
        } else {
            None
        };
        
        // Extract first entry as description
        let description = item.entries.as_ref()
            .and_then(|e| e.first())
            .and_then(|e| e.as_str())
            .map(|s| {
                if s.len() > 200 {
                    format!("{}...", &s[..197])
                } else {
                    s.to_string()
                }
            })
            .unwrap_or_else(|| "No description available".to_string());
        
        ItemSummary {
            name: item.name.clone(),
            item_type,
            type_name,
            source: item.source.clone(),
            rarity: item.rarity.clone().unwrap_or_else(|| "none".to_string()),
            value: item.value,
            weight: item.weight,
            ac: item.ac,
            damage,
            req_attune: match &item.req_attune {
                Some(serde_json::Value::Bool(true)) => Some("Yes".to_string()),
                Some(serde_json::Value::Bool(false)) => None,
                Some(serde_json::Value::String(s)) => Some(s.clone()),
                _ => None,
            },
            description,
        }
    }
}

/// A D&D 5e monster/creature
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Monster {
    pub name: String,
    pub source: String,
    pub page: Option<u32>,
    pub size: Option<Vec<String>>,  // S, M, L, etc.
    #[serde(rename = "type")]
    pub creature_type: Option<serde_json::Value>,  // Can be string or object
    pub alignment: Option<serde_json::Value>,  // Can be array of strings or array of objects
    pub ac: Option<serde_json::Value>,  // Can be number or array of AC objects
    pub hp: Option<serde_json::Value>,  // Can be object or number
    pub speed: Option<Speed>,
    pub str: Option<u8>,
    pub dex: Option<u8>,
    pub con: Option<u8>,
    pub int: Option<u8>,
    pub wis: Option<u8>,
    pub cha: Option<u8>,
    pub save: Option<serde_json::Value>,  // Saving throws
    pub skill: Option<serde_json::Value>,  // Skills
    pub senses: Option<Vec<String>>,
    pub passive: Option<u8>,
    pub languages: Option<Vec<String>>,
    pub cr: Option<serde_json::Value>,  // Challenge rating (can be string like "1/4")
    #[serde(rename = "trait")]
    pub trait_entries: Option<Vec<serde_json::Value>>,  // Named "trait" in JSON
    pub action: Option<Vec<serde_json::Value>>,
    pub reaction: Option<Vec<serde_json::Value>>,
    pub legendary: Option<Vec<serde_json::Value>>,
    pub legendary_group: Option<serde_json::Value>,
    pub environment: Option<Vec<String>>,
    pub srd: Option<serde_json::Value>,
    pub basic_rules: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatureType {
    #[serde(rename = "type")]
    pub base_type: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArmorClass {
    pub ac: u8,
    pub from: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HitPoints {
    pub average: u32,
    pub formula: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Speed {
    pub walk: Option<serde_json::Value>,  // Can be number or object
    pub fly: Option<serde_json::Value>,   // Can be number or object
    pub swim: Option<serde_json::Value>,  // Can be number or object
    pub climb: Option<serde_json::Value>, // Can be number or object
    pub burrow: Option<serde_json::Value>, // Can be number or object
    pub hover: Option<bool>,
    pub can_hover: Option<bool>,  // Alternative field name
}

/// Container for monster data from 5etools JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterData {
    pub monster: Vec<Monster>,
}

/// Simplified monster for search results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterSummary {
    pub name: String,
    pub source: String,
    pub size: String,
    pub creature_type: String,
    pub alignment: String,
    pub cr: String,
    pub cr_numeric: f32,  // For sorting
    pub hp: u32,
    pub ac: u8,
    pub environment: Vec<String>,
}

impl From<&Monster> for MonsterSummary {
    fn from(monster: &Monster) -> Self {
        // Format size
        let size = monster.size.as_ref()
            .and_then(|s| s.first())
            .map(|s| match s.as_str() {
                "T" => "Tiny",
                "S" => "Small",
                "M" => "Medium",
                "L" => "Large",
                "H" => "Huge",
                "G" => "Gargantuan",
                _ => s.as_str(),
            })
            .unwrap_or("Unknown")
            .to_string();
        
        // Format creature type (can be string or object)
        let creature_type = match &monster.creature_type {
            Some(serde_json::Value::String(s)) => s.clone(),
            Some(serde_json::Value::Object(obj)) => {
                let base_type = obj.get("type")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown");
                let tags = obj.get("tags")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str())
                            .collect::<Vec<_>>()
                            .join(", ")
                    });
                if let Some(tag_str) = tags {
                    if !tag_str.is_empty() {
                        format!("{} ({})", base_type, tag_str)
                    } else {
                        base_type.to_string()
                    }
                } else {
                    base_type.to_string()
                }
            },
            _ => "Unknown".to_string(),
        };
        
        // Format alignment (can be array of strings or array of objects with chance)
        let alignment = match &monster.alignment {
            Some(serde_json::Value::Array(arr)) => {
                // Check if first element is a string or object
                if let Some(first) = arr.first() {
                    if let serde_json::Value::String(s) = first {
                        // Simple array of strings
                        arr.iter()
                            .filter_map(|v| v.as_str())
                            .map(|align| match align {
                                "L" => "Lawful",
                                "N" => "Neutral",
                                "C" => "Chaotic",
                                "G" => "Good",
                                "E" => "Evil",
                                "U" => "Unaligned",
                                "A" => "Any",
                                _ => align,
                            })
                            .collect::<Vec<_>>()
                            .join(" ")
                    } else if let serde_json::Value::Object(obj) = first {
                        // Array of objects with alignment and chance
                        // Just take the first alignment option for summary
                        obj.get("alignment")
                            .and_then(|v| v.as_array())
                            .map(|align_arr| {
                                align_arr.iter()
                                    .filter_map(|v| v.as_str())
                                    .map(|align| match align {
                                        "L" => "Lawful",
                                        "N" => "Neutral",
                                        "C" => "Chaotic",
                                        "G" => "Good",
                                        "E" => "Evil",
                                        "U" => "Unaligned",
                                        "A" => "Any",
                                        _ => align,
                                    })
                                    .collect::<Vec<_>>()
                                    .join(" ")
                            })
                            .unwrap_or_else(|| "Unknown".to_string())
                    } else {
                        "Unknown".to_string()
                    }
                } else {
                    "Unknown".to_string()
                }
            },
            _ => "Unknown".to_string(),
        };
        
        // Parse CR for display and numeric value
        let (cr_display, cr_numeric) = match &monster.cr {
            Some(serde_json::Value::String(s)) => {
                let numeric = match s.as_str() {
                    "1/8" => 0.125,
                    "1/4" => 0.25,
                    "1/2" => 0.5,
                    "0" => 0.0,
                    num => num.parse::<f32>().unwrap_or(0.0),
                };
                (s.clone(), numeric)
            },
            Some(serde_json::Value::Object(obj)) => {
                if let Some(cr_str) = obj.get("cr").and_then(|v| v.as_str()) {
                    let numeric = match cr_str {
                        "1/8" => 0.125,
                        "1/4" => 0.25,
                        "1/2" => 0.5,
                        "0" => 0.0,
                        num => num.parse::<f32>().unwrap_or(0.0),
                    };
                    (cr_str.to_string(), numeric)
                } else {
                    ("0".to_string(), 0.0)
                }
            },
            _ => ("0".to_string(), 0.0),
        };
        
        // Get HP average (handle both object and number)
        let hp = match &monster.hp {
            Some(serde_json::Value::Object(obj)) => {
                obj.get("average")
                    .and_then(|v| v.as_u64())
                    .map(|v| v as u32)
                    .unwrap_or(0)
            },
            Some(serde_json::Value::Number(n)) => {
                n.as_u64().map(|v| v as u32).unwrap_or(0)
            },
            _ => 0,
        };
        
        // Get first AC value (handle both number and array)
        let ac = match &monster.ac {
            Some(serde_json::Value::Number(n)) => {
                n.as_u64().map(|v| v as u8).unwrap_or(10)
            },
            Some(serde_json::Value::Array(arr)) => {
                arr.first()
                    .and_then(|v| {
                        if let serde_json::Value::Object(obj) = v {
                            obj.get("ac").and_then(|ac| ac.as_u64()).map(|v| v as u8)
                        } else if let serde_json::Value::Number(n) = v {
                            n.as_u64().map(|v| v as u8)
                        } else {
                            None
                        }
                    })
                    .unwrap_or(10)
            },
            _ => 10,
        };
        
        // Get environments
        let environment = monster.environment.clone().unwrap_or_default();
        
        MonsterSummary {
            name: monster.name.clone(),
            source: monster.source.clone(),
            size,
            creature_type,
            alignment,
            cr: cr_display,
            cr_numeric,
            hp,
            ac,
            environment,
        }
    }
}

/// Simplified spell for search results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpellSummary {
    pub name: String,
    pub level: u8,
    pub school: String,
    pub source: String,
    pub concentration: bool,
    pub ritual: bool,
    pub casting_time: String,
    pub range: String,
    pub components: String,
    pub classes: Vec<String>,
    pub description: String,  // Add a brief description
}

impl From<&Spell> for SpellSummary {
    fn from(spell: &Spell) -> Self {
        // Format casting time
        let casting_time = spell.time.first()
            .map(|t| format!("{} {}", t.number, t.unit))
            .unwrap_or_else(|| "Unknown".to_string());
        
        // Format range
        let range = match &spell.range {
            SpellRange::Point { distance, .. } => {
                if let Some(amount) = distance.amount {
                    format!("{} {}", amount, distance.distance_type)
                } else {
                    distance.distance_type.clone()
                }
            },
            SpellRange::Special { range_type } => range_type.clone(),
        };
        
        // Format components
        let mut comp_parts = vec![];
        if spell.components.v { comp_parts.push("V"); }
        if spell.components.s { comp_parts.push("S"); }
        if spell.components.m.is_some() { comp_parts.push("M"); }
        let components = comp_parts.join(", ");
        
        // Extract class names
        let classes = spell.classes.as_ref()
            .map(|c| c.from_class_list.iter().map(|e| e.name.clone()).collect())
            .unwrap_or_default();
        
        // Extract first entry as description (usually the main spell text)
        let description = spell.entries.first()
            .and_then(|e| e.as_str())
            .map(|s| {
                // Truncate long descriptions for the summary
                if s.len() > 200 {
                    format!("{}...", &s[..197])
                } else {
                    s.to_string()
                }
            })
            .unwrap_or_else(|| "No description available".to_string());
        
        SpellSummary {
            name: spell.name.clone(),
            level: spell.level,
            school: spell.school.as_str().to_string(),
            source: spell.source.clone(),
            concentration: spell.duration.iter().any(|d| d.concentration.unwrap_or(false)),
            ritual: spell.meta.as_ref().map(|m| m.ritual).unwrap_or(false),
            casting_time,
            range,
            components,
            classes,
            description,
        }
    }
}