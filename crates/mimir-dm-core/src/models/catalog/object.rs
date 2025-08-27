use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArmorClass {
    Number(i32),
    Object {
        ac: Option<i32>,
        special: Option<String>,
        from: Option<Vec<String>>,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackEntry {
    #[serde(rename = "attackType")]
    pub attack_type: Option<String>, // MW (Melee Weapon), RW (Ranged Weapon), etc.
    
    #[serde(rename = "attackEntries")]
    pub attack_entries: Option<Vec<String>>,
    
    #[serde(rename = "hitEntries")]
    pub hit_entries: Option<Vec<String>>,
    
    #[serde(rename = "type")]
    pub entry_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionEntry {
    pub name: String,
    
    #[serde(rename = "type")]
    pub action_type: Option<String>,
    
    pub entries: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DndObject {
    pub name: String,
    pub source: String,
    pub page: Option<i32>,
    
    #[serde(rename = "objectType")]
    pub object_type: Option<String>, // SW (Siege Weapon), etc.
    
    pub size: Option<Vec<String>>,
    
    pub ac: Option<ArmorClass>,
    pub hp: Option<i32>,
    
    pub immune: Option<Vec<String>>,
    pub resist: Option<Vec<String>>,
    pub vulnerable: Option<Vec<String>>,
    
    #[serde(rename = "actionEntries")]
    pub action_entries: Option<Vec<ActionEntry>>,
    
    pub entries: Option<Vec<serde_json::Value>>,
    
    #[serde(rename = "hasToken")]
    pub has_token: Option<bool>,
    
    #[serde(rename = "tokenCredit")]
    pub token_credit: Option<String>,
    
    pub srd: Option<bool>,
    
    #[serde(rename = "hasFluff")]
    pub has_fluff: Option<bool>,
    
    #[serde(rename = "hasFluffImages")]
    pub has_fluff_images: Option<bool>,
}

// Summary for list views
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectSummary {
    pub name: String,
    pub source: String,
    pub object_type: String,
    pub size: String,
    pub ac: String,
    pub hp: String,
    pub is_srd: bool,
}

impl From<&DndObject> for ObjectSummary {
    fn from(obj: &DndObject) -> Self {
        Self {
            name: obj.name.clone(),
            source: obj.source.clone(),
            object_type: format_object_type(&obj.object_type),
            size: format_size(&obj.size),
            ac: format_ac(&obj.ac),
            hp: obj.hp.map_or("—".to_string(), |h| h.to_string()),
            is_srd: obj.srd.unwrap_or(false),
        }
    }
}

fn format_object_type(object_type: &Option<String>) -> String {
    match object_type.as_deref() {
        Some("SW") => "Siege Weapon".to_string(),
        Some("GEN") => "Generic".to_string(),
        Some(other) => other.to_string(),
        None => "Object".to_string(),
    }
}

fn format_size(size: &Option<Vec<String>>) -> String {
    if let Some(sizes) = size {
        sizes.iter().map(|s| {
            match s.as_str() {
                "T" => "Tiny",
                "S" => "Small",
                "M" => "Medium",
                "L" => "Large",
                "H" => "Huge",
                "G" => "Gargantuan",
                _ => s.as_str(),
            }
        }).collect::<Vec<_>>().join("/")
    } else {
        "—".to_string()
    }
}

fn format_ac(ac: &Option<ArmorClass>) -> String {
    match ac {
        Some(ArmorClass::Number(n)) => n.to_string(),
        Some(ArmorClass::Object { ac: Some(n), .. }) => n.to_string(),
        Some(ArmorClass::Object { special: Some(s), .. }) => s.clone(),
        _ => "—".to_string(),
    }
}

// Container for JSON parsing
#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectData {
    pub object: Option<Vec<DndObject>>,
}