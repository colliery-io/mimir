use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reward {
    pub name: String,
    pub source: String,
    pub page: Option<i32>,
    
    #[serde(rename = "type")]
    pub reward_type: Option<String>, // Blessing, Boon, Charm, etc.
    
    pub entries: Option<Vec<serde_json::Value>>,
    
    #[serde(rename = "prerequisite")]
    pub prerequisite: Option<Vec<serde_json::Value>>,
    
    // For boons that grant spells
    #[serde(rename = "additionalSpells")]
    pub additional_spells: Option<Vec<serde_json::Value>>,
    
    // Duration for temporary rewards
    pub duration: Option<String>,
    
    pub srd: Option<bool>,
    
    #[serde(rename = "basicRules")]
    pub basic_rules: Option<bool>,
    
    #[serde(rename = "hasFluff")]
    pub has_fluff: Option<bool>,
    
    #[serde(rename = "hasFluffImages")]
    pub has_fluff_images: Option<bool>,
}

// Summary for list views
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardSummary {
    pub name: String,
    pub source: String,
    pub reward_type: String,
    pub description: String,
    pub has_prerequisites: bool,
}

impl From<&Reward> for RewardSummary {
    fn from(reward: &Reward) -> Self {
        Self {
            name: reward.name.clone(),
            source: reward.source.clone(),
            reward_type: format_reward_type(&reward.reward_type),
            description: extract_first_entry(&reward.entries),
            has_prerequisites: reward.prerequisite.is_some(),
        }
    }
}

fn format_reward_type(reward_type: &Option<String>) -> String {
    match reward_type.as_deref() {
        Some("Blessing") => "Blessing".to_string(),
        Some("Boon") => "Epic Boon".to_string(),
        Some("Charm") => "Charm".to_string(),
        Some("Feat") => "Feat".to_string(),
        Some(other) => other.to_string(),
        None => "Reward".to_string(),
    }
}

fn extract_first_entry(entries: &Option<Vec<serde_json::Value>>) -> String {
    if let Some(entries) = entries {
        if let Some(first) = entries.first() {
            if let serde_json::Value::String(s) = first {
                // Truncate for summary
                let cleaned = s.replace(|c: char| c == '{' || c == '}' || c == '@', "");
                if cleaned.len() > 150 {
                    format!("{}...", &cleaned[..147])
                } else {
                    cleaned
                }
            } else {
                "Complex reward description".to_string()
            }
        } else {
            "—".to_string()
        }
    } else {
        "—".to_string()
    }
}

// Container for JSON parsing
#[derive(Debug, Serialize, Deserialize)]
pub struct RewardData {
    pub reward: Option<Vec<Reward>>,
}

// Fluff data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardFluff {
    pub name: String,
    pub source: String,
    pub entries: Option<Vec<serde_json::Value>>,
    pub images: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RewardFluffData {
    #[serde(rename = "rewardFluff")]
    pub reward_fluff: Option<Vec<RewardFluff>>,
}