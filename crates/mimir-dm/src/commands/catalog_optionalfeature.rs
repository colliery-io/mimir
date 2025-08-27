use std::fs;
use std::path::Path;
use tauri::State;
use tracing::{debug, error, info, warn};
use mimir_dm_core::models::catalog::{OptionalFeature, OptionalFeatureData, OptionalFeatureSummary};

/// In-memory catalog of optional features
pub struct OptionalFeatureCatalog {
    pub optional_features: Vec<OptionalFeature>,
}

impl OptionalFeatureCatalog {
    pub fn new() -> Self {
        Self {
            optional_features: Vec::new(),
        }
    }
    
    /// Load optional feature data from the books directory
    pub fn load_from_books_directory(&mut self, books_dir: &Path) -> Result<(), String> {
        info!("Loading optional features from {:?}", books_dir);
        
        self.optional_features.clear();
        
        if !books_dir.exists() {
            warn!("Books directory does not exist: {:?}", books_dir);
            return Ok(());
        }
        
        // Iterate through each book directory
        for entry in fs::read_dir(books_dir).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let book_path = entry.path();
            
            if !book_path.is_dir() {
                continue;
            }
            
            let book_id = book_path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");
            
            // Check for optionalfeatures directory
            let opt_features_dir = book_path.join("optionalfeatures");
            if opt_features_dir.exists() && opt_features_dir.is_dir() {
                debug!("Found optional features directory for book: {}", book_id);
                
                // Load all JSON files in the optionalfeatures directory
                for opt_entry in fs::read_dir(&opt_features_dir).map_err(|e| e.to_string())? {
                    let opt_entry = opt_entry.map_err(|e| e.to_string())?;
                    let opt_file = opt_entry.path();
                    
                    if opt_file.extension().and_then(|e| e.to_str()) != Some("json") {
                        continue;
                    }
                    
                    let filename = opt_file.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown");
                    
                    match fs::read_to_string(&opt_file) {
                        Ok(content) => {
                            match serde_json::from_str::<OptionalFeatureData>(&content) {
                                Ok(data) => {
                                    if let Some(features) = data.optional_features {
                                        if !features.is_empty() {
                                            info!("Loaded {} optional features from {}/{}", 
                                                  features.len(), book_id, filename);
                                            self.optional_features.extend(features);
                                        }
                                    }
                                }
                                Err(e) => {
                                    error!("Failed to parse optional features from {}/{}: {}", book_id, filename, e);
                                }
                            }
                        }
                        Err(e) => {
                            error!("Failed to read {}/{}: {}", book_id, filename, e);
                        }
                    }
                }
            }
        }
        
        info!("Total optional features loaded: {}", self.optional_features.len());
        Ok(())
    }
    
    /// Search optional features with filters
    pub fn search(&self, 
        query: Option<String>,
        sources: Vec<String>,
        feature_types: Vec<String>
    ) -> Vec<OptionalFeatureSummary> {
        info!("Searching optional features - query: {:?}, sources: {:?}, types: {:?}", 
                 query, sources, feature_types);
        
        let results: Vec<OptionalFeatureSummary> = self.optional_features.iter()
            .filter(|opt| {
                // Filter by query
                if let Some(q) = &query {
                    if !q.is_empty() {
                        let name_match = opt.name.to_lowercase().contains(&q.to_lowercase());
                        // Also search in entries
                        let entry_match = opt.entries.iter().any(|e| {
                            if let Some(s) = e.as_str() {
                                s.to_lowercase().contains(&q.to_lowercase())
                            } else {
                                false
                            }
                        });
                        if !name_match && !entry_match {
                            return false;
                        }
                    }
                }
                
                // Filter by sources
                if !sources.is_empty() && !sources.contains(&opt.source) {
                    return false;
                }
                
                // Filter by feature types
                if !feature_types.is_empty() {
                    let has_matching_type = opt.feature_type.iter()
                        .any(|ft| feature_types.contains(ft));
                    if !has_matching_type {
                        return false;
                    }
                }
                
                true
            })
            .map(OptionalFeatureSummary::from)
            .collect();
        
        info!("Found {} optional features matching filters", results.len());
        results
    }
    
    /// Get detailed optional feature information
    pub fn get_optional_feature_details(&self, name: &str, source: &str) -> Option<OptionalFeature> {
        self.optional_features.iter()
            .find(|opt| opt.name == name && opt.source == source)
            .cloned()
    }
    
    /// Get all unique feature types in the catalog
    pub fn get_feature_types(&self) -> Vec<String> {
        let mut types = std::collections::HashSet::new();
        for opt in &self.optional_features {
            for ft in &opt.feature_type {
                types.insert(ft.clone());
            }
        }
        let mut sorted: Vec<String> = types.into_iter().collect();
        sorted.sort();
        sorted
    }
}

/// Initialize the optional feature catalog
#[tauri::command]
pub async fn init_optional_feature_catalog(
    catalog: State<'_, std::sync::Mutex<OptionalFeatureCatalog>>
) -> Result<(), String> {
    use crate::APP_PATHS;
    
    // Get app paths to find the books directory
    let app_paths = APP_PATHS.get()
        .ok_or_else(|| "App paths not initialized".to_string())?;
    
    let books_dir = app_paths.data_dir.join("books");
    
    let mut catalog = catalog.lock().map_err(|e| e.to_string())?;
    catalog.load_from_books_directory(&books_dir)?;
    
    Ok(())
}

/// Search optional features with filters
#[tauri::command]
pub async fn search_optional_features(
    query: Option<String>,
    sources: Option<Vec<String>>,
    feature_types: Option<Vec<String>>,
    catalog: State<'_, std::sync::Mutex<OptionalFeatureCatalog>>
) -> Result<Vec<OptionalFeatureSummary>, String> {
    let catalog = catalog.lock().map_err(|e| e.to_string())?;
    
    Ok(catalog.search(
        query,
        sources.unwrap_or_default(),
        feature_types.unwrap_or_default()
    ))
}

/// Get detailed information about a specific optional feature
#[tauri::command]
pub async fn get_optional_feature_details(
    name: String,
    source: String,
    catalog: State<'_, std::sync::Mutex<OptionalFeatureCatalog>>
) -> Result<OptionalFeature, String> {
    let catalog = catalog.lock().map_err(|e| e.to_string())?;
    
    catalog.get_optional_feature_details(&name, &source)
        .ok_or_else(|| format!("Optional feature not found: {} from {}", name, source))
}

/// Get all available feature types
#[tauri::command]
pub async fn get_feature_types(
    catalog: State<'_, std::sync::Mutex<OptionalFeatureCatalog>>
) -> Result<Vec<String>, String> {
    let catalog = catalog.lock().map_err(|e| e.to_string())?;
    Ok(catalog.get_feature_types())
}