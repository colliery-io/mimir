//! Database-backed condition service

use crate::models::catalog::{CatalogCondition, ConditionFilters, ConditionSummary, Condition, Disease, ConditionOrDisease};
use crate::schema::catalog_conditions;
use diesel::prelude::*;
use tracing::{debug, error, info};

pub struct ConditionService;

impl ConditionService {
    /// Search conditions and diseases using database with filters
    pub fn search_conditions(
        conn: &mut SqliteConnection, 
        filters: ConditionFilters
    ) -> Result<Vec<ConditionSummary>, String> {
        debug!("Searching conditions with filters: {:?}", filters);
        
        let mut query = catalog_conditions::table.into_boxed();
        
        // Apply name filter
        if let Some(name) = &filters.name {
            if !name.is_empty() {
                let search_term = format!("%{}%", name.to_lowercase());
                query = query.filter(catalog_conditions::name.like(search_term));
            }
        }
        
        // Apply general search filter (searches name and description)
        if let Some(search) = &filters.search {
            if !search.is_empty() {
                let search_term = format!("%{}%", search.to_lowercase());
                query = query.filter(
                    catalog_conditions::name.like(search_term.clone())
                    .or(catalog_conditions::description.like(search_term))
                );
            }
        }
        
        // Apply item type filter (Condition, Disease)
        if let Some(item_types) = &filters.item_types {
            if !item_types.is_empty() {
                query = query.filter(catalog_conditions::item_type.eq_any(item_types));
            }
        }
        
        // Apply source filter
        if let Some(sources) = &filters.sources {
            if !sources.is_empty() {
                query = query.filter(catalog_conditions::source.eq_any(sources));
            }
        }
        
        let results: Vec<CatalogCondition> = query
            .order(catalog_conditions::name)
            .load(conn)
            .map_err(|e| format!("Database error: {}", e))?;
        
        let summaries: Vec<ConditionSummary> = results
            .into_iter()
            .map(ConditionSummary::from)
            .collect();
        
        info!("Found {} conditions matching filters", summaries.len());
        Ok(summaries)
    }
    
    /// Get a specific condition or disease by ID for modal display
    pub fn get_condition_by_id(
        conn: &mut SqliteConnection, 
        condition_id: i32
    ) -> Result<Option<ConditionOrDisease>, String> {
        debug!("Getting condition by ID: {}", condition_id);
        
        let catalog_condition: Option<CatalogCondition> = catalog_conditions::table
            .find(condition_id)
            .first(conn)
            .optional()
            .map_err(|e| format!("Database error: {}", e))?;
        
        match catalog_condition {
            Some(condition) => {
                // Parse the full JSON back to the original type
                match condition.item_type.as_str() {
                    "Condition" => {
                        let parsed: Condition = serde_json::from_str(&condition.full_condition_json)
                            .map_err(|e| format!("Failed to parse condition JSON: {}", e))?;
                        Ok(Some(ConditionOrDisease::Condition(parsed)))
                    }
                    "Disease" => {
                        let parsed: Disease = serde_json::from_str(&condition.full_condition_json)
                            .map_err(|e| format!("Failed to parse disease JSON: {}", e))?;
                        Ok(Some(ConditionOrDisease::Disease(parsed)))
                    }
                    _ => {
                        error!("Unknown condition type: {}", condition.item_type);
                        Ok(None)
                    }
                }
            }
            None => Ok(None)
        }
    }
    
    /// Get all available item types for filter dropdowns
    pub fn get_item_types(conn: &mut SqliteConnection) -> Result<Vec<String>, String> {
        debug!("Getting condition item types");
        
        let types: Vec<String> = catalog_conditions::table
            .select(catalog_conditions::item_type)
            .distinct()
            .order(catalog_conditions::item_type)
            .load(conn)
            .map_err(|e| format!("Database error: {}", e))?;
        
        debug!("Found {} condition item types", types.len());
        Ok(types)
    }
    
    /// Get all available sources for filter dropdowns
    pub fn get_sources(conn: &mut SqliteConnection) -> Result<Vec<String>, String> {
        debug!("Getting condition sources");
        
        let sources: Vec<String> = catalog_conditions::table
            .select(catalog_conditions::source)
            .distinct()
            .order(catalog_conditions::source)
            .load(conn)
            .map_err(|e| format!("Database error: {}", e))?;
        
        debug!("Found {} condition sources", sources.len());
        Ok(sources)
    }
    
    /// Get condition count for statistics
    pub fn get_condition_count(conn: &mut SqliteConnection) -> Result<i64, String> {
        debug!("Getting condition count");
        
        let count = catalog_conditions::table
            .count()
            .get_result(conn)
            .map_err(|e| format!("Database error: {}", e))?;
        
        debug!("Found {} conditions in database", count);
        Ok(count)
    }
}