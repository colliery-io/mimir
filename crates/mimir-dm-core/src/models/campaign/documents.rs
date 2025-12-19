//! Document model for tracking campaign documents

use crate::schema::documents;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// Represents a document instance created from a template
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name = documents)]
pub struct Document {
    pub id: i32,
    pub campaign_id: i32,
    pub module_id: Option<i32>,
    pub session_id: Option<i32>,
    pub template_id: String,
    pub document_type: String,
    pub title: String,
    pub file_path: String,
    pub created_at: String,
    pub updated_at: String,
    pub completed_at: Option<String>,
}

/// New document to be inserted
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = documents)]
pub struct NewDocument {
    pub campaign_id: i32,
    pub module_id: Option<i32>,
    pub session_id: Option<i32>,
    pub template_id: String,
    pub document_type: String,
    pub title: String,
    pub file_path: String,
}

/// Update existing document
#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = documents)]
#[derive(Default)]
pub struct UpdateDocument {
    pub title: Option<String>,
    pub updated_at: Option<String>,
    pub completed_at: Option<String>,
}

impl Document {
    /// Check if the document has been completed
    pub fn is_completed(&self) -> bool {
        self.completed_at.is_some()
    }

    /// Get the level of the document (campaign, module, session, or handout)
    pub fn level(&self) -> DocumentLevel {
        // Check if this is a handout based on document_type
        if self.document_type == "handout" {
            DocumentLevel::Handout
        } else if self.session_id.is_some() {
            DocumentLevel::Session
        } else if self.module_id.is_some() {
            DocumentLevel::Module
        } else {
            DocumentLevel::Campaign
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DocumentLevel {
    Campaign,
    Module,
    Session,
    Handout,
}

impl DocumentLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            DocumentLevel::Campaign => "campaign",
            DocumentLevel::Module => "module",
            DocumentLevel::Session => "session",
            DocumentLevel::Handout => "handout",
        }
    }
}
