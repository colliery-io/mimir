//! Creature model representing monsters and NPCs from D&D sources

use crate::schema::creatures;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents a D&D creature (monster, NPC, etc.)
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, QueryableByName)]
#[diesel(table_name = creatures)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Creature {
    pub id: String,
    pub name: String,
    pub rule_system_id: String,
    pub source_id: String,
    pub page: Option<i32>,
    pub size: Option<String>,
    #[diesel(column_name = creature_type)]
    pub creature_type: Option<String>,
    pub type_tags: Option<String>,
    pub alignment: Option<String>,
    pub armor_class: Option<String>,
    pub hit_points: Option<String>,
    pub speed: Option<String>,
    pub ability_scores: Option<String>,
    pub saving_throws: Option<String>,
    pub skills: Option<String>,
    pub damage_resistances: Option<String>,
    pub damage_immunities: Option<String>,
    pub condition_immunities: Option<String>,
    pub senses: Option<String>,
    pub languages: Option<String>,
    pub challenge_rating: Option<String>,
    pub proficiency_bonus: Option<i32>,
    pub traits: Option<String>,
    pub actions: Option<String>,
    pub reactions: Option<String>,
    pub legendary_actions: Option<String>,
    pub lair_actions: Option<String>,
    pub regional_effects: Option<String>,
    pub entries: String,
    pub environment: Option<String>,
    pub is_npc: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Hit points structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HitPoints {
    pub average: i32,
    pub formula: String,
}

/// Speed structure  
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Speed {
    pub walk: Option<i32>,
    pub fly: Option<i32>,
    pub swim: Option<i32>,
    pub climb: Option<i32>,
    pub burrow: Option<i32>,
    pub hover: Option<bool>,
}

/// Ability scores structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AbilityScores {
    pub str: i32,
    pub dex: i32,
    pub con: i32,
    pub int: i32,
    pub wis: i32,
    pub cha: i32,
}

/// Armor class entry
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArmorClass {
    pub ac: i32,
    pub from: Option<Vec<String>>,
    pub condition: Option<String>,
}

/// Creature action/trait
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreatureAction {
    pub name: String,
    pub entries: Vec<Value>,
}

/// Legendary action data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LegendaryActions {
    pub intro: Option<Vec<Value>>,
    pub actions: Vec<CreatureAction>,
}

/// Size enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CreatureSize {
    Tiny,
    Small, 
    Medium,
    Large,
    Huge,
    Gargantuan,
}

impl Creature {
    /// Create a new Creature with required fields
    pub fn new(
        id: String,
        name: String,
        rule_system_id: String,
        source_id: String,
    ) -> Result<Self, serde_json::Error> {
        let now = chrono::Utc::now().naive_utc();
        
        Ok(Creature {
            id,
            name,
            rule_system_id,
            source_id,
            page: None,
            size: None,
            creature_type: None,
            type_tags: None,
            alignment: None,
            armor_class: None,
            hit_points: None,
            speed: None,
            ability_scores: None,
            saving_throws: None,
            skills: None,
            damage_resistances: None,
            damage_immunities: None,
            condition_immunities: None,
            senses: None,
            languages: None,
            challenge_rating: None,
            proficiency_bonus: None,
            traits: None,
            actions: None,
            reactions: None,
            legendary_actions: None,
            lair_actions: None,
            regional_effects: None,
            entries: "[]".to_string(), // Default empty JSON array
            environment: None,
            is_npc: false,
            created_at: now,
            updated_at: now,
        })
    }

    /// Set page number
    pub fn with_page(mut self, page: i32) -> Self {
        self.page = Some(page);
        self
    }

    /// Set creature size
    pub fn with_size(mut self, size: CreatureSize) -> Self {
        self.size = Some(size.to_string());
        self
    }

    /// Set creature type
    pub fn with_type(mut self, creature_type: String) -> Self {
        self.creature_type = Some(creature_type);
        self
    }

