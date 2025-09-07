use diesel::prelude::*;
use tracing::{debug, error};
use crate::models::catalog::{FeatFilters, FeatSummary, CatalogFeat};
use crate::schema::catalog_feats;

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
}