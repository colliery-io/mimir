use diesel::prelude::*;
use crate::models::catalog::monster::{
    CatalogMonster, MonsterSummary, MonsterFilters, Monster, MonsterFluff
};

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
}