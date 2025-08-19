//! Catalog commands for searching D&D reference content

use std::path::PathBuf;
use std::fs;
use tauri::State;
use serde_json;
use tracing::{debug, info, warn, error};
use mimir_dm_core::models::rules::{
    Spell, SpellData, SpellSummary, 
    Item, ItemData, ItemSummary,
    Monster, MonsterData, MonsterSummary
};

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
            info!("Books directory not found: {:?}", books_dir);
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
                                        debug!("Loaded {} spells from {}/{}", 
                                                spell_data.spell.len(), book_id, filename);
                                        self.spells.extend(spell_data.spell);
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
        }
        
        info!("Total spells loaded: {}", self.spells.len());
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
        debug!("Searching spells - query: {:?}, sources: {:?}, total spells: {}", 
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
            
        debug!("Found {} spells matching criteria", results.len());
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
            info!("Books directory not found: {:?}", books_dir);
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
                                    debug!("Loaded {} items from {}/{}", 
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
        
        info!("Total items loaded: {}", self.items.len());
        Ok(())
    }
    
    /// Search items with filters
    pub fn search(&self, 
        query: Option<String>,
        sources: Vec<String>,
        types: Vec<String>,
        rarities: Vec<String>,
        min_value: Option<f64>,
        max_value: Option<f64>
    ) -> Vec<ItemSummary> {
        debug!("Searching items - query: {:?}, sources: {:?}, total items: {}", 
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
                if !types.is_empty() {
                    let item_type = item.item_type.as_ref().map(|t| t.as_str()).unwrap_or("?");
                    if !types.iter().any(|t| t == item_type) {
                        return false;
                    }
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
                    if item.value.unwrap_or(0.0) < min {
                        return false;
                    }
                }
                
                if let Some(max) = max_value {
                    if item.value.unwrap_or(f64::MAX) > max {
                        return false;
                    }
                }
                
                true
            })
            .map(ItemSummary::from)
            .collect();
            
        debug!("Found {} items matching criteria", results.len());
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
    min_value: Option<f64>,
    max_value: Option<f64>,
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

/// Monster catalog state - holds all loaded monsters in memory
pub struct MonsterCatalog {
    monsters: Vec<Monster>,
}

impl MonsterCatalog {
    pub fn new() -> Self {
        Self {
            monsters: Vec::new(),
        }
    }
    
    /// Load monsters from book directories (extracted archives)
    pub fn load_from_books_directory(&mut self, books_dir: &PathBuf) -> Result<(), String> {
        if !books_dir.exists() {
            info!("Books directory not found: {:?}", books_dir);
            return Ok(()); // Not an error, just no books yet
        }
        
        // Clear existing monsters
        self.monsters.clear();
        
        // Iterate through each book directory
        for entry in fs::read_dir(books_dir).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let book_path = entry.path();
            
            if !book_path.is_dir() {
                continue;
            }
            
            let book_id = entry.file_name().to_string_lossy().to_string();
            
            // Check the bestiary directory
            let bestiary_dir = book_path.join("bestiary");
            if bestiary_dir.exists() {
                for bestiary_entry in fs::read_dir(&bestiary_dir).map_err(|e| e.to_string())? {
                    let bestiary_entry = bestiary_entry.map_err(|e| e.to_string())?;
                    let bestiary_file = bestiary_entry.path();
                    
                    // Skip non-JSON files and fluff files
                    if bestiary_file.extension().and_then(|s| s.to_str()) != Some("json") {
                        continue;
                    }
                    
                    let filename = bestiary_file.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    if filename.starts_with("fluff-") || filename == "index.json" || filename == "foundry.json" {
                        continue;
                    }
                    
                    // This should match files like "bestiary-mm.json"
                    if filename.starts_with("bestiary-") {
                        match fs::read_to_string(&bestiary_file) {
                            Ok(content) => {
                                match serde_json::from_str::<MonsterData>(&content) {
                                    Ok(monster_data) => {
                                        debug!("Loaded {} monsters from {}/{}", 
                                                monster_data.monster.len(), book_id, filename);
                                        self.monsters.extend(monster_data.monster);
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
        }
        
        info!("Total monsters loaded: {}", self.monsters.len());
        Ok(())
    }
    
    /// Search monsters with filters
    pub fn search(&self, 
        query: Option<String>,
        sources: Vec<String>,
        sizes: Vec<String>,
        types: Vec<String>,
        min_cr: Option<f32>,
        max_cr: Option<f32>,
        environments: Vec<String>
    ) -> Vec<MonsterSummary> {
        debug!("Searching monsters - query: {:?}, sources: {:?}, total monsters: {}", 
                 query, sources, self.monsters.len());
        
        let results: Vec<MonsterSummary> = self.monsters.iter()
            .filter(|monster| {
                // Filter by query (search in name)
                if let Some(q) = &query {
                    if !q.is_empty() && !monster.name.to_lowercase().contains(&q.to_lowercase()) {
                        return false;
                    }
                }
                
                // Filter by sources
                if !sources.is_empty() && !sources.contains(&monster.source) {
                    return false;
                }
                
                // Filter by size
                if !sizes.is_empty() {
                    let monster_size = monster.size.as_ref()
                        .and_then(|s| s.first())
                        .map(|s| s.as_str())
                        .unwrap_or("");
                    if !sizes.iter().any(|s| s == monster_size) {
                        return false;
                    }
                }
                
                // Filter by type (handle both string and object)
                if !types.is_empty() {
                    let monster_type = match &monster.creature_type {
                        Some(serde_json::Value::String(s)) => s.as_str(),
                        Some(serde_json::Value::Object(obj)) => {
                            obj.get("type")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                        },
                        _ => "",
                    };
                    if !types.iter().any(|t| t == monster_type) {
                        return false;
                    }
                }
                
                // Filter by environment
                if !environments.is_empty() {
                    if let Some(envs) = &monster.environment {
                        let has_env = environments.iter().any(|e| envs.contains(e));
                        if !has_env {
                            return false;
                        }
                    } else {
                        return false;  // No environments means it doesn't match filter
                    }
                }
                
                true
            })
            .map(MonsterSummary::from)
            .filter(|summary| {
                // Apply CR range filter on the summary (which has cr_numeric)
                if let Some(min) = min_cr {
                    if summary.cr_numeric < min {
                        return false;
                    }
                }
                
                if let Some(max) = max_cr {
                    if summary.cr_numeric > max {
                        return false;
                    }
                }
                
                true
            })
            .collect();
            
        debug!("Found {} monsters matching criteria", results.len());
        results
    }
}

/// Initialize the monster catalog by loading data from the books directory
#[tauri::command]
pub async fn initialize_monster_catalog(
    catalog: State<'_, std::sync::Mutex<MonsterCatalog>>
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

/// Search monsters with optional filters
#[tauri::command]
pub async fn search_monsters(
    catalog: State<'_, std::sync::Mutex<MonsterCatalog>>,
    query: Option<String>,
    sources: Option<Vec<String>>,
    sizes: Option<Vec<String>>,
    types: Option<Vec<String>>,
    min_cr: Option<f32>,
    max_cr: Option<f32>,
    environments: Option<Vec<String>>,
) -> Result<Vec<MonsterSummary>, String> {
    let catalog = catalog.lock().map_err(|e| e.to_string())?;
    
    let results = catalog.search(
        query,
        sources.unwrap_or_default(),
        sizes.unwrap_or_default(),
        types.unwrap_or_default(),
        min_cr,
        max_cr,
        environments.unwrap_or_default()
    );
    
    Ok(results)
}

/// Get details for a specific monster
#[tauri::command]
pub async fn get_monster_details(
    catalog: State<'_, std::sync::Mutex<MonsterCatalog>>,
    name: String,
    source: String,
) -> Result<Option<Monster>, String> {
    let catalog = catalog.lock().map_err(|e| e.to_string())?;
    
    let monster = catalog.monsters.iter()
        .find(|m| m.name == name && m.source == source)
        .cloned();
    
    Ok(monster)
}