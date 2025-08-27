use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cult {
    pub name: String,
    pub source: String,
    
    #[serde(rename = "type")]
    pub cult_type: Option<String>,
    
    pub page: Option<i32>,
    
    #[serde(default)]
    pub entries: Option<Vec<serde_json::Value>>,
    
    // Cult-specific fields
    pub cultists: Option<CultistInfo>,
    pub goal: Option<GoalInfo>,
    
    #[serde(rename = "signatureSpells")]
    pub signature_spells: Option<SignatureSpellInfo>,
    
    #[serde(rename = "otherSources")]
    pub other_sources: Option<Vec<SourceReference>>,
    
    #[serde(flatten)]
    pub other_fields: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CultistInfo {
    pub entry: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalInfo {
    pub entry: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureSpellInfo {
    pub entry: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceReference {
    pub page: i32,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Boon {
    pub name: String,
    pub source: String,
    
    #[serde(rename = "type")]
    pub boon_type: Option<String>,
    
    pub page: Option<i32>,
    
    #[serde(default)]
    pub entries: Option<Vec<serde_json::Value>>,
    
    // Boon-specific fields
    pub ability: Option<AbilityInfo>,
    
    #[serde(rename = "signatureSpells")]
    pub signature_spells: Option<SignatureSpellInfo>,
    
    #[serde(rename = "reprintedAs")]
    pub reprinted_as: Option<Vec<ReprintInfo>>,
    
    #[serde(flatten)]
    pub other_fields: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbilityInfo {
    pub entry: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReprintInfo {
    pub tag: String,
    pub uid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CultData {
    pub cult: Option<Vec<Cult>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoonData {
    pub boon: Option<Vec<Boon>>,
}

// Combined summary for both cults and boons since they're related
#[derive(Debug, Clone, Serialize)]
pub struct CultBoonSummary {
    pub name: String,
    pub source: String,
    pub item_type: String, // "cult" or "boon"
    pub subtype: Option<String>, // Diabolical, Demonic, Elder Evil, etc.
    pub page: Option<i32>,
}

impl From<&Cult> for CultBoonSummary {
    fn from(cult: &Cult) -> Self {
        CultBoonSummary {
            name: cult.name.clone(),
            source: cult.source.clone(),
            item_type: "cult".to_string(),
            subtype: cult.cult_type.clone(),
            page: cult.page,
        }
    }
}

impl From<&Boon> for CultBoonSummary {
    fn from(boon: &Boon) -> Self {
        CultBoonSummary {
            name: boon.name.clone(),
            source: boon.source.clone(),
            item_type: "boon".to_string(),
            subtype: boon.boon_type.clone(),
            page: boon.page,
        }
    }
}