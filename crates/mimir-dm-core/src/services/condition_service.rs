//! Database-backed condition service

use crate::models::catalog::{CatalogCondition, ConditionFilters, ConditionSummary, Condition, Disease, ConditionOrDisease, NewCatalogCondition, ConditionData, DiseaseData};
use crate::schema::catalog_conditions;
use diesel::prelude::*;
use std::fs;
use std::path::Path;
use tracing::{debug, error, info};

pub struct ConditionService;

impl ConditionService {
    /// Search conditions and diseases using database with filters
    pub fn search_conditions(
        conn: &mut SqliteConnection, 
        filters: ConditionFilters
    ) -> Result<Vec<ConditionSummary>, String> {
        debug!("Searching conditions with filters: {:?}", filters);
        
        let mut query = catalog_conditions::table.into_boxed();
        
        // Apply name filter
        if let Some(name) = &filters.name {
            if !name.is_empty() {
                let search_term = format!("%{}%", name.to_lowercase());
                query = query.filter(catalog_conditions::name.like(search_term));
            }
        }
        
        // Apply general search filter (searches name and description)
        if let Some(search) = &filters.search {
            if !search.is_empty() {
                let search_term = format!("%{}%", search.to_lowercase());
                query = query.filter(
                    catalog_conditions::name.like(search_term.clone())
                    .or(catalog_conditions::description.like(search_term))
                );
            }
        }
        
        // Apply item type filter (Condition, Disease)
        if let Some(item_types) = &filters.item_types {
            if !item_types.is_empty() {
                query = query.filter(catalog_conditions::item_type.eq_any(item_types));
            }
        }
        
        // Apply source filter
        if let Some(sources) = &filters.sources {
            if !sources.is_empty() {
                query = query.filter(catalog_conditions::source.eq_any(sources));
            }
        }
        
        let results: Vec<CatalogCondition> = query
            .order(catalog_conditions::name)
            .load(conn)
            .map_err(|e| format!("Database error: {}", e))?;
        
        let summaries: Vec<ConditionSummary> = results
            .into_iter()
            .map(ConditionSummary::from)
            .collect();
        
        info!("Found {} conditions matching filters", summaries.len());
        Ok(summaries)
    }
    
    /// Get a specific condition or disease by ID for modal display
    pub fn get_condition_by_id(
        conn: &mut SqliteConnection, 
        condition_id: i32
    ) -> Result<Option<ConditionOrDisease>, String> {
        debug!("Getting condition by ID: {}", condition_id);
        
        let catalog_condition: Option<CatalogCondition> = catalog_conditions::table
            .find(condition_id)
            .first(conn)
            .optional()
            .map_err(|e| format!("Database error: {}", e))?;
        
        match catalog_condition {
            Some(condition) => {
                // Parse the full JSON back to the original type
                match condition.item_type.as_str() {
                    "Condition" => {
                        let parsed: Condition = serde_json::from_str(&condition.full_condition_json)
                            .map_err(|e| format!("Failed to parse condition JSON: {}", e))?;
                        Ok(Some(ConditionOrDisease::Condition(parsed)))
                    }
                    "Disease" => {
                        let parsed: Disease = serde_json::from_str(&condition.full_condition_json)
                            .map_err(|e| format!("Failed to parse disease JSON: {}", e))?;
                        Ok(Some(ConditionOrDisease::Disease(parsed)))
                    }
                    _ => {
                        error!("Unknown condition type: {}", condition.item_type);
                        Ok(None)
                    }
                }
            }
            None => Ok(None)
        }
    }
    
    /// Get all available item types for filter dropdowns
    pub fn get_item_types(conn: &mut SqliteConnection) -> Result<Vec<String>, String> {
        debug!("Getting condition item types");
        
        let types: Vec<String> = catalog_conditions::table
            .select(catalog_conditions::item_type)
            .distinct()
            .order(catalog_conditions::item_type)
            .load(conn)
            .map_err(|e| format!("Database error: {}", e))?;
        
        debug!("Found {} condition item types", types.len());
        Ok(types)
    }
    
