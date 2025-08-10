//! Item model representing equipment, weapons, armor, and magic items

use crate::schema::items;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents an item (mundane or magical)
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = items)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Item {
    pub id: String,
    pub name: String,
    pub rule_system_id: String,
    pub source_id: String,
    pub page: Option<i32>,
    pub base_item_id: Option<String>,
    pub item_type: Option<String>, // Maps to "type" column
    pub weight_lb: Option<f32>,
    pub value_cp: Option<i32>,
    pub armor_class: Option<i32>,
    pub damage: Option<String>,
    pub properties: Option<String>,
    pub rarity: Option<String>,
    pub requires_attunement: bool,
    pub attunement_prereq: Option<String>,
    pub magic_bonus: Option<i32>,
    pub additional_properties: Option<String>,
    pub entries: String,
    pub is_magic: bool, // Generated column - read-only
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Item structure for insertions and updates (excludes generated columns)
#[derive(Debug, Clone, Insertable, AsChangeset)]
#[diesel(table_name = items)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewItem {
    pub id: String,
    pub name: String,
    pub rule_system_id: String,
    pub source_id: String,
    pub page: Option<i32>,
    pub base_item_id: Option<String>,
    pub item_type: Option<String>,
    pub weight_lb: Option<f32>,
    pub value_cp: Option<i32>,
    pub armor_class: Option<i32>,
    pub damage: Option<String>,
    pub properties: Option<String>,
    pub rarity: Option<String>,
    pub requires_attunement: bool,
    pub attunement_prereq: Option<String>,
    pub magic_bonus: Option<i32>,
    pub additional_properties: Option<String>,
    pub entries: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    // Note: is_magic is excluded because it's a generated column
}

/// Damage structure for weapons
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Damage {
    pub dice: String,     // "1d8", "2d6"
    pub damage_type: String, // "slashing", "bludgeoning", "piercing"
}

/// Attunement prerequisite
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AttunementPrereq {
    pub class: Option<String>,
    pub race: Option<String>,
    pub alignment: Option<String>,
    pub other: Option<String>,
}

/// Additional magical properties
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdditionalProperties {
    pub charges: Option<ChargeInfo>,
    pub spells: Option<Vec<String>>,
    pub abilities: Option<Vec<String>>,
    pub bonuses: Option<Vec<Bonus>>,
}

/// Charge information for magic items
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChargeInfo {
    pub max: u8,
    pub per_day: Option<u8>,
    pub recharge: Option<String>, // "dawn", "short rest", etc.
}

/// Bonus information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Bonus {
    pub stat: String,
    pub value: i8,
}

impl Item {
    /// Get damage as typed struct
    pub fn damage_typed(&self) -> Result<Option<Damage>, serde_json::Error> {
        match &self.damage {
            Some(json_str) => Ok(Some(serde_json::from_str(json_str)?)),
            None => Ok(None),
        }
    }

    /// Get properties as vector
    pub fn properties_vec(&self) -> Result<Option<Vec<String>>, serde_json::Error> {
        match &self.properties {
            Some(json_str) => Ok(Some(serde_json::from_str(json_str)?)),
            None => Ok(None),
        }
    }

    /// Get attunement prerequisites as typed struct
    pub fn attunement_prereq_typed(&self) -> Result<Option<AttunementPrereq>, serde_json::Error> {
        match &self.attunement_prereq {
            Some(json_str) => Ok(Some(serde_json::from_str(json_str)?)),
            None => Ok(None),
        }
    }

    /// Get additional properties as typed struct
    pub fn additional_properties_typed(&self) -> Result<Option<AdditionalProperties>, serde_json::Error> {
        match &self.additional_properties {
            Some(json_str) => Ok(Some(serde_json::from_str(json_str)?)),
            None => Ok(None),
        }
    }

    /// Get entries as JSON value
    pub fn entries_value(&self) -> Result<Value, serde_json::Error> {
        serde_json::from_str(&self.entries)
    }

    /// Check if this is a base item
    pub fn is_base_item(&self) -> bool {
        self.base_item_id.is_none()
    }

    /// Check if this is a variant of another item
    pub fn is_variant(&self) -> bool {
        self.base_item_id.is_some()
    }

    /// Check if this is a weapon
    pub fn is_weapon(&self) -> bool {
        matches!(self.item_type.as_deref(), Some("weapon") | Some("simple weapon") | Some("martial weapon"))
    }

    /// Check if this is armor
    pub fn is_armor(&self) -> bool {
        matches!(self.item_type.as_deref(), Some("armor") | Some("light armor") | Some("medium armor") | Some("heavy armor"))
    }

    /// Check if this is a shield
    pub fn is_shield(&self) -> bool {
        self.item_type.as_deref() == Some("shield")
    }

    /// Check if this item requires attunement
    pub fn needs_attunement(&self) -> bool {
        self.requires_attunement
    }

    /// Get rarity level as enum-like string
    pub fn rarity_level(&self) -> String {
        self.rarity.clone().unwrap_or_else(|| "common".to_string())
    }

    /// Get item category
    pub fn category(&self) -> String {
        self.item_type.clone().unwrap_or_else(|| "miscellaneous".to_string())
    }

    /// Get value in gold pieces (converted from copper)
    pub fn value_gp(&self) -> Option<f32> {
        self.value_cp.map(|cp| cp as f32 / 100.0)
    }
}

