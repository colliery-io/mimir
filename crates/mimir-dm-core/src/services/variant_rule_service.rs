use diesel::prelude::*;
use crate::models::catalog::variant_rule::{
    CatalogVariantRule, VariantRuleSummary, VariantRuleFilters, VariantRule, NewCatalogVariantRule, VariantRuleData
};
use std::fs;
use std::path::Path;
use tracing::{debug, info, error};

pub struct VariantRuleService<'a> {
    pub conn: &'a mut SqliteConnection,
}

impl<'a> VariantRuleService<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }

    pub fn search_variant_rules(&mut self, filters: VariantRuleFilters) -> QueryResult<Vec<VariantRuleSummary>> {
        use crate::schema::catalog_variant_rules::dsl::*;
        
        let mut query = catalog_variant_rules.into_boxed();
        
        // Filter by name
        if let Some(search_name) = &filters.name {
            if !search_name.is_empty() {
                query = query.filter(name.like(format!("%{}%", search_name)));
            }
        }
        
        // Filter by rule types - simplified approach
        if let Some(types) = &filters.rule_types {
            if !types.is_empty() {
                // Handle "General" type by checking for null values
                let has_general = types.contains(&"General".to_string());
                let other_types: Vec<String> = types.iter()
                    .filter(|t| *t != "General")
                    .cloned()
                    .collect();
                
                if has_general && !other_types.is_empty() {
                    query = query.filter(rule_type.is_null().or(rule_type.eq_any(other_types)));
                } else if has_general {
                    query = query.filter(rule_type.is_null());
                } else if !other_types.is_empty() {
                    query = query.filter(rule_type.eq_any(other_types));
                }
            }
        }
        
        // Filter by sources
        if let Some(sources) = &filters.sources {
            if !sources.is_empty() {
                query = query.filter(source.eq_any(sources));
            }
        }
        
        let rules = query
            .order(name.asc())
            .load::<CatalogVariantRule>(self.conn)?;
            
        Ok(rules.iter().map(VariantRuleSummary::from).collect())
    }

    pub fn get_variant_rule_by_id(&mut self, rule_id: i32) -> QueryResult<Option<VariantRule>> {
        use crate::schema::catalog_variant_rules::dsl::*;
        
        let catalog_rule = catalog_variant_rules
            .find(rule_id)
            .first::<CatalogVariantRule>(self.conn)
            .optional()?;
            
        if let Some(rule) = catalog_rule {
            let parsed_rule: Result<VariantRule, _> = serde_json::from_str(&rule.full_variant_rule_json);
            match parsed_rule {
                Ok(rule) => Ok(Some(rule)),
                Err(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }

    pub fn get_variant_rule_by_name_and_source(&mut self, rule_name: &str, rule_source: &str) -> QueryResult<Option<VariantRule>> {
        use crate::schema::catalog_variant_rules::dsl::*;
        
        let catalog_rule = catalog_variant_rules
            .filter(name.eq(rule_name))
            .filter(source.eq(rule_source))
            .first::<CatalogVariantRule>(self.conn)
            .optional()?;
            
        if let Some(rule) = catalog_rule {
            let parsed_rule: Result<VariantRule, _> = serde_json::from_str(&rule.full_variant_rule_json);
            match parsed_rule {
                Ok(rule) => Ok(Some(rule)),
                Err(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }

    pub fn get_variant_rule_types(&mut self) -> QueryResult<Vec<String>> {
        use crate::schema::catalog_variant_rules::dsl::*;
        
        let types: Vec<Option<String>> = catalog_variant_rules
            .select(rule_type)
            .distinct()
            .load(self.conn)?;
            
        let mut result: Vec<String> = types
            .into_iter()
            .map(|t| t.unwrap_or_else(|| "General".to_string()))
            .collect();
        result.sort();
        result.dedup();
        Ok(result)
    }

    pub fn get_variant_rule_sources(&mut self) -> QueryResult<Vec<String>> {
        use crate::schema::catalog_variant_rules::dsl::*;

        let mut sources: Vec<String> = catalog_variant_rules
            .select(source)
            .distinct()
            .load(self.conn)?;
        sources.sort();
        Ok(sources)
    }

    /// Import all variant rule data from an uploaded book directory
    pub fn import_variant_rules_from_book(
        conn: &mut SqliteConnection,
        book_dir: &Path,
        source: &str
    ) -> Result<usize, String> {
        info!("Importing variant rules from book directory: {:?} (source: {})", book_dir, source);

        let mut total_imported = 0;
        let variant_rule_files = Self::find_variant_rule_files(book_dir)?;

        if variant_rule_files.is_empty() {
            info!("No variant rule files found in book directory");
            return Ok(0);
        }

        info!("Found {} variant rule files to process", variant_rule_files.len());

        for variant_rule_file in variant_rule_files {
            debug!("Processing variant rule file: {:?}", variant_rule_file);

            match Self::import_variant_rules_from_file(conn, &variant_rule_file, source) {
                Ok(count) => {
                    info!("Imported {} variant rules from {:?}", count, variant_rule_file);
                    total_imported += count;
                }
                Err(e) => {
                    error!("Failed to import variant rules from {:?}: {}", variant_rule_file, e);
                    // Continue processing other files instead of failing completely
                }
            }
        }

        info!("Successfully imported {} total variant rules from {}", total_imported, source);
        Ok(total_imported)
    }

    fn find_variant_rule_files(book_dir: &Path) -> Result<Vec<std::path::PathBuf>, String> {
        let mut files = Vec::new();

        // Look for variantrules directory
        let variantrules_dir = book_dir.join("variantrules");
        if variantrules_dir.exists() && variantrules_dir.is_dir() {
            let entries = fs::read_dir(&variantrules_dir)
                .map_err(|e| format!("Failed to read variantrules directory: {}", e))?;

            for entry in entries {
                let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
                let path = entry.path();

                if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("json") {
                    files.push(path);
                }
            }
        }

        Ok(files)
    }

    fn import_variant_rules_from_file(
        conn: &mut SqliteConnection,
        file_path: &Path,
        source: &str
    ) -> Result<usize, String> {
        debug!("Reading variant rule file: {:?}", file_path);

        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read variant rule file: {}", e))?;

        let variant_rule_data: VariantRuleData = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse variant rule JSON: {}", e))?;

        if let Some(variant_rules) = variant_rule_data.variantrule {
            let new_variant_rules: Vec<NewCatalogVariantRule> = variant_rules.iter().map(|rule| {
                let mut new_rule = NewCatalogVariantRule::from(rule);
                // Always override the source with the book source to ensure consistency
                new_rule.source = source.to_string();
                new_rule
            }).collect();

            debug!("Inserting {} variant rules individually (SQLite limitation)", new_variant_rules.len());

            use crate::schema::catalog_variant_rules;
            for rule in &new_variant_rules {
                diesel::insert_into(catalog_variant_rules::table)
                    .values(rule)
                    .on_conflict((catalog_variant_rules::name, catalog_variant_rules::source))
                    .do_nothing()
                    .execute(conn)
                    .map_err(|e| format!("Failed to insert variant rule: {}", e))?;
            }

            info!("Successfully imported {} variant rules into database", new_variant_rules.len());
            Ok(new_variant_rules.len())
        } else {
            Ok(0)
        }
    }

    /// Remove all variant rules from a specific source
    pub fn remove_variant_rules_from_source(
        conn: &mut SqliteConnection,
        source: &str
    ) -> Result<usize, String> {
        use crate::schema::catalog_variant_rules;
        info!("Removing variant rules from source: {}", source);

        let deleted = diesel::delete(catalog_variant_rules::table.filter(catalog_variant_rules::source.eq(source)))
            .execute(conn)
            .map_err(|e| format!("Failed to delete variant rules from source {}: {}", source, e))?;

        info!("Removed {} variant rules from source: {}", deleted, source);
        Ok(deleted)
    }
}