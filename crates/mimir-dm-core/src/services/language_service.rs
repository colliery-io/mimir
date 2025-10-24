use diesel::prelude::*;
use tracing::{debug, error, info};
use crate::models::catalog::{CatalogLanguage, LanguageFilters, LanguageSummary, Language, NewCatalogLanguage, LanguageData};
use crate::schema::catalog_languages;
use std::fs;
use std::path::Path;

pub struct LanguageService;

impl LanguageService {
    /// Search languages with optional filters
    pub fn search_languages(
        conn: &mut SqliteConnection,
        filters: LanguageFilters,
    ) -> Result<Vec<LanguageSummary>, String> {
        debug!("Searching languages with filters: {:?}", filters);
        
        let mut query = catalog_languages::table.into_boxed();
        
        // Apply name filter
        if let Some(name) = filters.name {
            query = query.filter(catalog_languages::name.eq(name));
        }
        
        // Apply search filter (searches name, script, and typical_speakers)
        if let Some(search) = filters.search {
            let search_pattern = format!("%{}%", search.to_lowercase());
            let pattern_clone1 = search_pattern.clone();
            let pattern_clone2 = search_pattern.clone();
            query = query.filter(
                catalog_languages::name.like(search_pattern)
                    .or(catalog_languages::script.like(pattern_clone1))
                    .or(catalog_languages::typical_speakers.like(pattern_clone2))
            );
        }
        
        // Apply language type filter
        if let Some(language_types) = filters.language_types {
            if !language_types.is_empty() {
                query = query.filter(catalog_languages::language_type.eq_any(language_types));
            }
        }
        
        // Apply script filter
        if let Some(scripts) = filters.scripts {
            if !scripts.is_empty() {
                query = query.filter(catalog_languages::script.eq_any(scripts));
            }
        }
        
        // Apply source filter
        if let Some(sources) = filters.sources {
            if !sources.is_empty() {
                query = query.filter(catalog_languages::source.eq_any(sources));
            }
        }
        
        let catalog_languages: Vec<CatalogLanguage> = query
            .select(CatalogLanguage::as_select())
            .load(conn)
            .map_err(|e| format!("Database error: {}", e))?;
        
        let summaries: Vec<LanguageSummary> = catalog_languages
            .into_iter()
            .map(|cl| LanguageSummary {
                name: cl.name,
                source: cl.source,
                language_type: cl.language_type,
                script: cl.script,
                typical_speakers: cl.typical_speakers,
            })
            .collect();
        
        debug!("Found {} languages matching filters", summaries.len());
        Ok(summaries)
    }
    
    /// Get a specific language by ID
    pub fn get_language_by_id(
        conn: &mut SqliteConnection,
        language_id: i32,
    ) -> Result<Option<Language>, String> {
        debug!("Getting language by ID: {}", language_id);
        
        let catalog_language: Option<CatalogLanguage> = catalog_languages::table
            .find(language_id)
            .first(conn)
            .optional()
            .map_err(|e| format!("Database error: {}", e))?;
        
        match catalog_language {
            Some(cl) => {
                // Parse the full JSON back to the original Language type
                match serde_json::from_str::<Language>(&cl.full_language_json) {
                    Ok(language) => Ok(Some(language)),
                    Err(e) => {
                        error!("Failed to parse language JSON for ID {}: {}", language_id, e);
                        Err(format!("Failed to parse language data: {}", e))
                    }
                }
            }
            None => Ok(None),
        }
    }
    
    /// Get a specific language by name and source
    pub fn get_language_by_name_and_source(
        conn: &mut SqliteConnection,
        name: &str,
        source: &str,
    ) -> Result<Option<Language>, String> {
        debug!("Getting language by name '{}' and source '{}'", name, source);
        
        let catalog_language: Option<CatalogLanguage> = catalog_languages::table
            .filter(catalog_languages::name.eq(name))
            .filter(catalog_languages::source.eq(source))
            .first(conn)
            .optional()
            .map_err(|e| format!("Database error: {}", e))?;
        
        match catalog_language {
            Some(cl) => {
                // Parse the full JSON back to the original Language type
                match serde_json::from_str::<Language>(&cl.full_language_json) {
                    Ok(language) => Ok(Some(language)),
                    Err(e) => {
                        error!("Failed to parse language JSON for '{}' from '{}': {}", name, source, e);
                        Err(format!("Failed to parse language data: {}", e))
                    }
                }
            }
            None => Ok(None),
        }
    }
    
