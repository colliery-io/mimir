//! Bundle manifest parsing and validation

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Bundle manifest containing metadata and validation information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleManifest {
    /// Format version for compatibility checking
    pub format_version: String,
    
    /// Unique identifier for this bundle
    pub bundle_id: String,
    
    /// Human-readable bundle name
    pub bundle_name: String,
    
    /// Bundle version string
    pub bundle_version: String,
    
    /// Rule system identifier
    pub rule_system: String,
    
    /// Description of bundle contents
    #[serde(default)]
    pub description: String,
    
    /// List of source books included
    #[serde(default)]
    pub sources_included: Vec<String>,
    
    /// Entity counts for validation
    pub entity_counts: EntityCounts,
    
    /// Optional metadata (free-form)
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Entity counts for validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityCounts {
    /// Number of source books
    #[serde(default)]
    pub sources: usize,
    
    /// Number of races (including subraces)
    #[serde(default)]
    pub races: usize,
    
    /// Number of classes (including subclasses)
    #[serde(default)]
    pub classes: usize,
    
    /// Number of items (including variants)
    #[serde(default)]
    pub items: usize,
    
    /// Number of spells
    #[serde(default)]
    pub spells: usize,
    
    /// Number of creatures
    #[serde(default)]
    pub creatures: usize,
    
    /// Number of backgrounds
    #[serde(default)]
    pub backgrounds: usize,
    
    /// Number of feats
    #[serde(default)]
    pub feats: usize,
}

impl BundleManifest {
    /// Get total entity count across all types
    pub fn total_entities(&self) -> usize {
        self.entity_counts.sources
            + self.entity_counts.races
            + self.entity_counts.classes
            + self.entity_counts.items
            + self.entity_counts.spells
            + self.entity_counts.creatures
            + self.entity_counts.backgrounds
            + self.entity_counts.feats
    }
    
    /// Check if this manifest is compatible with the current format version
    pub fn is_compatible(&self) -> bool {
        // For now, we only support format version 1.0
        self.format_version.starts_with("1.0")
    }
    
    /// Validate that required fields are present and valid
    pub fn validate(&self) -> Result<(), String> {
        if self.bundle_id.is_empty() {
            return Err("bundle_id cannot be empty".to_string());
        }
        
        if self.bundle_name.is_empty() {
            return Err("bundle_name cannot be empty".to_string());
        }
        
        if self.bundle_version.is_empty() {
            return Err("bundle_version cannot be empty".to_string());
        }
        
        if self.rule_system.is_empty() {
            return Err("rule_system cannot be empty".to_string());
        }
        
        if !self.is_compatible() {
            return Err(format!(
                "Unsupported format version: {}. Expected version 1.0.x",
                self.format_version
            ));
        }
        
        Ok(())
    }
}

impl Default for EntityCounts {
    fn default() -> Self {
        Self {
            sources: 0,
            races: 0,
            classes: 0,
            items: 0,
            spells: 0,
            creatures: 0,
            backgrounds: 0,
            feats: 0,
        }
    }
}