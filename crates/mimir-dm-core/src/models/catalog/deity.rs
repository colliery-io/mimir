use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deity {
    pub name: String,
    pub source: String,
    pub page: Option<i32>,
    
    pub title: Option<String>,
    pub pantheon: Option<String>,
    pub alignment: Option<Vec<String>>,
    pub domains: Option<Vec<String>>,
    pub symbol: Option<String>,
    
    #[serde(rename = "additionalSources")]
    pub additional_sources: Option<Vec<SourceReference>>,
    
    pub entries: Option<Vec<serde_json::Value>>,
    
    #[serde(rename = "hasFluff")]
    pub has_fluff: Option<bool>,
    
    #[serde(rename = "hasFluffImages")]
    pub has_fluff_images: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceReference {
    pub source: String,
    pub page: Option<i32>,
}

// Summary for list views
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeitySummary {
    pub name: String,
    pub source: String,
    pub title: String,
    pub pantheon: String,
    pub alignment: String,
    pub domains: Vec<String>,
    pub symbol: String,
}

impl From<&Deity> for DeitySummary {
    fn from(deity: &Deity) -> Self {
        Self {
            name: deity.name.clone(),
            source: deity.source.clone(),
            title: deity.title.clone().unwrap_or_default(),
            pantheon: deity.pantheon.clone().unwrap_or_default(),
            alignment: format_alignment(&deity.alignment),
            domains: deity.domains.clone().unwrap_or_default(),
            symbol: deity.symbol.clone().unwrap_or_default(),
        }
    }
}

fn format_alignment(alignment: &Option<Vec<String>>) -> String {
    if let Some(align) = alignment {
        align.iter().map(|a| {
            match a.as_str() {
                "L" => "Lawful",
                "N" => "Neutral",
                "C" => "Chaotic",
                "G" => "Good",
                "E" => "Evil",
                "U" => "Unaligned",
                "A" => "Any",
                _ => a,
            }
        }).collect::<Vec<_>>().join(" ")
    } else {
        String::new()
    }
}

// Container for JSON parsing
#[derive(Debug, Serialize, Deserialize)]
pub struct DeityData {
    pub deity: Option<Vec<Deity>>,
}