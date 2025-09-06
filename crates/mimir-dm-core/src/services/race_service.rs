use diesel::prelude::*;
use tracing::{debug, error};
use crate::models::catalog::{RaceFilters, RaceSummary, CatalogRace};
use crate::schema::catalog_races;

pub struct RaceService;

impl RaceService {
    pub fn search_races(
        conn: &mut SqliteConnection,
        filters: RaceFilters,
    ) -> Result<Vec<RaceSummary>, String> {
        debug!("Searching races with filters: {:?}", filters);

        let mut query = catalog_races::table.into_boxed();

        // Apply search pattern filter
        if let Some(search_pattern) = filters.search_pattern.clone() {
            if !search_pattern.is_empty() {
                let pattern = format!("%{}%", search_pattern.to_lowercase());
                query = query.filter(
                    catalog_races::name.like(pattern.clone())
                        .or(catalog_races::ability_bonuses.like(pattern.clone()))
                        .or(catalog_races::size.like(pattern))
                );
            }
        }

        // Apply source filter
        if let Some(sources) = filters.sources {
            if !sources.is_empty() {
                query = query.filter(catalog_races::source.eq_any(sources));
            }
        }

        // Apply size filter
        if let Some(sizes) = filters.sizes {
            if !sizes.is_empty() {
                query = query.filter(catalog_races::size.eq_any(sizes));
            }
        }

        // Apply darkvision filter (check JSON for darkvision field)
        if let Some(has_darkvision) = filters.has_darkvision {
            if has_darkvision {
                query = query.filter(catalog_races::full_race_json.like("%\"darkvision\":%"));
            } else {
                query = query.filter(catalog_races::full_race_json.not_like("%\"darkvision\":%"));
            }
        }

        // Apply flight filter (check JSON for fly speed)
        if let Some(has_flight) = filters.has_flight {
            if has_flight {
                query = query.filter(catalog_races::full_race_json.like("%\"fly\":%"));
            } else {
                query = query.filter(catalog_races::full_race_json.not_like("%\"fly\":%"));
            }
        }

        let races = query
            .order_by(catalog_races::name.asc())
            .select(CatalogRace::as_select())
            .load::<CatalogRace>(conn)
            .map_err(|e| {
                error!("Failed to search races: {}", e);
                format!("Database error: {}", e)
            })?;

        debug!("Found {} races", races.len());

        let summaries: Vec<RaceSummary> = races
            .iter()
            .map(|r| RaceSummary::from(r))
            .collect();

        Ok(summaries)
    }

    pub fn get_race_details(
        conn: &mut SqliteConnection,
        name: &str,
        source: &str,
    ) -> Result<Option<String>, String> {
        debug!("Getting race details for: {} ({})", name, source);

        let result = catalog_races::table
            .filter(catalog_races::name.eq(name))
            .filter(catalog_races::source.eq(source))
            .select(catalog_races::full_race_json)
            .first::<String>(conn)
            .optional()
            .map_err(|e| {
                error!("Failed to get race details: {}", e);
                format!("Database error: {}", e)
            })?;

        Ok(result)
    }

    pub fn get_race_sources(conn: &mut SqliteConnection) -> Result<Vec<String>, String> {
        debug!("Getting distinct race sources");

        let sources = catalog_races::table
            .select(catalog_races::source)
            .distinct()
            .order_by(catalog_races::source.asc())
            .load::<String>(conn)
            .map_err(|e| {
                error!("Failed to get race sources: {}", e);
                format!("Database error: {}", e)
            })?;

        debug!("Found {} race sources", sources.len());
        Ok(sources)
    }

    pub fn get_race_count(conn: &mut SqliteConnection) -> Result<i64, String> {
        debug!("Getting total race count");

        let count = catalog_races::table
            .count()
            .get_result(conn)
            .map_err(|e| {
                error!("Failed to get race count: {}", e);
                format!("Database error: {}", e)
            })?;

        debug!("Total races: {}", count);
        Ok(count)
    }

    pub fn get_race_sizes(conn: &mut SqliteConnection) -> Result<Vec<String>, String> {
        debug!("Getting distinct race sizes");

        let sizes: Vec<String> = catalog_races::table
            .select(catalog_races::size)
            .distinct()
            .filter(catalog_races::size.is_not_null())
            .order_by(catalog_races::size.asc())
            .load::<Option<String>>(conn)
            .map_err(|e| {
                error!("Failed to get race sizes: {}", e);
                format!("Database error: {}", e)
            })?
            .into_iter()
            .filter_map(|s| s)
            .collect();

        debug!("Found {} distinct race sizes", sizes.len());
        Ok(sizes)
    }
}