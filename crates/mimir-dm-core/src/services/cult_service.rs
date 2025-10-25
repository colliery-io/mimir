use crate::models::catalog::cult::{CatalogCult, CultFilters, CultBoonSummary, NewCatalogCult, CultData, BoonData};
use crate::schema::catalog_cults;
use diesel::prelude::*;
use std::fs;
use std::path::Path;
use tracing::{debug, info, error};

pub struct CultService;

impl CultService {
    pub fn search_cults(&self, conn: &mut SqliteConnection, filters: CultFilters) -> QueryResult<Vec<CultBoonSummary>> {
        debug!("Searching cults with filters: {:?}", filters);
        
        let mut query = catalog_cults::table.into_boxed();
        
        // Apply search filter
        if let Some(search) = &filters.name {
            if !search.is_empty() {
                let search_pattern = format!("%{}%", search);
                query = query.filter(catalog_cults::name.like(search_pattern));
            }
        }
        
        // Apply source filter
        if let Some(sources) = &filters.source {
            if !sources.is_empty() {
                query = query.filter(catalog_cults::source.eq_any(sources));
            }
        }
        
        // Apply category filter (cult vs boon)
        if let Some(categories) = &filters.category {
            if !categories.is_empty() {
                query = query.filter(catalog_cults::category.eq_any(categories));
            }
        }
        
        // Apply cult type filter (Diabolical, Demonic, Elder Evil, etc.)
        if let Some(cult_types) = &filters.cult_type {
            if !cult_types.is_empty() {
                query = query.filter(catalog_cults::cult_type.eq_any(cult_types));
            }
        }
        
        let results: Vec<CatalogCult> = query
            .select(CatalogCult::as_select())
            .order(catalog_cults::name.asc())
            .load(conn)?;
        
        // Convert to CultBoonSummary
        let summaries: Vec<CultBoonSummary> = results.iter().map(|cult| CultBoonSummary::from(cult)).collect();
        
        info!("Found {} cults/boons matching filters", summaries.len());
        Ok(summaries)
    }
    
    pub fn get_cult_details(&self, conn: &mut SqliteConnection, name: String, source: String) -> QueryResult<Option<CatalogCult>> {
        debug!("Getting cult details for: {} from {}", name, source);
        
        catalog_cults::table
            .select(CatalogCult::as_select())
            .filter(catalog_cults::name.eq(name))
            .filter(catalog_cults::source.eq(source))
            .first(conn)
            .optional()
    }
    
    pub fn get_cult_sources(&self, conn: &mut SqliteConnection) -> QueryResult<Vec<String>> {
        let sources: Vec<String> = catalog_cults::table
            .select(catalog_cults::source)
            .distinct()
            .order(catalog_cults::source.asc())
            .load(conn)?;
        
        Ok(sources)
    }
    
    pub fn get_cult_count(&self, conn: &mut SqliteConnection) -> QueryResult<i64> {
        catalog_cults::table
            .count()
            .get_result(conn)
    }
    
    pub fn get_cult_types(&self, conn: &mut SqliteConnection) -> QueryResult<Vec<String>> {
        let types: Vec<Option<String>> = catalog_cults::table
            .select(catalog_cults::cult_type)
            .distinct()
            .load(conn)?;
        
        let types: Vec<String> = types
            .into_iter()
            .filter_map(|t| t)
            .collect();
        
        Ok(types)
    }
    
    pub fn get_cult_categories(&self, conn: &mut SqliteConnection) -> QueryResult<Vec<String>> {
        let categories: Vec<String> = catalog_cults::table
            .select(catalog_cults::category)
            .distinct()
            .order(catalog_cults::category.asc())
            .load(conn)?;

        Ok(categories)
    }

