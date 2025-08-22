//! Monster catalog models

use serde::{Deserialize, Serialize};

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
    
    // Ability scores
    pub str: Option<u8>,
    pub dex: Option<u8>,
    pub con: Option<u8>,
    pub int: Option<u8>,
    pub wis: Option<u8>,
    pub cha: Option<u8>,
    
    // Saves and skills
    pub save: Option<Saves>,
    pub skill: Option<Skills>,
    
    // Resistances and immunities
    pub damage_vulnerabilities: Option<Vec<String>>,
    pub damage_resistances: Option<Vec<String>>,
    pub damage_immunities: Option<Vec<String>>,
    pub condition_immunities: Option<Vec<String>>,
    
    // Senses
    pub senses: Option<Vec<String>>,
    pub passive: Option<u8>,
    pub languages: Option<Vec<String>>,
    
    // Challenge rating
    pub cr: Option<serde_json::Value>,  // Can be string or object
    
    // Traits, actions, etc.
    pub trait_entries: Option<Vec<serde_json::Value>>,
    pub action: Option<Vec<serde_json::Value>>,
    pub bonus: Option<Vec<serde_json::Value>>,
    pub reaction: Option<Vec<serde_json::Value>>,
    pub legendary: Option<Vec<serde_json::Value>>,
    pub legendary_group: Option<serde_json::Value>,
    pub mythic: Option<Vec<serde_json::Value>>,
    
    // Environment
    pub environment: Option<Vec<String>>,
    
    // Flags
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
pub struct Speed {
    pub walk: Option<serde_json::Value>,  // Can be number or object
    pub burrow: Option<serde_json::Value>,
    pub climb: Option<serde_json::Value>,
    pub fly: Option<serde_json::Value>,  // Can be number or object with condition
    pub hover: Option<bool>,
    pub swim: Option<serde_json::Value>,
    #[serde(rename = "canHover")]
    pub can_hover: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Saves {
    pub str: Option<String>,
    pub dex: Option<String>,
    pub con: Option<String>,
    pub int: Option<String>,
    pub wis: Option<String>,
    pub cha: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skills {
    pub acrobatics: Option<String>,
    pub arcana: Option<String>,
    pub athletics: Option<String>,
    pub deception: Option<String>,
    pub history: Option<String>,
    pub insight: Option<String>,
    pub intimidation: Option<String>,
    pub investigation: Option<String>,
    pub medicine: Option<String>,
    pub nature: Option<String>,
    pub perception: Option<String>,
    pub performance: Option<String>,
    pub persuasion: Option<String>,
    pub religion: Option<String>,
    pub sleight_of_hand: Option<String>,
    pub stealth: Option<String>,
    pub survival: Option<String>,
}

/// Container for monster data from 5etools JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterData {
    pub monster: Vec<Monster>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterFluff {
    pub name: String,
    pub source: String,
    pub entries: Option<Vec<serde_json::Value>>,
    pub images: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterFluffData {
    #[serde(rename = "monsterFluff")]
    pub monster_fluff: Vec<MonsterFluff>,
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
    pub description: String,
}

impl From<&Monster> for MonsterSummary {
    fn from(monster: &Monster) -> Self {
        // Extract size
        let size = monster.size.as_ref()
            .and_then(|s| s.first())
            .cloned()
            .unwrap_or_else(|| "Medium".to_string());
        
        // Extract creature type
        let creature_type = if let Some(ct) = &monster.creature_type {
            match ct {
                serde_json::Value::String(s) => s.clone(),
                serde_json::Value::Object(obj) => {
                    obj.get("type")
                        .and_then(|t| t.as_str())
                        .unwrap_or("Unknown")
                        .to_string()
                },
                _ => "Unknown".to_string(),
            }
        } else {
            "Unknown".to_string()
        };
        
        // Extract alignment
        let alignment = if let Some(al) = &monster.alignment {
            match al {
                serde_json::Value::Array(arr) => {
                    arr.first()
                        .and_then(|v| v.as_str())
                        .unwrap_or("unaligned")
                        .to_string()
                },
                _ => "unaligned".to_string(),
            }
        } else {
            "unaligned".to_string()
        };
        
        // Extract CR
        let (cr, cr_numeric) = if let Some(cr_val) = &monster.cr {
            match cr_val {
                serde_json::Value::String(s) => {
                    let numeric = match s.as_str() {
                        "1/8" => 0.125,
                        "1/4" => 0.25,
                        "1/2" => 0.5,
                        _ => s.parse().unwrap_or(0.0),
                    };
                    (s.clone(), numeric as f32)
                },
                serde_json::Value::Object(obj) => {
                    let cr_str = obj.get("cr")
                        .and_then(|c| c.as_str())
                        .unwrap_or("0")
                        .to_string();
                    let numeric = match cr_str.as_str() {
                        "1/8" => 0.125,
                        "1/4" => 0.25,
                        "1/2" => 0.5,
                        _ => cr_str.parse().unwrap_or(0.0),
                    };
                    (cr_str, numeric as f32)
                },
                _ => ("0".to_string(), 0.0),
            }
        } else {
            ("0".to_string(), 0.0)
        };
        
        // Extract HP
        let hp = if let Some(hp_val) = &monster.hp {
            match hp_val {
                serde_json::Value::Number(n) => n.as_u64().unwrap_or(1) as u32,
                serde_json::Value::Object(obj) => {
                    obj.get("average")
                        .and_then(|a| a.as_u64())
                        .unwrap_or(1) as u32
                },
                _ => 1,
            }
        } else {
            1
        };
        
        // Extract AC
        let ac = if let Some(ac_val) = &monster.ac {
            match ac_val {
                serde_json::Value::Number(n) => n.as_u64().unwrap_or(10) as u8,
                serde_json::Value::Array(arr) => {
                    arr.first()
                        .and_then(|v| v.get("ac"))
                        .and_then(|a| a.as_u64())
                        .unwrap_or(10) as u8
                },
                _ => 10,
            }
        } else {
            10
        };
        
        // Extract environment
        let environment = monster.environment.clone().unwrap_or_default();
        
        // Get first trait or action for description
        let description = monster.trait_entries.as_ref()
            .and_then(|t| t.first())
            .and_then(|e| e.get("entries"))
            .and_then(|entries| entries.get(0))
            .and_then(|e| e.as_str())
            .unwrap_or("")
            .chars()
            .take(200)
            .collect::<String>();
        
        MonsterSummary {
            name: monster.name.clone(),
            source: monster.source.clone(),
            size,
            creature_type,
            alignment,
            cr,
            cr_numeric,
            hp,
            ac,
            environment,
            description,
        }
    }
}