use diesel::prelude::*;
use crate::models::catalog::optionalfeature::{
    CatalogOptionalFeature, OptionalFeatureSummary, OptionalFeatureFilters, OptionalFeature
};

pub struct OptionalFeatureService<'a> {
    pub conn: &'a mut SqliteConnection,
}

impl<'a> OptionalFeatureService<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }

    pub fn search_optional_features(&mut self, filters: OptionalFeatureFilters) -> QueryResult<Vec<OptionalFeatureSummary>> {
        use crate::schema::catalog_optional_features::dsl::*;
        
        let mut query = catalog_optional_features.into_boxed();
        
        // Filter by name
        if let Some(search_name) = &filters.name {
            if !search_name.is_empty() {
                query = query.filter(name.like(format!("%{}%", search_name)));
            }
        }
        
        // Filter by feature types - we'll do this in post-processing to avoid complex SQL
        let requested_types = filters.feature_types.clone();
        
        // Filter by sources
        if let Some(sources) = &filters.sources {
            if !sources.is_empty() {
                query = query.filter(source.eq_any(sources));
            }
        }
        
        // Filter by grants_spells
        if let Some(grants) = filters.grants_spells {
            query = query.filter(grants_spells.eq(grants));
        }
        
        let features = query
            .order(name.asc())
            .load::<CatalogOptionalFeature>(self.conn)?;
            
        let mut results: Vec<OptionalFeatureSummary> = features.iter().map(OptionalFeatureSummary::from).collect();
        
        // Apply feature type filtering in post-processing
        if let Some(types) = requested_types {
            if !types.is_empty() {
                results.retain(|feature| {
                    feature.feature_types.iter().any(|ft| types.contains(ft))
                });
            }
        }
        
        Ok(results)
    }

    pub fn get_optional_feature_by_id(&mut self, feature_id: i32) -> QueryResult<Option<OptionalFeature>> {
        use crate::schema::catalog_optional_features::dsl::*;
        
        let catalog_feature = catalog_optional_features
            .find(feature_id)
            .first::<CatalogOptionalFeature>(self.conn)
            .optional()?;
            
        if let Some(feature) = catalog_feature {
            let parsed_feature: Result<OptionalFeature, _> = serde_json::from_str(&feature.full_optional_feature_json);
            match parsed_feature {
                Ok(feature) => Ok(Some(feature)),
                Err(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }

    pub fn get_optional_feature_by_name_and_source(&mut self, feature_name: &str, feature_source: &str) -> QueryResult<Option<OptionalFeature>> {
        use crate::schema::catalog_optional_features::dsl::*;
        
        let catalog_feature = catalog_optional_features
            .filter(name.eq(feature_name))
            .filter(source.eq(feature_source))
            .first::<CatalogOptionalFeature>(self.conn)
            .optional()?;
            
        if let Some(feature) = catalog_feature {
            let parsed_feature: Result<OptionalFeature, _> = serde_json::from_str(&feature.full_optional_feature_json);
            match parsed_feature {
                Ok(feature) => Ok(Some(feature)),
                Err(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }

    pub fn get_optional_feature_types(&mut self) -> QueryResult<Vec<String>> {
        use crate::schema::catalog_optional_features::dsl::*;
        
        let features: Vec<Option<String>> = catalog_optional_features
            .select(feature_types)
            .distinct()
            .load(self.conn)?;
            
        let mut all_types = std::collections::HashSet::new();
        for feature_types_json in features.into_iter().flatten() {
            if let Ok(types) = serde_json::from_str::<Vec<String>>(&feature_types_json) {
                for feature_type in types {
                    all_types.insert(feature_type);
                }
            }
        }
        
        let mut result: Vec<String> = all_types.into_iter().collect();
        result.sort();
        Ok(result)
    }

    pub fn get_optional_feature_sources(&mut self) -> QueryResult<Vec<String>> {
        use crate::schema::catalog_optional_features::dsl::*;
        
        let mut sources: Vec<String> = catalog_optional_features
            .select(source)
            .distinct()
            .load(self.conn)?;
        sources.sort();
        Ok(sources)
    }
}