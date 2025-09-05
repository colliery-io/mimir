//! Catalog import service for D&D 5e content
//!
//! This service handles automatic import of catalog content from uploaded books
//! into the SQLite database for fast searching and filtering.

use crate::models::catalog::{NewCatalogSpell, Spell, SpellData, NewCatalogAction, Action};
use crate::schema::{catalog_spells, catalog_actions};
use diesel::prelude::*;
use std::fs;
use std::path::Path;
use tracing::{error, info, warn, debug};

pub struct CatalogService;

impl CatalogService {
    /// Import all spell data from an uploaded book directory
    pub fn import_spells_from_book(
        conn: &mut SqliteConnection, 
        book_dir: &Path, 
        source: &str
    ) -> Result<usize, String> {
        info!("Importing spells from book directory: {:?} (source: {})", book_dir, source);
        
        let mut total_imported = 0;
        let spell_files = Self::find_spell_files(book_dir)?;
        
        if spell_files.is_empty() {
            info!("No spell files found in book directory");
            return Ok(0);
        }
        
        info!("Found {} spell files to process", spell_files.len());
        
        for spell_file in spell_files {
            debug!("Processing spell file: {:?}", spell_file);
            
            match Self::import_spells_from_file(conn, &spell_file, source) {
                Ok(count) => {
                    info!("Imported {} spells from {:?}", count, spell_file);
                    total_imported += count;
                }
                Err(e) => {
                    error!("Failed to import spells from {:?}: {}", spell_file, e);
                    // Continue processing other files instead of failing completely
                }
            }
        }
        
        info!("Successfully imported {} total spells from {}", total_imported, source);
        Ok(total_imported)
    }
    
    /// Find all spell JSON files in a book directory
    fn find_spell_files(book_dir: &Path) -> Result<Vec<std::path::PathBuf>, String> {
        let mut spell_files = Vec::new();
        
        // Common locations for spell data
        let search_dirs = vec![
            book_dir.join("data"),
            book_dir.join("spells"),  
            book_dir.join("book"),
            book_dir.to_path_buf(),
        ];
        
        for dir in search_dirs {
            if !dir.exists() {
                continue;
            }
            
            debug!("Searching for spell files in: {:?}", dir);
            
            // Look for spell-specific files and general book files
            let file_patterns = vec!["spells-*.json", "spell*.json", "book-*.json"];
            
            if let Ok(entries) = fs::read_dir(&dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    
                    if path.is_file() {
                        let file_name = path.file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("");
                        
                        // Check if file matches any of our patterns
                        for pattern in &file_patterns {
                            if Self::matches_pattern(file_name, pattern) {
                                debug!("Found potential spell file: {:?}", path);
                                spell_files.push(path);
                                break;
                            }
                        }
                    }
                }
            }
        }
        
