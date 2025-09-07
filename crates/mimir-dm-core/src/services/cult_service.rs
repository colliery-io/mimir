use crate::models::catalog::cult::{CatalogCult, CultFilters, CultBoonSummary};
use crate::schema::catalog_cults;
use diesel::prelude::*;
use tracing::{debug, info};

pub struct CultService;

impl CultService {
    pub fn search_cults(&self, conn: &mut SqliteConnection, filters: CultFilters) -> QueryResult<Vec<CultBoonSummary>> {
        debug!("Searching cults with filters: {:?}", filters);
        
        let mut query = catalog_cults::table.into_boxed();
        
        // Apply search filter
        if let Some(search) = &filters.name {
            if !search.is_empty() {
                let search_pattern = format!("%{}%", search);
                query = query.filter(catalog_cults::name.like(search_pattern));
            }
        }
        
        // Apply source filter
        if let Some(sources) = &filters.source {
            if !sources.is_empty() {
                query = query.filter(catalog_cults::source.eq_any(sources));
            }
        }
        
        // Apply category filter (cult vs boon)
        if let Some(categories) = &filters.category {
            if !categories.is_empty() {
                query = query.filter(catalog_cults::category.eq_any(categories));
            }
        }
        
        // Apply cult type filter (Diabolical, Demonic, Elder Evil, etc.)
        if let Some(cult_types) = &filters.cult_type {
            if !cult_types.is_empty() {
                query = query.filter(catalog_cults::cult_type.eq_any(cult_types));
            }
        }
        
        let results: Vec<CatalogCult> = query
            .select(CatalogCult::as_select())
            .order(catalog_cults::name.asc())
            .load(conn)?;
        
        // Convert to CultBoonSummary
        let summaries: Vec<CultBoonSummary> = results.iter().map(|cult| CultBoonSummary::from(cult)).collect();
        
        info!("Found {} cults/boons matching filters", summaries.len());
        Ok(summaries)
    }
    
    pub fn get_cult_details(&self, conn: &mut SqliteConnection, name: String, source: String) -> QueryResult<Option<CatalogCult>> {
        debug!("Getting cult details for: {} from {}", name, source);
        
        catalog_cults::table
            .select(CatalogCult::as_select())
            .filter(catalog_cults::name.eq(name))
            .filter(catalog_cults::source.eq(source))
            .first(conn)
            .optional()
    }
    
    pub fn get_cult_sources(&self, conn: &mut SqliteConnection) -> QueryResult<Vec<String>> {
        let sources: Vec<String> = catalog_cults::table
            .select(catalog_cults::source)
            .distinct()
            .order(catalog_cults::source.asc())
            .load(conn)?;
        
        Ok(sources)
    }
    
    pub fn get_cult_count(&self, conn: &mut SqliteConnection) -> QueryResult<i64> {
        catalog_cults::table
            .count()
            .get_result(conn)
    }
    
    pub fn get_cult_types(&self, conn: &mut SqliteConnection) -> QueryResult<Vec<String>> {
        let types: Vec<Option<String>> = catalog_cults::table
            .select(catalog_cults::cult_type)
            .distinct()
            .load(conn)?;
        
        let types: Vec<String> = types
            .into_iter()
            .filter_map(|t| t)
            .collect();
        
        Ok(types)
    }
    
    pub fn get_cult_categories(&self, conn: &mut SqliteConnection) -> QueryResult<Vec<String>> {
        let categories: Vec<String> = catalog_cults::table
            .select(catalog_cults::category)
            .distinct()
            .order(catalog_cults::category.asc())
            .load(conn)?;
        
        Ok(categories)
    }
}