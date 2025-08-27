use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub name: String,
    pub source: String,
    pub page: Option<i32>,
    #[serde(default)]
    pub entries: Vec<serde_json::Value>,
    pub srd: Option<bool>,
    #[serde(rename = "basicRules")]
    pub basic_rules: Option<bool>,
    #[serde(rename = "hasFluffImages")]
    pub has_fluff_images: Option<bool>,
    #[serde(rename = "otherSources", default)]
    pub other_sources: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Disease {
    pub name: String,
    pub source: String,
    pub page: Option<i32>,
    #[serde(default)]
    pub entries: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConditionData {
    #[serde(default)]
    pub condition: Option<Vec<Condition>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiseaseData {
    #[serde(default)]
    pub disease: Option<Vec<Disease>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionFluff {
    pub name: String,
    pub source: String,
    #[serde(default)]
    pub entries: Vec<serde_json::Value>,
    #[serde(default)]
    pub images: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConditionFluffData {
    #[serde(rename = "conditionFluff", default)]
    pub condition_fluff: Option<Vec<ConditionFluff>>,
}

// Combined enum for both conditions and diseases
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ConditionOrDisease {
    Condition(Condition),
    Disease(Disease),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionSummary {
    pub name: String,
    pub source: String,
    pub item_type: String, // "Condition" or "Disease"
    pub description: String,
    pub is_srd: bool,
}

impl From<&Condition> for ConditionSummary {
    fn from(condition: &Condition) -> Self {
        // Extract first entry as description
        let description = extract_description(&condition.entries);
        
        Self {
            name: condition.name.clone(),
            source: condition.source.clone(),
            item_type: "Condition".to_string(),
            description,
            is_srd: condition.srd.unwrap_or(false),
        }
    }
}

impl From<&Disease> for ConditionSummary {
    fn from(disease: &Disease) -> Self {
        // Extract first entry as description
        let description = extract_description(&disease.entries);
        
        Self {
            name: disease.name.clone(),
            source: disease.source.clone(),
            item_type: "Disease".to_string(),
            description,
            is_srd: false, // Diseases typically aren't SRD
        }
    }
}

fn extract_description(entries: &[serde_json::Value]) -> String {
    if entries.is_empty() {
        return String::new();
    }
    
    // Get first entry or first item in list
    let first_text = if let Some(first) = entries.first() {
        if let Some(s) = first.as_str() {
            s.to_string()
        } else if let Some(obj) = first.as_object() {
            if let Some(items) = obj.get("items").and_then(|i| i.as_array()) {
                // Get first list item as description
                items.first()
                    .and_then(|item| item.as_str())
                    .unwrap_or("")
                    .to_string()
            } else {
                "Effect".to_string()
            }
        } else {
            "Effect".to_string()
        }
    } else {
        String::new()
    };
    
    // Clean up formatting tags and truncate if needed
    let cleaned = first_text
        .replace("{@", "")
        .replace("}", "")
        .split('|')
        .next()
        .unwrap_or(&first_text)
        .to_string();
    
    if cleaned.len() > 150 {
        format!("{}...", &cleaned[..147])
    } else {
        cleaned
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionWithDetails {
    #[serde(flatten)]
    pub item: ConditionOrDisease,
    pub fluff: Option<ConditionFluff>,
}