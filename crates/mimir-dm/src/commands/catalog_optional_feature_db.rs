use tauri::State;
use mimir_dm_core::models::catalog::optionalfeature::{OptionalFeatureSummary, OptionalFeatureFilters, OptionalFeature};
use mimir_dm_core::services::OptionalFeatureService;
use crate::services::database::DatabaseService;
use std::sync::Arc;
use tracing::{info, debug, error};

/// Search optional features in the database with filters
#[tauri::command]
pub async fn search_optional_features_db(
    name: Option<String>,
    feature_types: Option<Vec<String>>,
    sources: Option<Vec<String>>,
    grants_spells: Option<bool>,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<Vec<OptionalFeatureSummary>, String> {
    debug!("Searching optional features with name: {:?}, feature_types: {:?}, sources: {:?}, grants_spells: {:?}", 
           name, feature_types, sources, grants_spells);

    let mut conn = db_service.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let filters = OptionalFeatureFilters {
        name,
        feature_types,
        sources,
        grants_spells,
    };

    let mut service = OptionalFeatureService::new(&mut conn);
    let results = service.search_optional_features(filters)
        .map_err(|e| format!("Database query failed: {}", e))?;

    info!("Found {} optional features", results.len());
    Ok(results)
}

/// Get detailed optional feature information by ID
#[tauri::command]
pub async fn get_optional_feature_db(
    id: i32,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<OptionalFeature, String> {
    debug!("Getting optional feature details for ID: {}", id);

    let mut conn = db_service.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut service = OptionalFeatureService::new(&mut conn);
    let result = service.get_optional_feature_by_id(id)
        .map_err(|e| format!("Database query failed: {}", e))?;

    result.ok_or_else(|| format!("Optional feature with ID {} not found", id))
}

/// Get detailed optional feature information by name and source (for backward compatibility)
#[tauri::command]
pub async fn get_optional_feature_details(
    name: String,
    source: String,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<OptionalFeature, String> {
    debug!("Getting optional feature details for name: {}, source: {}", name, source);

    let mut conn = db_service.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut service = OptionalFeatureService::new(&mut conn);
    let result = service.get_optional_feature_by_name_and_source(&name, &source)
        .map_err(|e| format!("Database query failed: {}", e))?;

    result.ok_or_else(|| format!("Optional feature '{}' from source '{}' not found", name, source))
}

/// Get all available feature types for filtering
#[tauri::command]
pub async fn get_optional_feature_types(
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<Vec<String>, String> {
    debug!("Getting all optional feature types");

    let mut conn = db_service.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut service = OptionalFeatureService::new(&mut conn);
    let types = service.get_optional_feature_types()
        .map_err(|e| format!("Database query failed: {}", e))?;

    info!("Found {} feature types", types.len());
    Ok(types)
}

/// Get all available sources for filtering
#[tauri::command]
pub async fn get_optional_feature_sources(
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<Vec<String>, String> {
    debug!("Getting all optional feature sources");

    let mut conn = db_service.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut service = OptionalFeatureService::new(&mut conn);
    let sources = service.get_optional_feature_sources()
        .map_err(|e| format!("Database query failed: {}", e))?;

    info!("Found {} sources", sources.len());
    Ok(sources)
}