    /// Get all available sources for filter dropdowns
    pub fn get_sources(conn: &mut SqliteConnection) -> Result<Vec<String>, String> {
        debug!("Getting condition sources");
        
        let sources: Vec<String> = catalog_conditions::table
            .select(catalog_conditions::source)
            .distinct()
            .order(catalog_conditions::source)
            .load(conn)
            .map_err(|e| format!("Database error: {}", e))?;
        
        debug!("Found {} condition sources", sources.len());
        Ok(sources)
    }
    
    /// Get condition count for statistics
    pub fn get_condition_count(conn: &mut SqliteConnection) -> Result<i64, String> {
        debug!("Getting condition count");

        let count = catalog_conditions::table
            .count()
            .get_result(conn)
            .map_err(|e| format!("Database error: {}", e))?;

        debug!("Found {} conditions in database", count);
        Ok(count)
    }

    /// Import all condition and disease data from an uploaded book directory
    pub fn import_conditions_from_book(
        conn: &mut SqliteConnection,
        book_dir: &Path,
        source: &str
    ) -> Result<usize, String> {
        info!("Importing conditions/diseases from book directory: {:?} (source: {})", book_dir, source);

        let mut total_imported = 0;

        // Import conditions from conditions/ subdirectory
        if let Ok(count) = Self::import_conditions_from_subdirectory(conn, book_dir, "conditions", source) {
            total_imported += count;
        }

        // Import diseases from diseases/ subdirectory
        if let Ok(count) = Self::import_diseases_from_subdirectory(conn, book_dir, "diseases", source) {
            total_imported += count;
        }

        info!("Successfully imported {} total conditions/diseases from {}", total_imported, source);
        Ok(total_imported)
    }

    /// Import conditions from the conditions subdirectory
    fn import_conditions_from_subdirectory(
        conn: &mut SqliteConnection,
        book_dir: &Path,
        subdir: &str,
        source: &str
    ) -> Result<usize, String> {
        let conditions_dir = book_dir.join(subdir);
        if !conditions_dir.exists() || !conditions_dir.is_dir() {
            debug!("No {} directory found in book: {:?}", subdir, book_dir);
            return Ok(0);
        }

        debug!("Found {} directory in book: {:?}", subdir, book_dir);

        let mut total_imported = 0;

        // Load all JSON files in the conditions directory
        if let Ok(entries) = fs::read_dir(&conditions_dir) {
            for entry in entries.flatten() {
                let condition_file = entry.path();

                if condition_file.extension().and_then(|e| e.to_str()) != Some("json") {
                    continue;
                }

                let filename = condition_file.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown");

                // Skip fluff files for now
                if filename.starts_with("fluff-") {
                    continue;
                }

                match Self::import_conditions_from_file(conn, &condition_file, source) {
                    Ok(count) => {
                        debug!("Imported {} conditions from {:?}", count, condition_file);
                        total_imported += count;
                    }
                    Err(e) => {
                        error!("Failed to import conditions from {:?}: {}", condition_file, e);
                    }
                }
            }
        }

        Ok(total_imported)
    }

    /// Import diseases from the diseases subdirectory
    fn import_diseases_from_subdirectory(
        conn: &mut SqliteConnection,
        book_dir: &Path,
        subdir: &str,
        source: &str
    ) -> Result<usize, String> {
        let diseases_dir = book_dir.join(subdir);
        if !diseases_dir.exists() || !diseases_dir.is_dir() {
            debug!("No {} directory found in book: {:?}", subdir, book_dir);
            return Ok(0);
        }

        debug!("Found {} directory in book: {:?}", subdir, book_dir);

        let mut total_imported = 0;

        // Load all JSON files in the diseases directory
        if let Ok(entries) = fs::read_dir(&diseases_dir) {
            for entry in entries.flatten() {
                let disease_file = entry.path();

                if disease_file.extension().and_then(|e| e.to_str()) != Some("json") {
                    continue;
                }

                match Self::import_diseases_from_file(conn, &disease_file, source) {
                    Ok(count) => {
                        debug!("Imported {} diseases from {:?}", count, disease_file);
                        total_imported += count;
                    }
                    Err(e) => {
                        error!("Failed to import diseases from {:?}: {}", disease_file, e);
                    }
                }
            }
        }

        Ok(total_imported)
    }

