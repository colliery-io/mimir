//! Background model representing character backgrounds from D&D sources

use crate::schema::backgrounds;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents a character background
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = backgrounds)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Background {
    pub id: String,
    pub name: String,
    pub rule_system_id: String,
    pub source_id: String,
    pub page: Option<i32>,
    pub skill_proficiencies: Option<String>,
    pub language_proficiencies: Option<String>,
    pub tool_proficiencies: Option<String>,
    pub starting_equipment: Option<String>,
    pub feature_name: Option<String>,
    pub feature_text: Option<String>,
    pub entries: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Skill proficiency structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SkillProficiency {
    pub skill: String,
    pub choice: Option<SkillChoice>,
}

/// Skill choice options
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SkillChoice {
    pub from: Vec<String>,
    pub count: u8,
}

/// Language proficiency structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LanguageProficiency {
    pub language: Option<String>,
    pub choice: Option<LanguageChoice>,
}

/// Language choice options
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LanguageChoice {
    pub from: Vec<String>,
    pub count: u8,
}

/// Tool proficiency structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolProficiency {
    pub tool: Option<String>,
    pub choice: Option<ToolChoice>,
}

/// Tool choice options
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolChoice {
    pub from: Vec<String>,
    pub count: u8,
}

/// Starting equipment structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StartingEquipment {
    pub items: Vec<EquipmentItem>,
    pub choices: Option<Vec<EquipmentChoice>>,
    pub gold: Option<CurrencyAmount>,
}

/// Equipment item
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EquipmentItem {
    pub item: String,
    pub quantity: u32,
}

/// Equipment choice
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EquipmentChoice {
    pub from: Vec<EquipmentItem>,
    pub count: u8,
}

/// Currency amount
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CurrencyAmount {
    pub amount: u32,
    pub unit: String, // "gp", "sp", "cp"
}

impl Background {
    /// Create a new Background with required fields
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
            skill_proficiencies: None,
            language_proficiencies: None,
            tool_proficiencies: None,
            starting_equipment: None,
            feature_name: None,
            feature_text: None,
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

    /// Set skill proficiencies
    pub fn with_skill_proficiencies(mut self, skills: Vec<SkillProficiency>) -> Result<Self, serde_json::Error> {
        self.skill_proficiencies = Some(serde_json::to_string(&skills)?);
        Ok(self)
    }

    /// Set language proficiencies
    pub fn with_language_proficiencies(mut self, languages: Vec<LanguageProficiency>) -> Result<Self, serde_json::Error> {
        self.language_proficiencies = Some(serde_json::to_string(&languages)?);
        Ok(self)
    }

    /// Set tool proficiencies
    pub fn with_tool_proficiencies(mut self, tools: Vec<ToolProficiency>) -> Result<Self, serde_json::Error> {
        self.tool_proficiencies = Some(serde_json::to_string(&tools)?);
        Ok(self)
    }

    /// Set starting equipment
    pub fn with_starting_equipment(mut self, equipment: StartingEquipment) -> Result<Self, serde_json::Error> {
        self.starting_equipment = Some(serde_json::to_string(&equipment)?);
        Ok(self)
    }

    /// Set background feature
    pub fn with_feature(mut self, name: String, text: String) -> Self {
        self.feature_name = Some(name);
        self.feature_text = Some(text);
        self
    }

    /// Get skill proficiencies as typed struct
    pub fn skill_proficiencies_typed(&self) -> Result<Option<Vec<SkillProficiency>>, serde_json::Error> {
        match &self.skill_proficiencies {
            Some(json_str) => Ok(Some(serde_json::from_str(json_str)?)),
            None => Ok(None),
        }
    }

    /// Get language proficiencies as typed struct
    pub fn language_proficiencies_typed(&self) -> Result<Option<Vec<LanguageProficiency>>, serde_json::Error> {
        match &self.language_proficiencies {
            Some(json_str) => Ok(Some(serde_json::from_str(json_str)?)),
            None => Ok(None),
        }
    }

    /// Get tool proficiencies as typed struct
    pub fn tool_proficiencies_typed(&self) -> Result<Option<Vec<ToolProficiency>>, serde_json::Error> {
        match &self.tool_proficiencies {
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

    /// Get entries as JSON value
    pub fn entries_value(&self) -> Result<Value, serde_json::Error> {
        serde_json::from_str(&self.entries)
    }

    /// Check if background has a feature
    pub fn has_feature(&self) -> bool {
        self.feature_name.is_some() && self.feature_text.is_some()
    }

    /// Get background feature as tuple
    pub fn feature(&self) -> Option<(String, String)> {
        match (&self.feature_name, &self.feature_text) {
            (Some(name), Some(text)) => Some((name.clone(), text.clone())),
            _ => None,
        }
    }

    /// Check if background grants skill proficiencies
    pub fn grants_skill_proficiencies(&self) -> bool {
        self.skill_proficiencies.is_some()
    }

    /// Check if background grants language proficiencies
    pub fn grants_language_proficiencies(&self) -> bool {
        self.language_proficiencies.is_some()
    }

    /// Check if background grants tool proficiencies
    pub fn grants_tool_proficiencies(&self) -> bool {
        self.tool_proficiencies.is_some()
    }

    /// Check if background provides starting equipment
    pub fn provides_starting_equipment(&self) -> bool {
        self.starting_equipment.is_some()
    }
}