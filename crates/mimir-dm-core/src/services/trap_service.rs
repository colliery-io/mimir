use crate::models::catalog::{CatalogTrap, TrapFilters, TrapSummary};
use crate::schema::catalog_traps;
use diesel::prelude::*;
use tracing::{debug, info};

pub struct TrapService;

impl TrapService {
    pub fn search_traps(&self, conn: &mut SqliteConnection, filters: TrapFilters) -> QueryResult<Vec<TrapSummary>> {
        debug!("Searching traps with filters: {:?}", filters);
        
        let mut query = catalog_traps::table.into_boxed();
        
        // Apply search filter
        if let Some(search) = &filters.search {
            if !search.is_empty() {
                let search_pattern = format!("%{}%", search);
                query = query.filter(catalog_traps::name.like(search_pattern));
            }
        }
        
        // Apply source filter
        if let Some(sources) = &filters.sources {
            if !sources.is_empty() {
                query = query.filter(catalog_traps::source.eq_any(sources));
            }
        }
        
        // Apply category filter (Trap vs Hazard)
        if let Some(categories) = &filters.categories {
            if !categories.is_empty() {
                query = query.filter(catalog_traps::category.eq_any(categories));
            }
        }
        
        // Apply trap type filter
        if let Some(trap_types) = &filters.trap_types {
            if !trap_types.is_empty() {
                query = query.filter(catalog_traps::trap_type.eq_any(trap_types));
            }
        }
        
        let results: Vec<CatalogTrap> = query
            .order(catalog_traps::name.asc())
            .load(conn)?;
        
        // Convert to TrapSummary
        let summaries: Vec<TrapSummary> = results.iter().map(|trap| TrapSummary {
            name: trap.name.clone(),
            source: trap.source.clone(),
            trap_type: trap.trap_type.clone().unwrap_or_else(|| "Unknown".to_string()),
            category: trap.category.clone(),
        }).collect();
        
        info!("Found {} traps matching filters", summaries.len());
        Ok(summaries)
    }
    
    pub fn get_trap_details(&self, conn: &mut SqliteConnection, name: String, source: String) -> QueryResult<Option<CatalogTrap>> {
        debug!("Getting trap details for: {} from {}", name, source);
        
        catalog_traps::table
            .filter(catalog_traps::name.eq(name))
            .filter(catalog_traps::source.eq(source))
            .first(conn)
            .optional()
    }
    
    pub fn get_trap_sources(&self, conn: &mut SqliteConnection) -> QueryResult<Vec<String>> {
        let sources: Vec<String> = catalog_traps::table
            .select(catalog_traps::source)
            .distinct()
            .order(catalog_traps::source.asc())
            .load(conn)?;
        
        Ok(sources)
    }
    
    pub fn get_trap_count(&self, conn: &mut SqliteConnection) -> QueryResult<i64> {
        catalog_traps::table
            .count()
            .get_result(conn)
    }
    
    pub fn get_trap_types(&self, conn: &mut SqliteConnection) -> QueryResult<Vec<String>> {
        let types: Vec<Option<String>> = catalog_traps::table
            .select(catalog_traps::trap_type)
            .distinct()
            .load(conn)?;
        
        let types: Vec<String> = types
            .into_iter()
            .filter_map(|t| t)
            .collect();
        
        Ok(types)
    }
    
    pub fn get_trap_categories(&self, conn: &mut SqliteConnection) -> QueryResult<Vec<String>> {
        let categories: Vec<String> = catalog_traps::table
            .select(catalog_traps::category)
            .distinct()
            .order(catalog_traps::category.asc())
            .load(conn)?;
        
        Ok(categories)
    }
}