    /// Set type tags
    pub fn with_type_tags(mut self, type_tags: Vec<String>) -> Result<Self, serde_json::Error> {
        self.type_tags = Some(serde_json::to_string(&type_tags)?);
        Ok(self)
    }

    /// Set alignment
    pub fn with_alignment(mut self, alignment: Vec<String>) -> Result<Self, serde_json::Error> {
        self.alignment = Some(serde_json::to_string(&alignment)?);
        Ok(self)
    }

    /// Set armor class
    pub fn with_armor_class(mut self, ac: Vec<ArmorClass>) -> Result<Self, serde_json::Error> {
        self.armor_class = Some(serde_json::to_string(&ac)?);
        Ok(self)
    }

    /// Set hit points
    pub fn with_hit_points(mut self, hp: HitPoints) -> Result<Self, serde_json::Error> {
        self.hit_points = Some(serde_json::to_string(&hp)?);
        Ok(self)
    }

    /// Set speed
    pub fn with_speed(mut self, speed: Speed) -> Result<Self, serde_json::Error> {
        self.speed = Some(serde_json::to_string(&speed)?);
        Ok(self)
    }

    /// Set ability scores
    pub fn with_ability_scores(mut self, scores: AbilityScores) -> Result<Self, serde_json::Error> {
        self.ability_scores = Some(serde_json::to_string(&scores)?);
        Ok(self)
    }

    /// Set challenge rating
    pub fn with_challenge_rating(mut self, cr: String) -> Self {
        self.challenge_rating = Some(cr);
        self
    }

    /// Set proficiency bonus
    pub fn with_proficiency_bonus(mut self, bonus: i32) -> Self {
        self.proficiency_bonus = Some(bonus);
        self
    }

    /// Set actions
    pub fn with_actions(mut self, actions: Vec<CreatureAction>) -> Result<Self, serde_json::Error> {
        self.actions = Some(serde_json::to_string(&actions)?);
        Ok(self)
    }

    /// Set legendary actions
    pub fn with_legendary_actions(mut self, legendary: LegendaryActions) -> Result<Self, serde_json::Error> {
        self.legendary_actions = Some(serde_json::to_string(&legendary)?);
        Ok(self)
    }

    /// Set entries (description/stat block content)
    pub fn with_entries(mut self, entries: Vec<Value>) -> Result<Self, serde_json::Error> {
        self.entries = serde_json::to_string(&entries)?;
        Ok(self)
    }

    /// Set environment tags
    pub fn with_environment(mut self, environment: Vec<String>) -> Result<Self, serde_json::Error> {
        self.environment = Some(serde_json::to_string(&environment)?);
        Ok(self)
    }

    /// Set whether this is an NPC
    pub fn with_is_npc(mut self, is_npc: bool) -> Self {
        self.is_npc = is_npc;
        self
    }

    // Helper methods for parsing JSON fields

    /// Get armor class as typed structure
    pub fn armor_class_typed(&self) -> Result<Option<Vec<ArmorClass>>, serde_json::Error> {
        match &self.armor_class {
            Some(ac_json) => Ok(Some(serde_json::from_str(ac_json)?)),
            None => Ok(None),
        }
    }

    /// Get hit points as typed structure
    pub fn hit_points_typed(&self) -> Result<Option<HitPoints>, serde_json::Error> {
        match &self.hit_points {
            Some(hp_json) => Ok(Some(serde_json::from_str(hp_json)?)),
            None => Ok(None),
        }
    }

    /// Get speed as typed structure
    pub fn speed_typed(&self) -> Result<Option<Speed>, serde_json::Error> {
        match &self.speed {
            Some(speed_json) => Ok(Some(serde_json::from_str(speed_json)?)),
            None => Ok(None),
        }
    }

