use diesel::prelude::*;
use tracing::{debug, info};
use crate::error::Result;
use crate::models::catalog::{BackgroundFilters, BackgroundSummary, CatalogBackground, NewCatalogBackground, BackgroundData};
use crate::schema::catalog_backgrounds;
use std::fs;
use std::path::Path;

pub struct BackgroundService;

impl BackgroundService {
    pub fn search_backgrounds(
        conn: &mut SqliteConnection,
        filters: BackgroundFilters,
    ) -> Result<Vec<BackgroundSummary>> {
        debug!("Searching backgrounds with filters: {:?}", filters);

        let mut query = catalog_backgrounds::table.into_boxed();

        // Apply search pattern filter
        if let Some(search_pattern) = filters.search_pattern.clone() {
            if !search_pattern.is_empty() {
                let pattern = format!("%{}%", search_pattern.to_lowercase());
                query = query.filter(
                    catalog_backgrounds::name.like(pattern.clone())
                        .or(catalog_backgrounds::skills.like(pattern.clone()))
                        .or(catalog_backgrounds::tools.like(pattern.clone()))
                        .or(catalog_backgrounds::feature.like(pattern))
                );
            }
        }

        // Apply source filter
        if let Some(sources) = filters.sources {
            if !sources.is_empty() {
                query = query.filter(catalog_backgrounds::source.eq_any(sources));
            }
        }

        // Apply has_tools filter
        if let Some(has_tools) = filters.has_tools {
            if has_tools {
                query = query.filter(catalog_backgrounds::tools.ne("None"));
            } else {
                query = query.filter(catalog_backgrounds::tools.eq("None"));
            }
        }

        let backgrounds = query
            .select(CatalogBackground::as_select())
            .load::<CatalogBackground>(conn)?;

        Ok(backgrounds.iter().map(BackgroundSummary::from).collect())
    }

    pub fn get_background_by_name_and_source(
        conn: &mut SqliteConnection,
        name: &str,
        source: &str,
    ) -> Result<CatalogBackground> {

        catalog_backgrounds::table
            .filter(
                catalog_backgrounds::name.eq(name)
                    .and(catalog_backgrounds::source.eq(source))
            )
            .select(CatalogBackground::as_select())
            .first::<CatalogBackground>(conn)
            .map_err(Into::into)
    }

    pub fn get_background_sources(conn: &mut SqliteConnection) -> Result<Vec<String>> {
        catalog_backgrounds::table
            .select(catalog_backgrounds::source)
            .distinct()
            .order(catalog_backgrounds::source.asc())
            .load::<String>(conn)
            .map_err(Into::into)
    }

    pub fn get_background_count(conn: &mut SqliteConnection) -> Result<i64> {
        catalog_backgrounds::table
            .count()
            .get_result::<i64>(conn)
            .map_err(Into::into)
    }

    /// Import all background data from an uploaded book directory
    pub fn import_backgrounds_from_book(
        conn: &mut SqliteConnection,
        book_dir: &Path,
        source: &str
    ) -> Result<usize> {
        info!("Importing backgrounds from book directory: {:?} (source: {})", book_dir, source);
        let mut imported_count = 0;

        let backgrounds_dir = book_dir.join("backgrounds");
        if !backgrounds_dir.exists() || !backgrounds_dir.is_dir() {
            debug!("No backgrounds directory found in book: {:?}", book_dir);
            return Ok(0);
        }

        info!("Found backgrounds directory: {:?}", backgrounds_dir);

        // Read all JSON files in the backgrounds directory
        let entries = fs::read_dir(&backgrounds_dir)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if !path.is_file() || path.extension().and_then(|s| s.to_str()) != Some("json") {
                continue;
            }

            debug!("Processing background file: {:?}", path);

            let content = fs::read_to_string(&path)?;
            let background_data: BackgroundData = serde_json::from_str(&content)?;

            if let Some(backgrounds) = background_data.background {
                let new_backgrounds: Vec<NewCatalogBackground> = backgrounds
                    .into_iter()
                    .map(|mut background| {
                        background.source = source.to_string();
                        NewCatalogBackground::from(&background)
                    })
                    .collect();

                if !new_backgrounds.is_empty() {
                    let inserted = diesel::insert_into(catalog_backgrounds::table)
                        .values(&new_backgrounds)
                        .execute(conn)?;

                    imported_count += inserted;
                    info!("Imported {} backgrounds from {:?}", inserted, path);
                }
            }
        }

        info!("Successfully imported {} backgrounds from source: {}", imported_count, source);
        Ok(imported_count)
    }

    /// Remove all backgrounds from a specific source
    pub fn remove_backgrounds_by_source(
        conn: &mut SqliteConnection,
        source: &str
    ) -> Result<usize> {
        info!("Removing backgrounds from source: {}", source);

        let deleted = diesel::delete(catalog_backgrounds::table.filter(catalog_backgrounds::source.eq(source)))
            .execute(conn)?;

        info!("Removed {} backgrounds from source: {}", deleted, source);
        Ok(deleted)
    }
}