//! Extended catalog commands for Classes, Races, Feats, and Backgrounds

use std::path::PathBuf;
use std::fs;
use tauri::State;
use serde_json;
use tracing::{debug, info, warn, error};
use mimir_dm_core::models::rules_extended::{
    CharacterClass, ClassData, ClassSummary,
    Race, RaceData, RaceSummary,
    Feat, FeatData, FeatSummary,
    Background, BackgroundData, BackgroundSummary,
};

/// Class catalog state - holds all loaded classes in memory
pub struct ClassCatalog {
    classes: Vec<CharacterClass>,
}

impl ClassCatalog {
    pub fn new() -> Self {
        Self {
            classes: Vec::new(),
        }
    }
    
    /// Load classes from book directories
    pub fn load_from_books_directory(&mut self, books_dir: &PathBuf) -> Result<(), String> {
        if !books_dir.exists() {
            info!("Books directory not found: {:?}", books_dir);
            return Ok(());
        }
        
        self.classes.clear();
        
        for entry in fs::read_dir(books_dir).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let book_path = entry.path();
            
            if !book_path.is_dir() {
                continue;
            }
            
            let book_id = entry.file_name().to_string_lossy().to_string();
            let class_dir = book_path.join("class");
            
            if class_dir.exists() {
                for class_entry in fs::read_dir(&class_dir).map_err(|e| e.to_string())? {
                    let class_entry = class_entry.map_err(|e| e.to_string())?;
                    let class_file = class_entry.path();
                    
                    if class_file.extension().and_then(|s| s.to_str()) != Some("json") {
                        continue;
                    }
                    
                    let filename = class_file.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    if filename.starts_with("fluff-") || filename == "index.json" || filename == "foundry.json" {
                        continue;
                    }
                    
                    match fs::read_to_string(&class_file) {
                        Ok(content) => {
                            match serde_json::from_str::<ClassData>(&content) {
                                Ok(class_data) => {
                                    debug!("Loaded {} classes from {}/{}", 
                                            class_data.class.len(), book_id, filename);
                                    // Deduplicate by name + className combination
                                    for class in class_data.class {
                                        let is_duplicate = self.classes.iter().any(|existing| {
                                            existing.name == class.name && 
                                            existing.class_name == class.class_name &&
                                            existing.source == class.source
                                        });
                                        if !is_duplicate {
                                            self.classes.push(class);
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
        
        info!("Total classes loaded: {}", self.classes.len());
        Ok(())
    }
    
    /// Get a specific class by name and source
    pub fn get_class(&self, name: &str, source: &str) -> Option<CharacterClass> {
        self.classes.iter()
            .find(|c| c.name == name && c.source == source)
            .cloned()
    }
    
    /// Search classes with filters
    pub fn search(&self, 
        query: Option<String>,
        sources: Vec<String>,
        spellcaster: Option<bool>
    ) -> Vec<ClassSummary> {
        debug!("Searching classes - query: {:?}, sources: {:?}", query, sources);
        
        let mut results: Vec<ClassSummary> = self.classes.iter()
            .filter(|class| {
                if let Some(q) = &query {
                    if !q.is_empty() && !class.name.to_lowercase().contains(&q.to_lowercase()) {
                        return false;
                    }
                }
                
                if !sources.is_empty() && !sources.contains(&class.source) {
                    return false;
                }
                
                if let Some(is_spellcaster) = spellcaster {
                    let has_casting = class.caster_progression.is_some();
                    if has_casting != is_spellcaster {
                        return false;
                    }
                }
                
                true
            })
            .map(ClassSummary::from)
            .collect();
        
        // Sort so that classes come before their subclasses, and both are alphabetical
        results.sort_by(|a, b| {
            // Extract parent class name if it's a subclass (format is "Parent: Subclass")
            let a_parts: Vec<&str> = a.name.split(": ").collect();
            let b_parts: Vec<&str> = b.name.split(": ").collect();
            
            let a_is_subclass = a_parts.len() > 1;
            let b_is_subclass = b_parts.len() > 1;
            
            if a_is_subclass && b_is_subclass {
                // Both are subclasses - sort by parent then by subclass name
                let parent_cmp = a_parts[0].cmp(b_parts[0]);
                if parent_cmp == std::cmp::Ordering::Equal {
                    a_parts[1].cmp(b_parts[1])
                } else {
                    parent_cmp
                }
            } else if a_is_subclass {
                // a is subclass, b is class
                // If a's parent is b, a comes after b
                if a_parts[0] == b.name.as_str() {
                    std::cmp::Ordering::Greater
                } else {
                    // Otherwise sort by parent name vs class name
                    a_parts[0].cmp(&b.name.as_str())
                }
            } else if b_is_subclass {
                // b is subclass, a is class
                // If b's parent is a, b comes after a
                if b_parts[0] == a.name.as_str() {
                    std::cmp::Ordering::Less
                } else {
                    // Otherwise sort by class name vs parent name
                    a.name.as_str().cmp(b_parts[0])
                }
            } else {
                // Both are classes - simple alphabetical
                a.name.cmp(&b.name)
            }
        });
            
        debug!("Found {} classes matching criteria", results.len());
        results
    }
}

/// Race catalog state
pub struct RaceCatalog {
    races: Vec<Race>,
}

impl RaceCatalog {
    pub fn new() -> Self {
        Self {
            races: Vec::new(),
        }
    }
    
    pub fn load_from_books_directory(&mut self, books_dir: &PathBuf) -> Result<(), String> {
        if !books_dir.exists() {
            info!("Books directory not found: {:?}", books_dir);
            return Ok(());
        }
        
        self.races.clear();
        
        for entry in fs::read_dir(books_dir).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let book_path = entry.path();
            
            if !book_path.is_dir() {
                continue;
            }
            
            let book_id = entry.file_name().to_string_lossy().to_string();
            let races_dir = book_path.join("races");
            
            if races_dir.exists() {
                for race_entry in fs::read_dir(&races_dir).map_err(|e| e.to_string())? {
                    let race_entry = race_entry.map_err(|e| e.to_string())?;
                    let race_file = race_entry.path();
                    
                    if race_file.extension().and_then(|s| s.to_str()) != Some("json") {
                        continue;
                    }
                    
                    let filename = race_file.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    if filename.starts_with("fluff-") || filename == "index.json" || filename == "foundry.json" {
                        continue;
                    }
                    
                    match fs::read_to_string(&race_file) {
                        Ok(content) => {
                            match serde_json::from_str::<RaceData>(&content) {
                                Ok(race_data) => {
                                    debug!("Loaded {} races from {}/{}", 
                                            race_data.race.len(), book_id, filename);
                                    self.races.extend(race_data.race);
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
        
        info!("Total races loaded: {}", self.races.len());
        Ok(())
    }
    
    pub fn search(&self, 
        query: Option<String>,
        sources: Vec<String>,
        sizes: Vec<String>
    ) -> Vec<RaceSummary> {
        debug!("Searching races - query: {:?}, sources: {:?}", query, sources);
        
        let results: Vec<RaceSummary> = self.races.iter()
            .filter(|race| {
                if let Some(q) = &query {
                    if !q.is_empty() && !race.name.to_lowercase().contains(&q.to_lowercase()) {
                        return false;
                    }
                }
                
                if !sources.is_empty() && !sources.contains(&race.source) {
                    return false;
                }
                
                if !sizes.is_empty() {
                    let race_size = race.size.as_ref()
                        .and_then(|s| s.first())
                        .map(|s| s.as_str())
                        .unwrap_or("");
                    if !sizes.iter().any(|s| s == race_size) {
                        return false;
                    }
                }
                
                true
            })
            .map(RaceSummary::from)
            .collect();
            
        debug!("Found {} races matching criteria", results.len());
        results
    }
}

/// Feat catalog state
pub struct FeatCatalog {
    feats: Vec<Feat>,
}

impl FeatCatalog {
    pub fn new() -> Self {
        Self {
            feats: Vec::new(),
        }
    }
    
    pub fn load_from_books_directory(&mut self, books_dir: &PathBuf) -> Result<(), String> {
        if !books_dir.exists() {
            info!("Books directory not found: {:?}", books_dir);
            return Ok(());
        }
        
        self.feats.clear();
        
        for entry in fs::read_dir(books_dir).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let book_path = entry.path();
            
            if !book_path.is_dir() {
                continue;
            }
            
            let book_id = entry.file_name().to_string_lossy().to_string();
            let feats_dir = book_path.join("feats");
            
            if feats_dir.exists() {
                for feat_entry in fs::read_dir(&feats_dir).map_err(|e| e.to_string())? {
                    let feat_entry = feat_entry.map_err(|e| e.to_string())?;
                    let feat_file = feat_entry.path();
                    
                    if feat_file.extension().and_then(|s| s.to_str()) != Some("json") {
                        continue;
                    }
                    
                    let filename = feat_file.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    if filename.starts_with("fluff-") || filename == "index.json" || filename == "foundry.json" {
                        continue;
                    }
                    
                    match fs::read_to_string(&feat_file) {
                        Ok(content) => {
                            match serde_json::from_str::<FeatData>(&content) {
                                Ok(feat_data) => {
                                    debug!("Loaded {} feats from {}/{}", 
                                            feat_data.feat.len(), book_id, filename);
                                    self.feats.extend(feat_data.feat);
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
        
        info!("Total feats loaded: {}", self.feats.len());
        Ok(())
    }
    
    pub fn search(&self, 
        query: Option<String>,
        sources: Vec<String>
    ) -> Vec<FeatSummary> {
        debug!("Searching feats - query: {:?}, sources: {:?}", query, sources);
        
        let results: Vec<FeatSummary> = self.feats.iter()
            .filter(|feat| {
                if let Some(q) = &query {
                    if !q.is_empty() && !feat.name.to_lowercase().contains(&q.to_lowercase()) {
                        return false;
                    }
                }
                
                if !sources.is_empty() && !sources.contains(&feat.source) {
                    return false;
                }
                
                true
            })
            .map(FeatSummary::from)
            .collect();
            
        debug!("Found {} feats matching criteria", results.len());
        results
    }
}

/// Background catalog state
pub struct BackgroundCatalog {
    backgrounds: Vec<Background>,
}

impl BackgroundCatalog {
    pub fn new() -> Self {
        Self {
            backgrounds: Vec::new(),
        }
    }
    
    pub fn load_from_books_directory(&mut self, books_dir: &PathBuf) -> Result<(), String> {
        if !books_dir.exists() {
            info!("Books directory not found: {:?}", books_dir);
            return Ok(());
        }
        
        self.backgrounds.clear();
        
        for entry in fs::read_dir(books_dir).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let book_path = entry.path();
            
            if !book_path.is_dir() {
                continue;
            }
            
            let book_id = entry.file_name().to_string_lossy().to_string();
            let backgrounds_dir = book_path.join("backgrounds");
            
            if backgrounds_dir.exists() {
                for bg_entry in fs::read_dir(&backgrounds_dir).map_err(|e| e.to_string())? {
                    let bg_entry = bg_entry.map_err(|e| e.to_string())?;
                    let bg_file = bg_entry.path();
                    
                    if bg_file.extension().and_then(|s| s.to_str()) != Some("json") {
                        continue;
                    }
                    
                    let filename = bg_file.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    if filename.starts_with("fluff-") || filename == "index.json" || filename == "foundry.json" {
                        continue;
                    }
                    
                    match fs::read_to_string(&bg_file) {
                        Ok(content) => {
                            match serde_json::from_str::<BackgroundData>(&content) {
                                Ok(bg_data) => {
                                    debug!("Loaded {} backgrounds from {}/{}", 
                                            bg_data.background.len(), book_id, filename);
                                    self.backgrounds.extend(bg_data.background);
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
        Ok(())
    }
    
    pub fn search(&self, 
        query: Option<String>,
        sources: Vec<String>
    ) -> Vec<BackgroundSummary> {
        debug!("Searching backgrounds - query: {:?}, sources: {:?}", query, sources);
        
        let results: Vec<BackgroundSummary> = self.backgrounds.iter()
            .filter(|bg| {
                if let Some(q) = &query {
                    if !q.is_empty() && !bg.name.to_lowercase().contains(&q.to_lowercase()) {
                        return false;
                    }
                }
                
                if !sources.is_empty() && !sources.contains(&bg.source) {
                    return false;
                }
                
                true
            })
            .map(BackgroundSummary::from)
            .collect();
            
        debug!("Found {} backgrounds matching criteria", results.len());
        results
    }
}

// Tauri commands for Classes
#[tauri::command]
pub async fn initialize_class_catalog(
    catalog: State<'_, std::sync::Mutex<ClassCatalog>>
) -> Result<(), String> {
    use crate::APP_PATHS;
    
    let app_paths = APP_PATHS.get()
        .ok_or_else(|| "App paths not initialized".to_string())?;
    
    let books_dir = app_paths.data_dir.join("books");
    
    let mut catalog = catalog.lock().map_err(|e| e.to_string())?;
    catalog.load_from_books_directory(&books_dir)?;
    
    Ok(())
}

#[tauri::command]
pub async fn search_classes(
    catalog: State<'_, std::sync::Mutex<ClassCatalog>>,
    query: Option<String>,
    sources: Option<Vec<String>>,
    spellcaster: Option<bool>,
) -> Result<Vec<ClassSummary>, String> {
    let catalog = catalog.lock().map_err(|e| e.to_string())?;
    
    let results = catalog.search(
        query,
        sources.unwrap_or_default(),
        spellcaster
    );
    
    Ok(results)
}

#[tauri::command]
pub async fn get_class_details(
    catalog: State<'_, std::sync::Mutex<ClassCatalog>>,
    name: String,
    source: String,
) -> Result<Option<CharacterClass>, String> {
    let catalog = catalog.lock().map_err(|e| e.to_string())?;
    Ok(catalog.get_class(&name, &source))
}

// Tauri commands for Races
#[tauri::command]
pub async fn initialize_race_catalog(
    catalog: State<'_, std::sync::Mutex<RaceCatalog>>
) -> Result<(), String> {
    use crate::APP_PATHS;
    
    let app_paths = APP_PATHS.get()
        .ok_or_else(|| "App paths not initialized".to_string())?;
    
    let books_dir = app_paths.data_dir.join("books");
    
    let mut catalog = catalog.lock().map_err(|e| e.to_string())?;
    catalog.load_from_books_directory(&books_dir)?;
    
    Ok(())
}

#[tauri::command]
pub async fn search_races(
    catalog: State<'_, std::sync::Mutex<RaceCatalog>>,
    query: Option<String>,
    sources: Option<Vec<String>>,
    sizes: Option<Vec<String>>,
) -> Result<Vec<RaceSummary>, String> {
    let catalog = catalog.lock().map_err(|e| e.to_string())?;
    
    let results = catalog.search(
        query,
        sources.unwrap_or_default(),
        sizes.unwrap_or_default()
    );
    
    Ok(results)
}

// Tauri commands for Feats
#[tauri::command]
pub async fn initialize_feat_catalog(
    catalog: State<'_, std::sync::Mutex<FeatCatalog>>
) -> Result<(), String> {
    use crate::APP_PATHS;
    
    let app_paths = APP_PATHS.get()
        .ok_or_else(|| "App paths not initialized".to_string())?;
    
    let books_dir = app_paths.data_dir.join("books");
    
    let mut catalog = catalog.lock().map_err(|e| e.to_string())?;
    catalog.load_from_books_directory(&books_dir)?;
    
    Ok(())
}

#[tauri::command]
pub async fn search_feats(
    catalog: State<'_, std::sync::Mutex<FeatCatalog>>,
    query: Option<String>,
    sources: Option<Vec<String>>,
) -> Result<Vec<FeatSummary>, String> {
    let catalog = catalog.lock().map_err(|e| e.to_string())?;
    
    let results = catalog.search(
        query,
        sources.unwrap_or_default()
    );
    
    Ok(results)
}

// Tauri commands for Backgrounds
#[tauri::command]
pub async fn initialize_background_catalog(
    catalog: State<'_, std::sync::Mutex<BackgroundCatalog>>
) -> Result<(), String> {
    use crate::APP_PATHS;
    
    let app_paths = APP_PATHS.get()
        .ok_or_else(|| "App paths not initialized".to_string())?;
    
    let books_dir = app_paths.data_dir.join("books");
    
    let mut catalog = catalog.lock().map_err(|e| e.to_string())?;
    catalog.load_from_books_directory(&books_dir)?;
    
    Ok(())
}

#[tauri::command]
pub async fn search_backgrounds(
    catalog: State<'_, std::sync::Mutex<BackgroundCatalog>>,
    query: Option<String>,
    sources: Option<Vec<String>>,
) -> Result<Vec<BackgroundSummary>, String> {
    let catalog = catalog.lock().map_err(|e| e.to_string())?;
    
    let results = catalog.search(
        query,
        sources.unwrap_or_default()
    );
    
    Ok(results)
}