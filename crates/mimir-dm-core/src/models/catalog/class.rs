//! Class catalog models

use serde::{Deserialize, Serialize};

/// A D&D 5e character class
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Class {
    pub name: String,
    pub source: String,
    #[serde(default)]
    pub page: Option<u32>,
    #[serde(default)]
    pub srd: Option<bool>,
    #[serde(default)]
    pub basic_rules: Option<bool>,
    #[serde(default)]
    pub hd: Option<serde_json::Value>, // Can be HitDice object or something else
    #[serde(default)]
    pub proficiency: Option<serde_json::Value>, // Can be array or object
    #[serde(default)]
    pub class_features: Option<Vec<serde_json::Value>>, // Can be strings or objects
    #[serde(default)]
    pub starting_proficiencies: Option<serde_json::Value>,
    #[serde(default)]
    pub multiclassing: Option<serde_json::Value>,
    #[serde(default)]
    pub subclass_title: Option<String>,
    #[serde(default)]
    pub caster_progression: Option<String>, // "full", "1/2", "1/3", "pact"
    #[serde(default)]
    pub cantrip_progression: Option<Vec<u8>>,
    #[serde(default)]
    pub spells_known_progression: Option<Vec<u8>>,
    #[serde(default)]
    pub prepared_spells: Option<String>, // Formula for prepared spells
    #[serde(default)]
    pub spellcasting_ability: Option<String>, // "int", "wis", "cha"
    #[serde(default)]
    pub class_table_groups: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub starting_equipment: Option<serde_json::Value>,
    #[serde(default)]
    pub optionalfeature_progression: Option<Vec<serde_json::Value>>,
}

/// Hit dice specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HitDice {
    pub number: u8,
    pub faces: u8,
}

/// Starting proficiencies for a class
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartingProficiencies {
    #[serde(default)]
    pub armor: Option<Vec<String>>,
    #[serde(default)]
    pub weapons: Option<Vec<String>>,
    #[serde(default)]
    pub tools: Option<Vec<String>>,
    #[serde(default)]
    pub skills: Option<Vec<serde_json::Value>>, // Can be strings or choice objects
    #[serde(default)]
    pub saving_throws: Option<Vec<String>>,
}

/// Multiclassing requirements and benefits
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Multiclassing {
    #[serde(default)]
    pub requirements: Option<serde_json::Value>, // Complex requirement objects
    #[serde(default)]
    pub proficiencies_gained: Option<MulticlassingProficiencies>,
}

/// Proficiencies gained from multiclassing
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MulticlassingProficiencies {
    #[serde(default)]
    pub armor: Option<Vec<String>>,
    #[serde(default)]
    pub weapons: Option<Vec<String>>,
    #[serde(default)]
    pub tools: Option<Vec<String>>,
    #[serde(default)]
    pub skills: Option<Vec<serde_json::Value>>,
}

/// Starting equipment options
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartingEquipment {
    #[serde(default)]
    pub additional_from_background: Option<bool>,
    #[serde(default)]
    pub default: Option<Vec<String>>,
    #[serde(default)]
    pub gold_alternative: Option<String>,
}

/// Container for class data from JSON files
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassData {
    #[serde(default, rename = "class")]
    pub classes: Vec<Class>,
    #[serde(default)]
    pub subclass: Option<Vec<Subclass>>,
    #[serde(default, rename = "classFeature")]
    pub class_features: Option<Vec<ClassFeature>>,
    #[serde(default, rename = "subclassFeature")]
    pub subclass_features: Option<Vec<SubclassFeature>>,
}

/// A character subclass
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subclass {
    pub name: String,
    #[serde(default)]
    pub short_name: Option<String>,
    pub source: String,
    pub class_name: String,
    pub class_source: String,
    #[serde(default)]
    pub page: Option<u32>,
    #[serde(default)]
    pub subclass_features: Option<serde_json::Value>, // Can be array of strings or objects
    #[serde(default)]
    pub subclass_table_groups: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub caster_progression: Option<String>,
    #[serde(default)]
    pub spellcasting_ability: Option<String>,
    #[serde(default)]
    pub cantrip_progression: Option<Vec<u8>>,
    #[serde(default)]
    pub spells_known_progression: Option<Vec<u8>>,
    #[serde(default)]
    pub additional_spells: Option<Vec<serde_json::Value>>,
}

/// Class feature
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassFeature {
    pub name: String,
    pub source: String,
    pub class_name: String,
    pub class_source: String,
    pub level: u8,
    #[serde(default)]
    pub entries: Vec<serde_json::Value>,
    pub page: Option<u32>,
    #[serde(default)]
    pub srd: Option<bool>,
}

