//! Feat model representing character feats from D&D sources

use crate::schema::feats;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents a character feat
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = feats)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Feat {
    pub id: String,
    pub name: String,
    pub rule_system_id: String,
    pub source_id: String,
    pub page: Option<i32>,
    pub prerequisites: Option<String>,
    pub ability_increases: Option<String>,
    pub feat_type: Option<String>,
    pub entries: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Feat prerequisite structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Prerequisites {
    pub abilities: Option<Vec<AbilityPrereq>>,
    pub level: Option<u8>,
    pub classes: Option<Vec<String>>,
    pub races: Option<Vec<String>>,
    pub feats: Option<Vec<String>>,
    pub spells: Option<Vec<String>>,
    pub other: Option<Vec<String>>,
}

/// Ability score prerequisite
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AbilityPrereq {
    pub ability: String, // "str", "dex", "con", "int", "wis", "cha"
    pub score: u8,
}

/// Ability score increases from feat
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AbilityIncreases {
    pub fixed: Option<Vec<AbilityIncrease>>,
    pub choices: Option<Vec<AbilityChoice>>,
}

/// Fixed ability increase
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AbilityIncrease {
    pub ability: String,
    pub increase: u8,
}

/// Ability choice for increases
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AbilityChoice {
    pub from: Vec<String>,
    pub count: u8,
    pub increase: u8,
}

/// D&D 2024 feat types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FeatType {
    General,
    Origin,
    FightingStyle,
    Epic,
}

impl Feat {
    /// Create a new Feat with required fields
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
            prerequisites: None,
            ability_increases: None,
            feat_type: None,
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

    /// Set prerequisites
    pub fn with_prerequisites(mut self, prereqs: Prerequisites) -> Result<Self, serde_json::Error> {
        self.prerequisites = Some(serde_json::to_string(&prereqs)?);
        Ok(self)
    }

    /// Set ability increases
    pub fn with_ability_increases(mut self, increases: AbilityIncreases) -> Result<Self, serde_json::Error> {
        self.ability_increases = Some(serde_json::to_string(&increases)?);
        Ok(self)
    }

    /// Set feat type
    pub fn with_feat_type(mut self, feat_type: FeatType) -> Self {
        self.feat_type = Some(feat_type.to_string());
        self
    }

    /// Set feat type from string
    pub fn with_feat_type_str(mut self, feat_type: String) -> Self {
        self.feat_type = Some(feat_type);
        self
    }

    /// Get prerequisites as typed struct
    pub fn prerequisites_typed(&self) -> Result<Option<Prerequisites>, serde_json::Error> {
        match &self.prerequisites {
            Some(json_str) => Ok(Some(serde_json::from_str(json_str)?)),
            None => Ok(None),
        }
    }

    /// Get ability increases as typed struct
    pub fn ability_increases_typed(&self) -> Result<Option<AbilityIncreases>, serde_json::Error> {
        match &self.ability_increases {
            Some(json_str) => Ok(Some(serde_json::from_str(json_str)?)),
            None => Ok(None),
        }
    }

    /// Get entries as JSON value
    pub fn entries_value(&self) -> Result<Value, serde_json::Error> {
        serde_json::from_str(&self.entries)
    }

    /// Check if feat has prerequisites
    pub fn has_prerequisites(&self) -> bool {
        self.prerequisites.is_some()
    }

    /// Check if feat provides ability increases
    pub fn provides_ability_increases(&self) -> bool {
        self.ability_increases.is_some()
    }

    /// Get feat type as enum
    pub fn feat_type_enum(&self) -> Option<FeatType> {
        match self.feat_type.as_deref() {
            Some("general") => Some(FeatType::General),
            Some("origin") => Some(FeatType::Origin),
            Some("fighting-style") => Some(FeatType::FightingStyle),
            Some("epic") => Some(FeatType::Epic),
            _ => None,
        }
    }

    /// Check if feat is available at character creation (origin feat)
    pub fn is_origin_feat(&self) -> bool {
        matches!(self.feat_type.as_deref(), Some("origin"))
    }

    /// Check if feat is a fighting style
    pub fn is_fighting_style(&self) -> bool {
        matches!(self.feat_type.as_deref(), Some("fighting-style"))
    }

    /// Check if feat is epic level (20th level+)
    pub fn is_epic_feat(&self) -> bool {
        matches!(self.feat_type.as_deref(), Some("epic"))
    }

    /// Check if character meets ability prerequisites
    pub fn meets_ability_prerequisites(&self, abilities: &std::collections::HashMap<String, u8>) -> Result<bool, serde_json::Error> {
        let prereqs = match self.prerequisites_typed()? {
            Some(p) => p,
            None => return Ok(true), // No prerequisites
        };

        if let Some(ability_prereqs) = prereqs.abilities {
            for prereq in ability_prereqs {
                let score = abilities.get(&prereq.ability).unwrap_or(&0);
                if *score < prereq.score {
                    return Ok(false);
                }
            }
        }

        Ok(true)
    }

    /// Check if character meets level prerequisite
    pub fn meets_level_prerequisite(&self, character_level: u8) -> Result<bool, serde_json::Error> {
        let prereqs = match self.prerequisites_typed()? {
            Some(p) => p,
            None => return Ok(true), // No prerequisites
        };

        if let Some(required_level) = prereqs.level {
            return Ok(character_level >= required_level);
        }

        Ok(true)
    }

    /// Check if character meets class prerequisites
    pub fn meets_class_prerequisites(&self, character_classes: &[String]) -> Result<bool, serde_json::Error> {
        let prereqs = match self.prerequisites_typed()? {
            Some(p) => p,
            None => return Ok(true), // No prerequisites
        };

        if let Some(required_classes) = prereqs.classes {
            // Character must have at least one of the required classes
            return Ok(required_classes.iter().any(|req_class| {
                character_classes.iter().any(|char_class| char_class == req_class)
            }));
        }

        Ok(true)
    }

    /// Get total ability increase points from this feat
    pub fn total_ability_increase_points(&self) -> Result<u8, serde_json::Error> {
        let increases = match self.ability_increases_typed()? {
            Some(i) => i,
            None => return Ok(0),
        };

        let mut total = 0;

        if let Some(fixed) = increases.fixed {
            total += fixed.iter().map(|inc| inc.increase).sum::<u8>();
        }

        if let Some(choices) = increases.choices {
            total += choices.iter().map(|choice| choice.count * choice.increase).sum::<u8>();
        }

        Ok(total)
    }
}

impl std::fmt::Display for FeatType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FeatType::General => write!(f, "general"),
            FeatType::Origin => write!(f, "origin"),
            FeatType::FightingStyle => write!(f, "fighting-style"),
            FeatType::Epic => write!(f, "epic"),
        }
    }
}

impl std::str::FromStr for FeatType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "general" => Ok(FeatType::General),
            "origin" => Ok(FeatType::Origin),
            "fighting-style" => Ok(FeatType::FightingStyle),
            "epic" => Ok(FeatType::Epic),
            _ => Err(format!("Unknown feat type: {}", s)),
        }
    }
}