//! Class model representing character classes and subclasses

use crate::schema::classes;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents a character class or subclass
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = classes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Class {
    pub id: String,
    pub name: String,
    pub rule_system_id: String,
    pub source_id: String,
    pub page: Option<i32>,
    pub class_type: String, // "class" or "subclass"
    pub parent_class_id: Option<String>,
    pub hit_die: Option<i32>,
    pub primary_abilities: Option<String>,
    pub saving_throws: Option<String>,
    pub skill_proficiency_count: Option<i32>,
    pub skill_proficiency_choices: Option<String>,
    pub starting_proficiencies: Option<String>,
    pub starting_equipment: Option<String>,
    pub spell_ability: Option<String>,
    pub caster_progression: Option<String>,
    pub subclass_title: Option<String>,
    pub subclass_level: Option<i32>,
    pub features: Option<String>,
    pub spell_slots: Option<String>,
    pub entries: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Starting proficiencies structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StartingProficiencies {
    pub armor: Option<Vec<String>>,
    pub weapons: Option<Vec<String>>,
    pub tools: Option<Vec<String>>,
    pub saving_throws: Option<Vec<String>>,
    pub skills: Option<SkillChoice>,
}

/// Skill choice structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SkillChoice {
    pub choose: Option<u8>,
    pub from: Vec<String>,
}

/// Starting equipment structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StartingEquipment {
    pub default: Option<Vec<String>>,
    pub gold_alternative: Option<GoldAlternative>,
}

/// Gold alternative for starting equipment
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GoldAlternative {
    pub amount: String, // "5d4 × 10"
    pub note: Option<String>,
}

/// Class feature structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClassFeature {
    pub name: String,
    pub level: u8,
    pub entries: Vec<Value>,
}

/// Spell slot progression
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpellSlots {
    pub spell_slots: Vec<SpellSlotLevel>,
}

/// Spell slots per level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpellSlotLevel {
    pub level: u8,
    pub slots: Vec<u8>, // Slots for each spell level (0-9)
}