/// Subclass feature
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubclassFeature {
    pub name: String,
    pub source: String,
    pub class_name: String,
    pub class_source: String,
    #[serde(default)]
    pub subclass_short_name: Option<String>,
    pub subclass_source: String,
    pub level: u8,
    #[serde(default)]
    pub entries: Vec<serde_json::Value>,
    pub page: Option<u32>,
}

/// Container for class feature data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassFeatureData {
    #[serde(default)]
    pub class_feature: Option<Vec<ClassFeature>>,
    #[serde(default)]
    pub subclass_feature: Option<Vec<SubclassFeature>>,
}

/// Class fluff (descriptive text)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassFluff {
    pub name: String,
    pub source: String,
    pub entries: Vec<serde_json::Value>,
    #[serde(default)]
    pub images: Option<Vec<serde_json::Value>>,
}

/// Subclass fluff (descriptive text)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubclassFluff {
    pub name: String,
    #[serde(default)]
    pub short_name: Option<String>,
    pub source: String,
    pub class_name: String,
    pub class_source: String,
    pub entries: Vec<serde_json::Value>,
    #[serde(default)]
    pub images: Option<Vec<serde_json::Value>>,
}

/// Container for class fluff data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassFluffData {
    #[serde(default)]
    pub class_fluff: Option<Vec<ClassFluff>>,
    #[serde(default)]
    pub subclass_fluff: Option<Vec<SubclassFluff>>,
}

/// Simplified class for search results
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassSummary {
    pub name: String,
    pub source: String,
    pub page: Option<u32>,
    pub hit_dice: String,
    pub proficiency: String,
    pub primary_ability: String,
    pub spellcasting_ability: Option<String>,
    pub table_groups: Option<Vec<serde_json::Value>>,
    pub subclass_title: Option<String>,
    pub description: String,
}

impl From<&Class> for ClassSummary {
    fn from(class: &Class) -> Self {
        // Format hit die
        let hit_dice = if let Some(hd) = &class.hd {
            if let Some(obj) = hd.as_object() {
                if let (Some(number), Some(faces)) = (obj.get("number"), obj.get("faces")) {
                    format!("{}d{}", 
                        number.as_u64().unwrap_or(1), 
                        faces.as_u64().unwrap_or(6))
                } else {
                    "1d6".to_string()
                }
            } else {
                "1d6".to_string()
            }
        } else {
            "1d6".to_string()
        };
        
        // Format proficiency - extract saving throws from startingProficiencies
        let proficiency = if let Some(start_prof) = &class.starting_proficiencies {
            if let Some(obj) = start_prof.as_object() {
                if let Some(saves_val) = obj.get("savingThrows") {
                    if let Some(saves_arr) = saves_val.as_array() {
                        let saves: Vec<String> = saves_arr.iter()
                            .filter_map(|v| v.as_str())
                            .map(|s| match s {
                                "str" => "STR",
                                "dex" => "DEX",
                                "con" => "CON",
                                "int" => "INT",
                                "wis" => "WIS",
                                "cha" => "CHA",
                                _ => s,
                            })
                            .map(|s| s.to_string())
                            .collect();
                        if !saves.is_empty() {
                            saves.join(", ")
                        } else {
                            "None".to_string()
                        }
                    } else {
                        "None".to_string()
                    }
                } else {
                    "None".to_string()
                }
            } else {
                "None".to_string()
            }
        } else {
            "None".to_string()
        };
        
        // Determine primary ability based on class name
        let primary_ability = match class.name.to_lowercase().as_str() {
            "barbarian" | "fighter" => "Strength".to_string(),
            "rogue" | "ranger" | "monk" => "Dexterity".to_string(),  
            "wizard" => "Intelligence".to_string(),
            "cleric" | "druid" => "Wisdom".to_string(),
            "bard" | "paladin" | "sorcerer" | "warlock" => "Charisma".to_string(),
            _ => "Various".to_string(),
        };
        
        
        // Get a simple description from class features if available
        let description = if let Some(features) = &class.class_features {
            features.first()
                .and_then(|f| f.as_str())
                .unwrap_or("")
                .chars()
                .take(200)
                .collect::<String>()
        } else {
            format!("A {} class", class.name)
        };
        
        ClassSummary {
            name: class.name.clone(),
            source: class.source.clone(),
            page: class.page,
            hit_dice,
            proficiency,
            primary_ability,
            spellcasting_ability: class.spellcasting_ability.clone(),
            table_groups: class.class_table_groups.clone(),
            subclass_title: class.subclass_title.clone(),
            description,
        }
    }
}