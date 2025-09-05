//! Catalog import service for D&D 5e content
//!
//! This service handles automatic import of catalog content from uploaded books
//! into the SQLite database for fast searching and filtering.

use crate::models::catalog::{NewCatalogSpell, Spell, SpellData};
use crate::schema::catalog_spells;
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