//! Spell catalog models

use serde::{Deserialize, Serialize};

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
    pub fn as_str(&self) -> &str {
        match self {
            Self::Abjuration => "Abjuration",
            Self::Conjuration => "Conjuration",
            Self::Divination => "Divination",
            Self::Enchantment => "Enchantment",
            Self::Evocation => "Evocation",
            Self::Illusion => "Illusion",
            Self::Necromancy => "Necromancy",
            Self::Transmutation => "Transmutation",
        }
    }
}

/// Casting time specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CastingTime {
    pub number: u32,
    pub unit: String,
    #[serde(default)]
    pub condition: Option<String>,
}

/// Spell range specification
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SpellRange {
    Point {
        #[serde(rename = "type")]
        range_type: String,
        distance: Distance,
    },
    Special {
        #[serde(rename = "type")]
        range_type: String,
    },
}

/// Distance specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Distance {
    #[serde(rename = "type")]
    pub distance_type: String,
    #[serde(default)]
    pub amount: Option<u32>,
}

/// Spell components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Components {
    #[serde(default)]
    pub v: Option<bool>,
    #[serde(default)]
    pub s: Option<bool>,
    #[serde(default)]
    pub m: Option<MaterialComponent>,
    #[serde(default)]
    pub r: Option<bool>,
}

/// Material component
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MaterialComponent {
    Text(String),
    Object {
        text: String,
        #[serde(default)]
        cost: Option<u32>,
        #[serde(default)]
        consume: Option<serde_json::Value>, // Can be bool or "optional"
    },
    Bool(bool), // Sometimes it's just true/false
}

/// Spell duration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Duration {
    #[serde(rename = "type")]
    pub duration_type: String,
    #[serde(default)]
    pub duration: Option<DurationValue>,
    #[serde(default)]
    pub concentration: Option<bool>,
    #[serde(default)]
    pub ends: Option<Vec<String>>,
}

/// Duration value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DurationValue {
    #[serde(rename = "type")]
    pub value_type: String,
    #[serde(default)]
    pub amount: Option<u32>,
    #[serde(default)]
    pub up_to: Option<bool>,
}

/// Classes that can cast the spell
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Classes {
    #[serde(rename = "fromClassList")]
    pub from_class_list: Option<Vec<ClassReference>>,
    #[serde(rename = "fromSubclass")]
    pub from_subclass: Option<Vec<SubclassReference>>,
}

/// Class reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassReference {
    pub name: String,
    pub source: String,
}

/// Subclass reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubclassReference {
    pub class: ClassReference,
    pub subclass: SubclassReference2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubclassReference2 {
    pub name: String,
    pub source: String,
}

/// Scaling level dice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalingLevelDice {
    pub label: String,
    pub scaling: std::collections::HashMap<String, String>,
}

/// Container for spell data from JSON files
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
    pub description: String,
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
        if spell.components.v.unwrap_or(false) { comp_parts.push("V"); }
        if spell.components.s.unwrap_or(false) { comp_parts.push("S"); }
        if spell.components.m.is_some() { comp_parts.push("M"); }
        let components = comp_parts.join(", ");
        
        // Extract classes
        let classes = spell.classes.as_ref()
            .and_then(|c| c.from_class_list.as_ref())
            .map(|list| list.iter().map(|c| c.name.clone()).collect())
            .unwrap_or_default();
        
        // Check for concentration and ritual
        let concentration = spell.duration.iter()
            .any(|d| d.concentration.unwrap_or(false));
        
        let ritual = spell.meta.as_ref()
            .map(|m| m.ritual)
            .unwrap_or(false);
        
        // Get first line of description for summary
        let description = spell.entries.first()
            .and_then(|e| e.as_str())
            .unwrap_or("")
            .chars()
            .take(200)
            .collect::<String>();
        
        SpellSummary {
            name: spell.name.clone(),
            level: spell.level,
            school: spell.school.as_str().to_string(),
            source: spell.source.clone(),
            concentration,
            ritual,
            casting_time,
            range,
            components,
            classes,
            description,
        }
    }
}