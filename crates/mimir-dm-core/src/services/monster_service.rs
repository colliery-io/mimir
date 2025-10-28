use diesel::prelude::*;
use crate::models::catalog::monster::{
    CatalogMonster, MonsterSummary, MonsterFilters, Monster, MonsterFluff,
    NewCatalogMonster, MonsterData, MonsterFluffData
};
use crate::schema::catalog_monsters;
use std::fs;
use std::path::Path;
use tracing::{error, info, warn, debug};

pub struct MonsterService<'a> {
    pub conn: &'a mut SqliteConnection,
}

impl<'a> MonsterService<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }

    /// Search monsters with filters
    pub fn search_monsters(&mut self, filters: MonsterFilters) -> Result<Vec<MonsterSummary>, String> {
        use crate::schema::catalog_monsters::dsl::*;
        
        let mut query = catalog_monsters.into_boxed();
        
        // Filter by name (case-insensitive partial match)
        if let Some(name_filter) = &filters.name {
            if !name_filter.is_empty() {
                query = query.filter(name.like(format!("%{}%", name_filter)));
            }
        }
        
        // Filter by sizes
        if let Some(size_filters) = &filters.sizes {
            if !size_filters.is_empty() {
                query = query.filter(size.eq_any(size_filters));
            }
        }
        
        // Filter by creature types
        if let Some(type_filters) = &filters.creature_types {
            if !type_filters.is_empty() {
                query = query.filter(creature_type.eq_any(type_filters));
            }
        }
        
        // Filter by alignments
        if let Some(alignment_filters) = &filters.alignments {
            if !alignment_filters.is_empty() {
                query = query.filter(alignment.eq_any(alignment_filters));
            }
        }
        
        // Filter by sources
        if let Some(source_filters) = &filters.sources {
            if !source_filters.is_empty() {
                query = query.filter(source.eq_any(source_filters));
            }
        }
        
        // Filter by CR range
        if let Some(min_cr_filter) = filters.min_cr {
            query = query.filter(cr_numeric.ge(min_cr_filter));
        }
        if let Some(max_cr_filter) = filters.max_cr {
            query = query.filter(cr_numeric.le(max_cr_filter));
        }
        
        // Filter by HP range
        if let Some(min_hp_filter) = filters.min_hp {
            query = query.filter(hp.ge(min_hp_filter));
        }
        if let Some(max_hp_filter) = filters.max_hp {
            query = query.filter(hp.le(max_hp_filter));
        }
        
        let monsters = query
            .limit(1000) // Reasonable limit to prevent memory issues
            .load::<CatalogMonster>(self.conn)
            .map_err(|e| format!("Failed to search monsters: {}", e))?;
        
        Ok(monsters.iter().map(MonsterSummary::from).collect())
    }

    /// Get monster by name and source
    pub fn get_monster_by_name_and_source(&mut self, monster_name: &str, monster_source: &str) -> Result<Option<Monster>, String> {
        use crate::schema::catalog_monsters::dsl::*;
        
        let catalog_monster = catalog_monsters
            .filter(name.eq(monster_name))
            .filter(source.eq(monster_source))
            .first::<CatalogMonster>(self.conn)
            .optional()
            .map_err(|e| format!("Failed to get monster by name and source: {}", e))?;
        
        match catalog_monster {
            Some(monster_record) => {
                let parsed_monster: Monster = serde_json::from_str(&monster_record.full_monster_json)
                    .map_err(|e| format!("Failed to parse monster JSON: {}", e))?;
                
                // If fluff data exists, parse and merge it with the monster
                if let Some(fluff_json_str) = &monster_record.fluff_json {
                    if let Ok(_monster_fluff) = serde_json::from_str::<MonsterFluff>(&fluff_json_str) {
                        // Architectural Decision: Fluff data is stored separately and not merged into Monster struct
                        // Rationale: Keeps the core Monster struct focused on game mechanics, while fluff
                        // (lore, images, etc.) is handled at the formatting/presentation layer.
                        // The fluff_json field in the database ensures the data is preserved and accessible
                        // when needed for display purposes.
                    }
                }
                
                Ok(Some(parsed_monster))
            },
            None => Ok(None),
        }
    }

    /// Get all unique sizes for filtering
    pub fn get_all_sizes(&mut self) -> Result<Vec<String>, String> {
        use crate::schema::catalog_monsters::dsl::*;
        
        let sizes: Vec<Option<String>> = catalog_monsters
            .select(size)
            .distinct()
            .filter(size.is_not_null())
            .load(self.conn)
            .map_err(|e| format!("Failed to get monster sizes: {}", e))?;
        
        let mut unique_sizes: Vec<String> = sizes
            .into_iter()
            .filter_map(|s| s)
            .collect();
            
        unique_sizes.sort();
        Ok(unique_sizes)
    }

    /// Get all unique creature types for filtering
    pub fn get_all_creature_types(&mut self) -> Result<Vec<String>, String> {
        use crate::schema::catalog_monsters::dsl::*;
        
        let types: Vec<Option<String>> = catalog_monsters
            .select(creature_type)
            .distinct()
            .filter(creature_type.is_not_null())
            .load(self.conn)
            .map_err(|e| format!("Failed to get monster creature types: {}", e))?;
        
        let mut unique_types: Vec<String> = types
            .into_iter()
            .filter_map(|t| t)
            .collect();
            
        unique_types.sort();
        Ok(unique_types)
    }

    /// Get all unique alignments for filtering
    pub fn get_all_alignments(&mut self) -> Result<Vec<String>, String> {
        use crate::schema::catalog_monsters::dsl::*;
        
        let alignments: Vec<Option<String>> = catalog_monsters
            .select(alignment)
            .distinct()
            .filter(alignment.is_not_null())
            .load(self.conn)
            .map_err(|e| format!("Failed to get monster alignments: {}", e))?;
        
        let mut unique_alignments: Vec<String> = alignments
            .into_iter()
            .filter_map(|a| a)
            .collect();
            
        unique_alignments.sort();
        Ok(unique_alignments)
    }

    /// Get CR range (min and max) for filtering
    pub fn get_cr_range(&mut self) -> Result<(f64, f64), String> {
        use crate::schema::catalog_monsters::dsl::*;
        use diesel::dsl::{min, max};
        
        let result: Option<(Option<f64>, Option<f64>)> = catalog_monsters
            .select((min(cr_numeric), max(cr_numeric)))
            .first(self.conn)
            .optional()
            .map_err(|e| format!("Failed to get CR range: {}", e))?;
        
        match result {
            Some((Some(min_cr), Some(max_cr))) => Ok((min_cr, max_cr)),
            _ => Ok((0.0, 30.0)), // Default range if no data
        }
    }

    /// Get monster count by source for statistics
    pub fn get_monster_count_by_source(&mut self) -> Result<Vec<(String, i64)>, String> {
        use crate::schema::catalog_monsters::dsl::*;
        use diesel::dsl::count;

        let counts = catalog_monsters
            .group_by(source)
            .select((source, count(id)))
            .load::<(String, i64)>(self.conn)
            .map_err(|e| format!("Failed to get monster counts by source: {}", e))?;

        Ok(counts)
    }

    /// Import all monster data from an uploaded book directory
    pub fn import_monsters_from_book(
        conn: &mut SqliteConnection,
        book_dir: &Path,
        source: &str
    ) -> Result<usize, String> {
        info!("Importing monsters from book directory: {:?} (source: {})", book_dir, source);

        let mut total_imported = 0;
        let monster_files = Self::find_monster_files(book_dir)?;

        if monster_files.is_empty() {
            info!("No monster files found in book directory");
            return Ok(0);
        }

        info!("Found {} monster files to process", monster_files.len());

        for monster_file in monster_files {
            debug!("Processing monster file: {:?}", monster_file);

            match Self::import_monsters_from_file(conn, &monster_file, source) {
                Ok(count) => {
                    info!("Imported {} monsters from {:?}", count, monster_file);
                    total_imported += count;
                }
                Err(e) => {
                    error!("Failed to import monsters from {:?}: {}", monster_file, e);
                    // Continue processing other files instead of failing completely
                }
            }
        }

        info!("Total monsters imported: {}", total_imported);
        Ok(total_imported)
    }

    /// Find monster files in a book directory (bestiary/*.json files)
    fn find_monster_files(book_dir: &Path) -> Result<Vec<std::path::PathBuf>, String> {
        let mut files = Vec::new();

        // Check the bestiary directory
        let bestiary_dir = book_dir.join("bestiary");
        if bestiary_dir.exists() && bestiary_dir.is_dir() {
            let entries = fs::read_dir(&bestiary_dir)
                .map_err(|e| format!("Failed to read bestiary directory: {}", e))?;

            for entry in entries {
                let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
                let path = entry.path();

                if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("json") {
                    let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

                    // Skip fluff files, index files, and foundry files
                    if filename.starts_with("fluff-") || filename == "index.json" || filename == "foundry.json" {
                        continue;
                    }

                    debug!("Found monster file: {:?}", path);
                    files.push(path);
                }
            }
        }

        Ok(files)
    }

    /// Load monster fluff data from corresponding fluff file
    fn load_monster_fluff_data(monster_file_path: &Path) -> Option<std::collections::HashMap<String, MonsterFluff>> {
        // Get the bestiary directory and filename
        let bestiary_dir = monster_file_path.parent()?;
        let filename = monster_file_path.file_name()?.to_str()?;

        // Convert bestiary-*.json to fluff-bestiary-*.json
        if !filename.starts_with("bestiary-") {
            return None;
        }

        let fluff_filename = filename.replace("bestiary-", "fluff-bestiary-");
        let fluff_file = bestiary_dir.join(&fluff_filename);

        if !fluff_file.exists() {
            debug!("No fluff file found at: {:?}", fluff_file);
            return None;
        }

        debug!("Loading fluff data from: {:?}", fluff_file);

        match fs::read_to_string(&fluff_file) {
            Ok(fluff_content) => {
                match serde_json::from_str::<MonsterFluffData>(&fluff_content) {
                    Ok(fluff_data) => {
                        let mut fluff_map = std::collections::HashMap::new();

                        for fluff in fluff_data.monster_fluff {
                            fluff_map.insert(fluff.name.to_lowercase(), fluff);
                        }

                        debug!("Loaded fluff data for {} monsters", fluff_map.len());
                        Some(fluff_map)
                    }
                    Err(e) => {
                        warn!("Failed to parse fluff file {:?}: {}", fluff_file, e);
                        None
                    }
                }
            }
            Err(e) => {
                warn!("Failed to read fluff file {:?}: {}", fluff_file, e);
                None
            }
        }
    }

    /// Import monsters from a single JSON file
    fn import_monsters_from_file(
        conn: &mut SqliteConnection,
        file_path: &Path,
        source: &str
    ) -> Result<usize, String> {
        debug!("Reading monsters from file: {:?}", file_path);

        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read file {:?}: {}", file_path, e))?;

        let data: MonsterData = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse JSON from {:?}: {}", file_path, e))?;

        // Load fluff data if available
        let fluff_data = Self::load_monster_fluff_data(file_path);

        if !data.monster.is_empty() {
            let new_monsters: Vec<NewCatalogMonster> = data.monster.iter().map(|monster| {
                let mut new_monster = NewCatalogMonster::from(monster);
                // Always override the source with the book source to ensure consistency
                new_monster.source = source.to_string();

                // Also update the source in the full_monster_json to maintain consistency
                if let Ok(mut monster_json) = serde_json::from_str::<serde_json::Value>(&new_monster.full_monster_json) {
                    if let Some(obj) = monster_json.as_object_mut() {
                        obj.insert("source".to_string(), serde_json::Value::String(source.to_string()));
                        if let Ok(updated_json) = serde_json::to_string(&monster_json) {
                            new_monster.full_monster_json = updated_json;
                        }
                    }
                }

                // Add fluff data if available for this monster
                if let Some(ref fluff_map) = fluff_data {
                    let monster_name_lower = monster.name.to_lowercase();
                    if let Some(monster_fluff) = fluff_map.get(&monster_name_lower) {
                        if let Ok(fluff_json) = serde_json::to_string(monster_fluff) {
                            new_monster.fluff_json = Some(fluff_json);
                        }
                    }
                }

                new_monster
            }).collect();

            debug!("Inserting {} monsters individually (SQLite limitation)", new_monsters.len());

            for monster in &new_monsters {
                diesel::insert_into(catalog_monsters::table)
                    .values(monster)
                    .on_conflict((catalog_monsters::name, catalog_monsters::source))
                    .do_nothing()
                    .execute(conn)
                    .map_err(|e| format!("Failed to insert monster: {}", e))?;
            }

            info!("Successfully imported {} monsters into database", new_monsters.len());
            Ok(new_monsters.len())
        } else {
            Ok(0)
        }
    }

    /// Remove all monsters from a specific source
    pub fn remove_monsters_from_source(
        conn: &mut SqliteConnection,
        source: &str
    ) -> Result<usize, String> {
        info!("Removing monsters from source: {}", source);

        let deleted = diesel::delete(catalog_monsters::table.filter(catalog_monsters::source.eq(source)))
            .execute(conn)
            .map_err(|e| format!("Failed to delete monsters from source {}: {}", source, e))?;

        info!("Removed {} monsters from source: {}", deleted, source);
        Ok(deleted)
    }
}