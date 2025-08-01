//! Template frontmatter schema and parsing

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

/// Frontmatter structure for all template documents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateFrontmatter {
    /// Unique identifier for this template
    pub id: String,
    
    /// Human-readable title
    pub title: String,
    
    /// Template type (campaign_pitch, module_overview, etc.)
    #[serde(rename = "type")]
    pub template_type: String,
    
    /// Document level (campaign, module, session, handout)
    pub level: String,
    
    /// Brief description of the template's purpose
    pub purpose: String,
    
    /// List of template variables used in this document
    #[serde(default)]
    pub variables: Vec<TemplateVariable>,
    
    /// Author of the template
    #[serde(default = "default_author")]
    pub author: String,
}

fn default_author() -> String {
    "Mimir Team".to_string()
}

/// Template variable definition with required default value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateVariable {
    /// Variable name (without delimiters)
    pub name: String,
    
    /// Data type (string, number, boolean, list, object, etc.)
    #[serde(rename = "type")]
    pub var_type: String,
    
    /// Description of what this variable represents
    pub description: String,
    
    /// Default value for this variable (required)
    pub default: JsonValue,
    
    /// Whether this variable is required
    #[serde(default = "default_true")]
    pub required: bool,
}

fn default_true() -> bool {
    true
}

/// Example frontmatter YAML structure:
/// ```yaml
/// ---
/// id: campaign-pitch
/// title: Campaign Pitch Template
/// type: campaign_pitch
/// level: campaign
/// purpose: Create a one-page pitch to excite players about your campaign concept
/// author: Mimir Team
/// variables:
///   - name: campaign_name
///     type: string
///     description: The name of your campaign
///     default: "[Campaign Name]"
///     required: true
///   - name: genre
///     type: string
///     description: Primary genre and tone
///     default: "Fantasy Adventure"
///     required: true
///   - name: pillars
///     type: object
///     description: Campaign pillar ratings (1-5)
///     default:
///       combat: 3
///       exploration: 3
///       social: 3
///       mystery: 3
///     required: true
/// ---
/// ```

impl TemplateFrontmatter {
    /// Parse frontmatter from a markdown document
    pub fn parse_from_markdown(content: &str) -> Option<Self> {
        // Look for frontmatter delimited by ---
        if !content.starts_with("---\n") {
            return None;
        }
        
        let end_delimiter = content[4..].find("\n---\n")?;
        let frontmatter_yaml = &content[4..end_delimiter + 4];
        
        serde_yaml::from_str(frontmatter_yaml).ok()
    }
    
    /// Extract the content after frontmatter
    pub fn extract_content(markdown: &str) -> String {
        if let Some(end_pos) = markdown.find("\n---\n") {
            if markdown.starts_with("---\n") {
                // Find the second ---
                if let Some(second_end) = markdown[end_pos + 5..].find("\n---\n") {
                    return markdown[end_pos + 5 + second_end + 5..].trim().to_string();
                } else if let Some(second_end) = markdown[end_pos + 5..].find("---\n") {
                    return markdown[end_pos + 5 + second_end + 4..].trim().to_string();
                }
            }
        }
        markdown.to_string()
    }
    
    /// Convert to JSON for database storage
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }
    
    /// Create variables schema JSON
    pub fn variables_schema(&self) -> serde_json::Result<String> {
        serde_json::to_string(&self.variables)
    }
    
    /// Extract defaults as a map from variables
    pub fn defaults_map(&self) -> serde_json::Map<String, JsonValue> {
        let mut defaults = serde_json::Map::new();
        for var in &self.variables {
            defaults.insert(var.name.clone(), var.default.clone());
        }
        defaults
    }
}