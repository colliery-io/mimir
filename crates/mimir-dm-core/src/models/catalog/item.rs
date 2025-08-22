//! Item catalog models

use serde::{Deserialize, Serialize};

/// A D&D 5e item
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub name: String,
    pub source: String,
    pub page: Option<u32>,
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    pub rarity: Option<String>,
    #[serde(default)]
    pub weight: Option<f32>,
    #[serde(default)]
    pub value: Option<f64>,  // Value in copper pieces (can be fractional)
    #[serde(default)]
    pub ac: Option<u8>,
    #[serde(default)]
    pub dmg1: Option<String>,
    #[serde(default)]
    pub dmg2: Option<String>,
    #[serde(default)]
    pub dmg_type: Option<String>,
    #[serde(default)]
    pub property: Option<Vec<String>>,
    #[serde(default)]
    pub range: Option<String>,
    #[serde(default)]
    pub reload: Option<u8>,
    #[serde(default)]
    pub requires_attunement: Option<String>,
    #[serde(default)]
    pub entries: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub srd: Option<bool>,
}

/// Container for item data from JSON files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemData {
    pub item: Vec<Item>,
}

/// Simplified item for search results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemSummary {
    pub name: String,
    pub source: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub rarity: String,
    pub attunement: bool,
    pub description: String,
}

impl From<&Item> for ItemSummary {
    fn from(item: &Item) -> Self {
        let item_type = item.item_type.clone().unwrap_or_else(|| "Unknown".to_string());
        let rarity = item.rarity.clone().unwrap_or_else(|| "Common".to_string());
        let attunement = item.requires_attunement.is_some();
        
        // Get first line of description for summary
        let description = item.entries.as_ref()
            .and_then(|entries| entries.first())
            .and_then(|e| e.as_str())
            .unwrap_or("")
            .chars()
            .take(200)
            .collect::<String>();
        
        ItemSummary {
            name: item.name.clone(),
            source: item.source.clone(),
            item_type,
            rarity,
            attunement,
            description,
        }
    }
}