    /// Get all unique language types
    pub fn get_language_types(conn: &mut SqliteConnection) -> Result<Vec<String>, String> {
        debug!("Getting all language types");
        
        let types: Vec<String> = catalog_languages::table
            .select(catalog_languages::language_type)
            .distinct()
            .load(conn)
            .map_err(|e| format!("Database error: {}", e))?;
        
        let mut sorted_types = types;
        sorted_types.sort();
        Ok(sorted_types)
    }
    
    /// Get all unique scripts
    pub fn get_scripts(conn: &mut SqliteConnection) -> Result<Vec<String>, String> {
        debug!("Getting all scripts");
        
        let scripts: Vec<String> = catalog_languages::table
            .select(catalog_languages::script)
            .distinct()
            .filter(catalog_languages::script.ne("—")) // Exclude placeholder
            .load(conn)
            .map_err(|e| format!("Database error: {}", e))?;
        
        let mut sorted_scripts = scripts;
        sorted_scripts.sort();
        Ok(sorted_scripts)
    }
    
    /// Get all unique sources
    pub fn get_sources(conn: &mut SqliteConnection) -> Result<Vec<String>, String> {
        debug!("Getting all sources");
        
        let sources: Vec<String> = catalog_languages::table
            .select(catalog_languages::source)
            .distinct()
            .load(conn)
            .map_err(|e| format!("Database error: {}", e))?;
        
        let mut sorted_sources = sources;
        sorted_sources.sort();
        Ok(sorted_sources)
    }
    
    /// Get total count of languages
    pub fn get_language_count(conn: &mut SqliteConnection) -> Result<i64, String> {
        debug!("Getting language count");

        let count: i64 = catalog_languages::table
            .count()
            .get_result(conn)
            .map_err(|e| format!("Database error: {}", e))?;

        Ok(count)
    }

    /// Import all language data from an uploaded book directory
    pub fn import_languages_from_book(
        conn: &mut SqliteConnection,
        book_dir: &Path,
        source: &str,
    ) -> Result<usize, String> {
        info!("Importing languages from book directory: {:?} (source: {})", book_dir, source);

        let languages_dir = book_dir.join("languages");
        if !languages_dir.exists() || !languages_dir.is_dir() {
            debug!("No languages directory found in book: {:?}", book_dir);
            return Ok(0);
        }

        let mut imported_count = 0;

        // Read all JSON files in the languages directory
        let entries = fs::read_dir(&languages_dir)
            .map_err(|e| format!("Failed to read languages directory: {}", e))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let path = entry.path();

            // Skip fluff files and non-JSON files
            if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                if Self::matches_pattern(filename, &["fluff"]) || !filename.ends_with(".json") {
                    continue;
                }
            }

            debug!("Processing language file: {:?}", path);

            let content = fs::read_to_string(&path)
                .map_err(|e| format!("Failed to read file {:?}: {}", path, e))?;

            let language_data: LanguageData = serde_json::from_str(&content)
                .map_err(|e| format!("Failed to parse language data from {:?}: {}", path, e))?;

            if let Some(languages) = language_data.language {
                let new_languages: Vec<NewCatalogLanguage> = languages
                    .into_iter()
                    .map(|mut lang| {
                        lang.source = source.to_string();
                        NewCatalogLanguage::from(lang)
                    })
                    .collect();

                if !new_languages.is_empty() {
                    let inserted = diesel::insert_into(catalog_languages::table)
                        .values(&new_languages)
                        .execute(conn)
                        .map_err(|e| format!("Failed to insert languages: {}", e))?;

                    imported_count += inserted;
                    info!("Imported {} languages from {:?}", inserted, path);
                }
            }
        }

        info!("Successfully imported {} languages from source: {}", imported_count, source);
        Ok(imported_count)
    }

    /// Remove all languages from a specific source
    pub fn remove_languages_by_source(
        conn: &mut SqliteConnection,
        source: &str,
    ) -> Result<usize, String> {
        info!("Removing languages from source: {}", source);

        let deleted = diesel::delete(catalog_languages::table.filter(catalog_languages::source.eq(source)))
            .execute(conn)
            .map_err(|e| format!("Failed to delete languages from source {}: {}", source, e))?;

        info!("Removed {} languages from source: {}", deleted, source);
        Ok(deleted)
    }

    /// Helper function to check if a filename matches any of the given patterns
    fn matches_pattern(filename: &str, patterns: &[&str]) -> bool {
        patterns.iter().any(|pattern| filename.starts_with(pattern))
    }
}