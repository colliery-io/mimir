//! Database service layer for spell operations
//!
//! This service provides database-backed spell search and retrieval,
//! replacing the in-memory catalog system.

use crate::models::catalog::{CatalogSpell, SpellFilters, SpellSummary, Spell, NewCatalogSpell, SpellData};
use crate::schema::catalog_spells;
use diesel::prelude::*;
use std::fs;
use std::path::Path;
use tracing::{debug, error, info, warn};

pub struct SpellService;

impl SpellService {
    /// Search spells with optional filters
    pub fn search_spells(
        conn: &mut SqliteConnection,
        filters: SpellFilters,
    ) -> Result<Vec<SpellSummary>, String> {
        debug!("Searching spells with filters: {:?}", filters);
        
        let mut query = catalog_spells::table.into_boxed();
        
        // Apply name search filter
        if let Some(name_query) = &filters.query {
            if !name_query.trim().is_empty() {
                query = query.filter(
                    catalog_spells::name.like(format!("%{}%", name_query.trim()))
                );
            }
        }
        
        // Apply level filters
        if !filters.levels.is_empty() {
            query = query.filter(catalog_spells::level.eq_any(&filters.levels));
        }
        
        // Apply school filters
        if !filters.schools.is_empty() {
            query = query.filter(catalog_spells::school.eq_any(&filters.schools));
        }
        
        // Apply source filters
        if !filters.sources.is_empty() {
            query = query.filter(catalog_spells::source.eq_any(&filters.sources));
        }
        
        // Apply tag filters (requires JSON containment check)
        if !filters.tags.is_empty() {
            for tag in &filters.tags {
                // SQLite doesn't have native JSON operators, so we use LIKE
                query = query.filter(catalog_spells::tags.like(format!("%\"{}\"%%", tag)));
            }
        }
        
        // Apply pagination
        if let Some(offset) = filters.offset {
            query = query.offset(offset as i64);
        }
        
        // Apply limit only if explicitly requested
        if let Some(limit) = filters.limit {
            query = query.limit(limit as i64);
        }
        
        // Execute query with explicit select
        let catalog_spells: Vec<CatalogSpell> = query
            .select(CatalogSpell::as_select())
            .load(conn)
            .map_err(|e| format!("Failed to search spells: {}", e))?;
        
        let summaries: Vec<SpellSummary> = catalog_spells
            .iter()
            .map(|spell| spell.to_summary())
            .collect();
        
        info!("Found {} spells matching search criteria", summaries.len());
        Ok(summaries)
    }
    
    /// Get detailed spell information by name and source
    pub fn get_spell_details(
        conn: &mut SqliteConnection,
        name: &str,
        source: &str,
    ) -> Result<Option<Spell>, String> {
        debug!("Getting spell details: {} from {}", name, source);
        
        let catalog_spell: Option<CatalogSpell> = catalog_spells::table
            .filter(catalog_spells::name.eq(name))
            .filter(catalog_spells::source.eq(source))
            .select(CatalogSpell::as_select())
            .first(conn)
            .optional()
            .map_err(|e| format!("Failed to fetch spell details: {}", e))?;
        
        if let Some(spell_record) = catalog_spell {
            // Parse the full JSON spell data
            let spell: Spell = serde_json::from_str(&spell_record.full_spell_json)
                .map_err(|e| format!("Failed to parse spell JSON: {}", e))?;
            
            debug!("Found spell details for: {}", name);
            Ok(Some(spell))
        } else {
            debug!("No spell found with name '{}' from source '{}'", name, source);
            Ok(None)
        }
    }
    
    /// Get unique spell sources for filter dropdown
    pub fn get_spell_sources(conn: &mut SqliteConnection) -> Result<Vec<String>, String> {
        debug!("Getting unique spell sources");
        
        let sources: Vec<String> = catalog_spells::table
            .select(catalog_spells::source)
            .distinct()
            .order(catalog_spells::source)
            .load(conn)
            .map_err(|e| format!("Failed to fetch spell sources: {}", e))?;
        
        debug!("Found {} unique spell sources", sources.len());
        Ok(sources)
    }
    
    /// Get unique spell schools for filter dropdown
    pub fn get_spell_schools(conn: &mut SqliteConnection) -> Result<Vec<String>, String> {
        debug!("Getting unique spell schools");
        
        let schools: Vec<String> = catalog_spells::table
            .select(catalog_spells::school)
            .distinct()
            .order(catalog_spells::school)
            .load(conn)
            .map_err(|e| format!("Failed to fetch spell schools: {}", e))?;
        
        debug!("Found {} unique spell schools", schools.len());
        Ok(schools)
    }
    
    /// Get spell count by source for statistics
    pub fn get_spell_count_by_source(conn: &mut SqliteConnection) -> Result<Vec<(String, i64)>, String> {
        debug!("Getting spell count by source");
        
        use diesel::dsl::count_star;
        
        let counts: Vec<(String, i64)> = catalog_spells::table
            .group_by(catalog_spells::source)
            .select((catalog_spells::source, count_star()))
            .order(catalog_spells::source)
            .load(conn)
            .map_err(|e| format!("Failed to fetch spell counts: {}", e))?;
        
        debug!("Found spell counts for {} sources", counts.len());
        Ok(counts)
    }
    
    /// Get total spell count
    pub fn get_total_spell_count(conn: &mut SqliteConnection) -> Result<i64, String> {
        debug!("Getting total spell count");

        use diesel::dsl::count_star;

        let count: i64 = catalog_spells::table
            .select(count_star())
            .first(conn)
            .map_err(|e| format!("Failed to count spells: {}", e))?;

        debug!("Total spells in database: {}", count);
        Ok(count)
    }

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

    /// Simple pattern matching helper for file name patterns
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