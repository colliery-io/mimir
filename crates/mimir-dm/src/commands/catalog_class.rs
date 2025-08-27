//! Class catalog commands for D&D class reference content

use std::path::PathBuf;
use std::fs;
use tauri::State;
use serde_json;
use tracing::{debug, info, warn};
use mimir_dm_core::models::catalog::{
    Class, ClassData, ClassSummary,
    Subclass, ClassFeature, SubclassFeature,
    ClassFeatureData, ClassFluff, SubclassFluff, ClassFluffData
};
use serde::{Deserialize, Serialize};

/// Class catalog state - holds all loaded classes in memory
pub struct ClassCatalog {
    classes: Vec<Class>,
    subclasses: Vec<Subclass>,
    features: Vec<ClassFeature>,
    subclass_features: Vec<SubclassFeature>,
    class_fluff: Vec<ClassFluff>,
    subclass_fluff: Vec<SubclassFluff>,
}

impl ClassCatalog {
    pub fn new() -> Self {
        Self {
            classes: Vec::new(),
            subclasses: Vec::new(),
            features: Vec::new(),
            subclass_features: Vec::new(),
            class_fluff: Vec::new(),
            subclass_fluff: Vec::new(),
        }
    }
    
    /// Load classes from book directories (extracted archives)
    pub fn load_from_books_directory(&mut self, books_dir: &PathBuf) -> Result<(), String> {
        if !books_dir.exists() {
            info!("Books directory not found: {:?}", books_dir);
            return Ok(()); // Not an error, just no books yet
        }
        
        // Clear existing data
        self.classes.clear();
        self.subclasses.clear();
        self.features.clear();
        self.subclass_features.clear();
        self.class_fluff.clear();
        self.subclass_fluff.clear();
        
        // Iterate through each book directory
        for entry in fs::read_dir(books_dir).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let book_path = entry.path();
            
            if !book_path.is_dir() {
                continue;
            }
            
            let book_id = entry.file_name();
            let book_id_str = book_id.to_string_lossy();
            
            debug!("Checking book directory: {}", book_id_str);
            
            // Look for class files in multiple possible locations
            let search_dirs = [
                book_path.join("class"),
                book_path.join("classes"),
                book_path.join("data"),
                book_path.clone(),
            ];
            
            for search_dir in &search_dirs {
                if !search_dir.exists() {
                    continue;
                }
                
                debug!("Searching in directory: {:?}", search_dir);
                
                // Look for any JSON files that might contain class data
                if let Ok(entries) = fs::read_dir(search_dir) {
                    for file_entry in entries.flatten() {
                        let file_path = file_entry.path();
                        
                        // Skip if not a JSON file
                        if !file_path.extension().map_or(false, |ext| ext == "json") {
                            continue;
                        }
                        
                        let filename = file_entry.file_name();
                        let filename_str = filename.to_string_lossy();
                        
                        // Check if this might be a class file based on naming patterns
                        // In the class directory, the main file is usually just the book ID (e.g., "phb.json")
                        let is_main_class_file = search_dir.file_name()
                            .and_then(|n| n.to_str())
                            .map(|n| n == "class" || n == "classes")
                            .unwrap_or(false) && 
                            !filename_str.contains("fluff") &&
                            !filename_str.contains("feature");
                        
                        let is_class_named_file = filename_str.contains("class") && 
                                          !filename_str.contains("fluff") &&
                                          !filename_str.contains("feature") &&
                                          !filename_str.contains("subclass-feature");
                        
                        let is_main_book_file = filename_str == format!("{}.json", book_id_str.to_lowercase());
                        
                        if is_main_class_file || is_class_named_file || is_main_book_file {
                            info!("Found potential class file: {:?} (main_class={}, class_named={}, main_book={})", 
                                  file_path, is_main_class_file, is_class_named_file, is_main_book_file);
                            
                            match fs::read_to_string(&file_path) {
                                Ok(content) => {
                                    debug!("File content length: {} bytes", content.len());
                                    // First check if it's a ClassData structure
                                    match serde_json::from_str::<ClassData>(&content) {
                                        Ok(class_data) => {
                                            let mut loaded_something = false;
                                            
                                            if !class_data.classes.is_empty() {
                                                info!("Loaded {} classes from {}/{}", 
                                                      class_data.classes.len(), book_id_str, filename_str);
                                                self.classes.extend(class_data.classes);
                                                loaded_something = true;
                                            }
                                            
                                            // Add subclasses if present
                                            if let Some(subclasses) = class_data.subclass {
                                                if !subclasses.is_empty() {
                                                    info!("Loaded {} subclasses from {}/{}", 
                                                          subclasses.len(), book_id_str, filename_str);
                                                    self.subclasses.extend(subclasses);
                                                    loaded_something = true;
                                                }
                                            }
                                            
                                            // Add class features if present
                                            if let Some(features) = class_data.class_features {
                                                if !features.is_empty() {
                                                    info!("Loaded {} class features from {}/{}", 
                                                          features.len(), book_id_str, filename_str);
                                                    self.features.extend(features);
                                                    loaded_something = true;
                                                }
                                            }
                                            
                                            // Add subclass features if present
                                            if let Some(subclass_features) = class_data.subclass_features {
                                                if !subclass_features.is_empty() {
                                                    info!("Loaded {} subclass features from {}/{}", 
                                                          subclass_features.len(), book_id_str, filename_str);
                                                    self.subclass_features.extend(subclass_features);
                                                    loaded_something = true;
                                                }
                                            }
                                            
                                            if !loaded_something {
                                                debug!("No data loaded from {}/{} despite successful parse", book_id_str, filename_str);
                                            }
                                        }
                                        Err(e) => {
                                            debug!("Failed to parse as ClassData: {}", e);
                                            // If not ClassData, check if it's a direct array of classes
                                            match serde_json::from_str::<Vec<Class>>(&content) {
                                                Ok(classes) => {
                                                    if !classes.is_empty() {
                                                        info!("Loaded {} classes (direct array) from {}/{}", 
                                                              classes.len(), book_id_str, filename_str);
                                                        self.classes.extend(classes);
                                                    }
                                                }
                                                Err(e2) => {
                                                    warn!("Failed to parse {}/{} as class data: ClassData error: {}, Array error: {}", 
                                                          book_id_str, filename_str, e, e2);
                                                }
                                            }
                                        }
                                    }
                                }
                                Err(e) => {
                                    warn!("Failed to read {}/{}: {}", book_id_str, filename_str, e);
                                }
                            }
                        }
                        
                        // Check for feature files
                        // Files like "features-phb.json" or "subclass-features-phb.json"
                        let is_feature_file = filename_str.starts_with("features-") || 
                                             filename_str.starts_with("class-features-") ||
                                             (filename_str.contains("feature") && 
                                              !filename_str.contains("fluff") && 
                                              !filename_str.starts_with("subclass-features"));
                        
                        let is_subclass_feature_file = filename_str.starts_with("subclass-features-");
                        
                        if is_feature_file || is_subclass_feature_file {
                            debug!("Checking potential feature file: {:?}", file_path);
                            
                            match fs::read_to_string(&file_path) {
                                Ok(content) => {
                                    match serde_json::from_str::<ClassFeatureData>(&content) {
                                        Ok(feature_data) => {
                                            if let Some(features) = feature_data.class_feature {
                                                if !features.is_empty() {
                                                    info!("Loaded {} class features from {}/{}", 
                                                          features.len(), book_id_str, filename_str);
                                                    self.features.extend(features);
                                                }
                                            }
                                            
                                            if let Some(subclass_features) = feature_data.subclass_feature {
                                                if !subclass_features.is_empty() {
                                                    info!("Loaded {} subclass features from {}/{}", 
                                                          subclass_features.len(), book_id_str, filename_str);
                                                    self.subclass_features.extend(subclass_features);
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            warn!("Failed to parse features from {}/{}: {}", 
                                                  book_id_str, filename_str, e);
                                        }
                                    }
                                }
                                Err(e) => {
                                    warn!("Failed to read {}/{}: {}", book_id_str, filename_str, e);
                                }
                            }
                        }
                        
                        // Check for subclass-specific files
                        let is_subclass_file = filename_str.contains("subclass") && 
                                              !filename_str.contains("feature") &&
                                              !filename_str.contains("fluff");
                        
                        if is_subclass_file {
                            debug!("Checking potential subclass file: {:?}", file_path);
                            
                            match fs::read_to_string(&file_path) {
                                Ok(content) => {
                                    if let Ok(subclasses) = serde_json::from_str::<Vec<Subclass>>(&content) {
                                        if !subclasses.is_empty() {
                                            info!("Loaded {} subclasses from {}/{}", 
                                                  subclasses.len(), book_id_str, filename_str);
                                            self.subclasses.extend(subclasses);
                                        }
                                    }
                                }
                                Err(e) => {
                                    warn!("Failed to read {}/{}: {}", book_id_str, filename_str, e);
                                }
                            }
                        }
                        
                        // Check for fluff files
                        let is_fluff_file = filename_str.contains("fluff");
                        
                        if is_fluff_file {
                            debug!("Checking potential fluff file: {:?}", file_path);
                            
                            match fs::read_to_string(&file_path) {
                                Ok(content) => {
                                    // Try parsing as ClassFluffData
                                    if let Ok(fluff_data) = serde_json::from_str::<ClassFluffData>(&content) {
                                        if let Some(class_fluff) = fluff_data.class_fluff {
                                            if !class_fluff.is_empty() {
                                                info!("Loaded {} class fluff entries from {}/{}", 
                                                      class_fluff.len(), book_id_str, filename_str);
                                                self.class_fluff.extend(class_fluff);
                                            }
                                        }
                                        
                                        if let Some(subclass_fluff) = fluff_data.subclass_fluff {
                                            if !subclass_fluff.is_empty() {
                                                info!("Loaded {} subclass fluff entries from {}/{}", 
                                                      subclass_fluff.len(), book_id_str, filename_str);
                                                self.subclass_fluff.extend(subclass_fluff);
                                            }
                                        }
                                    }
                                }
                                Err(e) => {
                                    warn!("Failed to read fluff file {}/{}: {}", book_id_str, filename_str, e);
                                }
                            }
                        }
                    }
                }
            }
        }
        
        info!("Total classes loaded: {}", self.classes.len());
        info!("Total subclasses loaded: {}", self.subclasses.len());
        info!("Total features loaded: {}", self.features.len());
        info!("Total subclass features loaded: {}", self.subclass_features.len());
        info!("Total class fluff loaded: {}", self.class_fluff.len());
        info!("Total subclass fluff loaded: {}", self.subclass_fluff.len());
        
        Ok(())
    }
    
    /// Search classes with optional filters
    pub fn search(&self, query: Option<&str>, source: Option<&str>) -> Vec<ClassSummary> {
        let mut results: Vec<ClassSummary> = self.classes.iter()
            .filter(|class| {
                // Filter by source if provided
                if let Some(src) = source {
                    if !class.source.eq_ignore_ascii_case(src) {
                        return false;
                    }
                }
                
                // Filter by search query if provided
                if let Some(q) = query {
                    let q_lower = q.to_lowercase();
                    if !class.name.to_lowercase().contains(&q_lower) {
                        return false;
                    }
                }
                
                true
            })
            .map(|class| ClassSummary::from(class))
            .collect();
        
        // Sort by name
        results.sort_by(|a, b| a.name.cmp(&b.name));
        
        debug!("Found {} classes matching criteria", results.len());
        results
    }
    
    /// Get detailed class information
    pub fn get_class_details(&self, name: &str, source: &str) -> Option<ClassWithDetails> {
        let class = self.classes.iter()
            .find(|c| c.name.eq_ignore_ascii_case(name) && c.source.eq_ignore_ascii_case(source))?;
        
        let subclasses: Vec<Subclass> = self.subclasses.iter()
            .filter(|s| s.class_name.eq_ignore_ascii_case(name) && s.class_source.eq_ignore_ascii_case(source))
            .cloned()
            .collect();
        
        let features: Vec<ClassFeature> = self.features.iter()
            .filter(|f| f.class_name.eq_ignore_ascii_case(name) && f.class_source.eq_ignore_ascii_case(source))
            .cloned()
            .collect();
        
        let subclass_features: Vec<SubclassFeature> = self.subclass_features.iter()
            .filter(|f| f.class_name.eq_ignore_ascii_case(name) && f.class_source.eq_ignore_ascii_case(source))
            .cloned()
            .collect();
        
        debug!("Found {} subclass features for {} ({})", 
               subclass_features.len(), name, source);
        
        let fluff = self.class_fluff.iter()
            .find(|f| f.name.eq_ignore_ascii_case(name) && f.source.eq_ignore_ascii_case(source))
            .cloned();
        
        let subclass_fluff: Vec<SubclassFluff> = self.subclass_fluff.iter()
            .filter(|f| f.class_name.eq_ignore_ascii_case(name) && f.class_source.eq_ignore_ascii_case(source))
            .cloned()
            .collect();
        
        Some(ClassWithDetails {
            class: class.clone(),
            subclasses,
            features,
            subclass_features,
            fluff,
            subclass_fluff,
        })
    }
    
    /// Get unique sources for classes
    pub fn get_sources(&self) -> Vec<String> {
        let mut sources: Vec<String> = self.classes.iter()
            .map(|c| c.source.clone())
            .collect();
        sources.sort();
        sources.dedup();
        sources
    }
}

/// Class with all related details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassWithDetails {
    pub class: Class,
    pub subclasses: Vec<Subclass>,
    pub features: Vec<ClassFeature>,
    pub subclass_features: Vec<SubclassFeature>,
    pub fluff: Option<ClassFluff>,
    pub subclass_fluff: Vec<SubclassFluff>,
}