    /// Import cults from a book directory
    pub fn import_cults_from_book(
        conn: &mut SqliteConnection,
        book_dir: &Path,
        source: &str
    ) -> Result<usize, String> {
        info!("Importing cults from book directory: {:?} (source: {})", book_dir, source);
        let mut total_imported = 0;

        // Import cults
        let cults_dir = book_dir.join("cults");
        if cults_dir.exists() {
            info!("Found cults directory: {:?}", cults_dir);
            let mut cult_entries = fs::read_dir(&cults_dir)
                .map_err(|e| format!("Failed to read cults directory: {}", e))?;

            while let Some(entry) = cult_entries.next() {
                let entry = entry.map_err(|e| format!("Failed to read cult directory entry: {}", e))?;
                let path = entry.path();

                if path.extension().and_then(|e| e.to_str()) == Some("json") {
                    debug!("Processing cult file: {:?}", path.file_name().unwrap_or_default());
                    match Self::import_cults_from_file(conn, &path, source) {
                        Ok(count) => {
                            info!("Imported {} cults from {:?}", count, path);
                            total_imported += count;
                        }
                        Err(e) => {
                            error!("Failed to import cults from {:?}: {}", path, e);
                            return Err(e);
                        }
                    }
                }
            }
        }

        // Import boons from dedicated boons directory
        let boons_dir = book_dir.join("boons");
        if boons_dir.exists() {
            info!("Found boons directory: {:?}", boons_dir);
            let mut boon_entries = fs::read_dir(&boons_dir)
                .map_err(|e| format!("Failed to read boons directory: {}", e))?;

            while let Some(entry) = boon_entries.next() {
                let entry = entry.map_err(|e| format!("Failed to read boon directory entry: {}", e))?;
                let path = entry.path();

                if path.extension().and_then(|e| e.to_str()) == Some("json") {
                    debug!("Processing boon file: {:?}", path.file_name().unwrap_or_default());
                    match Self::import_boons_from_file(conn, &path, source) {
                        Ok(count) => {
                            info!("Imported {} boons from {:?}", count, path);
                            total_imported += count;
                        }
                        Err(e) => {
                            error!("Failed to import boons from {:?}: {}", path, e);
                            return Err(e);
                        }
                    }
                }
            }
        }


        info!("Successfully imported {} total cults/boons from {}", total_imported, source);
        Ok(total_imported)
    }

    fn import_cults_from_file(
        conn: &mut SqliteConnection,
        file_path: &Path,
        source: &str
    ) -> Result<usize, String> {
        debug!("Reading cult file: {:?}", file_path);

        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read cult file: {}", e))?;

        let cult_data: CultData = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse cult JSON: {}", e))?;

        if let Some(cults) = cult_data.cult {
            let new_cults: Vec<NewCatalogCult> = cults.iter().map(|cult| {
                let mut new_cult = NewCatalogCult::from(cult);
                if new_cult.source.is_empty() {
                    new_cult.source = source.to_string();
                }
                new_cult
            }).collect();

            debug!("Inserting {} cults individually (SQLite limitation)", new_cults.len());

            for cult in &new_cults {
                diesel::insert_into(catalog_cults::table)
                    .values(cult)
                    .on_conflict((catalog_cults::name, catalog_cults::source))
                    .do_nothing()
                    .execute(conn)
                    .map_err(|e| format!("Failed to insert cult: {}", e))?;
            }

            info!("Successfully imported {} cults into database", new_cults.len());
            Ok(new_cults.len())
        } else {
            Ok(0)
        }
    }

    fn import_boons_from_file(
        conn: &mut SqliteConnection,
        file_path: &Path,
        source: &str
    ) -> Result<usize, String> {
        debug!("Reading boon file: {:?}", file_path);

        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read boon file: {}", e))?;

        let boon_data: BoonData = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse boon JSON: {}", e))?;

        if let Some(boons) = boon_data.boon {
            let new_boons: Vec<NewCatalogCult> = boons.iter().map(|boon| {
                let mut new_boon = NewCatalogCult::from(boon);
                if new_boon.source.is_empty() {
                    new_boon.source = source.to_string();
                }
                new_boon
            }).collect();

            debug!("Inserting {} boons individually (SQLite limitation)", new_boons.len());

            for boon in &new_boons {
                diesel::insert_into(catalog_cults::table)
                    .values(boon)
                    .on_conflict((catalog_cults::name, catalog_cults::source))
                    .do_nothing()
                    .execute(conn)
                    .map_err(|e| format!("Failed to insert boon: {}", e))?;
            }

            info!("Successfully imported {} boons into database", new_boons.len());
            Ok(new_boons.len())
        } else {
            Ok(0)
        }
    }

    /// Remove all cults/boons from a specific source
    pub fn remove_cults_from_source(
        conn: &mut SqliteConnection,
        source: &str
    ) -> Result<usize, String> {
        info!("Removing cults/boons from source: {}", source);

        let deleted = diesel::delete(catalog_cults::table.filter(catalog_cults::source.eq(source)))
            .execute(conn)
            .map_err(|e| format!("Failed to delete cults/boons from source {}: {}", source, e))?;

        info!("Removed {} cults/boons from source: {}", deleted, source);
        Ok(deleted)
    }
}