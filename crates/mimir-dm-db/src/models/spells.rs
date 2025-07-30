//! Spell model representing spells from D&D sources

use crate::schema::spells;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents a D&D spell
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, QueryableByName)]
#[diesel(table_name = spells)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Spell {
    pub id: String,
    pub name: String,
    pub rule_system_id: String,
    pub source_id: String,
    pub page: Option<i32>,
    pub level: Option<i32>,
    pub school: Option<String>,
    pub casting_time: Option<String>,
    pub range: Option<String>,
    pub components: Option<String>,
    pub duration: Option<String>,
    pub is_ritual: bool,
    pub is_concentration: bool,
    pub saving_throw: Option<String>,
    pub damage_type: Option<String>,
    pub entries: String,
    pub upcast_info: Option<String>,
    pub classes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Casting time structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CastingTime {
    pub number: u32,
    pub unit: String, // "action", "bonus action", "reaction", "minute", "hour"
    pub condition: Option<String>, // For reactions
}

/// Range structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Range {
    #[serde(rename = "type")]
    pub range_type: String, // "point", "sphere", "cube", "cone", "line", "self", "touch", "sight", "unlimited"
    pub distance: Option<Distance>,
}

/// Distance structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Distance {
    #[serde(rename = "type")]
    pub distance_type: String, // "feet", "miles", "self", "touch", "sight", "unlimited"
    pub amount: Option<u32>,
}

/// Components structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Components {
    pub v: Option<bool>, // Verbal
    pub s: Option<bool>, // Somatic
    pub m: Option<String>, // Material (description)
}

/// Duration structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Duration {
    #[serde(rename = "type")]
    pub duration_type: String, // "instant", "timed", "permanent", "dispel"
    pub duration: Option<TimeDuration>,
    pub concentration: Option<bool>,
    pub ends: Option<Vec<String>>, // End conditions
}

/// Time duration structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimeDuration {
    #[serde(rename = "type")]
    pub time_type: String, // "turn", "round", "minute", "hour", "day"
    pub amount: u32,
    pub upTo: Option<bool>, // "up to X minutes"
}

/// Upcast information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpcastInfo {
    pub entries: Vec<Value>,
}

/// Spell schools
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SpellSchool {
    Abjuration,
    Conjuration,
    Divination,
    Enchantment,
    Evocation,
    Illusion,
    Necromancy,
    Transmutation,
}

impl Spell {
    /// Create a new Spell with required fields
    pub fn new(
        id: String,
        name: String,
        rule_system_id: String,
        source_id: String,
        entries: Value,
    ) -> Result<Self, serde_json::Error> {
        let now = chrono::Utc::now().naive_utc();
        Ok(Self {
            id,
            name,
            rule_system_id,
            source_id,
            page: None,
            level: None,
            school: None,
            casting_time: None,
            range: None,
            components: None,
            duration: None,
            is_ritual: false,
            is_concentration: false,
            saving_throw: None,
            damage_type: None,
            entries: serde_json::to_string(&entries)?,
            upcast_info: None,
            classes: None,
            created_at: now,
            updated_at: now,
        })
    }

    /// Set the page number
    pub fn with_page(mut self, page: i32) -> Self {
        self.page = Some(page);
        self
    }

    /// Set the spell level
    pub fn with_level(mut self, level: i32) -> Self {
        self.level = Some(level);
        self
    }

    /// Set the spell school
    pub fn with_school(mut self, school: SpellSchool) -> Self {
        self.school = Some(school.to_string());
        self
    }

    /// Set the spell school from string
    pub fn with_school_str(mut self, school: String) -> Self {
        self.school = Some(school);
        self
    }

    /// Set casting time
    pub fn with_casting_time(mut self, casting_time: CastingTime) -> Result<Self, serde_json::Error> {
        self.casting_time = Some(serde_json::to_string(&casting_time)?);
        Ok(self)
    }

    /// Set range
    pub fn with_range(mut self, range: Range) -> Result<Self, serde_json::Error> {
        self.range = Some(serde_json::to_string(&range)?);
        Ok(self)
    }

    /// Set components
    pub fn with_components(mut self, components: Components) -> Result<Self, serde_json::Error> {
        self.components = Some(serde_json::to_string(&components)?);
        Ok(self)
    }

