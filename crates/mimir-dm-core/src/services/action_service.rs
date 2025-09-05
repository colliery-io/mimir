//! Database service layer for action operations
//!
//! This service provides database-backed action search and retrieval,
//! following the same pattern as the spell service.

use crate::models::catalog::{CatalogAction, ActionFilters, ActionSummary, Action};
use crate::schema::catalog_actions;
use diesel::prelude::*;
use tracing::{debug, info};

pub struct ActionService;

impl ActionService {
    /// Search actions with optional filters
    pub fn search_actions(
        conn: &mut SqliteConnection,
        filters: ActionFilters,
    ) -> Result<Vec<ActionSummary>, String> {
        debug!("Searching actions with filters: {:?}", filters);
        
        let mut query = catalog_actions::table.into_boxed();
        
        // Apply name search filter
        if let Some(name_query) = &filters.name {
            if !name_query.trim().is_empty() {
                query = query.filter(
                    catalog_actions::name.like(format!("%{}%", name_query.trim()))
                );
            }
        }
        
        // Apply general search filter (searches name and description)
        if let Some(search_query) = &filters.search {
            if !search_query.trim().is_empty() {
                let search_term = format!("%{}%", search_query.trim());
                query = query.filter(
                    catalog_actions::name.like(search_term.clone())
                        .or(catalog_actions::description.like(search_term))
                );
            }
        }
        
        // Apply time type filters
        if let Some(time_types) = &filters.time_types {
            if !time_types.is_empty() {
                query = query.filter(catalog_actions::time_type.eq_any(time_types));
            }
        }
        
        // Apply source filters
        if let Some(sources) = &filters.sources {
            if !sources.is_empty() {
                query = query.filter(catalog_actions::source.eq_any(sources));
            }
        }
        
        // Execute query and order by name
        let actions: Vec<CatalogAction> = query
            .order(catalog_actions::name.asc())
            .load(conn)
            .map_err(|e| {
                debug!("Database error searching actions: {}", e);
                format!("Failed to search actions: {}", e)
            })?;

        info!("Found {} actions", actions.len());
        
        // Convert to summaries
        let summaries: Vec<ActionSummary> = actions
            .into_iter()
            .map(ActionSummary::from)
            .collect();
        
        Ok(summaries)
    }

    /// Get a specific action by ID
    pub fn get_action_by_id(
        conn: &mut SqliteConnection,
        action_id: i32,
    ) -> Result<Option<Action>, String> {
        debug!("Getting action by ID: {}", action_id);
        
        let catalog_action: Option<CatalogAction> = catalog_actions::table
            .find(action_id)
            .first(conn)
            .optional()
            .map_err(|e| {
                debug!("Database error getting action: {}", e);
                format!("Failed to get action: {}", e)
            })?;
            
        match catalog_action {
            Some(action) => {
                // Parse the full JSON back to Action struct
                let full_action: Action = serde_json::from_str(&action.full_action_json)
                    .map_err(|e| format!("Failed to parse action JSON: {}", e))?;
                Ok(Some(full_action))
            }
            None => Ok(None),
        }
    }

    /// Get all unique time types for filter options
    pub fn get_time_types(conn: &mut SqliteConnection) -> Result<Vec<String>, String> {
        debug!("Getting unique time types");
        
        let time_types: Vec<String> = catalog_actions::table
            .select(catalog_actions::time_type)
            .distinct()
            .order(catalog_actions::time_type.asc())
            .load(conn)
            .map_err(|e| {
                debug!("Database error getting time types: {}", e);
                format!("Failed to get time types: {}", e)
            })?;
            
        Ok(time_types)
    }

    /// Get all unique sources for filter options
    pub fn get_sources(conn: &mut SqliteConnection) -> Result<Vec<String>, String> {
        debug!("Getting unique sources");
        
        let sources: Vec<String> = catalog_actions::table
            .select(catalog_actions::source)
            .distinct()
            .order(catalog_actions::source.asc())
            .load(conn)
            .map_err(|e| {
                debug!("Database error getting sources: {}", e);
                format!("Failed to get sources: {}", e)
            })?;
            
        Ok(sources)
    }

    /// Get action count for statistics
    pub fn get_action_count(conn: &mut SqliteConnection) -> Result<i64, String> {
        use diesel::dsl::count_star;
        
        let count: i64 = catalog_actions::table
            .select(count_star())
            .first(conn)
            .map_err(|e| {
                debug!("Database error counting actions: {}", e);
                format!("Failed to count actions: {}", e)
            })?;
            
        Ok(count)
    }
}