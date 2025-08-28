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
    #[serde(rename = "itemType")]
    pub item_type: String,
    #[serde(rename = "typeName")]
    pub type_name: String,
    pub rarity: String,
    pub value: Option<f64>,
    pub weight: Option<f32>,
    pub ac: Option<u8>,
    pub damage: Option<String>,
    #[serde(rename = "reqAttune")]
    pub req_attune: Option<String>,
    pub description: String,
}

impl From<&Item> for ItemSummary {
    fn from(item: &Item) -> Self {
        let item_type = item.item_type.clone().unwrap_or_else(|| "Unknown".to_string());
        let type_name = get_type_name(&item_type);
        let rarity = item.rarity.clone().unwrap_or_else(|| "none".to_string());
        
        // Get first line of description for summary
        let description = item.entries.as_ref()
            .and_then(|entries| entries.first())
            .and_then(|e| e.as_str())
            .unwrap_or("")
            .chars()
            .take(200)
            .collect::<String>();
        
        // Combine damage fields if present
        let damage = match (&item.dmg1, &item.dmg2) {
            (Some(d1), Some(d2)) => Some(format!("{}/{}", d1, d2)),
            (Some(d1), None) => Some(d1.clone()),
            (None, Some(d2)) => Some(d2.clone()),
            _ => None,
        };
        
        ItemSummary {
            name: item.name.clone(),
            source: item.source.clone(),
            item_type,
            type_name,
            rarity,
            value: item.value,
            weight: item.weight,
            ac: item.ac,
            damage,
            req_attune: item.requires_attunement.clone(),
            description,
        }
    }
}

fn get_type_name(item_type: &str) -> String {
    match item_type {
        "M" => "Melee Weapon",
        "R" => "Ranged Weapon",
        "A" => "Ammunition",
        "LA" => "Light Armor",
        "MA" => "Medium Armor",
        "HA" => "Heavy Armor",
        "S" => "Shield",
        "G" => "Adventuring Gear",
        "AT" => "Artisan's Tools",
        "T" => "Tools",
        "GS" => "Gaming Set",
        "SCF" => "Spellcasting Focus",
        "INS" => "Instrument",
        "MNT" => "Mount",
        "TAH" => "Tack & Harness",
        "VEH" => "Vehicle",
        "FD" => "Food & Drink",
        "TG" => "Trade Good",
        "$C" => "Treasure",
        "W" => "Wondrous Item",
        "P" => "Potion",
        "RG" => "Ring",
        "RD" => "Rod",
        "SC" => "Scroll",
        "ST" => "Staff",
        "WD" => "Wand",
        _ => item_type,
    }.to_string()
}