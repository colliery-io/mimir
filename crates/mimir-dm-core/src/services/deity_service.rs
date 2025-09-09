use diesel::prelude::*;
use crate::models::catalog::deity::{
    CatalogDeity, DeitySummary, DeityFilters, Deity
};

pub struct DeityService<'a> {
    pub conn: &'a mut SqliteConnection,
}

impl<'a> DeityService<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }

    /// Search deities with filters
    pub fn search_deities(&mut self, filters: DeityFilters) -> Result<Vec<DeitySummary>, String> {
        use crate::schema::catalog_deities::dsl::*;
        
        let mut query = catalog_deities.into_boxed();
        
        // Filter by name (partial match)
        if let Some(name_filter) = &filters.name {
            if !name_filter.is_empty() {
                let search_pattern = format!("%{}%", name_filter.to_lowercase());
                query = query.filter(name.like(search_pattern));
            }
        }
        
        // Filter by sources
        if let Some(source_filters) = &filters.sources {
            if !source_filters.is_empty() {
                query = query.filter(source.eq_any(source_filters));
            }
        }
        
        // Filter by pantheons
        if let Some(pantheon_filters) = &filters.pantheons {
            if !pantheon_filters.is_empty() {
                query = query.filter(pantheon.eq_any(pantheon_filters));
            }
        }
        
        // Filter by alignments
        if let Some(alignment_filters) = &filters.alignments {
            if !alignment_filters.is_empty() {
                query = query.filter(alignment.eq_any(alignment_filters));
            }
        }
        
        // Filter by domains (partial match in comma-separated string)
        if let Some(domain_filters) = &filters.domains {
            if !domain_filters.is_empty() {
                for domain in domain_filters {
                    let search_pattern = format!("%{}%", domain);
                    query = query.filter(domains.like(search_pattern));
                }
            }
        }
        
        let deities = query
            .limit(1000) // Reasonable limit to prevent memory issues
            .load::<CatalogDeity>(self.conn)
            .map_err(|e| format!("Failed to search deities: {}", e))?;
        
        Ok(deities.iter().map(DeitySummary::from).collect())
    }

    /// Get deity by name and source
    pub fn get_deity_by_name_and_source(&mut self, deity_name: &str, deity_source: &str) -> Result<Option<Deity>, String> {
        use crate::schema::catalog_deities::dsl::*;
        
        let catalog_deity = catalog_deities
            .filter(name.eq(deity_name))
            .filter(source.eq(deity_source))
            .first::<CatalogDeity>(self.conn)
            .optional()
            .map_err(|e| format!("Failed to get deity by name and source: {}", e))?;
        
        match catalog_deity {
            Some(deity_record) => {
                let parsed_deity: Deity = serde_json::from_str(&deity_record.full_deity_json)
                    .map_err(|e| format!("Failed to parse deity JSON: {}", e))?;
                Ok(Some(parsed_deity))
            },
            None => Ok(None),
        }
    }

    /// Get all unique pantheons for filtering
    pub fn get_all_pantheons(&mut self) -> Result<Vec<String>, String> {
        use crate::schema::catalog_deities::dsl::*;
        
        let pantheons: Vec<Option<String>> = catalog_deities
            .select(pantheon)
            .distinct()
            .filter(pantheon.is_not_null())
            .load(self.conn)
            .map_err(|e| format!("Failed to get pantheons: {}", e))?;
        
        let mut result: Vec<String> = pantheons
            .into_iter()
            .filter_map(|p| p)
            .filter(|p| !p.is_empty())
            .collect();
        
        result.sort();
        Ok(result)
    }

    /// Get all unique domains for filtering
    pub fn get_all_domains(&mut self) -> Result<Vec<String>, String> {
        use crate::schema::catalog_deities::dsl::*;
        
        let domain_strings: Vec<Option<String>> = catalog_deities
            .select(domains)
            .distinct()
            .filter(domains.is_not_null())
            .load(self.conn)
            .map_err(|e| format!("Failed to get domains: {}", e))?;
        
        let mut all_domains = std::collections::HashSet::new();
        
        // Parse comma-separated domains
        for domain_str in domain_strings.into_iter().filter_map(|d| d) {
            for domain in domain_str.split(',') {
                let trimmed = domain.trim();
                if !trimmed.is_empty() {
                    all_domains.insert(trimmed.to_string());
                }
            }
        }
        
        let mut result: Vec<String> = all_domains.into_iter().collect();
        result.sort();
        Ok(result)
    }

    /// Get all unique alignments for filtering
    pub fn get_all_alignments(&mut self) -> Result<Vec<String>, String> {
        use crate::schema::catalog_deities::dsl::*;
        
        let alignments: Vec<Option<String>> = catalog_deities
            .select(alignment)
            .distinct()
            .filter(alignment.is_not_null())
            .load(self.conn)
            .map_err(|e| format!("Failed to get alignments: {}", e))?;
        
        let mut result: Vec<String> = alignments
            .into_iter()
            .filter_map(|a| a)
            .filter(|a| !a.is_empty())
            .collect();
        
        result.sort();
        Ok(result)
    }

    /// Get deity statistics by source
    pub fn get_deity_count_by_source(&mut self) -> Result<Vec<(String, i64)>, String> {
        use crate::schema::catalog_deities::dsl::*;
        
        let counts = catalog_deities
            .group_by(source)
            .select((source, diesel::dsl::count_star()))
            .load::<(String, i64)>(self.conn)
            .map_err(|e| format!("Failed to get deity counts: {}", e))?;
        
        Ok(counts)
    }
}