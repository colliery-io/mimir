use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Background {
    pub name: String,
    pub source: String,
    pub page: Option<i32>,
    #[serde(rename = "skillProficiencies", default)]
    pub skill_proficiencies: Vec<serde_json::Value>,
    #[serde(rename = "languageProficiencies", default)]
    pub language_proficiencies: Vec<serde_json::Value>,
    #[serde(rename = "toolProficiencies", default)]
    pub tool_proficiencies: Vec<serde_json::Value>,
    #[serde(rename = "startingEquipment", default)]
    pub starting_equipment: Vec<serde_json::Value>,
    #[serde(default)]
    pub entries: Vec<serde_json::Value>,
    #[serde(rename = "hasFluff")]
    pub has_fluff: Option<bool>,
    #[serde(rename = "basicRules")]
    pub basic_rules: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundFluff {
    pub name: String,
    pub source: String,
    #[serde(default)]
    pub entries: Vec<serde_json::Value>,
    #[serde(default)]
    pub images: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BackgroundData {
    #[serde(default)]
    pub background: Option<Vec<Background>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BackgroundFluffData {
    #[serde(rename = "backgroundFluff", default)]
    pub background_fluff: Option<Vec<BackgroundFluff>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundSummary {
    pub name: String,
    pub source: String,
    pub skills: String,
    pub languages: String,
    pub tools: String,
    pub feature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundWithDetails {
    pub background: Background,
    pub fluff: Option<BackgroundFluff>,
}

impl From<&Background> for BackgroundSummary {
    fn from(bg: &Background) -> Self {
        // Extract skills
        let skills = if !bg.skill_proficiencies.is_empty() {
            bg.skill_proficiencies.iter()
                .filter_map(|s| {
                    if let Some(obj) = s.as_object() {
                        let skill_names: Vec<String> = obj.keys()
                            .filter(|k| *k != "any" && *k != "choose")
                            .map(|k| titlecase(k))
                            .collect();
                        if !skill_names.is_empty() {
                            Some(skill_names.join(", "))
                        } else if let Some(any) = obj.get("any").and_then(|v| v.as_i64()) {
                            Some(format!("Any {}", any))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect::<Vec<String>>()
                .join(", ")
        } else {
            "None".to_string()
        };
        
        // Extract languages
        let languages = if !bg.language_proficiencies.is_empty() {
            bg.language_proficiencies.iter()
                .filter_map(|l| {
                    if let Some(obj) = l.as_object() {
                        let lang_names: Vec<String> = obj.keys()
                            .filter(|k| *k != "anyStandard" && *k != "choose" && *k != "any")
                            .map(|k| titlecase(k))
                            .collect();
                        if !lang_names.is_empty() {
                            Some(lang_names.join(", "))
                        } else if let Some(any) = obj.get("anyStandard").and_then(|v| v.as_i64()) {
                            Some(format!("Any {} standard", any))
                        } else if let Some(any) = obj.get("any").and_then(|v| v.as_i64()) {
                            Some(format!("Any {}", any))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect::<Vec<String>>()
                .join(", ")
        } else {
            "None".to_string()
        };
        
        // Extract tools
        let tools = if !bg.tool_proficiencies.is_empty() {
            bg.tool_proficiencies.iter()
                .filter_map(|t| {
                    if let Some(obj) = t.as_object() {
                        let tool_names: Vec<String> = obj.keys()
                            .filter(|k| *k != "any" && *k != "choose")
                            .map(|k| titlecase(k))
                            .collect();
                        if !tool_names.is_empty() {
                            Some(tool_names.join(", "))
                        } else if let Some(any) = obj.get("any").and_then(|v| v.as_i64()) {
                            Some(format!("Any {}", any))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect::<Vec<String>>()
                .join(", ")
        } else {
            "None".to_string()
        };
        
        // Extract feature name
        let feature = bg.entries.iter()
            .filter_map(|e| {
                if let Some(obj) = e.as_object() {
                    if let Some(name) = obj.get("name").and_then(|n| n.as_str()) {
                        if name.starts_with("Feature:") {
                            return Some(name.replace("Feature: ", ""));
                        }
                    }
                }
                None
            })
            .next()
            .unwrap_or_else(|| "Special Feature".to_string());
        
        Self {
            name: bg.name.clone(),
            source: bg.source.clone(),
            skills,
            languages,
            tools,
            feature,
        }
    }
}

fn titlecase(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().chain(chars).collect(),
    }
}

// Database models for catalog_backgrounds table
use diesel::prelude::*;
use crate::schema::catalog_backgrounds;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = catalog_backgrounds)]
pub struct CatalogBackground {
    pub id: i32,
    pub name: String,
    pub skills: String,
    pub languages: String,
    pub tools: String,
    pub feature: String,
    pub source: String,
    pub full_background_json: String,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = catalog_backgrounds)]
pub struct NewCatalogBackground {
    pub name: String,
    pub skills: String,
    pub languages: String,
    pub tools: String,
    pub feature: String,
    pub source: String,
    pub full_background_json: String,
}

impl From<&Background> for NewCatalogBackground {
    fn from(background: &Background) -> Self {
        let summary = BackgroundSummary::from(background);
        let full_json = serde_json::to_string(background).unwrap_or_default();

        Self {
            name: summary.name,
            skills: summary.skills,
            languages: summary.languages,
            tools: summary.tools,
            feature: summary.feature,
            source: summary.source,
            full_background_json: full_json,
        }
    }
}

impl From<&CatalogBackground> for BackgroundSummary {
    fn from(bg: &CatalogBackground) -> Self {
        Self {
            name: bg.name.clone(),
            source: bg.source.clone(),
            skills: bg.skills.clone(),
            languages: bg.languages.clone(),
            tools: bg.tools.clone(),
            feature: bg.feature.clone(),
        }
    }
}

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct BackgroundFilters {
    pub search_pattern: Option<String>,
    pub sources: Option<Vec<String>>,
    pub has_tools: Option<bool>,
}