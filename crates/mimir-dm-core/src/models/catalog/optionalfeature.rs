use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionalFeature {
    pub name: String,
    pub source: String,
    pub page: Option<i32>,
    
    #[serde(rename = "featureType")]
    pub feature_type: Vec<String>, // EI, MM, FS:F, etc.
    
    pub prerequisite: Option<Vec<Prerequisite>>,
    pub entries: Vec<serde_json::Value>,
    
    #[serde(rename = "isClassFeatureVariant")]
    pub is_class_feature_variant: Option<bool>,
    
    pub consumes: Option<Consumes>,
    
    #[serde(rename = "additionalSpells")]
    pub additional_spells: Option<Vec<serde_json::Value>>,
    
    pub srd: Option<bool>,
    
    #[serde(rename = "hasFluffImages")]
    pub has_fluff_images: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prerequisite {
    pub level: Option<PrerequisiteLevel>,
    pub spell: Option<Vec<String>>,
    pub pact: Option<String>,
    pub patron: Option<String>,
    pub feature: Option<Vec<String>>,
    pub item: Option<Vec<String>>,
    pub other_summary: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PrerequisiteLevel {
    Simple(i32),
    Complex {
        level: i32,
        class: Option<ClassRef>,
        subclass: Option<SubclassRef>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassRef {
    pub name: String,
    pub source: Option<String>,  // Make source optional
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubclassRef {
    pub name: String,
    pub source: Option<String>,  // Make source optional
    #[serde(rename = "shortName")]
    pub short_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Consumes {
    pub name: String,
    pub amount: Option<i32>,
}

// Summary for list views
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionalFeatureSummary {
    pub name: String,
    pub source: String,
    pub feature_types: Vec<String>,
    pub feature_type_full: String, // Formatted display name
    pub prerequisite_text: String,
    pub is_srd: bool,
    pub grants_spells: bool,
}

impl From<&OptionalFeature> for OptionalFeatureSummary {
    fn from(opt: &OptionalFeature) -> Self {
        Self {
            name: opt.name.clone(),
            source: opt.source.clone(),
            feature_types: opt.feature_type.clone(),
            feature_type_full: format_feature_types(&opt.feature_type),
            prerequisite_text: format_prerequisites(&opt.prerequisite),
            is_srd: opt.srd.unwrap_or(false),
            grants_spells: opt.additional_spells.is_some(),
        }
    }
}

fn format_feature_types(types: &[String]) -> String {
    let formatted: Vec<String> = types.iter().map(|t| {
        match t.as_str() {
            "AI" => "Artificer Infusion",
            "ED" => "Elemental Discipline",
            "EI" => "Eldritch Invocation",
            "MM" => "Metamagic",
            "MV" => "Maneuver",
            "MV:B" => "Maneuver (Battle Master)",
            "MV:C2-UA" => "Maneuver (Cavalier V2 UA)",
            "AS:V1-UA" => "Arcane Shot (V1 UA)",
            "AS:V2-UA" => "Arcane Shot (V2 UA)",
            "AS" => "Arcane Shot",
            "OTH" => "Other",
            "FS:F" => "Fighting Style (Fighter)",
            "FS:B" => "Fighting Style (Bard)",
            "FS:P" => "Fighting Style (Paladin)",
            "FS:R" => "Fighting Style (Ranger)",
            "PB" => "Pact Boon",
            "OR" => "Onomancy Resonant",
            "RN" => "Rune Knight Rune",
            "AF" => "Alchemical Formula",
            "TT" => "Traveler's Trick",
            _ => t,
        }
    }.to_string()).collect();
    
    formatted.join(", ")
}

fn format_prerequisites(prereqs: &Option<Vec<Prerequisite>>) -> String {
    if let Some(prereqs) = prereqs {
        let parts: Vec<String> = prereqs.iter().filter_map(|p| {
            if let Some(level) = &p.level {
                match level {
                    PrerequisiteLevel::Simple(lvl) => Some(format!("Level {}", lvl)),
                    PrerequisiteLevel::Complex { level, class, subclass } => {
                        let mut text = String::new();
                        if let Some(cls) = class {
                            text.push_str(&cls.name);
                            if let Some(sub) = subclass {
                                text.push_str(&format!(" ({})", sub.name));
                            }
                            text.push_str(&format!(" Level {}", level));
                        } else {
                            text.push_str(&format!("Level {}", level));
                        }
                        Some(text)
                    }
                }
            } else if let Some(pact) = &p.pact {
                Some(format!("Pact of the {}", pact))
            } else if let Some(patron) = &p.patron {
                Some(format!("{} Patron", patron))
            } else if let Some(spells) = &p.spell {
                if !spells.is_empty() {
                    Some(format!("{} cantrip", spells[0].replace("#c", "")))
                } else {
                    None
                }
            } else if let Some(features) = &p.feature {
                if !features.is_empty() {
                    Some(features.join(", "))
                } else {
                    None
                }
            } else if let Some(summary) = &p.other_summary {
                Some(summary.clone())
            } else {
                None
            }
        }).collect();
        
        if parts.is_empty() {
            String::new()
        } else {
            parts.join("; ")
        }
    } else {
        String::new()
    }
}

// Container for JSON parsing
#[derive(Debug, Serialize, Deserialize)]
pub struct OptionalFeatureData {
    #[serde(rename = "optionalfeature")]
    pub optional_features: Option<Vec<OptionalFeature>>,
}