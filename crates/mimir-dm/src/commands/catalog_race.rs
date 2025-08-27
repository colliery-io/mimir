//! Race catalog commands for D&D race reference content

use std::path::PathBuf;
use std::fs;
use tauri::State;
use serde_json;
use tracing::{debug, info, warn, error};
use mimir_dm_core::models::catalog::{
    Race, RaceData, RaceSummary, Subrace, RaceFluff, RaceFluffData
};

/// Race catalog state - holds all loaded races in memory
pub struct RaceCatalog {
    races: Vec<Race>,
    subraces: Vec<Subrace>,
    race_fluff: Vec<RaceFluff>,
}

impl RaceCatalog {
    pub fn new() -> Self {
        Self {
            races: Vec::new(),
            subraces: Vec::new(),
            race_fluff: Vec::new(),
        }
    }
    
    /// Load races from book directories (extracted archives)
    pub fn load_from_books_directory(&mut self, books_dir: &PathBuf) -> Result<(), String> {
        if !books_dir.exists() {
            info!("Books directory not found: {:?}", books_dir);
            return Ok(()); // Not an error, just no books yet
        }
        
        // Clear existing races
        self.races.clear();
        self.subraces.clear();
        self.race_fluff.clear();
        
        // Iterate through each book directory
        for entry in fs::read_dir(books_dir).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let book_path = entry.path();
            
            if !book_path.is_dir() {
                continue;
            }
            
            let book_id = entry.file_name().to_string_lossy().to_string();
            
            // Check the races directory
            let races_dir = book_path.join("races");
            if races_dir.exists() {
                for race_entry in fs::read_dir(&races_dir).map_err(|e| e.to_string())? {
                    let race_entry = race_entry.map_err(|e| e.to_string())?;
                    let race_file = race_entry.path();
                    
                    // Skip non-JSON files
                    if race_file.extension().and_then(|s| s.to_str()) != Some("json") {
                        continue;
                    }
                    
                    let filename = race_file.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    
                    // Handle fluff files
                    if filename.starts_with("fluff-") {
                        match fs::read_to_string(&race_file) {
                            Ok(content) => {
                                match serde_json::from_str::<RaceFluffData>(&content) {
                                    Ok(fluff_data) => {
                                        if let Some(fluff) = fluff_data.race_fluff {
                                            info!("Loaded {} race fluff entries from {}/{}", 
                                                  fluff.len(), book_id, filename);
                                            self.race_fluff.extend(fluff);
                                        }
                                    }
                                    Err(e) => {
                                        warn!("Failed to parse race fluff {}/{}: {}", book_id, filename, e);
                                    }
                                }
                            }
                            Err(e) => {
                                error!("Failed to read {}/{}: {}", book_id, filename, e);
                            }
                        }
                        continue;
                    }
                    
                    // Handle race data files
                    match fs::read_to_string(&race_file) {
                        Ok(content) => {
                            match serde_json::from_str::<RaceData>(&content) {
                                Ok(race_data) => {
                                    if let Some(races) = race_data.race {
                                        if !races.is_empty() {
                                            debug!("Loaded {} races from {}/{}", 
                                                    races.len(), book_id, filename);
                                            self.races.extend(races);
                                        }
                                    }
                                    
                                    if let Some(subraces) = race_data.subrace {
                                        if !subraces.is_empty() {
                                            debug!("Loaded {} subraces from {}/{}", 
                                                    subraces.len(), book_id, filename);
                                            self.subraces.extend(subraces);
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
        
        info!("Total races loaded: {}", self.races.len());
        info!("Total subraces loaded: {}", self.subraces.len());
        info!("Total race fluff loaded: {}", self.race_fluff.len());
        Ok(())
    }
    
    /// Search races with filters
    pub fn search(&self, 
        query: Option<String>,
        sources: Vec<String>,
        sizes: Vec<String>,
        has_darkvision: Option<bool>,
        has_flight: Option<bool>
    ) -> Vec<RaceSummary> {
        info!("Searching races - query: {:?}, sources: {:?}, total races: {}, total subraces: {}", 
                 query, sources, self.races.len(), self.subraces.len());
        
        let mut results: Vec<RaceSummary> = Vec::new();
        
        // Search main races
        let race_results: Vec<RaceSummary> = self.races.iter()
            .filter(|race| {
                // Filter by query (search in name)
                if let Some(q) = &query {
                    if !q.is_empty() && !race.name.to_lowercase().contains(&q.to_lowercase()) {
                        return false;
                    }
                }
                
                // Filter by sources
                if !sources.is_empty() && !sources.contains(&race.source) {
                    return false;
                }
                
                // Filter by size
                if !sizes.is_empty() {
                    if let Some(race_sizes) = &race.size {
                        let has_size = race_sizes.iter().any(|s| sizes.contains(s));
                        if !has_size {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                
                // Filter by darkvision
                if let Some(wants_darkvision) = has_darkvision {
                    let has_it = race.darkvision.is_some();
                    if has_it != wants_darkvision {
                        return false;
                    }
                }
                
                // Filter by flight
                if let Some(wants_flight) = has_flight {
                    let has_it = race.speed.as_ref()
                        .and_then(|s| s.as_object())
                        .and_then(|obj| obj.get("fly"))
                        .is_some();
                    if has_it != wants_flight {
                        return false;
                    }
                }
                
                true
            })
            .map(RaceSummary::from)
            .collect();
        
        results.extend(race_results);
        
        // Search subraces
        let subrace_results: Vec<RaceSummary> = self.subraces.iter()
            .filter(|subrace| {
                // Skip unnamed subraces - these are just metadata entries in 5etools
                if subrace.name.is_none() {
                    return false;
                }
                
                // Filter by query (search in name or parent race)
                if let Some(q) = &query {
                    if !q.is_empty() {
                        let name_match = subrace.name.as_ref()
                            .map(|n| n.to_lowercase().contains(&q.to_lowercase()))
                            .unwrap_or(false);
                        let parent_match = subrace.race_name.to_lowercase().contains(&q.to_lowercase());
                        if !name_match && !parent_match {
                            return false;
                        }
                    }
                }
                
                // Filter by sources
                if !sources.is_empty() && !sources.contains(&subrace.source) {
                    return false;
                }
                
                // Filter by darkvision
                if let Some(wants_darkvision) = has_darkvision {
                    let has_it = subrace.darkvision.is_some();
                    if has_it != wants_darkvision {
                        return false;
                    }
                }
                
                // Filter by flight
                if let Some(wants_flight) = has_flight {
                    let has_it = subrace.speed.as_ref()
                        .and_then(|s| s.as_object())
                        .and_then(|obj| obj.get("fly"))
                        .is_some();
                    if has_it != wants_flight {
                        return false;
                    }
                }
                
                true
            })
            .map(RaceSummary::from)
            .collect();
        
        results.extend(subrace_results);
        
        // Sort by name, but group subraces with their parent race
        results.sort_by(|a, b| {
            // Extract the base race name for sorting
            let a_sort_key = if a.is_subrace {
                // For subraces like "Hill, Dwarf", sort by "Dwarf"
                a.parent_race.as_ref().unwrap_or(&a.name).clone()
            } else {
                a.name.clone()
            };
            
            let b_sort_key = if b.is_subrace {
                b.parent_race.as_ref().unwrap_or(&b.name).clone()
            } else {
                b.name.clone()
            };
            
            // First sort by race name, then by full name
            match a_sort_key.cmp(&b_sort_key) {
                std::cmp::Ordering::Equal => a.name.cmp(&b.name),
                other => other
            }
        });
        
        debug!("Found {} races/subraces matching criteria", results.len());
        results
    }
    
    /// Get details for a specific race
    pub fn get_race_details(&self, name: &str, source: &str) -> Option<RaceWithDetails> {
        // For subraces, the name might be formatted as "Subrace, Race"
        // We need to extract just the subrace part
        let (search_name, is_formatted_subrace) = if name.contains(", ") {
            let parts: Vec<&str> = name.split(", ").collect();
            if parts.len() == 2 {
                (parts[0], true) // Just the subrace name like "Hill"
            } else {
                (name, false)
            }
        } else {
            (name, false)
        };
        
        // Check if it's a main race (only if not formatted as subrace)
        if !is_formatted_subrace {
            if let Some(race) = self.races.iter()
                .find(|r| r.name == name && r.source == source) {
            
            // Find associated subraces
            let subraces: Vec<Subrace> = self.subraces.iter()
                .filter(|s| s.race_name == name && s.race_source == source)
                .cloned()
                .collect();
            
            // Find fluff
            let fluff = self.race_fluff.iter()
                .find(|f| f.name == name && f.source == source)
                .cloned();
            
            return Some(RaceWithDetails {
                race: Some(race.clone()),
                subrace: None,
                related_subraces: subraces,
                fluff,
            });
        }
        }
        
        // Check if it's a subrace
        if let Some(subrace) = self.subraces.iter()
            .find(|s| s.name.as_deref() == Some(search_name) && s.source == source) {
            
            // Find parent race
            let parent_race = self.races.iter()
                .find(|r| r.name == subrace.race_name && r.source == subrace.race_source)
                .cloned();
            
            // Find sibling subraces
            let related_subraces: Vec<Subrace> = self.subraces.iter()
                .filter(|s| s.race_name == subrace.race_name && 
                           s.race_source == subrace.race_source &&
                           s.name.as_ref() != subrace.name.as_ref())
                .cloned()
                .collect();
            
            // Find fluff (might be under parent race name or actual subrace name)
            let fluff = self.race_fluff.iter()
                .find(|f| (f.name == search_name && f.source == source) ||
                         (f.name == subrace.race_name && f.source == subrace.race_source))
                .cloned();
            
            return Some(RaceWithDetails {
                race: parent_race,
                subrace: Some(subrace.clone()),
                related_subraces,
                fluff,
            });
        }
        
        None
    }
}

/// Complete race details including subraces and fluff
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RaceWithDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub race: Option<Race>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subrace: Option<Subrace>,
    pub related_subraces: Vec<Subrace>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fluff: Option<RaceFluff>,
}

/// Initialize the race catalog by loading data from the books directory
#[tauri::command]
pub async fn initialize_race_catalog(
    catalog: State<'_, std::sync::Mutex<RaceCatalog>>
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

/// Search races with optional filters
#[tauri::command]
pub async fn search_races(
    catalog: State<'_, std::sync::Mutex<RaceCatalog>>,
    query: Option<String>,
    sources: Option<Vec<String>>,
    sizes: Option<Vec<String>>,
    has_darkvision: Option<bool>,
    has_flight: Option<bool>,
) -> Result<Vec<RaceSummary>, String> {
    let catalog = catalog.lock().map_err(|e| e.to_string())?;
    
    let results = catalog.search(
        query,
        sources.unwrap_or_default(),
        sizes.unwrap_or_default(),
        has_darkvision,
        has_flight
    );
    
    Ok(results)
}

/// Get details for a specific race or subrace
#[tauri::command]
pub async fn get_race_details(
    catalog: State<'_, std::sync::Mutex<RaceCatalog>>,
    name: String,
    source: String,
) -> Result<Option<RaceWithDetails>, String> {
    let catalog = catalog.lock().map_err(|e| e.to_string())?;
    
    Ok(catalog.get_race_details(&name, &source))
}