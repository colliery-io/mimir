use serde::{Deserialize, Serialize};

/// Feat from D&D 5e
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Feat {
    pub name: String,
    pub source: String,
    #[serde(default)]
    pub page: Option<u32>,
    #[serde(default)]
    pub srd: Option<bool>,
    #[serde(default)]
    pub entries: Vec<serde_json::Value>,
    #[serde(default)]
    pub prerequisite: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub ability: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub skill_proficiencies: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub language_proficiencies: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub tool_proficiencies: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub weapon_proficiencies: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub armor_proficiencies: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub saving_throw_proficiencies: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub expertise: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resist: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub immune: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub senses: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub additional_spells: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub other_sources: Option<Vec<serde_json::Value>>,
}

/// Container for feat data from JSON files
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatData {
    #[serde(default)]
    pub feat: Option<Vec<Feat>>,
}

/// Summary of a feat for search results
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatSummary {
    pub name: String,
    pub source: String,
    pub page: Option<u32>,
    pub prerequisites: Option<String>,
    pub brief: Option<String>,
}

impl From<&Feat> for FeatSummary {
    fn from(feat: &Feat) -> Self {
        // Extract prerequisites as a simple string
        let prerequisites = feat.prerequisite.as_ref().and_then(|prereqs| {
            if prereqs.is_empty() {
                None
            } else {
                // Try to format prerequisites in a readable way
                let mut prereq_parts = Vec::new();
                for prereq in prereqs {
                    if let Some(obj) = prereq.as_object() {
                        // Check for ability requirements
                        if let Some(ability) = obj.get("ability") {
                            if let Some(ability_arr) = ability.as_array() {
                                for ab in ability_arr {
                                    if let Some(ab_obj) = ab.as_object() {
                                        for (stat, value) in ab_obj {
                                            if let Some(val) = value.as_u64() {
                                                prereq_parts.push(format!("{} {}", 
                                                    stat.to_uppercase(), val));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        // Check for race requirements
                        if let Some(race) = obj.get("race") {
                            if let Some(race_arr) = race.as_array() {
                                for r in race_arr {
                                    if let Some(name) = r.get("name").and_then(|n| n.as_str()) {
                                        prereq_parts.push(format!("{}", name));
                                    }
                                }
                            }
                        }
                        // Check for level requirements
                        if let Some(level) = obj.get("level").and_then(|l| l.as_u64()) {
                            prereq_parts.push(format!("Level {}", level));
                        }
                        // Check for spellcasting
                        if obj.get("spellcasting").and_then(|s| s.as_bool()).unwrap_or(false) {
                            prereq_parts.push("Spellcasting".to_string());
                        }
                    }
                }
                if !prereq_parts.is_empty() {
                    Some(prereq_parts.join(", "))
                } else {
                    None
                }
            }
        });
        
        // Extract a brief description from the first entry
        let brief = feat.entries.first().and_then(|entry| {
            if let Some(text) = entry.as_str() {
                // Take first sentence or first 100 chars
                let truncated = if text.len() > 100 {
                    let end = text.char_indices()
                        .take_while(|(i, _)| *i < 100)
                        .map(|(i, _)| i)
                        .last()
                        .unwrap_or(100);
                    format!("{}...", &text[..end])
                } else {
                    text.to_string()
                };
                Some(truncated)
            } else {
                None
            }
        });
        
        FeatSummary {
            name: feat.name.clone(),
            source: feat.source.clone(),
            page: feat.page,
            prerequisites,
            brief,
        }
    }
}