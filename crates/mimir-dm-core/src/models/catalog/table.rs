use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    pub name: String,
    pub source: String,
    pub page: Option<i32>,
    
    #[serde(rename = "caption")]
    pub caption: Option<String>,
    
    #[serde(rename = "colLabels")]
    pub col_labels: Option<Vec<String>>,
    
    #[serde(rename = "colStyles")]
    pub col_styles: Option<Vec<String>>,
    
    pub rows: Vec<Vec<serde_json::Value>>,
    
    // Optional fields
    pub intro: Option<Vec<serde_json::Value>>,
    pub outro: Option<Vec<serde_json::Value>>,
    
    #[serde(rename = "tableInclude")]
    pub table_include: Option<serde_json::Value>,
    
    #[serde(rename = "footnotes")]
    pub footnotes: Option<Vec<serde_json::Value>>,
    
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
pub struct TableSummary {
    pub name: String,
    pub source: String,
    pub caption: String,
    pub columns: usize,
    pub rows: usize,
    pub category: String,
}

impl From<&Table> for TableSummary {
    fn from(table: &Table) -> Self {
        Self {
            name: table.name.clone(),
            source: table.source.clone(),
            caption: table.caption.clone().unwrap_or_else(|| table.name.clone()),
            columns: table.col_labels.as_ref().map(|c| c.len()).unwrap_or(
                table.rows.first().map(|r| r.len()).unwrap_or(0)
            ),
            rows: table.rows.len(),
            category: categorize_table(&table.name),
        }
    }
}

fn categorize_table(name: &str) -> String {
    let name_lower = name.to_lowercase();
    
    if name_lower.contains("madness") || name_lower.contains("insanity") {
        "Madness".to_string()
    } else if name_lower.contains("treasure") || name_lower.contains("loot") || name_lower.contains("hoard") {
        "Treasure".to_string()
    } else if name_lower.contains("encounter") || name_lower.contains("random") {
        "Encounters".to_string()
    } else if name_lower.contains("trinket") {
        "Trinkets".to_string()
    } else if name_lower.contains("wild magic") || name_lower.contains("surge") {
        "Wild Magic".to_string()
    } else if name_lower.contains("damage") || name_lower.contains("critical") {
        "Combat".to_string()
    } else if name_lower.contains("npc") || name_lower.contains("name") || name_lower.contains("personality") {
        "NPCs".to_string()
    } else if name_lower.contains("quest") || name_lower.contains("adventure") || name_lower.contains("plot") {
        "Adventures".to_string()
    } else if name_lower.contains("magic item") || name_lower.contains("artifact") {
        "Magic Items".to_string()
    } else {
        "Miscellaneous".to_string()
    }
}

// Container for JSON parsing
#[derive(Debug, Serialize, Deserialize)]
pub struct TableData {
    pub table: Option<Vec<Table>>,
}

// Fluff data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableFluff {
    pub name: String,
    pub source: String,
    pub entries: Option<Vec<serde_json::Value>>,
    pub images: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableFluffData {
    #[serde(rename = "tableFluff")]
    pub table_fluff: Option<Vec<TableFluff>>,
}