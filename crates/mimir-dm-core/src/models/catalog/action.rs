use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub name: String,
    pub source: String,
    pub page: Option<i32>,
    #[serde(default)]
    pub entries: Vec<serde_json::Value>,
    #[serde(default)]
    pub time: Vec<serde_json::Value>,
    #[serde(rename = "seeAlsoAction", default)]
    pub see_also_action: Vec<String>,
    pub srd: Option<bool>,
    #[serde(rename = "basicRules")]
    pub basic_rules: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionData {
    #[serde(default)]
    pub action: Option<Vec<Action>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionSummary {
    pub name: String,
    pub source: String,
    pub time: String,
    pub description: String,
    pub see_also: Vec<String>,
}

impl From<&Action> for ActionSummary {
    fn from(action: &Action) -> Self {
        // Format time
        let time = format_action_time(&action.time);
        
        // Extract first sentence or paragraph as description
        let description = extract_description(&action.entries);
        
        // Format see also actions
        let see_also = action.see_also_action.iter()
            .map(|s| {
                // Remove source suffix if present (e.g., "Disarm|DMG" -> "Disarm")
                s.split('|').next().unwrap_or(s).to_string()
            })
            .collect();
        
        Self {
            name: action.name.clone(),
            source: action.source.clone(),
            time,
            description,
            see_also,
        }
    }
}

fn format_action_time(time: &[serde_json::Value]) -> String {
    if time.is_empty() {
        return "1 action".to_string();
    }
    
    let mut times = Vec::new();
    for t in time {
        if let Some(s) = t.as_str() {
            times.push(s.to_string());
        } else if let Some(obj) = t.as_object() {
            if let (Some(number), Some(unit)) = 
                (obj.get("number").and_then(|n| n.as_i64()),
                 obj.get("unit").and_then(|u| u.as_str())) {
                if number == 1 {
                    times.push(format!("1 {}", unit));
                } else {
                    times.push(format!("{} {}s", number, unit));
                }
            }
        }
    }
    
    if times.is_empty() {
        "1 action".to_string()
    } else {
        times.join(" or ")
    }
}

fn extract_description(entries: &[serde_json::Value]) -> String {
    if entries.is_empty() {
        return String::new();
    }
    
    // Get first entry as description
    if let Some(first) = entries.first() {
        if let Some(s) = first.as_str() {
            // Take first sentence or up to 150 chars
            let desc = if let Some(period_idx) = s.find(". ") {
                &s[..=period_idx]
            } else if s.len() > 150 {
                &s[..147]
            } else {
                s
            };
            
            // Strip any formatting tags for summary
            desc.replace("{@", "")
                .replace("}", "")
                .split('|')
                .next()
                .unwrap_or(desc)
                .to_string()
        } else {
            "Action".to_string()
        }
    } else {
        String::new()
    }
}