impl NewItem {
    /// Create a new NewItem with required fields
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
            base_item_id: None,
            item_type: None,
            weight_lb: None,
            value_cp: None,
            armor_class: None,
            damage: None,
            properties: None,
            rarity: None,
            requires_attunement: false,
            attunement_prereq: None,
            magic_bonus: None,
            additional_properties: None,
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

    /// Set the base item ID (for variants)
    pub fn with_base_item(mut self, base_item_id: String) -> Self {
        self.base_item_id = Some(base_item_id);
        self
    }

    /// Set the item type
    pub fn with_type(mut self, item_type: String) -> Self {
        self.item_type = Some(item_type);
        self
    }

    /// Set the weight in pounds
    pub fn with_weight(mut self, weight_lb: f32) -> Self {
        self.weight_lb = Some(weight_lb);
        self
    }

    /// Set the value in copper pieces
    pub fn with_value(mut self, value_cp: i32) -> Self {
        self.value_cp = Some(value_cp);
        self
    }

    /// Set the armor class
    pub fn with_armor_class(mut self, ac: i32) -> Self {
        self.armor_class = Some(ac);
        self
    }

    /// Set the damage from a Damage struct
    pub fn with_damage(mut self, damage: Damage) -> Result<Self, serde_json::Error> {
        self.damage = Some(serde_json::to_string(&damage)?);
        Ok(self)
    }

    /// Set properties as a vector of strings
    pub fn with_properties(mut self, properties: Vec<String>) -> Result<Self, serde_json::Error> {
        self.properties = Some(serde_json::to_string(&properties)?);
        Ok(self)
    }

    /// Set the magic item rarity
    pub fn with_rarity(mut self, rarity: String) -> Self {
        self.rarity = Some(rarity);
        self
    }

    /// Set attunement requirement
    pub fn with_attunement(mut self, required: bool) -> Self {
        self.requires_attunement = required;
        self
    }

    /// Set attunement prerequisites
    pub fn with_attunement_prereq(mut self, prereq: AttunementPrereq) -> Result<Self, serde_json::Error> {
        self.attunement_prereq = Some(serde_json::to_string(&prereq)?);
        Ok(self)
    }

    /// Set magic bonus
    pub fn with_magic_bonus(mut self, bonus: i32) -> Self {
        self.magic_bonus = Some(bonus);
        self
    }

    /// Set additional magical properties
    pub fn with_additional_properties(mut self, props: AdditionalProperties) -> Result<Self, serde_json::Error> {
        self.additional_properties = Some(serde_json::to_string(&props)?);
        Ok(self)
    }

    /// Get damage as typed struct
    pub fn damage_typed(&self) -> Result<Option<Damage>, serde_json::Error> {
        match &self.damage {
            Some(json_str) => Ok(Some(serde_json::from_str(json_str)?)),
            None => Ok(None),
        }
    }

    /// Get properties as vector
    pub fn properties_vec(&self) -> Result<Option<Vec<String>>, serde_json::Error> {
        match &self.properties {
            Some(json_str) => Ok(Some(serde_json::from_str(json_str)?)),
            None => Ok(None),
        }
    }

    /// Get attunement prerequisites as typed struct
    pub fn attunement_prereq_typed(&self) -> Result<Option<AttunementPrereq>, serde_json::Error> {
        match &self.attunement_prereq {
            Some(json_str) => Ok(Some(serde_json::from_str(json_str)?)),
            None => Ok(None),
        }
    }

    /// Get additional properties as typed struct
    pub fn additional_properties_typed(&self) -> Result<Option<AdditionalProperties>, serde_json::Error> {
        match &self.additional_properties {
            Some(json_str) => Ok(Some(serde_json::from_str(json_str)?)),
            None => Ok(None),
        }
    }

    /// Get entries as JSON value
    pub fn entries_value(&self) -> Result<Value, serde_json::Error> {
        serde_json::from_str(&self.entries)
    }

    /// Check if this is a base item
    pub fn is_base_item(&self) -> bool {
        self.base_item_id.is_none()
    }

    /// Check if this is a variant of another item
    pub fn is_variant(&self) -> bool {
        self.base_item_id.is_some()
    }

    /// Check if this is a weapon
    pub fn is_weapon(&self) -> bool {
        matches!(self.item_type.as_deref(), Some("weapon") | Some("simple weapon") | Some("martial weapon"))
    }

    /// Check if this is armor
    pub fn is_armor(&self) -> bool {
        matches!(self.item_type.as_deref(), Some("armor") | Some("light armor") | Some("medium armor") | Some("heavy armor"))
    }

    /// Check if this is a shield
    pub fn is_shield(&self) -> bool {
        self.item_type.as_deref() == Some("shield")
    }

    /// Check if this item requires attunement
    pub fn needs_attunement(&self) -> bool {
        self.requires_attunement
    }

    /// Get rarity level as enum-like string
    pub fn rarity_level(&self) -> String {
        self.rarity.clone().unwrap_or_else(|| "common".to_string())
    }

    /// Get item category
    pub fn category(&self) -> String {
        self.item_type.clone().unwrap_or_else(|| "miscellaneous".to_string())
    }

    /// Get value in gold pieces (converted from copper)
    pub fn value_gp(&self) -> Option<f32> {
        self.value_cp.map(|cp| cp as f32 / 100.0)
    }
}