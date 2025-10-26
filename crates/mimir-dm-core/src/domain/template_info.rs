//! Template information domain types
//!
//! This module contains domain types for representing template metadata
//! in a structured, user-friendly format.

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

/// Structured information about a template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateInfo {
    pub id: String,
    pub title: String,
    pub purpose: String,
    pub level: String,
    pub template_type: String,
    pub variables: Vec<TemplateVariable>,
}

/// Template variable definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateVariable {
    pub name: String,
    pub var_type: String,
    pub description: String,
    pub default: JsonValue,
    pub required: bool,
}
