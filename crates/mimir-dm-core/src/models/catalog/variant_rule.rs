use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantRule {
    pub name: String,
    pub source: String,
    
    #[serde(rename = "ruleType")]
    pub rule_type: Option<String>,
    
    pub page: Option<i32>,
    
    #[serde(default)]
    pub entries: Option<Vec<serde_json::Value>>,
    
    #[serde(flatten)]
    pub other_fields: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantRuleData {
    pub variantrule: Option<Vec<VariantRule>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct VariantRuleSummary {
    pub name: String,
    pub source: String,
    pub rule_type: Option<String>,
    pub page: Option<i32>,
}

impl From<&VariantRule> for VariantRuleSummary {
    fn from(rule: &VariantRule) -> Self {
        VariantRuleSummary {
            name: rule.name.clone(),
            source: rule.source.clone(),
            rule_type: rule.rule_type.clone(),
            page: rule.page,
        }
    }
}