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