impl Class {
    /// Create a new Class with required fields
    pub fn new(
        id: String,
        name: String,
        rule_system_id: String,
        source_id: String,
        class_type: String,
        entries: Value,
    ) -> Result<Self, serde_json::Error> {
        let now = chrono::Utc::now().naive_utc();
        Ok(Self {
            id,
            name,
            rule_system_id,
            source_id,
            page: None,
            class_type,
            parent_class_id: None,
            hit_die: None,
            primary_abilities: None,
            saving_throws: None,
            skill_proficiency_count: None,
            skill_proficiency_choices: None,
            starting_proficiencies: None,
            starting_equipment: None,
            spell_ability: None,
            caster_progression: None,
            subclass_title: None,
            subclass_level: None,
            features: None,
            spell_slots: None,
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

    /// Set the parent class ID (for subclasses)
    pub fn with_parent_class(mut self, parent_class_id: String) -> Self {
        self.parent_class_id = Some(parent_class_id);
        self
    }

    /// Set the hit die
    pub fn with_hit_die(mut self, hit_die: i32) -> Self {
        self.hit_die = Some(hit_die);
        self
    }

    /// Set primary abilities
    pub fn with_primary_abilities(mut self, abilities: Vec<String>) -> Result<Self, serde_json::Error> {
        self.primary_abilities = Some(serde_json::to_string(&abilities)?);
        Ok(self)
    }

    /// Set saving throw proficiencies
    pub fn with_saving_throws(mut self, saving_throws: Vec<String>) -> Result<Self, serde_json::Error> {
        self.saving_throws = Some(serde_json::to_string(&saving_throws)?);
        Ok(self)
    }

    /// Set skill proficiency count
    pub fn with_skill_proficiency_count(mut self, count: i32) -> Self {
        self.skill_proficiency_count = Some(count);
        self
    }

    /// Set skill proficiency choices
    pub fn with_skill_proficiency_choices(mut self, choices: Vec<String>) -> Result<Self, serde_json::Error> {
        self.skill_proficiency_choices = Some(serde_json::to_string(&choices)?);
        Ok(self)
    }

    /// Set starting proficiencies
    pub fn with_starting_proficiencies(mut self, proficiencies: StartingProficiencies) -> Result<Self, serde_json::Error> {
        self.starting_proficiencies = Some(serde_json::to_string(&proficiencies)?);
        Ok(self)
    }

    /// Set starting equipment
    pub fn with_starting_equipment(mut self, equipment: StartingEquipment) -> Result<Self, serde_json::Error> {
        self.starting_equipment = Some(serde_json::to_string(&equipment)?);
        Ok(self)
    }

    /// Set spell ability
    pub fn with_spell_ability(mut self, ability: String) -> Self {
        self.spell_ability = Some(ability);
        self
    }

    /// Set caster progression
    pub fn with_caster_progression(mut self, progression: String) -> Self {
        self.caster_progression = Some(progression);
        self
    }

    /// Set subclass title
    pub fn with_subclass_title(mut self, title: String) -> Self {
        self.subclass_title = Some(title);
        self
    }

    /// Set subclass level
    pub fn with_subclass_level(mut self, level: i32) -> Self {
        self.subclass_level = Some(level);
        self
    }

    /// Set class features
    pub fn with_features(mut self, features: Vec<ClassFeature>) -> Result<Self, serde_json::Error> {
        self.features = Some(serde_json::to_string(&features)?);
        Ok(self)
    }

    /// Set spell slots
    pub fn with_spell_slots(mut self, spell_slots: SpellSlots) -> Result<Self, serde_json::Error> {
        self.spell_slots = Some(serde_json::to_string(&spell_slots)?);
        Ok(self)
    }

    /// Get primary abilities as vector
    pub fn primary_abilities_vec(&self) -> Result<Option<Vec<String>>, serde_json::Error> {
        match &self.primary_abilities {
            Some(json_str) => Ok(Some(serde_json::from_str(json_str)?)),
            None => Ok(None),
        }
    }

    /// Get saving throws as vector
    pub fn saving_throws_vec(&self) -> Result<Option<Vec<String>>, serde_json::Error> {
        match &self.saving_throws {
            Some(json_str) => Ok(Some(serde_json::from_str(json_str)?)),
            None => Ok(None),
        }
    }

    /// Get skill proficiency choices as vector
    pub fn skill_proficiency_choices_vec(&self) -> Result<Option<Vec<String>>, serde_json::Error> {
        match &self.skill_proficiency_choices {
            Some(json_str) => Ok(Some(serde_json::from_str(json_str)?)),
            None => Ok(None),
        }
    }

    /// Get starting proficiencies as typed struct
    pub fn starting_proficiencies_typed(&self) -> Result<Option<StartingProficiencies>, serde_json::Error> {
        match &self.starting_proficiencies {
            Some(json_str) => Ok(Some(serde_json::from_str(json_str)?)),
            None => Ok(None),
        }
    }

    /// Get starting equipment as typed struct
    pub fn starting_equipment_typed(&self) -> Result<Option<StartingEquipment>, serde_json::Error> {
        match &self.starting_equipment {
            Some(json_str) => Ok(Some(serde_json::from_str(json_str)?)),
            None => Ok(None),
        }
    }

    /// Get features as vector
    pub fn features_vec(&self) -> Result<Option<Vec<ClassFeature>>, serde_json::Error> {
        match &self.features {
            Some(json_str) => Ok(Some(serde_json::from_str(json_str)?)),
            None => Ok(None),
        }
    }

    /// Get spell slots as typed struct
    pub fn spell_slots_typed(&self) -> Result<Option<SpellSlots>, serde_json::Error> {
        match &self.spell_slots {
            Some(json_str) => Ok(Some(serde_json::from_str(json_str)?)),
            None => Ok(None),
        }
    }

    /// Get entries as JSON value
    pub fn entries_value(&self) -> Result<Value, serde_json::Error> {
        serde_json::from_str(&self.entries)
    }

    /// Check if this is a base class
    pub fn is_base_class(&self) -> bool {
        self.class_type == "class"
    }

    /// Check if this is a subclass
    pub fn is_subclass(&self) -> bool {
        self.class_type == "subclass"
    }

    /// Check if this class has spellcasting
    pub fn is_spellcaster(&self) -> bool {
        self.spell_ability.is_some() || self.caster_progression.is_some()
    }

    /// Get caster type (full, half, third, pact, or none)
    pub fn caster_type(&self) -> String {
        self.caster_progression.clone().unwrap_or_else(|| "none".to_string())
    }
}