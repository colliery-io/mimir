use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PsionicSummary {
    pub name: String,
    pub source: String,
    #[serde(rename = "type")]
    pub psionic_type: String, // "D" for Discipline, "T" for Talent
    pub order: Option<String>, // Avatar, Awakened, Immortal, Nomad, Wu Jen, etc.
    pub page: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Psionic {
    pub name: String,
    pub source: String,
    #[serde(rename = "type")]
    pub psionic_type: String,
    pub order: Option<String>,
    pub page: Option<i32>,
    pub entries: Option<Vec<serde_json::Value>>,
    pub focus: Option<String>, // Focus benefit for disciplines
    pub modes: Option<Vec<PsionicMode>>, // Modes for disciplines
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PsionicMode {
    pub name: String,
    pub cost: PsionicCost,
    pub entries: Vec<serde_json::Value>,
    pub concentration: Option<ConcentrationDuration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PsionicCost {
    pub min: i32,
    pub max: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConcentrationDuration {
    pub duration: i32,
    pub unit: String, // "min", "hr", etc.
}