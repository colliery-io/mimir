use std::collections::HashMap;
use std::path::Path;
use tracing::{error, info, warn};

use mimir_dm_core::models::catalog::{VariantRule, VariantRuleData, VariantRuleSummary};

pub struct VariantRuleCatalog {
    rules: Vec<VariantRule>,
    by_type: HashMap<String, Vec<usize>>,
    by_source: HashMap<String, Vec<usize>>,
}

impl VariantRuleCatalog {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            by_type: HashMap::new(),
            by_source: HashMap::new(),
        }
    }

    pub async fn load(&mut self, data_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        self.rules.clear();
        self.by_type.clear();
        self.by_source.clear();

        let books_path = data_path.join("books");
        if !books_path.exists() {
            warn!("Books directory not found at: {:?}", books_path);
            return Ok(());
        }

        // Read all book directories
        let mut entries = tokio::fs::read_dir(&books_path).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }

            let book_name = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");

            // Look for variantrules directory
            let variantrules_dir = path.join("variantrules");
            if !variantrules_dir.exists() {
                continue;
            }

            // Read all JSON files in variantrules directory
            let mut rule_entries = tokio::fs::read_dir(&variantrules_dir).await?;
            while let Some(rule_entry) = rule_entries.next_entry().await? {
                let rule_path = rule_entry.path();
                if rule_path.extension().and_then(|e| e.to_str()) != Some("json") {
                    continue;
                }

                match self.load_variantrules_file(&rule_path, book_name).await {
                    Ok(count) => {
                        info!("Loaded {} variant rules from {:?}", count, rule_path);
                    }
                    Err(e) => {
                        error!("Failed to parse variant rules from {:?}: {}", rule_path, e);
                    }
                }
            }
        }

        // Build indices
        for (idx, rule) in self.rules.iter().enumerate() {
            // By type
            if let Some(rule_type) = &rule.rule_type {
                self.by_type
                    .entry(rule_type.clone())
                    .or_insert_with(Vec::new)
                    .push(idx);
            } else {
                self.by_type
                    .entry("General".to_string())
                    .or_insert_with(Vec::new)
                    .push(idx);
            }

            // By source
            self.by_source
                .entry(rule.source.clone())
                .or_insert_with(Vec::new)
                .push(idx);
        }

        info!("Loaded {} total variant rules", self.rules.len());
        Ok(())
    }

    async fn load_variantrules_file(&mut self, path: &Path, source: &str) -> Result<usize, Box<dyn std::error::Error>> {
        let content = tokio::fs::read_to_string(path).await?;
        let data: VariantRuleData = serde_json::from_str(&content)?;
        
        let mut count = 0;
        if let Some(rules) = data.variantrule {
            for mut rule in rules {
                // Ensure source is set
                if rule.source.is_empty() {
                    rule.source = source.to_string();
                }
                self.rules.push(rule);
                count += 1;
            }
        }
        
        Ok(count)
    }

    pub fn search(&self, params: SearchParams) -> Vec<VariantRuleSummary> {
        let mut results: Vec<(usize, &VariantRule)> = self.rules
            .iter()
            .enumerate()
            .filter(|(_, rule)| {
                // Filter by query
                if let Some(ref query) = params.query {
                    let query_lower = query.to_lowercase();
                    if !rule.name.to_lowercase().contains(&query_lower) {
                        return false;
                    }
                }

                // Filter by types
                if let Some(ref types) = params.types {
                    let rule_type = rule.rule_type.as_ref().map(|s| s.as_str()).unwrap_or("General");
                    if !types.iter().any(|t| t == rule_type) {
                        return false;
                    }
                }

                // Filter by sources
                if let Some(ref sources) = params.sources {
                    if !sources.contains(&rule.source) {
                        return false;
                    }
                }

                true
            })
            .collect();

        // Sort by name
        results.sort_by(|a, b| a.1.name.cmp(&b.1.name));

        results.into_iter()
            .map(|(_, rule)| VariantRuleSummary::from(rule))
            .collect()
    }

    pub fn get_details(&self, name: &str, source: &str) -> Option<VariantRule> {
        self.rules
            .iter()
            .find(|r| r.name.eq_ignore_ascii_case(name) && r.source == source)
            .cloned()
    }

    pub fn get_types(&self) -> Vec<String> {
        let mut types: Vec<String> = self.by_type.keys().cloned().collect();
        types.sort();
        types
    }

    pub fn get_sources(&self) -> Vec<String> {
        let mut sources: Vec<String> = self.by_source.keys().cloned().collect();
        sources.sort();
        sources
    }
}

#[derive(Debug, Clone)]
pub struct SearchParams {
    pub query: Option<String>,
    pub types: Option<Vec<String>>,
    pub sources: Option<Vec<String>>,
}

// Tauri command interface
#[tauri::command]
pub async fn init_variant_rule_catalog(
    catalog: tauri::State<'_, std::sync::Mutex<VariantRuleCatalog>>
) -> Result<(), String> {
    use crate::APP_PATHS;
    
    // Get app paths to find the books directory
    let app_paths = APP_PATHS.get()
        .ok_or_else(|| "App paths not initialized".to_string())?;
    
    let data_dir = app_paths.data_dir.clone();
    
    // We need to clone the catalog to avoid holding the lock across await
    let mut new_catalog = VariantRuleCatalog::new();
    new_catalog.load(&data_dir)
        .await
        .map_err(|e| format!("Failed to load variant rule catalog: {}", e))?;
    
    // Now update the shared catalog
    let mut cat = catalog.lock()
        .map_err(|e| format!("Failed to lock catalog: {}", e))?;
    *cat = new_catalog;
    
    Ok(())
}

#[tauri::command]
pub async fn search_variant_rules(
    catalog: tauri::State<'_, std::sync::Mutex<VariantRuleCatalog>>,
    query: Option<String>,
    types: Option<Vec<String>>,
    sources: Option<Vec<String>>,
) -> Result<Vec<VariantRuleSummary>, String> {
    let cat = catalog.lock()
        .map_err(|e| format!("Failed to lock catalog: {}", e))?;
    Ok(cat.search(SearchParams {
        query,
        types,
        sources,
    }))
}

#[tauri::command]
pub async fn get_variant_rule_details(
    catalog: tauri::State<'_, std::sync::Mutex<VariantRuleCatalog>>,
    name: String, 
    source: String
) -> Result<Option<VariantRule>, String> {
    let cat = catalog.lock()
        .map_err(|e| format!("Failed to lock catalog: {}", e))?;
    Ok(cat.get_details(&name, &source))
}

#[tauri::command]
pub async fn get_variant_rule_types(
    catalog: tauri::State<'_, std::sync::Mutex<VariantRuleCatalog>>
) -> Result<Vec<String>, String> {
    let cat = catalog.lock()
        .map_err(|e| format!("Failed to lock catalog: {}", e))?;
    Ok(cat.get_types())
}

#[tauri::command]
pub async fn get_variant_rule_sources(
    catalog: tauri::State<'_, std::sync::Mutex<VariantRuleCatalog>>
) -> Result<Vec<String>, String> {
    let cat = catalog.lock()
        .map_err(|e| format!("Failed to lock catalog: {}", e))?;
    Ok(cat.get_sources())
}