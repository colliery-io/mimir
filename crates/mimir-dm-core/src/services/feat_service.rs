use diesel::prelude::*;
use tracing::{debug, error, info};
use crate::models::catalog::{FeatFilters, FeatSummary, CatalogFeat, NewCatalogFeat, FeatData};
use crate::schema::catalog_feats;
use std::fs;
use std::path::Path;

pub struct FeatService;

impl FeatService {
    pub fn search_feats(
        conn: &mut SqliteConnection,
        filters: FeatFilters,
    ) -> Result<Vec<FeatSummary>, String> {
        debug!("Searching feats with filters: {:?}", filters);

        let mut query = catalog_feats::table.into_boxed();

        // Apply search pattern filter
        if let Some(search_pattern) = filters.search_pattern.clone() {
            if !search_pattern.is_empty() {
                let pattern = format!("%{}%", search_pattern.to_lowercase());
                query = query.filter(
                    catalog_feats::name.like(pattern.clone())
                        .or(catalog_feats::prerequisites.like(pattern.clone()))
                        .or(catalog_feats::brief.like(pattern))
                );
            }
        }

        // Apply source filter
        if let Some(sources) = filters.sources {
            if !sources.is_empty() {
                query = query.filter(catalog_feats::source.eq_any(sources));
            }
        }


        // Apply has_prerequisites filter
        if let Some(has_prerequisites) = filters.has_prerequisites {
            if has_prerequisites {
                query = query.filter(catalog_feats::prerequisites.is_not_null());
            } else {
                query = query.filter(catalog_feats::prerequisites.is_null());
            }
        }

        let feats = query
            .select(CatalogFeat::as_select())
            .load::<CatalogFeat>(conn)
            .map_err(|e| {
                error!("Database feat search failed: {}", e);
                format!("Database feat search failed: {}", e)
            })?;

        Ok(feats.iter().map(FeatSummary::from).collect())
    }

    pub fn get_feat_by_name_and_source(
        conn: &mut SqliteConnection,
        name: &str,
        source: &str,
    ) -> Result<CatalogFeat, String> {
        catalog_feats::table
            .filter(
                catalog_feats::name.eq(name)
                    .and(catalog_feats::source.eq(source))
            )
            .select(CatalogFeat::as_select())
            .first::<CatalogFeat>(conn)
            .map_err(|e| {
                error!("Feat not found: {} from {}: {}", name, source, e);
                format!("Feat not found: {} from {}", name, source)
            })
    }

    pub fn get_feat_sources(conn: &mut SqliteConnection) -> Result<Vec<String>, String> {
        catalog_feats::table
            .select(catalog_feats::source)
            .distinct()
            .order(catalog_feats::source.asc())
            .load::<String>(conn)
            .map_err(|e| {
                error!("Failed to get feat sources: {}", e);
                format!("Failed to get feat sources: {}", e)
            })
    }

    pub fn get_feat_count(conn: &mut SqliteConnection) -> Result<i64, String> {
        catalog_feats::table
            .count()
            .get_result::<i64>(conn)
            .map_err(|e| {
                error!("Failed to get feat count: {}", e);
                format!("Failed to get feat count: {}", e)
            })
    }

    /// Import all feat data from an uploaded book directory
    pub fn import_feats_from_book(
        conn: &mut SqliteConnection,
        book_dir: &Path,
        source: &str
    ) -> Result<usize, String> {
        info!("Importing feats from book directory: {:?} (source: {})", book_dir, source);
        let mut imported_count = 0;

        let feats_dir = book_dir.join("feats");
        if !feats_dir.exists() || !feats_dir.is_dir() {
            debug!("No feats directory found in book: {:?}", book_dir);
            return Ok(0);
        }

        info!("Found feats directory: {:?}", feats_dir);

        // Read all JSON files in the feats directory
        let entries = fs::read_dir(&feats_dir)
            .map_err(|e| format!("Failed to read feats directory: {}", e))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
            let path = entry.path();

            if !path.is_file() || path.extension().and_then(|s| s.to_str()) != Some("json") {
                continue;
            }

            debug!("Processing feat file: {:?}", path);

            let content = fs::read_to_string(&path)
                .map_err(|e| format!("Failed to read feat file {:?}: {}", path, e))?;

            let feat_data: FeatData = serde_json::from_str(&content)
                .map_err(|e| format!("Failed to parse feat data from {:?}: {}", path, e))?;

            if let Some(feats) = feat_data.feat {
                let new_feats: Vec<NewCatalogFeat> = feats
                    .into_iter()
                    .map(|mut feat| {
                        feat.source = source.to_string();
                        NewCatalogFeat::from(&feat)
                    })
                    .collect();

                if !new_feats.is_empty() {
                    let inserted = diesel::insert_into(catalog_feats::table)
                        .values(&new_feats)
                        .execute(conn)
                        .map_err(|e| format!("Failed to insert feats: {}", e))?;

                    imported_count += inserted;
                    info!("Imported {} feats from {:?}", inserted, path);
                }
            }
        }

        info!("Successfully imported {} feats from source: {}", imported_count, source);
        Ok(imported_count)
    }

    /// Remove all feats from a specific source
    pub fn remove_feats_by_source(
        conn: &mut SqliteConnection,
        source: &str
    ) -> Result<usize, String> {
        info!("Removing feats from source: {}", source);

        let deleted = diesel::delete(catalog_feats::table)
            .filter(catalog_feats::source.eq(source))
            .execute(conn)
            .map_err(|e| format!("Failed to delete feats: {}", e))?;

        info!("Removed {} feats from source: {}", deleted, source);
        Ok(deleted)
    }
}