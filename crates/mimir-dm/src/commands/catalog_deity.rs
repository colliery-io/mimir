use std::fs;
use std::path::Path;
use tauri::State;
use tracing::{debug, error, info, warn};
use mimir_dm_core::models::catalog::{Deity, DeityData, DeitySummary};

/// In-memory catalog of deities
pub struct DeityCatalog {
    pub deities: Vec<Deity>,
}

impl DeityCatalog {
    pub fn new() -> Self {
        Self {
            deities: Vec::new(),
        }
    }
    
    /// Load deity data from the books directory
    pub fn load_from_books_directory(&mut self, books_dir: &Path) -> Result<(), String> {
        info!("Loading deities from {:?}", books_dir);
        
        self.deities.clear();
        
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
            
            // Check for deities directory
            let deities_dir = book_path.join("deities");
            if deities_dir.exists() && deities_dir.is_dir() {
                debug!("Found deities directory for book: {}", book_id);
                
                // Load all JSON files in the deities directory
                for deity_entry in fs::read_dir(&deities_dir).map_err(|e| e.to_string())? {
                    let deity_entry = deity_entry.map_err(|e| e.to_string())?;
                    let deity_file = deity_entry.path();
                    
                    if deity_file.extension().and_then(|e| e.to_str()) != Some("json") {
                        continue;
                    }
                    
                    let filename = deity_file.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown");
                    
                    match fs::read_to_string(&deity_file) {
                        Ok(content) => {
                            match serde_json::from_str::<DeityData>(&content) {
                                Ok(data) => {
                                    if let Some(deities) = data.deity {
                                        if !deities.is_empty() {
                                            info!("Loaded {} deities from {}/{}", 
                                                  deities.len(), book_id, filename);
                                            self.deities.extend(deities);
                                        }
                                    }
                                }
                                Err(e) => {
                                    error!("Failed to parse deities from {}/{}: {}", book_id, filename, e);
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
        
        info!("Total deities loaded: {}", self.deities.len());
        Ok(())
    }
    
    /// Search deities with filters
    pub fn search(&self, 
        query: Option<String>,
        sources: Vec<String>,
        pantheons: Vec<String>,
        domains: Vec<String>
    ) -> Vec<DeitySummary> {
        info!("Searching deities - query: {:?}, sources: {:?}, pantheons: {:?}, domains: {:?}", 
                 query, sources, pantheons, domains);
        
        let results: Vec<DeitySummary> = self.deities.iter()
            .filter(|deity| {
                // Filter by query
                if let Some(q) = &query {
                    if !q.is_empty() {
                        let name_match = deity.name.to_lowercase().contains(&q.to_lowercase());
                        let title_match = deity.title.as_ref()
                            .map(|t| t.to_lowercase().contains(&q.to_lowercase()))
                            .unwrap_or(false);
                        let symbol_match = deity.symbol.as_ref()
                            .map(|s| s.to_lowercase().contains(&q.to_lowercase()))
                            .unwrap_or(false);
                        if !name_match && !title_match && !symbol_match {
                            return false;
                        }
                    }
                }
                
                // Filter by sources
                if !sources.is_empty() && !sources.contains(&deity.source) {
                    return false;
                }
                
                // Filter by pantheons
                if !pantheons.is_empty() {
                    if let Some(pantheon) = &deity.pantheon {
                        if !pantheons.contains(pantheon) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                
                // Filter by domains
                if !domains.is_empty() {
                    if let Some(deity_domains) = &deity.domains {
                        let has_matching_domain = deity_domains.iter()
                            .any(|d| domains.contains(d));
                        if !has_matching_domain {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                
                true
            })
            .map(DeitySummary::from)
            .collect();
        
        info!("Found {} deities matching filters", results.len());
        results
    }
    
    /// Get detailed deity information
    pub fn get_deity_details(&self, name: &str, source: &str) -> Option<Deity> {
        self.deities.iter()
            .find(|d| d.name == name && d.source == source)
            .cloned()
    }
    
    /// Get all unique pantheons in the catalog
    pub fn get_pantheons(&self) -> Vec<String> {
        let mut pantheons = std::collections::HashSet::new();
        for deity in &self.deities {
            if let Some(pantheon) = &deity.pantheon {
                pantheons.insert(pantheon.clone());
            }
        }
        let mut sorted: Vec<String> = pantheons.into_iter().collect();
        sorted.sort();
        sorted
    }
    
    /// Get all unique domains in the catalog
    pub fn get_domains(&self) -> Vec<String> {
        let mut domains = std::collections::HashSet::new();
        for deity in &self.deities {
            if let Some(deity_domains) = &deity.domains {
                for domain in deity_domains {
                    domains.insert(domain.clone());
                }
            }
        }
        let mut sorted: Vec<String> = domains.into_iter().collect();
        sorted.sort();
        sorted
    }
}

/// Initialize the deity catalog
#[tauri::command]
pub async fn init_deity_catalog(
    catalog: State<'_, std::sync::Mutex<DeityCatalog>>
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

/// Search deities with filters
#[tauri::command]
pub async fn search_deities(
    query: Option<String>,
    sources: Option<Vec<String>>,
    pantheons: Option<Vec<String>>,
    domains: Option<Vec<String>>,
    catalog: State<'_, std::sync::Mutex<DeityCatalog>>
) -> Result<Vec<DeitySummary>, String> {
    let catalog = catalog.lock().map_err(|e| e.to_string())?;
    
    Ok(catalog.search(
        query,
        sources.unwrap_or_default(),
        pantheons.unwrap_or_default(),
        domains.unwrap_or_default()
    ))
}

/// Get detailed information about a specific deity
#[tauri::command]
pub async fn get_deity_details(
    name: String,
    source: String,
    catalog: State<'_, std::sync::Mutex<DeityCatalog>>
) -> Result<Deity, String> {
    let catalog = catalog.lock().map_err(|e| e.to_string())?;
    
    catalog.get_deity_details(&name, &source)
        .ok_or_else(|| format!("Deity not found: {} from {}", name, source))
}

/// Get all available pantheons
#[tauri::command]
pub async fn get_pantheons(
    catalog: State<'_, std::sync::Mutex<DeityCatalog>>
) -> Result<Vec<String>, String> {
    let catalog = catalog.lock().map_err(|e| e.to_string())?;
    Ok(catalog.get_pantheons())
}

/// Get all available domains
#[tauri::command]
pub async fn get_domains(
    catalog: State<'_, std::sync::Mutex<DeityCatalog>>
) -> Result<Vec<String>, String> {
    let catalog = catalog.lock().map_err(|e| e.to_string())?;
    Ok(catalog.get_domains())
}