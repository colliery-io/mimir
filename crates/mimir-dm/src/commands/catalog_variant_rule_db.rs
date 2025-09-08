use std::sync::Arc;
use tauri::State;
use crate::services::database::DatabaseService;
use mimir_dm_core::services::VariantRuleService;
use mimir_dm_core::models::catalog::variant_rule::{VariantRuleFilters, VariantRuleSummary, VariantRule};

#[tauri::command]
pub async fn search_variant_rules(
    query: Option<String>,
    rule_types: Option<Vec<String>>,
    sources: Option<Vec<String>>,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<Vec<serde_json::Value>, String> {
    let mut conn = db_service.get_connection()
        .map_err(|e| format!("Database connection failed: {}", e))?;
    
    let mut service = VariantRuleService::new(&mut conn);
    
    let filters = VariantRuleFilters {
        name: query,
        rule_types,
        sources,
    };
    
    let variant_rules = service.search_variant_rules(filters)
        .map_err(|e| format!("Failed to search variant rules: {}", e))?;
    
    // Convert to JSON values to match frontend expectations
    let json_results: Vec<serde_json::Value> = variant_rules
        .into_iter()
        .map(|rule| serde_json::to_value(rule).unwrap_or_default())
        .collect();
    
    Ok(json_results)
}

#[tauri::command]
pub async fn get_variant_rule(
    id: i32,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<Option<VariantRule>, String> {
    let mut conn = db_service.get_connection()
        .map_err(|e| format!("Database connection failed: {}", e))?;
    
    let mut service = VariantRuleService::new(&mut conn);
    
    service.get_variant_rule_by_id(id)
        .map_err(|e| format!("Failed to get variant rule: {}", e))
}

#[tauri::command]
pub async fn get_variant_rule_details(
    name: String,
    source: String,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<Option<VariantRule>, String> {
    let mut conn = db_service.get_connection()
        .map_err(|e| format!("Database connection failed: {}", e))?;
    
    let mut service = VariantRuleService::new(&mut conn);
    
    service.get_variant_rule_by_name_and_source(&name, &source)
        .map_err(|e| format!("Failed to get variant rule details: {}", e))
}

#[tauri::command]
pub async fn get_variant_rule_types(
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<Vec<String>, String> {
    let mut conn = db_service.get_connection()
        .map_err(|e| format!("Database connection failed: {}", e))?;
    
    let mut service = VariantRuleService::new(&mut conn);
    
    service.get_variant_rule_types()
        .map_err(|e| format!("Failed to get variant rule types: {}", e))
}

#[tauri::command]
pub async fn get_variant_rule_sources(
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<Vec<String>, String> {
    let mut conn = db_service.get_connection()
        .map_err(|e| format!("Database connection failed: {}", e))?;
    
    let mut service = VariantRuleService::new(&mut conn);
    
    service.get_variant_rule_sources()
        .map_err(|e| format!("Failed to get variant rule sources: {}", e))
}