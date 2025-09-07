use crate::models::catalog::{CatalogPsionic, NewCatalogPsionic, PsionicFilters, PsionicSummary, Psionic};
use crate::schema::catalog_psionics;
use diesel::prelude::*;

pub struct PsionicService;

impl PsionicService {
    pub fn search_psionics(
        conn: &mut SqliteConnection,
        filters: PsionicFilters,
    ) -> Result<Vec<PsionicSummary>, String> {
        let mut query = catalog_psionics::table.into_boxed();
        
        // Filter by name
        if let Some(name_filter) = &filters.name {
            query = query.filter(catalog_psionics::name.like(format!("%{}%", name_filter)));
        }
        
        // Filter by psionic types ("D", "T")
        if let Some(psionic_types) = &filters.psionic_types {
            if !psionic_types.is_empty() {
                query = query.filter(catalog_psionics::psionic_type.eq_any(psionic_types));
            }
        }
        
        // Filter by orders (Avatar, Awakened, etc.)
        if let Some(orders) = &filters.orders {
            if !orders.is_empty() {
                query = query.filter(catalog_psionics::psionic_order.eq_any(orders));
            }
        }
        
        // Filter by sources
        if let Some(sources) = &filters.sources {
            if !sources.is_empty() {
                query = query.filter(catalog_psionics::source.eq_any(sources));
            }
        }
        
        let catalog_psionics: Vec<CatalogPsionic> = query
            .select(CatalogPsionic::as_select())
            .order(catalog_psionics::name.asc())
            .load(conn)
            .map_err(|e| format!("Failed to load psionics: {}", e))?;
            
        let summaries: Vec<PsionicSummary> = catalog_psionics
            .iter()
            .map(|cp| PsionicSummary::from(cp))
            .collect();
            
        Ok(summaries)
    }
    
    pub fn get_psionic_by_name_and_source(
        conn: &mut SqliteConnection,
        name: &str,
        source: &str,
    ) -> Result<Option<Psionic>, String> {
        let catalog_psionic: Option<CatalogPsionic> = catalog_psionics::table
            .filter(catalog_psionics::name.eq(name))
            .filter(catalog_psionics::source.eq(source))
            .select(CatalogPsionic::as_select())
            .first(conn)
            .optional()
            .map_err(|e| format!("Failed to query psionic: {}", e))?;
            
        if let Some(cp) = catalog_psionic {
            // Parse the full JSON back to the original Psionic type
            match serde_json::from_str::<Psionic>(&cp.full_psionic_json) {
                Ok(psionic) => Ok(Some(psionic)),
                Err(e) => Err(format!("Failed to parse stored psionic JSON: {}", e)),
            }
        } else {
            Ok(None)
        }
    }
    
    pub fn get_psionic_by_id(
        conn: &mut SqliteConnection,
        id: i32,
    ) -> Result<Option<Psionic>, String> {
        let catalog_psionic: Option<CatalogPsionic> = catalog_psionics::table
            .filter(catalog_psionics::id.eq(id))
            .select(CatalogPsionic::as_select())
            .first(conn)
            .optional()
            .map_err(|e| format!("Failed to query psionic: {}", e))?;
            
        if let Some(cp) = catalog_psionic {
            // Parse the full JSON back to the original Psionic type
            match serde_json::from_str::<Psionic>(&cp.full_psionic_json) {
                Ok(psionic) => Ok(Some(psionic)),
                Err(e) => Err(format!("Failed to parse stored psionic JSON: {}", e)),
            }
        } else {
            Ok(None)
        }
    }
    
    pub fn get_all_psionic_types(
        conn: &mut SqliteConnection,
    ) -> Result<Vec<String>, String> {
        let types: Vec<String> = catalog_psionics::table
            .select(catalog_psionics::psionic_type)
            .distinct()
            .order(catalog_psionics::psionic_type.asc())
            .load(conn)
            .map_err(|e| format!("Failed to load psionic types: {}", e))?;
            
        Ok(types)
    }
    
    pub fn get_all_psionic_orders(
        conn: &mut SqliteConnection,
    ) -> Result<Vec<String>, String> {
        let orders: Vec<Option<String>> = catalog_psionics::table
            .select(catalog_psionics::psionic_order)
            .distinct()
            .order(catalog_psionics::psionic_order.asc())
            .load(conn)
            .map_err(|e| format!("Failed to load psionic orders: {}", e))?;
            
        // Filter out None values and collect
        let filtered_orders: Vec<String> = orders
            .into_iter()
            .filter_map(|order| order)
            .collect();
            
        Ok(filtered_orders)
    }
    
    pub fn get_all_psionic_sources(
        conn: &mut SqliteConnection,
    ) -> Result<Vec<String>, String> {
        let sources: Vec<String> = catalog_psionics::table
            .select(catalog_psionics::source)
            .distinct()
            .order(catalog_psionics::source.asc())
            .load(conn)
            .map_err(|e| format!("Failed to load psionic sources: {}", e))?;
            
        Ok(sources)
    }
}