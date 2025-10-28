use crate::models::catalog::{CatalogTrap, TrapFilters, TrapSummary, NewCatalogTrap, TrapData, HazardData, TrapOrHazard};
use crate::schema::catalog_traps;
use diesel::prelude::*;
use tracing::{debug, info, error, warn};
use std::fs;
use std::path::Path;

pub struct TrapService;

impl TrapService {
    pub fn search_traps(&self, conn: &mut SqliteConnection, filters: TrapFilters) -> QueryResult<Vec<TrapSummary>> {
        debug!("Searching traps with filters: {:?}", filters);
        
        let mut query = catalog_traps::table.into_boxed();
        
        // Apply search filter
        if let Some(search) = &filters.search {
            if !search.is_empty() {
                let search_pattern = format!("%{}%", search);
                query = query.filter(catalog_traps::name.like(search_pattern));
            }
        }
        
        // Apply source filter
        if let Some(sources) = &filters.sources {
            if !sources.is_empty() {
                query = query.filter(catalog_traps::source.eq_any(sources));
            }
        }
        
        // Apply category filter (Trap vs Hazard)
        if let Some(categories) = &filters.categories {
            if !categories.is_empty() {
                query = query.filter(catalog_traps::category.eq_any(categories));
            }
        }
        
        // Apply trap type filter
        if let Some(trap_types) = &filters.trap_types {
            if !trap_types.is_empty() {
                query = query.filter(catalog_traps::trap_type.eq_any(trap_types));
            }
        }
        
        let results: Vec<CatalogTrap> = query
            .order(catalog_traps::name.asc())
            .load(conn)?;
        
        // Convert to TrapSummary
        let summaries: Vec<TrapSummary> = results.iter().map(|trap| TrapSummary {
            name: trap.name.clone(),
            source: trap.source.clone(),
            trap_type: trap.trap_type.clone().unwrap_or_else(|| "Unknown".to_string()),
            category: trap.category.clone(),
        }).collect();
        
        info!("Found {} traps matching filters", summaries.len());
        Ok(summaries)
    }
    
    pub fn get_trap_details(&self, conn: &mut SqliteConnection, name: String, source: String) -> QueryResult<Option<CatalogTrap>> {
        debug!("Getting trap details for: {} from {}", name, source);
        
        catalog_traps::table
            .filter(catalog_traps::name.eq(name))
            .filter(catalog_traps::source.eq(source))
            .first(conn)
            .optional()
    }
    
    pub fn get_trap_sources(&self, conn: &mut SqliteConnection) -> QueryResult<Vec<String>> {
        let sources: Vec<String> = catalog_traps::table
            .select(catalog_traps::source)
            .distinct()
            .order(catalog_traps::source.asc())
            .load(conn)?;
        
        Ok(sources)
    }
    
    pub fn get_trap_count(&self, conn: &mut SqliteConnection) -> QueryResult<i64> {
        catalog_traps::table
            .count()
            .get_result(conn)
    }
    
    pub fn get_trap_types(&self, conn: &mut SqliteConnection) -> QueryResult<Vec<String>> {
        let types: Vec<Option<String>> = catalog_traps::table
            .select(catalog_traps::trap_type)
            .distinct()
            .load(conn)?;
        
        let types: Vec<String> = types
            .into_iter()
            .filter_map(|t| t)
            .collect();
        
        Ok(types)
    }
    
    pub fn get_trap_categories(&self, conn: &mut SqliteConnection) -> QueryResult<Vec<String>> {
        let categories: Vec<String> = catalog_traps::table
            .select(catalog_traps::category)
            .distinct()
            .order(catalog_traps::category.asc())
            .load(conn)?;

        Ok(categories)
    }

    /// Import all trap and hazard data from an uploaded book directory
    pub fn import_traps_from_book(
        conn: &mut SqliteConnection,
        book_dir: &Path,
        source: &str
    ) -> Result<usize, String> {
        info!("Importing traps from book directory: {:?} (source: {})", book_dir, source);

        let mut total_imported = 0;

        // Import traps
        let traps_dir = book_dir.join("traps");
        if traps_dir.exists() && traps_dir.is_dir() {
            info!("Found traps directory: {:?}", traps_dir);
            total_imported += Self::import_traps_from_directory(conn, &traps_dir, source)?;
        } else {
            debug!("No traps directory found in book: {:?}", book_dir);
        }

        // Import hazards
        let hazards_dir = book_dir.join("hazards");
        if hazards_dir.exists() && hazards_dir.is_dir() {
            info!("Found hazards directory: {:?}", hazards_dir);
            total_imported += Self::import_hazards_from_directory(conn, &hazards_dir, source)?;
        } else {
            debug!("No hazards directory found in book: {:?}", book_dir);
        }

        info!("Successfully imported {} total traps/hazards from {}", total_imported, source);
        Ok(total_imported)
    }

