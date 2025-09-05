use diesel::prelude::*;
use tracing::{debug, error};
use crate::models::catalog::{BackgroundFilters, BackgroundSummary, CatalogBackground};
use crate::schema::catalog_backgrounds;

pub struct BackgroundService;

impl BackgroundService {
    pub fn search_backgrounds(
        conn: &mut SqliteConnection,
        filters: BackgroundFilters,
    ) -> Result<Vec<BackgroundSummary>, String> {
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
            .load::<CatalogBackground>(conn)
            .map_err(|e| {
                error!("Database background search failed: {}", e);
                format!("Database background search failed: {}", e)
            })?;

        Ok(backgrounds.iter().map(BackgroundSummary::from).collect())
    }

    pub fn get_background_by_name_and_source(
        conn: &mut SqliteConnection,
        name: &str,
        source: &str,
    ) -> Result<CatalogBackground, String> {

        catalog_backgrounds::table
            .filter(
                catalog_backgrounds::name.eq(name)
                    .and(catalog_backgrounds::source.eq(source))
            )
            .select(CatalogBackground::as_select())
            .first::<CatalogBackground>(conn)
            .map_err(|e| {
                error!("Background not found: {} from {}: {}", name, source, e);
                format!("Background not found: {} from {}", name, source)
            })
    }

    pub fn get_background_sources(conn: &mut SqliteConnection) -> Result<Vec<String>, String> {

        catalog_backgrounds::table
            .select(catalog_backgrounds::source)
            .distinct()
            .order(catalog_backgrounds::source.asc())
            .load::<String>(conn)
            .map_err(|e| {
                error!("Failed to get background sources: {}", e);
                format!("Failed to get background sources: {}", e)
            })
    }

    pub fn get_background_count(conn: &mut SqliteConnection) -> Result<i64, String> {
        catalog_backgrounds::table
            .count()
            .get_result::<i64>(conn)
            .map_err(|e| {
                error!("Failed to get background count: {}", e);
                format!("Failed to get background count: {}", e)
            })
    }
}