use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tauri::State;
use tracing::{debug, error, info, warn};
use mimir_dm_core::models::catalog::{
    Background, BackgroundData, BackgroundSummary, BackgroundFluff, 
    BackgroundFluffData, BackgroundWithDetails
};

/// In-memory catalog of backgrounds
pub struct BackgroundCatalog {
    pub backgrounds: Vec<Background>,
    pub background_fluff: Vec<BackgroundFluff>,
}

impl BackgroundCatalog {
    pub fn new() -> Self {
        Self {
            backgrounds: Vec::new(),
            background_fluff: Vec::new(),
        }
    }
    
    /// Load background data from the books directory
    pub fn load_from_books_directory(&mut self, books_dir: &Path) -> Result<(), String> {
        info!("Loading backgrounds from {:?}", books_dir);
        
        self.backgrounds.clear();
        self.background_fluff.clear();
        
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
            
            // Check for backgrounds directory
            let backgrounds_dir = book_path.join("backgrounds");
            if backgrounds_dir.exists() && backgrounds_dir.is_dir() {
                debug!("Found backgrounds directory for book: {}", book_id);
                
                // Load all JSON files in the backgrounds directory
                for bg_entry in fs::read_dir(&backgrounds_dir).map_err(|e| e.to_string())? {
                    let bg_entry = bg_entry.map_err(|e| e.to_string())?;
                    let bg_file = bg_entry.path();
                    
                    if bg_file.extension().and_then(|e| e.to_str()) != Some("json") {
                        continue;
                    }
                    
                    let filename = bg_file.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown");
                    
                    // Check if it's a fluff file
                    if filename.starts_with("fluff-") {
                        match fs::read_to_string(&bg_file) {
                            Ok(content) => {
                                match serde_json::from_str::<BackgroundFluffData>(&content) {
                                    Ok(fluff_data) => {
                                        if let Some(fluff) = fluff_data.background_fluff {
                                            info!("Loaded {} background fluff entries from {}/{}", 
                                                  fluff.len(), book_id, filename);
                                            self.background_fluff.extend(fluff);
                                        }
                                    }
                                    Err(e) => {
                                        warn!("Failed to parse background fluff {}/{}: {}", book_id, filename, e);
                                    }
                                }
                            }
                            Err(e) => {
                                error!("Failed to read {}/{}: {}", book_id, filename, e);
                            }
                        }
                        continue;
                    }
                    
                    // Handle background data files
                    match fs::read_to_string(&bg_file) {
                        Ok(content) => {
                            match serde_json::from_str::<BackgroundData>(&content) {
                                Ok(bg_data) => {
                                    if let Some(backgrounds) = bg_data.background {
                                        if !backgrounds.is_empty() {
                                            debug!("Loaded {} backgrounds from {}/{}", 
                                                    backgrounds.len(), book_id, filename);
                                            self.backgrounds.extend(backgrounds);
                                        }
                                    }
                                }
                                Err(e) => {
                                    warn!("Failed to parse {}/{}: {}", book_id, filename, e);
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
        
        info!("Total backgrounds loaded: {}", self.backgrounds.len());
        info!("Total background fluff loaded: {}", self.background_fluff.len());
        Ok(())
    }
    
    /// Search backgrounds with filters
    pub fn search(&self, 
        query: Option<String>,
        sources: Vec<String>,
        has_tools: Option<bool>
    ) -> Vec<BackgroundSummary> {
        info!("Searching backgrounds - query: {:?}, sources: {:?}, total backgrounds: {}", 
                 query, sources, self.backgrounds.len());
        
        let results: Vec<BackgroundSummary> = self.backgrounds.iter()
            .filter(|bg| {
                // Filter by query (search in name or feature)
                if let Some(q) = &query {
                    if !q.is_empty() {
                        let name_match = bg.name.to_lowercase().contains(&q.to_lowercase());
                        // Also search in feature names
                        let feature_match = bg.entries.iter().any(|e| {
                            if let Some(obj) = e.as_object() {
                                if let Some(name) = obj.get("name").and_then(|n| n.as_str()) {
                                    return name.to_lowercase().contains(&q.to_lowercase());
                                }
                            }
                            false
                        });
                        if !name_match && !feature_match {
                            return false;
                        }
                    }
                }
                
                // Filter by sources
                if !sources.is_empty() && !sources.contains(&bg.source) {
                    return false;
                }
                
                // Filter by has_tools
                if let Some(wants_tools) = has_tools {
                    let has_tool_profs = !bg.tool_proficiencies.is_empty();
                    if has_tool_profs != wants_tools {
                        return false;
                    }
                }
                
                true
            })
            .map(BackgroundSummary::from)
            .collect();
        
        info!("Found {} backgrounds matching filters", results.len());
        results
    }
    
    /// Get detailed background information
    pub fn get_background_details(&self, name: &str, source: &str) -> Option<BackgroundWithDetails> {
        info!("Getting background details for {} from {}", name, source);
        
        // Find the background
        let background = self.backgrounds.iter()
            .find(|bg| bg.name == name && bg.source == source)?
            .clone();
        
        // Find associated fluff
        let fluff = self.background_fluff.iter()
            .find(|f| f.name == name && f.source == source)
            .cloned();
        
        Some(BackgroundWithDetails {
            background,
            fluff,
        })
    }
}

/// Initialize the background catalog
#[tauri::command]
pub async fn init_background_catalog(
    catalog: State<'_, std::sync::Mutex<BackgroundCatalog>>
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

/// Search backgrounds with filters
#[tauri::command]
pub async fn search_backgrounds(
    query: Option<String>,
    sources: Option<Vec<String>>,
    has_tools: Option<bool>,
    catalog: State<'_, std::sync::Mutex<BackgroundCatalog>>
) -> Result<Vec<BackgroundSummary>, String> {
    let catalog = catalog.lock().map_err(|e| e.to_string())?;
    
    Ok(catalog.search(
        query,
        sources.unwrap_or_default(),
        has_tools
    ))
}

/// Get detailed information about a specific background
#[tauri::command]
pub async fn get_background_details(
    name: String,
    source: String,
    catalog: State<'_, std::sync::Mutex<BackgroundCatalog>>
) -> Result<BackgroundWithDetails, String> {
    let catalog = catalog.lock().map_err(|e| e.to_string())?;
    
    catalog.get_background_details(&name, &source)
        .ok_or_else(|| format!("Background not found: {} from {}", name, source))
}