    /// Import traps from a directory
    fn import_traps_from_directory(
        conn: &mut SqliteConnection,
        traps_dir: &Path,
        source: &str
    ) -> Result<usize, String> {
        let mut total_imported = 0;

        for entry in fs::read_dir(traps_dir).map_err(|e| format!("Failed to read traps directory: {}", e))? {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let file_path = entry.path();

            if !file_path.is_file() || file_path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }

            debug!("Processing trap file: {:?}", file_path.file_name().unwrap_or_default());

            match Self::import_traps_from_file(conn, &file_path, source) {
                Ok(count) => {
                    info!("Imported {} traps from {:?}", count, file_path);
                    total_imported += count;
                }
                Err(e) => {
                    error!("Failed to import traps from {:?}: {}", file_path, e);
                }
            }
        }

        Ok(total_imported)
    }

    /// Import hazards from a directory
    fn import_hazards_from_directory(
        conn: &mut SqliteConnection,
        hazards_dir: &Path,
        source: &str
    ) -> Result<usize, String> {
        let mut total_imported = 0;

        for entry in fs::read_dir(hazards_dir).map_err(|e| format!("Failed to read hazards directory: {}", e))? {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let file_path = entry.path();

            if !file_path.is_file() || file_path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }

            debug!("Processing hazard file: {:?}", file_path.file_name().unwrap_or_default());

            match Self::import_hazards_from_file(conn, &file_path, source) {
                Ok(count) => {
                    info!("Imported {} hazards from {:?}", count, file_path);
                    total_imported += count;
                }
                Err(e) => {
                    error!("Failed to import hazards from {:?}: {}", file_path, e);
                }
            }
        }

        Ok(total_imported)
    }

    /// Import traps from a single JSON file
    fn import_traps_from_file(
        conn: &mut SqliteConnection,
        file_path: &Path,
        _source: &str
    ) -> Result<usize, String> {
        debug!("Reading trap file: {:?}", file_path);

        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read trap file: {}", e))?;

        let trap_data: TrapData = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse trap JSON: {}", e))?;

        if let Some(traps) = trap_data.trap {
            let traps: Vec<TrapOrHazard> = traps.into_iter().map(TrapOrHazard::Trap).collect();
            let new_traps: Vec<NewCatalogTrap> = traps.iter().map(|trap| NewCatalogTrap::from(trap)).collect();

            debug!("Inserting {} traps individually (SQLite limitation)", new_traps.len());

            for trap in &new_traps {
                let result = diesel::insert_into(catalog_traps::table)
                    .values(trap)
                    .on_conflict((catalog_traps::name, catalog_traps::source))
                    .do_nothing()
                    .execute(conn);

                if let Err(e) = result {
                    warn!("Failed to insert trap {}: {}", trap.name, e);
                }
            }

            info!("Successfully imported {} traps into database", new_traps.len());
            Ok(new_traps.len())
        } else {
            Ok(0)
        }
    }

    /// Import hazards from a single JSON file
    fn import_hazards_from_file(
        conn: &mut SqliteConnection,
        file_path: &Path,
        _source: &str
    ) -> Result<usize, String> {
        debug!("Reading hazard file: {:?}", file_path);

        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read hazard file: {}", e))?;

        let hazard_data: HazardData = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse hazard JSON: {}", e))?;

        if let Some(hazards) = hazard_data.hazard {
            let hazards: Vec<TrapOrHazard> = hazards.into_iter().map(TrapOrHazard::Hazard).collect();
            let new_hazards: Vec<NewCatalogTrap> = hazards.iter().map(|hazard| NewCatalogTrap::from(hazard)).collect();

            debug!("Inserting {} hazards individually (SQLite limitation)", new_hazards.len());

            for hazard in &new_hazards {
                let result = diesel::insert_into(catalog_traps::table)
                    .values(hazard)
                    .on_conflict((catalog_traps::name, catalog_traps::source))
                    .do_nothing()
                    .execute(conn);

                if let Err(e) = result {
                    warn!("Failed to insert hazard {}: {}", hazard.name, e);
                }
            }

            info!("Successfully imported {} hazards into database", new_hazards.len());
            Ok(new_hazards.len())
        } else {
            Ok(0)
        }
    }

    /// Remove all traps from a specific source
    pub fn remove_traps_from_source(
        conn: &mut SqliteConnection,
        source: &str
    ) -> Result<usize, String> {
        info!("Removing traps from source: {}", source);

        let deleted = diesel::delete(catalog_traps::table.filter(catalog_traps::source.eq(source)))
            .execute(conn)
            .map_err(|e| format!("Failed to delete traps from source {}: {}", source, e))?;

        info!("Removed {} traps from source: {}", deleted, source);
        Ok(deleted)
    }
}