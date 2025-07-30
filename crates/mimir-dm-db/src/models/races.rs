//! Race model representing playable races and subraces

use crate::schema::races;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents a playable race or subrace
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = races)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Race {
    pub id: String,
    pub name: String,
    pub rule_system_id: String,
    pub source_id: String,
    pub page: Option<i32>,
    pub race_type: String, // "race" or "subrace"
    pub parent_race_id: Option<String>,
    pub size: Option<String>,
    pub speed: Option<String>,
    pub ability_scores: Option<String>,
    pub age: Option<String>,
    pub alignment_tendency: Option<String>,
    pub language_proficiencies: Option<String>,
    pub trait_tags: Option<String>,
    pub entries: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Speed structure for movement types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Speed {
    pub walk: Option<u32>,
    pub fly: Option<u32>,
    pub swim: Option<u32>,
    pub climb: Option<u32>,
    pub burrow: Option<u32>,
}

/// Ability score adjustments
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AbilityScores {
    pub strength: Option<i8>,
    pub dexterity: Option<i8>,
    pub constitution: Option<i8>,
    pub intelligence: Option<i8>,
    pub wisdom: Option<i8>,
    pub charisma: Option<i8>,
    pub choose: Option<AbilityScoreChoice>,
}

/// Choice for ability score increases
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AbilityScoreChoice {
    pub count: u8,
    pub amount: i8,
    pub from: Option<Vec<String>>, // If None, can choose any
}

/// Age information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Age {
    pub mature: u32,
    pub max: u32,
}

impl Race {
    /// Create a new Race with required fields
    pub fn new(
        id: String, 
        name: String, 
        rule_system_id: String, 
        source_id: String,
        race_type: String,
        entries: Value
    ) -> Result<Self, serde_json::Error> {
        let now = chrono::Utc::now().naive_utc();
        Ok(Self {
            id,
            name,
            rule_system_id,
            source_id,
            page: None,
            race_type,
            parent_race_id: None,
            size: None,
            speed: None,
            ability_scores: None,
            age: None,
            alignment_tendency: None,
            language_proficiencies: None,
            trait_tags: None,
            entries: serde_json::to_string(&entries)?,
            created_at: now,
            updated_at: now,
        })
    }

    /// Set the page number
    pub fn with_page(mut self, page: i32) -> Self {
        self.page = Some(page);
        self
    }

    /// Set the parent race ID (for subraces)
    pub fn with_parent_race(mut self, parent_race_id: String) -> Self {
        self.parent_race_id = Some(parent_race_id);
        self
    }

    /// Set the size
    pub fn with_size(mut self, size: String) -> Self {
        self.size = Some(size);
        self
    }

    /// Set the speed from a Speed struct
    pub fn with_speed(mut self, speed: Speed) -> Result<Self, serde_json::Error> {
        self.speed = Some(serde_json::to_string(&speed)?);
        Ok(self)
    }

    /// Set the ability scores
    pub fn with_ability_scores(mut self, scores: AbilityScores) -> Result<Self, serde_json::Error> {
        self.ability_scores = Some(serde_json::to_string(&scores)?);
        Ok(self)
    }

    /// Set age information
    pub fn with_age(mut self, age: Age) -> Result<Self, serde_json::Error> {
        self.age = Some(serde_json::to_string(&age)?);
        Ok(self)
    }

    /// Set alignment tendency
    pub fn with_alignment_tendency(mut self, alignment: String) -> Self {
        self.alignment_tendency = Some(alignment);
        self
    }

    /// Set language proficiencies
    pub fn with_language_proficiencies<T: Serialize>(mut self, languages: T) -> Result<Self, serde_json::Error> {
        self.language_proficiencies = Some(serde_json::to_string(&languages)?);
        Ok(self)
    }

    /// Set trait tags
    pub fn with_trait_tags(mut self, tags: Vec<String>) -> Result<Self, serde_json::Error> {
        self.trait_tags = Some(serde_json::to_string(&tags)?);
        Ok(self)
    }

    /// Get speed as typed struct
    pub fn speed_typed(&self) -> Result<Option<Speed>, serde_json::Error> {
        match &self.speed {
            Some(json_str) => Ok(Some(serde_json::from_str(json_str)?)),
            None => Ok(None),
        }
    }

    /// Get ability scores as typed struct
    pub fn ability_scores_typed(&self) -> Result<Option<AbilityScores>, serde_json::Error> {
        match &self.ability_scores {
            Some(json_str) => Ok(Some(serde_json::from_str(json_str)?)),
            None => Ok(None),
        }
    }

    /// Get age as typed struct
    pub fn age_typed(&self) -> Result<Option<Age>, serde_json::Error> {
        match &self.age {
            Some(json_str) => Ok(Some(serde_json::from_str(json_str)?)),
            None => Ok(None),
        }
    }

    /// Get language proficiencies as vector
    pub fn language_proficiencies_vec(&self) -> Result<Option<Vec<String>>, serde_json::Error> {
        match &self.language_proficiencies {
            Some(json_str) => Ok(Some(serde_json::from_str(json_str)?)),
            None => Ok(None),
        }
    }

    /// Get trait tags as vector
    pub fn trait_tags_vec(&self) -> Result<Option<Vec<String>>, serde_json::Error> {
        match &self.trait_tags {
            Some(json_str) => Ok(Some(serde_json::from_str(json_str)?)),
            None => Ok(None),
        }
    }

    /// Get entries as JSON value
    pub fn entries_value(&self) -> Result<Value, serde_json::Error> {
        serde_json::from_str(&self.entries)
    }

    /// Check if this is a base race
    pub fn is_base_race(&self) -> bool {
        self.race_type == "race"
    }

    /// Check if this is a subrace
    pub fn is_subrace(&self) -> bool {
        self.race_type == "subrace"
    }
}