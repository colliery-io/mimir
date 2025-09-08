use diesel::prelude::*;
use crate::models::catalog::variant_rule::{
    CatalogVariantRule, VariantRuleSummary, VariantRuleFilters, VariantRule
};
use crate::schema::catalog_variant_rules;

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
}