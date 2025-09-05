//! Database service layer for spell operations
//!
//! This service provides database-backed spell search and retrieval,
//! replacing the in-memory catalog system.

use crate::models::catalog::{CatalogSpell, SpellFilters, SpellSummary, Spell};
use crate::schema::catalog_spells;
use diesel::prelude::*;
use tracing::{debug, info};

pub struct SpellService;

impl SpellService {
    /// Search spells with optional filters
    pub fn search_spells(
        conn: &mut SqliteConnection,
        filters: SpellFilters,
    ) -> Result<Vec<SpellSummary>, String> {
        debug!("Searching spells with filters: {:?}", filters);
        
        let mut query = catalog_spells::table.into_boxed();
        
        // Apply name search filter
        if let Some(name_query) = &filters.query {
            if !name_query.trim().is_empty() {
                query = query.filter(
                    catalog_spells::name.like(format!("%{}%", name_query.trim()))
                );
            }
        }
        
        // Apply level filters
        if !filters.levels.is_empty() {
            query = query.filter(catalog_spells::level.eq_any(&filters.levels));
        }
        
        // Apply school filters
        if !filters.schools.is_empty() {
            query = query.filter(catalog_spells::school.eq_any(&filters.schools));
        }
        
        // Apply source filters
        if !filters.sources.is_empty() {
            query = query.filter(catalog_spells::source.eq_any(&filters.sources));
        }
        
        // Apply tag filters (requires JSON containment check)
        if !filters.tags.is_empty() {
            for tag in &filters.tags {
                // SQLite doesn't have native JSON operators, so we use LIKE
                query = query.filter(catalog_spells::tags.like(format!("%\"{}\"%%", tag)));
            }
        }
        
        // Apply pagination
        if let Some(offset) = filters.offset {
            query = query.offset(offset as i64);
        }
        
        // Apply limit only if explicitly requested
        if let Some(limit) = filters.limit {
            query = query.limit(limit as i64);
        }
        
        // Execute query with explicit select
        let catalog_spells: Vec<CatalogSpell> = query
            .select(CatalogSpell::as_select())
            .load(conn)
            .map_err(|e| format!("Failed to search spells: {}", e))?;
        
        let summaries: Vec<SpellSummary> = catalog_spells
            .iter()
            .map(|spell| spell.to_summary())
            .collect();
        
        info!("Found {} spells matching search criteria", summaries.len());
        Ok(summaries)
    }
    
    /// Get detailed spell information by name and source
    pub fn get_spell_details(
        conn: &mut SqliteConnection,
        name: &str,
        source: &str,
    ) -> Result<Option<Spell>, String> {
        debug!("Getting spell details: {} from {}", name, source);
        
        let catalog_spell: Option<CatalogSpell> = catalog_spells::table
            .filter(catalog_spells::name.eq(name))
            .filter(catalog_spells::source.eq(source))
            .select(CatalogSpell::as_select())
            .first(conn)
            .optional()
            .map_err(|e| format!("Failed to fetch spell details: {}", e))?;
        
        if let Some(spell_record) = catalog_spell {
            // Parse the full JSON spell data
            let spell: Spell = serde_json::from_str(&spell_record.full_spell_json)
                .map_err(|e| format!("Failed to parse spell JSON: {}", e))?;
            
            debug!("Found spell details for: {}", name);
            Ok(Some(spell))
        } else {
            debug!("No spell found with name '{}' from source '{}'", name, source);
            Ok(None)
        }
    }
    
    /// Get unique spell sources for filter dropdown
    pub fn get_spell_sources(conn: &mut SqliteConnection) -> Result<Vec<String>, String> {
        debug!("Getting unique spell sources");
        
        let sources: Vec<String> = catalog_spells::table
            .select(catalog_spells::source)
            .distinct()
            .order(catalog_spells::source)
            .load(conn)
            .map_err(|e| format!("Failed to fetch spell sources: {}", e))?;
        
        debug!("Found {} unique spell sources", sources.len());
        Ok(sources)
    }
    
    /// Get unique spell schools for filter dropdown
    pub fn get_spell_schools(conn: &mut SqliteConnection) -> Result<Vec<String>, String> {
        debug!("Getting unique spell schools");
        
        let schools: Vec<String> = catalog_spells::table
            .select(catalog_spells::school)
            .distinct()
            .order(catalog_spells::school)
            .load(conn)
            .map_err(|e| format!("Failed to fetch spell schools: {}", e))?;
        
        debug!("Found {} unique spell schools", schools.len());
        Ok(schools)
    }
    
    /// Get spell count by source for statistics
    pub fn get_spell_count_by_source(conn: &mut SqliteConnection) -> Result<Vec<(String, i64)>, String> {
        debug!("Getting spell count by source");
        
        use diesel::dsl::count_star;
        
        let counts: Vec<(String, i64)> = catalog_spells::table
            .group_by(catalog_spells::source)
            .select((catalog_spells::source, count_star()))
            .order(catalog_spells::source)
            .load(conn)
            .map_err(|e| format!("Failed to fetch spell counts: {}", e))?;
        
        debug!("Found spell counts for {} sources", counts.len());
        Ok(counts)
    }
    
    /// Get total spell count
    pub fn get_total_spell_count(conn: &mut SqliteConnection) -> Result<i64, String> {
        debug!("Getting total spell count");
        
        use diesel::dsl::count_star;
        
        let count: i64 = catalog_spells::table
            .select(count_star())
            .first(conn)
            .map_err(|e| format!("Failed to count spells: {}", e))?;
        
        debug!("Total spells in database: {}", count);
        Ok(count)
    }
}