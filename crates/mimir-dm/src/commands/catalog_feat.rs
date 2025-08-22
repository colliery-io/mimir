use mimir_dm_core::models::catalog::{Feat, FeatData, FeatSummary};
use std::fs;
use std::path::Path;
use tauri::State;
use tracing::{debug, info, warn};

/// Feat catalog that holds all loaded feats
pub struct FeatCatalog {
    feats: Vec<Feat>,
}

impl FeatCatalog {
    pub fn new() -> Self {
        Self {
            feats: Vec::new(),
        }
    }
    
    /// Load feats from a book directory
    pub fn load_from_book(&mut self, book_path: &Path) -> Result<(), String> {
        let book_id = book_path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| "Invalid book path".to_string())?;
        
        info!("Loading feats from book: {}", book_id);
        
        // Look for feat files in the feats directory
        let feats_dir = book_path.join("feats");
        
        if !feats_dir.exists() {
            debug!("No feats directory found for book {}", book_id);
            return Ok(());
        }
        
        // Read all JSON files in the feats directory
        match fs::read_dir(&feats_dir) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let file_path = entry.path();
                        if file_path.extension().and_then(|s| s.to_str()) == Some("json") {
                            let filename = file_path
                                .file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("unknown");
                            
                            debug!("Checking feat file: {:?}", file_path);
                            
                            match fs::read_to_string(&file_path) {
                                Ok(content) => {
                                    match serde_json::from_str::<FeatData>(&content) {
                                        Ok(feat_data) => {
                                            if let Some(feats) = feat_data.feat {
                                                if !feats.is_empty() {
                                                    info!("Loaded {} feats from {}/{}", 
                                                          feats.len(), book_id, filename);
                                                    self.feats.extend(feats);
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            warn!("Failed to parse feats from {}/{}: {}", 
                                                  book_id, filename, e);
                                        }
                                    }
                                }
                                Err(e) => {
                                    warn!("Failed to read {}/{}: {}", book_id, filename, e);
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                warn!("Failed to read feats directory for {}: {}", book_id, e);
            }
        }
        
        info!("Total feats loaded: {}", self.feats.len());
        Ok(())
    }
    
    /// Search feats with optional filters
    pub fn search(&self, query: Option<&str>, source: Option<&str>) -> Vec<FeatSummary> {
        let mut results: Vec<FeatSummary> = self.feats.iter()
            .filter(|feat| {
                // Filter by source if provided
                if let Some(src) = source {
                    if !feat.source.eq_ignore_ascii_case(src) {
                        return false;
                    }
                }
                
                // Filter by search query if provided
                if let Some(q) = query {
                    let q_lower = q.to_lowercase();
                    let name_match = feat.name.to_lowercase().contains(&q_lower);
                    
                    // Also search in prerequisites
                    let prereq_match = if let Some(prereqs) = &feat.prerequisite {
                        let prereq_str = serde_json::to_string(prereqs).unwrap_or_default();
                        prereq_str.to_lowercase().contains(&q_lower)
                    } else {
                        false
                    };
                    
                    // Search in entries
                    let entry_match = feat.entries.iter().any(|e| {
                        if let Some(text) = e.as_str() {
                            text.to_lowercase().contains(&q_lower)
                        } else {
                            false
                        }
                    });
                    
                    if !name_match && !prereq_match && !entry_match {
                        return false;
                    }
                }
                
                true
            })
            .map(|feat| FeatSummary::from(feat))
            .collect();
        
        // Sort by name
        results.sort_by(|a, b| a.name.cmp(&b.name));
        
        debug!("Found {} feats matching criteria", results.len());
        results
    }
    
    /// Get detailed feat information
    pub fn get_feat_details(&self, name: &str, source: &str) -> Option<Feat> {
        self.feats.iter()
            .find(|f| f.name.eq_ignore_ascii_case(name) && f.source.eq_ignore_ascii_case(source))
            .cloned()
    }
    
    /// Get unique sources for feats
    pub fn get_sources(&self) -> Vec<String> {
        let mut sources: Vec<String> = self.feats.iter()
            .map(|f| f.source.clone())
            .collect();
        sources.sort();
        sources.dedup();
        sources
    }
    
    /// Load feats from all books in the books directory
    pub fn load_from_books_directory(&mut self, books_dir: &Path) -> Result<(), String> {
        self.feats.clear();
        
        if !books_dir.exists() {
            warn!("Books directory does not exist: {:?}", books_dir);
            return Ok(());
        }
        
        // Read all book directories
        match fs::read_dir(books_dir) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let book_path = entry.path();
                        if book_path.is_dir() {
                            if let Err(e) = self.load_from_book(&book_path) {
                                warn!("Failed to load feats from book {:?}: {}", book_path, e);
                            }
                        }
                    }
                }
            }
            Err(e) => {
                return Err(format!("Failed to read books directory: {}", e));
            }
        }
        
        Ok(())
    }
}

/// Initialize the feat catalog by loading data from the books directory
#[tauri::command]
pub async fn initialize_feat_catalog(
    catalog: State<'_, std::sync::Mutex<FeatCatalog>>
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

/// Search feats
#[tauri::command]
pub async fn search_feats(
    catalog: State<'_, std::sync::Mutex<FeatCatalog>>,
    query: Option<String>,
    source: Option<String>,
) -> Result<Vec<FeatSummary>, String> {
    let catalog = catalog.lock().map_err(|e| e.to_string())?;
    
    Ok(catalog.search(query.as_deref(), source.as_deref()))
}

/// Get detailed feat information
#[tauri::command]
pub async fn get_feat_details(
    catalog: State<'_, std::sync::Mutex<FeatCatalog>>,
    name: String,
    source: String,
) -> Result<Option<Feat>, String> {
    let catalog = catalog.lock().map_err(|e| e.to_string())?;
    
    Ok(catalog.get_feat_details(&name, &source))
}

/// Get all unique feat sources
#[tauri::command]
pub async fn get_feat_sources(
    catalog: State<'_, std::sync::Mutex<FeatCatalog>>,
) -> Result<Vec<String>, String> {
    let catalog = catalog.lock().map_err(|e| e.to_string())?;
    
    Ok(catalog.get_sources())
}