    /// Get ability scores as typed structure
    pub fn ability_scores_typed(&self) -> Result<Option<AbilityScores>, serde_json::Error> {
        match &self.ability_scores {
            Some(scores_json) => Ok(Some(serde_json::from_str(scores_json)?)),
            None => Ok(None),
        }
    }

    /// Get actions as typed structure
    pub fn actions_typed(&self) -> Result<Option<Vec<CreatureAction>>, serde_json::Error> {
        match &self.actions {
            Some(actions_json) => Ok(Some(serde_json::from_str(actions_json)?)),
            None => Ok(None),
        }
    }

    /// Get legendary actions as typed structure
    pub fn legendary_actions_typed(&self) -> Result<Option<LegendaryActions>, serde_json::Error> {
        match &self.legendary_actions {
            Some(legendary_json) => Ok(Some(serde_json::from_str(legendary_json)?)),
            None => Ok(None),
        }
    }

    /// Get type tags as vector
    pub fn type_tags_vec(&self) -> Result<Option<Vec<String>>, serde_json::Error> {
        match &self.type_tags {
            Some(tags_json) => Ok(Some(serde_json::from_str(tags_json)?)),
            None => Ok(None),
        }
    }

    /// Get alignment as vector
    pub fn alignment_vec(&self) -> Result<Option<Vec<String>>, serde_json::Error> {
        match &self.alignment {
            Some(alignment_json) => Ok(Some(serde_json::from_str(alignment_json)?)),
            None => Ok(None),
        }
    }

    // Utility methods

    /// Check if creature has legendary actions
    pub fn has_legendary_actions(&self) -> bool {
        self.legendary_actions.is_some()
    }

    /// Check if creature has lair actions
    pub fn has_lair_actions(&self) -> bool {
        self.lair_actions.is_some()
    }

    /// Get size enum
    pub fn size_enum(&self) -> Option<CreatureSize> {
        self.size.as_ref().and_then(|s| s.parse().ok())
    }

    /// Get challenge rating as float
    pub fn challenge_rating_numeric(&self) -> Option<f32> {
        self.challenge_rating.as_ref().and_then(|cr| {
            if cr.contains('/') {
                let parts: Vec<&str> = cr.split('/').collect();
                if parts.len() == 2 {
                    let num: f32 = parts[0].parse().ok()?;
                    let den: f32 = parts[1].parse().ok()?;
                    Some(num / den)
                } else {
                    None
                }
            } else {
                cr.parse().ok()
            }
        })
    }

    /// Calculate modifier from ability score
    pub fn ability_modifier(score: i32) -> i32 {
        (score - 10) / 2
    }

    /// Get entries as typed structure
    pub fn entries_typed(&self) -> Result<Vec<Value>, serde_json::Error> {
        serde_json::from_str(&self.entries)
    }

    /// Get environment as vec of strings
    pub fn environment_vec(&self) -> Result<Option<Vec<String>>, serde_json::Error> {
        match &self.environment {
            Some(env_json) => Ok(Some(serde_json::from_str(env_json)?)),
            None => Ok(None),
        }
    }
}

impl std::fmt::Display for CreatureSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreatureSize::Tiny => write!(f, "T"),
            CreatureSize::Small => write!(f, "S"),
            CreatureSize::Medium => write!(f, "M"),
            CreatureSize::Large => write!(f, "L"),
            CreatureSize::Huge => write!(f, "H"),
            CreatureSize::Gargantuan => write!(f, "G"),
        }
    }
}

impl std::str::FromStr for CreatureSize {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "T" | "TINY" => Ok(CreatureSize::Tiny),
            "S" | "SMALL" => Ok(CreatureSize::Small),
            "M" | "MEDIUM" => Ok(CreatureSize::Medium),
            "L" | "LARGE" => Ok(CreatureSize::Large),
            "H" | "HUGE" => Ok(CreatureSize::Huge),
            "G" | "GARGANTUAN" => Ok(CreatureSize::Gargantuan),
            _ => Err(format!("Unknown creature size: {}", s)),
        }
    }
}