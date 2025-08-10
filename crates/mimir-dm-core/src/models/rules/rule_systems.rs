//! Rule system model

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// Represents a rule system (e.g., D&D 5e 2014, D&D 5e 2024, Pathfinder 2e)
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::rule_systems)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct RuleSystem {
    /// Unique identifier (e.g., 'dnd5e-2014', 'dnd5e-2024', 'pf2e')
    pub id: String,
    /// Full name (e.g., 'D&D 5th Edition (2014)')
    pub name: String,
    /// Short name (e.g., 'D&D 5e 2014')
    pub short_name: Option<String>,
    /// Publisher (e.g., 'Wizards of the Coast', 'Paizo')
    pub publisher: Option<String>,
    /// Version (e.g., '2014', '2024', '2.0')
    pub version: Option<String>,
    /// Whether this rule system is active
    pub is_active: bool,
    /// Additional metadata as JSON
    pub metadata: Option<String>,
}

impl RuleSystem {
    /// Create a new rule system
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            short_name: None,
            publisher: None,
            version: None,
            is_active: true,
            metadata: None,
        }
    }

    /// Builder method to set short name
    pub fn with_short_name(mut self, short_name: String) -> Self {
        self.short_name = Some(short_name);
        self
    }

    /// Builder method to set publisher
    pub fn with_publisher(mut self, publisher: String) -> Self {
        self.publisher = Some(publisher);
        self
    }

    /// Builder method to set version
    pub fn with_version(mut self, version: String) -> Self {
        self.version = Some(version);
        self
    }

    /// Builder method to set metadata
    pub fn with_metadata<T: Serialize>(mut self, metadata: T) -> Result<Self, serde_json::Error> {
        self.metadata = Some(serde_json::to_string(&metadata)?);
        Ok(self)
    }

    /// Get metadata as a typed value
    pub fn metadata_as<T: for<'a> Deserialize<'a>>(&self) -> Result<Option<T>, serde_json::Error> {
        match &self.metadata {
            Some(json) => Ok(Some(serde_json::from_str(json)?)),
            None => Ok(None),
        }
    }
}