    /// Import conditions from a specific JSON file
    fn import_conditions_from_file(
        conn: &mut SqliteConnection,
        file_path: &Path,
        source: &str
    ) -> Result<usize, String> {
        debug!("Reading condition file: {:?}", file_path);

        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read file {:?}: {}", file_path, e))?;

        let condition_data: ConditionData = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse ConditionData from {:?}: {}", file_path, e))?;

        let mut conditions_to_import: Vec<Condition> = Vec::new();

        if let Some(conditions) = condition_data.condition {
            if !conditions.is_empty() {
                debug!("Found {} conditions in file {:?}", conditions.len(), file_path);
                for mut condition in conditions {
                    // Ensure the source is set correctly
                    condition.source = source.to_string();
                    conditions_to_import.push(condition);
                }
            }
        }

        if conditions_to_import.is_empty() {
            debug!("No conditions found in file: {:?}", file_path);
            return Ok(0);
        }

        // Transform conditions to database format
        let catalog_conditions: Vec<NewCatalogCondition> = conditions_to_import
            .into_iter()
            .map(NewCatalogCondition::from)
            .collect();

        // Batch insert conditions
        Self::batch_insert_conditions(conn, catalog_conditions)
    }

    /// Import diseases from a specific JSON file
    fn import_diseases_from_file(
        conn: &mut SqliteConnection,
        file_path: &Path,
        source: &str
    ) -> Result<usize, String> {
        debug!("Reading disease file: {:?}", file_path);

        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read file {:?}: {}", file_path, e))?;

        let disease_data: DiseaseData = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse DiseaseData from {:?}: {}", file_path, e))?;

        let mut diseases_to_import: Vec<Disease> = Vec::new();

        if let Some(diseases) = disease_data.disease {
            if !diseases.is_empty() {
                debug!("Found {} diseases in file {:?}", diseases.len(), file_path);
                for mut disease in diseases {
                    // Ensure the source is set correctly
                    disease.source = source.to_string();
                    diseases_to_import.push(disease);
                }
            }
        }

        if diseases_to_import.is_empty() {
            debug!("No diseases found in file: {:?}", file_path);
            return Ok(0);
        }

        // Transform diseases to database format
        let catalog_conditions: Vec<NewCatalogCondition> = diseases_to_import
            .into_iter()
            .map(NewCatalogCondition::from)
            .collect();

        // Batch insert diseases
        Self::batch_insert_conditions(conn, catalog_conditions)
    }

    /// Batch insert conditions/diseases into the database
    fn batch_insert_conditions(
        conn: &mut SqliteConnection,
        conditions: Vec<NewCatalogCondition>,
    ) -> Result<usize, String> {
        let mut total_inserted = 0;

        debug!("Inserting {} conditions individually (SQLite limitation)", conditions.len());

        for condition in conditions {
            let inserted = diesel::insert_into(catalog_conditions::table)
                .values(&condition)
                .on_conflict((catalog_conditions::name, catalog_conditions::source))
                .do_update()
                .set((
                    catalog_conditions::item_type.eq(&condition.item_type),
                    catalog_conditions::description.eq(&condition.description),
                    catalog_conditions::full_condition_json.eq(&condition.full_condition_json),
                ))
                .execute(conn)
                .map_err(|e| format!("Failed to insert condition '{}': {}", condition.name, e))?;

            total_inserted += inserted;
        }

        info!("Successfully imported {} conditions into database", total_inserted);
        Ok(total_inserted)
    }

    /// Remove all conditions from a specific source (for book removal)
    pub fn remove_conditions_by_source(
        conn: &mut SqliteConnection,
        source: &str,
    ) -> Result<usize, String> {
        info!("Removing conditions from source: {}", source);

        let deleted = diesel::delete(catalog_conditions::table.filter(catalog_conditions::source.eq(source)))
            .execute(conn)
            .map_err(|e| format!("Failed to delete conditions from source {}: {}", source, e))?;

        info!("Removed {} conditions from source: {}", deleted, source);
        Ok(deleted)
    }
}