//! Action catalog models

use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::schema::catalog_actions;

/// A D&D 5e action from the catalog
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    pub name: String,
    pub source: String,
    #[serde(default)]
    pub page: Option<u32>,
    pub time: Vec<ActionTime>,
    pub entries: Vec<serde_json::Value>,  // Can be strings or objects
    #[serde(default)]
    pub see_also: Option<Vec<String>>,
}

/// Action time (how long the action takes)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionTime {
    #[serde(rename = "type")]
    pub time_type: String,  // "action", "bonus", "reaction", etc.
    #[serde(default)]
    pub number: Option<u32>,
    #[serde(default)]
    pub unit: Option<String>, // "minute", "hour", etc.
    #[serde(default)]
    pub condition: Option<String>, // For reactions
}

/// Database model for catalog_actions table
#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = catalog_actions)]
pub struct CatalogAction {
    pub id: i32,
    pub name: String,
    pub time_type: String,    // Simplified time display string
    pub description: String,  // First entry as description
    pub see_also: Option<String>, // JSON string of see_also array
    pub source: String,
    pub full_action_json: String, // Complete action JSON for modal display
}

/// For inserting new actions into the database
#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = catalog_actions)]
pub struct NewCatalogAction {
    pub name: String,
    pub time_type: String,
    pub description: String,
    pub see_also: Option<String>,
    pub source: String,
    pub full_action_json: String,
}

/// Action search filters for database queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionFilters {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub time_types: Option<Vec<String>>, // Filter by action time types
    #[serde(default)]
    pub sources: Option<Vec<String>>,
    #[serde(default)]
    pub search: Option<String>, // General search term
}

/// Action summary for table display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionSummary {
    pub id: i32,
    pub name: String,
    pub time: String,        // Formatted time display
    pub description: String,
    pub see_also: Vec<String>, // Parsed from JSON
    pub source: String,
}

impl From<CatalogAction> for ActionSummary {
    fn from(catalog_action: CatalogAction) -> Self {
        let see_also = match catalog_action.see_also {
            Some(json_str) => serde_json::from_str::<Vec<String>>(&json_str).unwrap_or_default(),
            None => vec![],
        };

        Self {
            id: catalog_action.id,
            name: catalog_action.name,
            time: catalog_action.time_type,
            description: catalog_action.description,
            see_also,
            source: catalog_action.source,
        }
    }
}

impl From<Action> for NewCatalogAction {
    fn from(action: Action) -> Self {
        // Format time display
        let time_display = if action.time.is_empty() {
            "Unknown".to_string()
        } else {
            action.time.iter()
                .map(|t| {
                    match t.time_type.as_str() {
                        "action" => "Action".to_string(),
                        "bonus" => "Bonus Action".to_string(),
                        "reaction" => {
                            if let Some(condition) = &t.condition {
                                format!("Reaction ({})", condition)
                            } else {
                                "Reaction".to_string()
                            }
                        }
                        "free" => "Free".to_string(),
                        other => other.to_string(),
                    }
                })
                .collect::<Vec<_>>()
                .join(", ")
        };

        // Extract description from first entry
        let description = if action.entries.is_empty() {
            "No description available".to_string()
        } else {
            match &action.entries[0] {
                serde_json::Value::String(s) => s.clone(),
                serde_json::Value::Object(obj) => {
                    // Handle complex entry objects if needed
                    obj.get("entries").and_then(|v| v.as_str()).unwrap_or("Complex entry").to_string()
                }
                _ => "Complex entry".to_string(),
            }
        };

        // Convert see_also to JSON string
        let see_also_json = action.see_also.as_ref()
            .map(|sa| serde_json::to_string(sa).unwrap_or_default());

        Self {
            name: action.name.clone(),
            time_type: time_display,
            description,
            see_also: see_also_json,
            source: action.source.clone(),
            full_action_json: serde_json::to_string(&action).unwrap_or_default(),
        }
    }
}