    /// Set duration
    pub fn with_duration(mut self, duration: Duration) -> Result<Self, serde_json::Error> {
        self.is_concentration = duration.concentration.unwrap_or(false);
        self.duration = Some(serde_json::to_string(&duration)?);
        Ok(self)
    }

    /// Set ritual flag
    pub fn with_ritual(mut self, is_ritual: bool) -> Self {
        self.is_ritual = is_ritual;
        self
    }

    /// Set saving throw types
    pub fn with_saving_throw(mut self, saving_throws: Vec<String>) -> Result<Self, serde_json::Error> {
        self.saving_throw = Some(serde_json::to_string(&saving_throws)?);
        Ok(self)
    }

    /// Set damage types
    pub fn with_damage_type(mut self, damage_types: Vec<String>) -> Result<Self, serde_json::Error> {
        self.damage_type = Some(serde_json::to_string(&damage_types)?);
        Ok(self)
    }

    /// Set upcast information
    pub fn with_upcast_info(mut self, upcast: UpcastInfo) -> Result<Self, serde_json::Error> {
        self.upcast_info = Some(serde_json::to_string(&upcast)?);
        Ok(self)
    }

    /// Set classes that can cast this spell
    pub fn with_classes(mut self, classes: Vec<String>) -> Result<Self, serde_json::Error> {
        self.classes = Some(serde_json::to_string(&classes)?);
        Ok(self)
    }

    /// Get casting time as typed struct
    pub fn casting_time_typed(&self) -> Result<Option<CastingTime>, serde_json::Error> {
        match &self.casting_time {
            Some(json_str) => Ok(Some(serde_json::from_str(json_str)?)),
            None => Ok(None),
        }
    }

    /// Get range as typed struct
    pub fn range_typed(&self) -> Result<Option<Range>, serde_json::Error> {
        match &self.range {
            Some(json_str) => Ok(Some(serde_json::from_str(json_str)?)),
            None => Ok(None),
        }
    }

    /// Get components as typed struct
    pub fn components_typed(&self) -> Result<Option<Components>, serde_json::Error> {
        match &self.components {
            Some(json_str) => Ok(Some(serde_json::from_str(json_str)?)),
            None => Ok(None),
        }
    }

    /// Get duration as typed struct
    pub fn duration_typed(&self) -> Result<Option<Duration>, serde_json::Error> {
        match &self.duration {
            Some(json_str) => Ok(Some(serde_json::from_str(json_str)?)),
            None => Ok(None),
        }
    }

    /// Get saving throw types as vector
    pub fn saving_throws_vec(&self) -> Result<Option<Vec<String>>, serde_json::Error> {
        match &self.saving_throw {
            Some(json_str) => Ok(Some(serde_json::from_str(json_str)?)),
            None => Ok(None),
        }
    }

    /// Get damage types as vector
    pub fn damage_types_vec(&self) -> Result<Option<Vec<String>>, serde_json::Error> {
        match &self.damage_type {
            Some(json_str) => Ok(Some(serde_json::from_str(json_str)?)),
            None => Ok(None),
        }
    }

    /// Get upcast information as typed struct
    pub fn upcast_info_typed(&self) -> Result<Option<UpcastInfo>, serde_json::Error> {
        match &self.upcast_info {
            Some(json_str) => Ok(Some(serde_json::from_str(json_str)?)),
            None => Ok(None),
        }
    }

    /// Get classes as vector
    pub fn classes_vec(&self) -> Result<Option<Vec<String>>, serde_json::Error> {
        match &self.classes {
            Some(json_str) => Ok(Some(serde_json::from_str(json_str)?)),
            None => Ok(None),
        }
    }

    /// Get entries as JSON value
    pub fn entries_value(&self) -> Result<Value, serde_json::Error> {
        serde_json::from_str(&self.entries)
    }

    /// Get spell level
    pub fn spell_level(&self) -> i32 {
        self.level.unwrap_or(0)
    }

    /// Check if spell is a cantrip (level 0)
    pub fn is_cantrip(&self) -> bool {
        self.spell_level() == 0
    }

