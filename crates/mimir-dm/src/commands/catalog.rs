//! Catalog commands for searching D&D reference content

use std::path::PathBuf;
use std::fs;
use tauri::State;
use serde_json;
use mimir_dm_core::models::rules::{Spell, SpellData, SpellSummary, Item, ItemData, ItemSummary};

/// Spell catalog state - holds all loaded spells in memory
pub struct SpellCatalog {
    spells: Vec<Spell>,
}

impl SpellCatalog {
    pub fn new() -> Self {
        Self {
            spells: Vec::new(),
        }
    }
    
    /// Load spells from book directories (extracted archives)
    pub fn load_from_books_directory(&mut self, books_dir: &PathBuf) -> Result<(), String> {
        if !books_dir.exists() {
            println!("Books directory not found: {:?}", books_dir);
            return Ok(()); // Not an error, just no books yet
        }
        
        // Clear existing spells
        self.spells.clear();
        
        // Iterate through each book directory
        for entry in fs::read_dir(books_dir).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let book_path = entry.path();
            
            if !book_path.is_dir() {
                continue;
            }
            
            let book_id = entry.file_name().to_string_lossy().to_string();
            
            // Check the spells directory (the actual structure used by the splitter)
            let spells_dir = book_path.join("spells");
            if spells_dir.exists() {
                for spell_entry in fs::read_dir(&spells_dir).map_err(|e| e.to_string())? {
                    let spell_entry = spell_entry.map_err(|e| e.to_string())?;
                    let spell_file = spell_entry.path();
                    
                    // Skip non-JSON files and fluff/index files
                    if spell_file.extension().and_then(|s| s.to_str()) != Some("json") {
                        continue;
                    }
                    
                    let filename = spell_file.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    if filename.starts_with("fluff-") || filename == "index.json" || filename == "foundry.json" {
                        continue;
                    }
                    
                    // This should match files like "spells-phb.json"
                    if filename.starts_with("spells-") {
                        match fs::read_to_string(&spell_file) {
                            Ok(content) => {
                                match serde_json::from_str::<SpellData>(&content) {
                                    Ok(spell_data) => {
                                        println!("Loaded {} spells from {}/{}", 
                                                spell_data.spell.len(), book_id, filename);
                                        self.spells.extend(spell_data.spell);
                                    }
                                    Err(e) => {
                                        eprintln!("Failed to parse {}/{}: {}", book_id, filename, e);
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to read {}/{}: {}", book_id, filename, e);
                            }
                        }
                    }
                }
            }
        }
        
        println!("Total spells loaded: {}", self.spells.len());
        Ok(())
    }
    
    /// Search spells with filters
    pub fn search(&self, 
        query: Option<String>,
        sources: Vec<String>,
        levels: Vec<u8>,
        schools: Vec<String>,
        ritual: Option<bool>,
        concentration: Option<bool>
    ) -> Vec<SpellSummary> {
        println!("Searching spells - query: {:?}, sources: {:?}, total spells: {}", 
                 query, sources, self.spells.len());
        
        let results: Vec<SpellSummary> = self.spells.iter()
            .filter(|spell| {
                // Filter by query (search in name)
                if let Some(q) = &query {
                    if !q.is_empty() && !spell.name.to_lowercase().contains(&q.to_lowercase()) {
                        return false;
                    }
                }
                
                // Filter by sources
                if !sources.is_empty() && !sources.contains(&spell.source) {
                    return false;
                }
                
                // Filter by level
                if !levels.is_empty() && !levels.contains(&spell.level) {
                    return false;
                }
                
                // Filter by school
                if !schools.is_empty() && !schools.contains(&spell.school.as_str().to_string()) {
                    return false;
                }
                
                // Filter by ritual
                if let Some(is_ritual) = ritual {
                    let spell_is_ritual = spell.meta.as_ref().map(|m| m.ritual).unwrap_or(false);
                    if spell_is_ritual != is_ritual {
                        return false;
                    }
                }
                
                // Filter by concentration
                if let Some(is_concentration) = concentration {
                    let spell_concentration = spell.duration.iter().any(|d| d.concentration.unwrap_or(false));
                    if spell_concentration != is_concentration {
                        return false;
                    }
                }
                
                true
            })
            .map(SpellSummary::from)
            .collect();
            
        println!("Found {} spells matching criteria", results.len());
        results
    }
}

/// Initialize the spell catalog by loading data from the books directory
#[tauri::command]
pub async fn initialize_spell_catalog(
    catalog: State<'_, std::sync::Mutex<SpellCatalog>>
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

/// Search spells with optional filters
#[tauri::command]
pub async fn search_spells(
    catalog: State<'_, std::sync::Mutex<SpellCatalog>>,
    query: Option<String>,
    sources: Option<Vec<String>>,
    levels: Option<Vec<u8>>,
    schools: Option<Vec<String>>,
    ritual: Option<bool>,
    concentration: Option<bool>,
) -> Result<Vec<SpellSummary>, String> {
    let catalog = catalog.lock().map_err(|e| e.to_string())?;
    
    let results = catalog.search(
        query,
        sources.unwrap_or_default(),
        levels.unwrap_or_default(),
        schools.unwrap_or_default(),
        ritual,
        concentration
    );
    
    Ok(results)
}

/// Get details for a specific spell
#[tauri::command]
pub async fn get_spell_details(
    catalog: State<'_, std::sync::Mutex<SpellCatalog>>,
    name: String,
    source: String,
) -> Result<Option<Spell>, String> {
    let catalog = catalog.lock().map_err(|e| e.to_string())?;
    
    let spell = catalog.spells.iter()
        .find(|s| s.name == name && s.source == source)
        .cloned();
    
    Ok(spell)
}

/// Item catalog state - holds all loaded items in memory
pub struct ItemCatalog {
    items: Vec<Item>,
}

impl ItemCatalog {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
        }
    }
    
