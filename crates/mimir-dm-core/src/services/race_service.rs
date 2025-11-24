use diesel::prelude::*;
use tracing::{debug, error, info};
use crate::error::Result;
use crate::models::catalog::{RaceFilters, RaceSummary, CatalogRace, NewCatalogRace, RaceData};
use crate::schema::catalog_races;
use std::fs;
use std::path::Path;

pub struct RaceService;

impl RaceService {
    pub fn search_races(
        conn: &mut SqliteConnection,
        filters: RaceFilters,
    ) -> Result<Vec<RaceSummary>> {
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
            .load::<CatalogRace>(conn)?;

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
    ) -> Result<Option<String>> {
        debug!("Getting race details for: {} ({})", name, source);

        let result = catalog_races::table
            .filter(catalog_races::name.eq(name))
            .filter(catalog_races::source.eq(source))
            .select(catalog_races::full_race_json)
            .first::<String>(conn)
            .optional()?;

        Ok(result)
    }

    pub fn get_race_sources(conn: &mut SqliteConnection) -> Result<Vec<String>> {
        debug!("Getting distinct race sources");

        let sources = catalog_races::table
            .select(catalog_races::source)
            .distinct()
            .order_by(catalog_races::source.asc())
            .load::<String>(conn)?;

        debug!("Found {} race sources", sources.len());
        Ok(sources)
    }

    pub fn get_race_count(conn: &mut SqliteConnection) -> Result<i64> {
        debug!("Getting total race count");

        let count = catalog_races::table
            .count()
            .get_result(conn)?;

        debug!("Total races: {}", count);
        Ok(count)
    }

    pub fn get_race_sizes(conn: &mut SqliteConnection) -> Result<Vec<String>> {
        debug!("Getting distinct race sizes");

        let sizes: Vec<String> = catalog_races::table
            .select(catalog_races::size)
            .distinct()
            .filter(catalog_races::size.is_not_null())
            .order_by(catalog_races::size.asc())
            .load::<Option<String>>(conn)?
            .into_iter()
            .filter_map(|s| s)
            .collect();

        debug!("Found {} distinct race sizes", sizes.len());
        Ok(sizes)
    }

    /// Import all race data from an uploaded book directory
    pub fn import_races_from_book(
        conn: &mut SqliteConnection,
        book_dir: &Path,
        source: &str
    ) -> Result<usize> {
        info!("Importing races from book directory: {:?} (source: {})", book_dir, source);
        let mut imported_count = 0;

        let races_dir = book_dir.join("races");
        if !races_dir.exists() || !races_dir.is_dir() {
            debug!("No races directory found in book: {:?}", book_dir);
            return Ok(0);
        }

        info!("Found races directory: {:?}", races_dir);

        // Read all JSON files in the races directory
        let entries = fs::read_dir(&races_dir)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if !path.is_file() || path.extension().and_then(|s| s.to_str()) != Some("json") {
                continue;
            }

            let filename = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");

            // Skip fluff files - we only want race data
            if filename.starts_with("fluff-") {
                debug!("Skipping fluff file: {}", filename);
                continue;
            }

            debug!("Processing race file: {}", filename);

            let content = fs::read_to_string(&path)?;
            let race_data: RaceData = serde_json::from_str(&content)?;

            // Import main races
            if let Some(races) = race_data.race {
                for race in &races {
                    let new_race = NewCatalogRace::from(race);

                    match diesel::insert_into(catalog_races::table)
                        .values(&new_race)
                        .on_conflict((catalog_races::name, catalog_races::source))
                        .do_update()
                        .set((
                            catalog_races::size.eq(&new_race.size),
                            catalog_races::speed.eq(&new_race.speed),
                            catalog_races::ability_bonuses.eq(&new_race.ability_bonuses),
                            catalog_races::traits_count.eq(&new_race.traits_count),
                            catalog_races::full_race_json.eq(&new_race.full_race_json),
                        ))
                        .execute(conn) {
                        Ok(_) => {
                            imported_count += 1;
                            debug!("Imported race: {} ({})", race.name, source);
                        }
                        Err(e) => {
                            error!("Failed to insert race {}: {}", race.name, e);
                        }
                    }
                }
            }

            // Import subraces
            if let Some(subraces) = race_data.subrace {
                for subrace in &subraces {
                    // Skip unnamed subraces
                    if subrace.name.is_none() {
                        continue;
                    }

                    let new_subrace = NewCatalogRace::from(subrace);

                    match diesel::insert_into(catalog_races::table)
                        .values(&new_subrace)
                        .on_conflict((catalog_races::name, catalog_races::source))
                        .do_update()
                        .set((
                            catalog_races::size.eq(&new_subrace.size),
                            catalog_races::speed.eq(&new_subrace.speed),
                            catalog_races::ability_bonuses.eq(&new_subrace.ability_bonuses),
                            catalog_races::traits_count.eq(&new_subrace.traits_count),
                            catalog_races::full_race_json.eq(&new_subrace.full_race_json),
                        ))
                        .execute(conn) {
                        Ok(_) => {
                            imported_count += 1;
                            debug!("Imported subrace: {} ({})", new_subrace.name, source);
                        }
                        Err(e) => {
                            error!("Failed to insert subrace {}: {}", new_subrace.name, e);
                        }
                    }
                }
            }
        }

        info!("Successfully imported {} races/subraces from source: {}", imported_count, source);
        Ok(imported_count)
    }

    /// Remove all races from a specific source
    pub fn remove_races_by_source(
        conn: &mut SqliteConnection,
        source: &str
    ) -> Result<usize> {
        info!("Removing races from source: {}", source);

        let deleted = diesel::delete(catalog_races::table)
            .filter(catalog_races::source.eq(source))
            .execute(conn)?;

        info!("Removed {} races from source: {}", deleted, source);
        Ok(deleted)
    }
}