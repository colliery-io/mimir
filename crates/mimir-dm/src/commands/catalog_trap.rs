use std::fs;
use std::path::Path;
use tauri::State;
use tracing::{debug, error, info, warn};
use mimir_dm_core::models::catalog::{TrapData, HazardData, TrapOrHazard, TrapSummary};

/// In-memory catalog of traps and hazards
pub struct TrapCatalog {
    pub items: Vec<TrapOrHazard>,
}

impl TrapCatalog {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
        }
    }
    
    /// Load trap and hazard data from the books directory
    pub fn load_from_books_directory(&mut self, books_dir: &Path) -> Result<(), String> {
        info!("Loading traps and hazards from {:?}", books_dir);
        
        self.items.clear();
        
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
            
            // Check for traps directory
            let traps_dir = book_path.join("traps");
            if traps_dir.exists() && traps_dir.is_dir() {
                debug!("Found traps directory for book: {}", book_id);
                
                // Load all JSON files in the traps directory
                for trap_entry in fs::read_dir(&traps_dir).map_err(|e| e.to_string())? {
                    let trap_entry = trap_entry.map_err(|e| e.to_string())?;
                    let trap_file = trap_entry.path();
                    
                    if trap_file.extension().and_then(|e| e.to_str()) != Some("json") {
                        continue;
                    }
                    
                    let filename = trap_file.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown");
                    
                    match fs::read_to_string(&trap_file) {
                        Ok(content) => {
                            match serde_json::from_str::<TrapData>(&content) {
                                Ok(data) => {
                                    if let Some(traps) = data.trap {
                                        if !traps.is_empty() {
                                            info!("Loaded {} traps from {}/{}", 
                                                  traps.len(), book_id, filename);
                                            self.items.extend(
                                                traps.into_iter().map(TrapOrHazard::Trap)
                                            );
                                        }
                                    }
                                }
                                Err(e) => {
                                    error!("Failed to parse traps from {}/{}: {}", book_id, filename, e);
                                }
                            }
                        }
                        Err(e) => {
                            error!("Failed to read {}/{}: {}", book_id, filename, e);
                        }
                    }
                }
            }
            
            // Check for hazards directory
            let hazards_dir = book_path.join("hazards");
            if hazards_dir.exists() && hazards_dir.is_dir() {
                debug!("Found hazards directory for book: {}", book_id);
                
                // Load all JSON files in the hazards directory
                for hazard_entry in fs::read_dir(&hazards_dir).map_err(|e| e.to_string())? {
                    let hazard_entry = hazard_entry.map_err(|e| e.to_string())?;
                    let hazard_file = hazard_entry.path();
                    
                    if hazard_file.extension().and_then(|e| e.to_str()) != Some("json") {
                        continue;
                    }
                    
                    let filename = hazard_file.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown");
                    
                    match fs::read_to_string(&hazard_file) {
                        Ok(content) => {
                            match serde_json::from_str::<HazardData>(&content) {
                                Ok(data) => {
                                    if let Some(hazards) = data.hazard {
                                        if !hazards.is_empty() {
                                            info!("Loaded {} hazards from {}/{}", 
                                                  hazards.len(), book_id, filename);
                                            self.items.extend(
                                                hazards.into_iter().map(TrapOrHazard::Hazard)
                                            );
                                        }
                                    }
                                }
                                Err(e) => {
                                    error!("Failed to parse hazards from {}/{}: {}", book_id, filename, e);
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
        
        info!("Total traps and hazards loaded: {}", self.items.len());
        Ok(())
    }
    
    /// Search traps and hazards with filters
    pub fn search(&self, 
        query: Option<String>,
        sources: Vec<String>,
        categories: Vec<String>, // "trap" or "hazard"
        trap_types: Vec<String>  // MECH, MAG, WLD, WTH, ENV
    ) -> Vec<TrapSummary> {
        info!("Searching traps/hazards - query: {:?}, sources: {:?}, categories: {:?}, types: {:?}", 
                 query, sources, categories, trap_types);
        
        let results: Vec<TrapSummary> = self.items.iter()
            .filter(|item| {
                // Filter by query
                if let Some(q) = &query {
                    if !q.is_empty() {
                        let name_match = item.name().to_lowercase().contains(&q.to_lowercase());
                        if !name_match {
                            return false;
                        }
                    }
                }
                
                // Filter by sources
                if !sources.is_empty() && !sources.contains(&item.source().to_string()) {
                    return false;
                }
                
                // Filter by category (trap vs hazard)
                if !categories.is_empty() {
                    let is_trap = item.is_trap();
                    let category_match = (is_trap && categories.contains(&"trap".to_string())) ||
                                        (!is_trap && categories.contains(&"hazard".to_string()));
                    if !category_match {
                        return false;
                    }
                }
                
                // Filter by trap types
                if !trap_types.is_empty() {
                    if let Some(trap_type) = item.trap_haz_type() {
                        if !trap_types.contains(trap_type) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                
                true
            })
            .map(TrapSummary::from)
            .collect();
        
        info!("Found {} items matching filters", results.len());
        results
    }
    
    /// Get detailed trap or hazard information
    pub fn get_trap_details(&self, name: &str, source: &str) -> Option<TrapOrHazard> {
        self.items.iter()
            .find(|t| t.name() == name && t.source() == source)
            .cloned()
    }
    
    /// Get all unique trap types in the catalog
    pub fn get_trap_types(&self) -> Vec<String> {
        let mut types = std::collections::HashSet::new();
        for item in &self.items {
            if let Some(trap_type) = item.trap_haz_type() {
                types.insert(trap_type.clone());
            }
        }
        let mut sorted: Vec<String> = types.into_iter().collect();
        sorted.sort();
        sorted
    }
}

/// Initialize the trap catalog
#[tauri::command]
pub async fn init_trap_catalog(
    catalog: State<'_, std::sync::Mutex<TrapCatalog>>
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

/// Search traps and hazards with filters
#[tauri::command]
pub async fn search_traps(
    query: Option<String>,
    sources: Option<Vec<String>>,
    categories: Option<Vec<String>>,
    trap_types: Option<Vec<String>>,
    catalog: State<'_, std::sync::Mutex<TrapCatalog>>
) -> Result<Vec<TrapSummary>, String> {
    let catalog = catalog.lock().map_err(|e| e.to_string())?;
    
    Ok(catalog.search(
        query,
        sources.unwrap_or_default(),
        categories.unwrap_or_default(),
        trap_types.unwrap_or_default()
    ))
}

/// Get detailed information about a specific trap or hazard
#[tauri::command]
pub async fn get_trap_details(
    name: String,
    source: String,
    catalog: State<'_, std::sync::Mutex<TrapCatalog>>
) -> Result<TrapOrHazard, String> {
    let catalog = catalog.lock().map_err(|e| e.to_string())?;
    
    catalog.get_trap_details(&name, &source)
        .ok_or_else(|| format!("Trap/hazard not found: {} from {}", name, source))
}

/// Get all available trap types
#[tauri::command]
pub async fn get_trap_types(
    catalog: State<'_, std::sync::Mutex<TrapCatalog>>
) -> Result<Vec<String>, String> {
    let catalog = catalog.lock().map_err(|e| e.to_string())?;
    Ok(catalog.get_trap_types())
}