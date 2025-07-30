//! Source model representing content sources (books, modules, etc.)

use crate::schema::sources;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents a content source (book, module, etc.)
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = sources)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Source {
    pub id: String,
    pub rule_system_id: String,
    pub full_name: String,
    pub abbreviation: Option<String>,
    pub published_date: Option<chrono::NaiveDate>,
    pub version: Option<String>,
    pub is_official: bool,
    pub is_srd: bool,
    pub metadata: Option<String>,
}

impl Source {
    /// Create a new Source with required fields
    pub fn new(id: String, rule_system_id: String, full_name: String) -> Self {
        Self {
            id,
            rule_system_id,
            full_name,
            abbreviation: None,
            published_date: None,
            version: None,
            is_official: true,
            is_srd: false,
            metadata: None,
        }
    }

    /// Set the abbreviation
    pub fn with_abbreviation(mut self, abbreviation: String) -> Self {
        self.abbreviation = Some(abbreviation);
        self
    }

    /// Set the published date
    pub fn with_published_date(mut self, published_date: chrono::NaiveDate) -> Self {
        self.published_date = Some(published_date);
        self
    }

    /// Set the version
    pub fn with_version(mut self, version: String) -> Self {
        self.version = Some(version);
        self
    }

    /// Set whether this is official content
    pub fn with_official(mut self, is_official: bool) -> Self {
        self.is_official = is_official;
        self
    }

    /// Set whether this is SRD/OGL content
    pub fn with_srd(mut self, is_srd: bool) -> Self {
        self.is_srd = is_srd;
        self
    }

    /// Set metadata from a serializable value
    pub fn with_metadata<T: Serialize>(mut self, metadata: T) -> Result<Self, serde_json::Error> {
        self.metadata = Some(serde_json::to_string(&metadata)?);
        Ok(self)
    }

    /// Get metadata as a specific type
    pub fn metadata_as<T: for<'de> Deserialize<'de>>(&self) -> Result<Option<T>, serde_json::Error> {
        match &self.metadata {
            Some(json_str) => {
                let value: T = serde_json::from_str(json_str)?;
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }

    /// Get metadata as raw JSON value
    pub fn metadata_value(&self) -> Result<Option<Value>, serde_json::Error> {
        match &self.metadata {
            Some(json_str) => Ok(Some(serde_json::from_str(json_str)?)),
            None => Ok(None),
        }
    }
}