    /// Load items from book directories (extracted archives)
    pub fn load_from_books_directory(&mut self, books_dir: &PathBuf) -> Result<(), String> {
        if !books_dir.exists() {
            println!("Books directory not found: {:?}", books_dir);
            return Ok(()); // Not an error, just no books yet
        }
        
        // Clear existing items
        self.items.clear();
        
        // Iterate through each book directory
        for entry in fs::read_dir(books_dir).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let book_path = entry.path();
            
            if !book_path.is_dir() {
                continue;
            }
            
            let book_id = entry.file_name().to_string_lossy().to_string();
            
            // Check the items directory
            let items_dir = book_path.join("items");
            if items_dir.exists() {
                for item_entry in fs::read_dir(&items_dir).map_err(|e| e.to_string())? {
                    let item_entry = item_entry.map_err(|e| e.to_string())?;
                    let item_file = item_entry.path();
                    
                    // Skip non-JSON files and fluff files
                    if item_file.extension().and_then(|s| s.to_str()) != Some("json") {
                        continue;
                    }
                    
                    let filename = item_file.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    if filename.starts_with("fluff-") || filename == "index.json" || filename == "foundry.json" {
                        continue;
                    }
                    
                    match fs::read_to_string(&item_file) {
                        Ok(content) => {
                            match serde_json::from_str::<ItemData>(&content) {
                                Ok(item_data) => {
                                    println!("Loaded {} items from {}/{}", 
                                            item_data.item.len(), book_id, filename);
                                    self.items.extend(item_data.item);
                                }
                                Err(e) => {
                                    eprintln!("Failed to parse {}/{}: {}", book_id, filename, e);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to read {}/{}: {}", book_id, filename, e);
                        }
                    }
                }
            }
        }
        
        println!("Total items loaded: {}", self.items.len());
        Ok(())
    }
    
    /// Search items with filters
    pub fn search(&self, 
        query: Option<String>,
        sources: Vec<String>,
        types: Vec<String>,
        rarities: Vec<String>,
        min_value: Option<u32>,
        max_value: Option<u32>
    ) -> Vec<ItemSummary> {
        println!("Searching items - query: {:?}, sources: {:?}, total items: {}", 
                 query, sources, self.items.len());
        
        let results: Vec<ItemSummary> = self.items.iter()
            .filter(|item| {
                // Filter by query (search in name)
                if let Some(q) = &query {
                    if !q.is_empty() && !item.name.to_lowercase().contains(&q.to_lowercase()) {
                        return false;
                    }
                }
                
                // Filter by sources
                if !sources.is_empty() && !sources.contains(&item.source) {
                    return false;
                }
                
                // Filter by type
                if !types.is_empty() && !types.contains(&item.item_type) {
                    return false;
                }
                
                // Filter by rarity
                if !rarities.is_empty() {
                    let item_rarity = item.rarity.as_ref().map(|r| r.as_str()).unwrap_or("none");
                    if !rarities.iter().any(|r| r == item_rarity) {
                        return false;
                    }
                }
                
                // Filter by value range
                if let Some(min) = min_value {
                    if item.value.unwrap_or(0) < min {
                        return false;
                    }
                }
                
                if let Some(max) = max_value {
                    if item.value.unwrap_or(u32::MAX) > max {
                        return false;
                    }
                }
                
                true
            })
            .map(ItemSummary::from)
            .collect();
            
        println!("Found {} items matching criteria", results.len());
        results
    }
}

/// Initialize the item catalog by loading data from the books directory
#[tauri::command]
pub async fn initialize_item_catalog(
    catalog: State<'_, std::sync::Mutex<ItemCatalog>>
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

/// Search items with optional filters
#[tauri::command]
pub async fn search_items(
    catalog: State<'_, std::sync::Mutex<ItemCatalog>>,
    query: Option<String>,
    sources: Option<Vec<String>>,
    types: Option<Vec<String>>,
    rarities: Option<Vec<String>>,
    min_value: Option<u32>,
    max_value: Option<u32>,
) -> Result<Vec<ItemSummary>, String> {
    let catalog = catalog.lock().map_err(|e| e.to_string())?;
    
    let results = catalog.search(
        query,
        sources.unwrap_or_default(),
        types.unwrap_or_default(),
        rarities.unwrap_or_default(),
        min_value,
        max_value
    );
    
    Ok(results)
}

/// Get details for a specific item
#[tauri::command]
pub async fn get_item_details(
    catalog: State<'_, std::sync::Mutex<ItemCatalog>>,
    name: String,
    source: String,
) -> Result<Option<Item>, String> {
    let catalog = catalog.lock().map_err(|e| e.to_string())?;
    
    let item = catalog.items.iter()
        .find(|i| i.name == name && i.source == source)
        .cloned();
    
    Ok(item)
}