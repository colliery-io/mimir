use std::fs;
use std::path::Path;
use tauri::State;
use tracing::{debug, error, info, warn};
use mimir_dm_core::models::catalog::{Language, LanguageData, LanguageSummary};

/// In-memory catalog of languages
pub struct LanguageCatalog {
    pub items: Vec<Language>,
}

impl LanguageCatalog {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
        }
    }
    
    /// Load language data from the books directory
    pub fn load_from_books_directory(&mut self, books_dir: &Path) -> Result<(), String> {
        info!("Loading languages from {:?}", books_dir);
        
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
            
            // Check for languages directory
            let languages_dir = book_path.join("languages");
            if languages_dir.exists() && languages_dir.is_dir() {
                debug!("Found languages directory for book: {}", book_id);
                
                // Load all JSON files in the languages directory
                for lang_entry in fs::read_dir(&languages_dir).map_err(|e| e.to_string())? {
                    let lang_entry = lang_entry.map_err(|e| e.to_string())?;
                    let lang_file = lang_entry.path();
                    
                    // Skip fluff files
                    if lang_file.file_name()
                        .and_then(|n| n.to_str())
                        .map(|n| n.starts_with("fluff"))
                        .unwrap_or(false) {
                        continue;
                    }
                    
                    if lang_file.extension().and_then(|e| e.to_str()) != Some("json") {
                        continue;
                    }
                    
                    let filename = lang_file.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown");
                    
                    match fs::read_to_string(&lang_file) {
                        Ok(content) => {
                            match serde_json::from_str::<LanguageData>(&content) {
                                Ok(data) => {
                                    if let Some(languages) = data.language {
                                        if !languages.is_empty() {
                                            info!("Loaded {} languages from {}/{}", 
                                                  languages.len(), book_id, filename);
                                            self.items.extend(languages);
                                        }
                                    }
                                }
                                Err(e) => {
                                    error!("Failed to parse languages from {}/{}: {}", book_id, filename, e);
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
        
        info!("Total languages loaded: {}", self.items.len());
        Ok(())
    }
    
    /// Search languages with filters
    pub fn search(&self, 
        query: Option<String>,
        sources: Vec<String>,
        types: Vec<String>, // standard, exotic, secret, dead
        scripts: Vec<String>
    ) -> Vec<LanguageSummary> {
        info!("Searching languages - query: {:?}, sources: {:?}, types: {:?}, scripts: {:?}", 
                 query, sources, types, scripts);
        
        let results: Vec<LanguageSummary> = self.items.iter()
            .filter(|lang| {
                // Filter by query
                if let Some(q) = &query {
                    if !q.is_empty() {
                        let name_match = lang.name.to_lowercase().contains(&q.to_lowercase());
                        let script_match = lang.script.as_ref()
                            .map(|s| s.to_lowercase().contains(&q.to_lowercase()))
                            .unwrap_or(false);
                        if !name_match && !script_match {
                            return false;
                        }
                    }
                }
                
                // Filter by sources
                if !sources.is_empty() && !sources.contains(&lang.source) {
                    return false;
                }
                
                // Filter by language types
                if !types.is_empty() {
                    let lang_type = lang.language_type.as_deref().unwrap_or("standard");
                    if !types.contains(&lang_type.to_string()) {
                        return false;
                    }
                }
                
                // Filter by scripts
                if !scripts.is_empty() {
                    if let Some(script) = &lang.script {
                        if !scripts.contains(script) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                
                true
            })
            .map(LanguageSummary::from)
            .collect();
        
        info!("Found {} languages matching filters", results.len());
        results
    }
    
    /// Get detailed language information
    pub fn get_language_details(&self, name: &str, source: &str) -> Option<Language> {
        self.items.iter()
            .find(|l| l.name == name && l.source == source)
            .cloned()
    }
    
    /// Get all unique language types in the catalog
    pub fn get_language_types(&self) -> Vec<String> {
        let mut types = std::collections::HashSet::new();
        for lang in &self.items {
            let lang_type = lang.language_type.as_deref().unwrap_or("standard");
            types.insert(lang_type.to_string());
        }
        let mut sorted: Vec<String> = types.into_iter().collect();
        sorted.sort();
        sorted
    }
    
    /// Get all unique scripts in the catalog
    pub fn get_scripts(&self) -> Vec<String> {
        let mut scripts = std::collections::HashSet::new();
        for lang in &self.items {
            if let Some(script) = &lang.script {
                scripts.insert(script.clone());
            }
        }
        let mut sorted: Vec<String> = scripts.into_iter().collect();
        sorted.sort();
        sorted
    }
}

/// Initialize the language catalog
#[tauri::command]
pub async fn init_language_catalog(
    catalog: State<'_, std::sync::Mutex<LanguageCatalog>>
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

/// Search languages with filters
#[tauri::command]
pub async fn search_languages(
    query: Option<String>,
    sources: Option<Vec<String>>,
    types: Option<Vec<String>>,
    scripts: Option<Vec<String>>,
    catalog: State<'_, std::sync::Mutex<LanguageCatalog>>
) -> Result<Vec<LanguageSummary>, String> {
    let catalog = catalog.lock().map_err(|e| e.to_string())?;
    
    Ok(catalog.search(
        query,
        sources.unwrap_or_default(),
        types.unwrap_or_default(),
        scripts.unwrap_or_default()
    ))
}

/// Get detailed information about a specific language
#[tauri::command]
pub async fn get_language_details(
    name: String,
    source: String,
    catalog: State<'_, std::sync::Mutex<LanguageCatalog>>
) -> Result<Language, String> {
    let catalog = catalog.lock().map_err(|e| e.to_string())?;
    
    catalog.get_language_details(&name, &source)
        .ok_or_else(|| format!("Language not found: {} from {}", name, source))
}

/// Get all available language types
#[tauri::command]
pub async fn get_language_types(
    catalog: State<'_, std::sync::Mutex<LanguageCatalog>>
) -> Result<Vec<String>, String> {
    let catalog = catalog.lock().map_err(|e| e.to_string())?;
    Ok(catalog.get_language_types())
}

/// Get all available scripts
#[tauri::command]
pub async fn get_language_scripts(
    catalog: State<'_, std::sync::Mutex<LanguageCatalog>>
) -> Result<Vec<String>, String> {
    let catalog = catalog.lock().map_err(|e| e.to_string())?;
    Ok(catalog.get_scripts())
}