/// Initialize the class catalog by loading data from the books directory
#[tauri::command]
pub async fn initialize_class_catalog(
    catalog: State<'_, std::sync::Mutex<ClassCatalog>>
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

/// Search classes with optional filters
#[tauri::command]
pub async fn search_classes(
    catalog: State<'_, std::sync::Mutex<ClassCatalog>>,
    query: Option<String>,
    source: Option<String>,
) -> Result<Vec<ClassSummary>, String> {
    let catalog = catalog.lock().map_err(|e| e.to_string())?;
    
    Ok(catalog.search(
        query.as_deref(),
        source.as_deref(),
    ))
}

/// Get detailed class information
#[tauri::command]
pub async fn get_class_details(
    catalog: State<'_, std::sync::Mutex<ClassCatalog>>,
    name: String,
    source: String,
) -> Result<Option<ClassWithDetails>, String> {
    let catalog = catalog.lock().map_err(|e| e.to_string())?;
    
    Ok(catalog.get_class_details(&name, &source))
}

/// Get all subclasses for a class
#[tauri::command]
pub async fn get_class_subclasses(
    catalog: State<'_, std::sync::Mutex<ClassCatalog>>,
    class_name: String,
    class_source: String,
) -> Result<Vec<Subclass>, String> {
    let catalog = catalog.lock().map_err(|e| e.to_string())?;
    
    let subclasses: Vec<Subclass> = catalog.subclasses.iter()
        .filter(|s| s.class_name.eq_ignore_ascii_case(&class_name) && 
                    s.class_source.eq_ignore_ascii_case(&class_source))
        .cloned()
        .collect();
    
    Ok(subclasses)
}

/// Get all unique sources for classes
#[tauri::command]
pub async fn get_class_sources(
    catalog: State<'_, std::sync::Mutex<ClassCatalog>>
) -> Result<Vec<String>, String> {
    let catalog = catalog.lock().map_err(|e| e.to_string())?;
    Ok(catalog.get_sources())
}