        spell_files.sort();
        spell_files.dedup();
        Ok(spell_files)
    }
    
    /// Import spells from a single JSON file
    fn import_spells_from_file(
        conn: &mut SqliteConnection,
        file_path: &Path,
        source: &str
    ) -> Result<usize, String> {
        debug!("Reading spell file: {:?}", file_path);
        
        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read file {:?}: {}", file_path, e))?;
        
        let json_value: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse JSON from {:?}: {}", file_path, e))?;
        
        let mut spells_to_import = Vec::new();
        
        // Try to parse as SpellData structure first (spells-*.json files)
        if let Ok(spell_data) = serde_json::from_value::<SpellData>(json_value.clone()) {
            debug!("Parsed as SpellData, found {} spells", spell_data.spell.len());
            
            for spell in spell_data.spell {
                spells_to_import.push(spell);
            }
        }
        // Try to extract spells from book structure (book-*.json files)
        else if let Some(spell_array) = json_value.get("spell").and_then(|v| v.as_array()) {
            debug!("Found spell array in book structure with {} entries", spell_array.len());
            
            for spell_value in spell_array {
                match serde_json::from_value::<Spell>(spell_value.clone()) {
                    Ok(spell) => spells_to_import.push(spell),
                    Err(e) => {
                        warn!("Failed to parse spell from book structure: {}", e);
                        continue;
                    }
                }
            }
        }
        // Check for nested data structure
        else if let Some(data_array) = json_value.get("data").and_then(|v| v.as_array()) {
            debug!("Checking nested data structure");
            
            for data_section in data_array {
                if let Some(spell_array) = data_section.get("spell").and_then(|v| v.as_array()) {
                    debug!("Found nested spell array with {} entries", spell_array.len());
                    
                    for spell_value in spell_array {
                        match serde_json::from_value::<Spell>(spell_value.clone()) {
                            Ok(spell) => spells_to_import.push(spell),
                            Err(e) => {
                                warn!("Failed to parse nested spell: {}", e);
                                continue;
                            }
                        }
                    }
                }
            }
        }
        
        if spells_to_import.is_empty() {
            debug!("No spells found in file: {:?}", file_path);
            return Ok(0);
        }
        
        debug!("Processing {} spells for database import", spells_to_import.len());
        
        // Transform spells to database format
        let catalog_spells: Vec<NewCatalogSpell> = spells_to_import
            .into_iter()
            .map(|spell| NewCatalogSpell::from_spell(spell, source))
            .collect();
        
        // Batch insert spells
        Self::batch_insert_spells(conn, catalog_spells)
    }
    
    /// Batch insert spells into the database
    fn batch_insert_spells(
        conn: &mut SqliteConnection,
        spells: Vec<NewCatalogSpell>
    ) -> Result<usize, String> {
        if spells.is_empty() {
            return Ok(0);
        }
        
        debug!("Batch inserting {} spells into database", spells.len());
        
        // Use INSERT OR IGNORE to handle duplicates gracefully
        let inserted = diesel::insert_or_ignore_into(catalog_spells::table)
            .values(&spells)
            .execute(conn)
            .map_err(|e| format!("Failed to insert spells: {}", e))?;
        
        debug!("Successfully inserted {} spells (duplicates ignored)", inserted);
        Ok(inserted)
    }
    
    /// Remove all spells from a specific source
    pub fn remove_spells_by_source(
        conn: &mut SqliteConnection,
        source: &str
    ) -> Result<usize, String> {
        info!("Removing all spells from source: {}", source);
        
        let deleted = diesel::delete(catalog_spells::table.filter(catalog_spells::source.eq(source)))
            .execute(conn)
            .map_err(|e| format!("Failed to delete spells from source {}: {}", source, e))?;
        
        info!("Removed {} spells from source: {}", deleted, source);
        Ok(deleted)
    }
    
    /// Import all action data from an uploaded book directory
    pub fn import_actions_from_book(
        conn: &mut SqliteConnection, 
        book_dir: &Path, 
        source: &str
    ) -> Result<usize, String> {
        info!("Importing actions from book directory: {:?} (source: {})", book_dir, source);
        
        let mut total_imported = 0;
        let action_files = Self::find_action_files(book_dir)?;
        
        if action_files.is_empty() {
            info!("No action files found in book directory");
            return Ok(0);
        }
        
        info!("Found {} action files to process", action_files.len());
        
        for action_file in action_files {
            debug!("Processing action file: {:?}", action_file);
            
            match Self::import_actions_from_file(conn, &action_file, source) {
                Ok(count) => {
                    info!("Imported {} actions from {:?}", count, action_file);
                    total_imported += count;
                }
                Err(e) => {
                    error!("Failed to import actions from {:?}: {}", action_file, e);
                    // Continue processing other files instead of failing completely
                }
            }
        }
        
        info!("Successfully imported {} total actions from {}", total_imported, source);
        Ok(total_imported)
    }

    /// Find action JSON files in a book directory
    fn find_action_files(book_dir: &Path) -> Result<Vec<std::path::PathBuf>, String> {
        let mut action_files = Vec::new();
        
        // Common action file patterns
        let patterns = vec![
            "actions.json",
            "action.json", 
            "*action*.json",
        ];
        
        if let Ok(entries) = fs::read_dir(book_dir) {
            for entry in entries.flatten() {
                if let Some(filename) = entry.file_name().to_str() {
                    if filename.ends_with(".json") {
                        for pattern in &patterns {
                            if Self::matches_pattern(filename, pattern) {
                                debug!("Found potential action file: {}", filename);
                                action_files.push(entry.path());
                                break;
                            }
                        }
                    }
                }
            }
        }
        
        // Also check common subdirectories
        let subdirs = vec!["data", "content", "json"];
        for subdir in subdirs {
            let subdir_path = book_dir.join(subdir);
            if subdir_path.is_dir() {
                if let Ok(entries) = fs::read_dir(&subdir_path) {
                    for entry in entries.flatten() {
                        if let Some(filename) = entry.file_name().to_str() {
                            if filename.ends_with(".json") {
                                for pattern in &patterns {
                                    if Self::matches_pattern(filename, pattern) {
                                        debug!("Found action file in subdir: {:?}", entry.path());
                                        action_files.push(entry.path());
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        debug!("Found {} action files in book directory", action_files.len());
        Ok(action_files)
    }

    /// Import actions from a specific JSON file
    fn import_actions_from_file(
        conn: &mut SqliteConnection,
        file_path: &Path, 
        source: &str
    ) -> Result<usize, String> {
        debug!("Reading action file: {:?}", file_path);
        
        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read file {:?}: {}", file_path, e))?;
            
        let json: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse JSON from {:?}: {}", file_path, e))?;
        
        let mut actions_to_import: Vec<Action> = Vec::new();
        
        // Handle different JSON structures
        if let Some(action_array) = json.get("action").and_then(|v| v.as_array()) {
            // Structure: { "action": [...] }
            for action_value in action_array {
                match serde_json::from_value::<Action>(action_value.clone()) {
                    Ok(mut action) => {
                        action.source = source.to_string();
                        actions_to_import.push(action);
                    }
                    Err(e) => {
                        warn!("Failed to parse action: {}", e);
                        continue;
                    }
                }
            }
        } else if json.is_array() {
            // Structure: [...]
            if let Some(action_array) = json.as_array() {
                for action_value in action_array {
                    match serde_json::from_value::<Action>(action_value.clone()) {
                        Ok(mut action) => {
                            action.source = source.to_string();
                            actions_to_import.push(action);
                        }
                        Err(e) => {
                            warn!("Failed to parse action: {}", e);
                            continue;
                        }
                    }
                }
            }
        } else if let Some(action_obj) = json.as_object() {
            // Look for nested action arrays in other keys
            for (key, value) in action_obj {
                if key.contains("action") || key == "data" {
                    if let Some(action_array) = value.as_array() {
                        for action_value in action_array {
                            match serde_json::from_value::<Action>(action_value.clone()) {
                                Ok(mut action) => {
                                    action.source = source.to_string();
                                    actions_to_import.push(action);
                                }
                                Err(e) => {
                                    warn!("Failed to parse nested action: {}", e);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        }
        
        if actions_to_import.is_empty() {
            debug!("No actions found in file: {:?}", file_path);
            return Ok(0);
        }
        
        debug!("Processing {} actions for database import", actions_to_import.len());
        
        // Transform actions to database format
        let catalog_actions: Vec<NewCatalogAction> = actions_to_import
            .into_iter()
            .map(NewCatalogAction::from)
            .collect();
        
        // Batch insert actions
        Self::batch_insert_actions(conn, catalog_actions)
    }
    
    /// Batch insert actions into the database
    fn batch_insert_actions(
        conn: &mut SqliteConnection,
        actions: Vec<NewCatalogAction>,
    ) -> Result<usize, String> {
        let mut total_inserted = 0;
        
        debug!("Inserting {} actions individually (SQLite limitation)", actions.len());
        
        for action in actions {
            let inserted = diesel::insert_into(catalog_actions::table)
                .values(&action)
                .on_conflict((catalog_actions::name, catalog_actions::source))
                .do_update()
                .set((
                    catalog_actions::time_type.eq(&action.time_type),
                    catalog_actions::description.eq(&action.description),
                    catalog_actions::see_also.eq(&action.see_also),
                    catalog_actions::full_action_json.eq(&action.full_action_json),
                ))
                .execute(conn)
                .map_err(|e| format!("Failed to insert action '{}': {}", action.name, e))?;
                
            total_inserted += inserted;
        }
        
        info!("Successfully imported {} actions into database", total_inserted);
        Ok(total_inserted)
    }

    /// Remove all actions from a specific source (for book removal)
    pub fn remove_actions_by_source(
        conn: &mut SqliteConnection,
        source: &str,
    ) -> Result<usize, String> {
        info!("Removing actions from source: {}", source);
        
        let deleted = diesel::delete(catalog_actions::table.filter(catalog_actions::source.eq(source)))
            .execute(conn)
            .map_err(|e| format!("Failed to delete actions from source {}: {}", source, e))?;
        
        info!("Removed {} actions from source: {}", deleted, source);
        Ok(deleted)
    }

    /// Check if a filename matches a simple glob pattern
    fn matches_pattern(filename: &str, pattern: &str) -> bool {
        if pattern.contains('*') {
            let parts: Vec<&str> = pattern.split('*').collect();
            if parts.len() == 2 {
                return filename.starts_with(parts[0]) && filename.ends_with(parts[1]);
            }
        }
        filename == pattern
    }
}