    /// Check if spell can be cast as a ritual
    pub fn can_be_ritual(&self) -> bool {
        self.is_ritual
    }

    /// Check if spell requires concentration
    pub fn requires_concentration(&self) -> bool {
        self.is_concentration
    }

    /// Get spell school as enum
    pub fn school_enum(&self) -> Option<SpellSchool> {
        match self.school.as_deref() {
            Some("A") | Some("abjuration") => Some(SpellSchool::Abjuration),
            Some("C") | Some("conjuration") => Some(SpellSchool::Conjuration),
            Some("D") | Some("divination") => Some(SpellSchool::Divination),
            Some("E") | Some("enchantment") => Some(SpellSchool::Enchantment),
            Some("V") | Some("evocation") => Some(SpellSchool::Evocation),
            Some("I") | Some("illusion") => Some(SpellSchool::Illusion),
            Some("N") | Some("necromancy") => Some(SpellSchool::Necromancy),
            Some("T") | Some("transmutation") => Some(SpellSchool::Transmutation),
            _ => None,
        }
    }

    /// Check if spell can be cast by a specific class
    pub fn can_be_cast_by_class(&self, class_name: &str) -> Result<bool, serde_json::Error> {
        let classes = match self.classes_vec()? {
            Some(c) => c,
            None => return Ok(false),
        };
        
        Ok(classes.iter().any(|c| c.to_lowercase() == class_name.to_lowercase()))
    }

    /// Check if spell has verbal components
    pub fn has_verbal_components(&self) -> Result<bool, serde_json::Error> {
        let components = match self.components_typed()? {
            Some(c) => c,
            None => return Ok(false),
        };
        
        Ok(components.v.unwrap_or(false))
    }

    /// Check if spell has somatic components
    pub fn has_somatic_components(&self) -> Result<bool, serde_json::Error> {
        let components = match self.components_typed()? {
            Some(c) => c,
            None => return Ok(false),
        };
        
        Ok(components.s.unwrap_or(false))
    }

    /// Check if spell has material components
    pub fn has_material_components(&self) -> Result<bool, serde_json::Error> {
        let components = match self.components_typed()? {
            Some(c) => c,
            None => return Ok(false),
        };
        
        Ok(components.m.is_some())
    }

    /// Get material component description
    pub fn material_component_description(&self) -> Result<Option<String>, serde_json::Error> {
        let components = match self.components_typed()? {
            Some(c) => c,
            None => return Ok(None),
        };
        
        Ok(components.m)
    }

    /// Check if spell has upcast effects
    pub fn has_upcast_effects(&self) -> bool {
        self.upcast_info.is_some()
    }

    /// Check if spell deals damage
    pub fn deals_damage(&self) -> bool {
        self.damage_type.is_some()
    }

    /// Check if spell allows saving throw
    pub fn allows_saving_throw(&self) -> bool {
        self.saving_throw.is_some()
    }
}

impl std::fmt::Display for SpellSchool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpellSchool::Abjuration => write!(f, "A"),
            SpellSchool::Conjuration => write!(f, "C"),
            SpellSchool::Divination => write!(f, "D"),
            SpellSchool::Enchantment => write!(f, "E"),
            SpellSchool::Evocation => write!(f, "V"),
            SpellSchool::Illusion => write!(f, "I"),
            SpellSchool::Necromancy => write!(f, "N"),
            SpellSchool::Transmutation => write!(f, "T"),
        }
    }
}

impl std::str::FromStr for SpellSchool {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "A" | "ABJURATION" => Ok(SpellSchool::Abjuration),
            "C" | "CONJURATION" => Ok(SpellSchool::Conjuration),
            "D" | "DIVINATION" => Ok(SpellSchool::Divination),
            "E" | "ENCHANTMENT" => Ok(SpellSchool::Enchantment),
            "V" | "EVOCATION" => Ok(SpellSchool::Evocation),
            "I" | "ILLUSION" => Ok(SpellSchool::Illusion),
            "N" | "NECROMANCY" => Ok(SpellSchool::Necromancy),
            "T" | "TRANSMUTATION" => Ok(SpellSchool::Transmutation),
            _ => Err(format!("Unknown spell school: {}", s)),
        }
    }
}