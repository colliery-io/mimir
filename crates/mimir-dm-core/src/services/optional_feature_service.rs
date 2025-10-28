use diesel::prelude::*;
use crate::models::catalog::optionalfeature::{
    CatalogOptionalFeature, OptionalFeatureSummary, OptionalFeatureFilters, OptionalFeature, NewCatalogOptionalFeature, OptionalFeatureData
};
use std::fs;
use std::path::Path;
use tracing::{debug, info, error};

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

    /// Import all optional feature data from an uploaded book directory
    pub fn import_optional_features_from_book(
        conn: &mut SqliteConnection,
        book_dir: &Path,
        source: &str
    ) -> Result<usize, String> {
        info!("Importing optional features from book directory: {:?} (source: {})", book_dir, source);

        let mut total_imported = 0;
        let optional_feature_files = Self::find_optional_feature_files(book_dir)?;

        if optional_feature_files.is_empty() {
            info!("No optional feature files found in book directory");
            return Ok(0);
        }

        info!("Found {} optional feature files to process", optional_feature_files.len());

        for optional_feature_file in optional_feature_files {
            debug!("Processing optional feature file: {:?}", optional_feature_file);

            match Self::import_optional_features_from_file(conn, &optional_feature_file, source) {
                Ok(count) => {
                    info!("Imported {} optional features from {:?}", count, optional_feature_file);
                    total_imported += count;
                }
                Err(e) => {
                    error!("Failed to import optional features from {:?}: {}", optional_feature_file, e);
                    // Continue processing other files instead of failing completely
                }
            }
        }

        info!("Total optional features imported: {}", total_imported);
        Ok(total_imported)
    }

    /// Find optional feature files in a book directory
    fn find_optional_feature_files(book_dir: &Path) -> Result<Vec<std::path::PathBuf>, String> {
        let mut files = Vec::new();

        // Look for optionalfeatures directory (matching existing catalog structure from previous implementation)
        let optionalfeatures_dir = book_dir.join("optionalfeatures");
        if optionalfeatures_dir.exists() && optionalfeatures_dir.is_dir() {
            let entries = fs::read_dir(&optionalfeatures_dir)
                .map_err(|e| format!("Failed to read optionalfeatures directory: {}", e))?;

            for entry in entries {
                let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
                let path = entry.path();

                if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("json") {
                    debug!("Found optional feature file: {:?}", path);
                    files.push(path);
                }
            }
        }

        // Also check the main data directory for optionalfeatures.json files
        let search_dirs = vec![
            book_dir.join("data"),
            book_dir.to_path_buf(),
        ];

        for search_dir in search_dirs {
            if !search_dir.exists() || !search_dir.is_dir() {
                continue;
            }

            if let Ok(entries) = fs::read_dir(&search_dir) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        let filename = path.file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("");

                        if path.is_file() &&
                           path.extension().and_then(|e| e.to_str()) == Some("json") &&
                           (filename.contains("optionalfeature") || filename.contains("optional-feature")) {
                            debug!("Found optional feature file: {:?}", path);
                            files.push(path);
                        }
                    }
                }
            }
        }

        Ok(files)
    }

    /// Import optional features from a single JSON file
    fn import_optional_features_from_file(
        conn: &mut SqliteConnection,
        file_path: &Path,
        source: &str
    ) -> Result<usize, String> {
        debug!("Reading optional features from file: {:?}", file_path);

        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read file {:?}: {}", file_path, e))?;

        let data: OptionalFeatureData = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse JSON from {:?}: {}", file_path, e))?;

        if let Some(optional_features) = data.optional_features {
            let new_optional_features: Vec<NewCatalogOptionalFeature> = optional_features.iter().map(|feature| {
                let mut new_feature = NewCatalogOptionalFeature::from(feature);
                // Always override the source with the book source to ensure consistency
                new_feature.source = source.to_string();
                new_feature
            }).collect();

            debug!("Inserting {} optional features individually (SQLite limitation)", new_optional_features.len());

            use crate::schema::catalog_optional_features;
            for feature in &new_optional_features {
                diesel::insert_into(catalog_optional_features::table)
                    .values(feature)
                    .on_conflict((catalog_optional_features::name, catalog_optional_features::source))
                    .do_nothing()
                    .execute(conn)
                    .map_err(|e| format!("Failed to insert optional feature: {}", e))?;
            }

            info!("Successfully imported {} optional features into database", new_optional_features.len());
            Ok(new_optional_features.len())
        } else {
            Ok(0)
        }
    }

    /// Remove all optional features from a specific source
    pub fn remove_optional_features_from_source(
        conn: &mut SqliteConnection,
        source: &str
    ) -> Result<usize, String> {
        use crate::schema::catalog_optional_features;
        info!("Removing optional features from source: {}", source);

        let deleted = diesel::delete(catalog_optional_features::table.filter(catalog_optional_features::source.eq(source)))
            .execute(conn)
            .map_err(|e| format!("Failed to delete optional features from source {}: {}", source, e))?;

        info!("Removed {} optional features from source: {}", deleted, source);
        Ok(deleted)
    }
}