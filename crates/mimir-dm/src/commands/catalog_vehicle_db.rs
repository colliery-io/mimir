use std::sync::Arc;
use tauri::State;
use tracing::{debug, error};

use crate::services::database::DatabaseService;
use mimir_dm_core::services::VehicleService;
use mimir_dm_core::models::catalog::vehicle::{VehicleSummary, VehicleFilters, Vehicle};

/// Search vehicles from database with filters
#[tauri::command]
pub async fn search_vehicles_db(
    filters: VehicleFilters,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<Vec<VehicleSummary>, String> {
    debug!("Searching vehicles with filters: {:?}", filters);

    let mut conn = db_service.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut service = VehicleService::new(&mut conn);
    service.search_vehicles(filters)
        .map_err(|e| format!("Database query failed: {}", e))
}

/// Get vehicle details by name and source
#[tauri::command]
pub async fn get_vehicle_details_db(
    vehicle_name: String,
    vehicle_source: String,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<Option<Vehicle>, String> {
    debug!("Getting vehicle details for name: {}, source: {}", vehicle_name, vehicle_source);

    let mut conn = db_service.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut service = VehicleService::new(&mut conn);
    service.get_vehicle_by_name_and_source(&vehicle_name, &vehicle_source)
        .map_err(|e| format!("Database query failed: {}", e))
}

/// Get all vehicle types for filter dropdowns
#[tauri::command]
pub async fn get_vehicle_types_db(
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<Vec<String>, String> {
    debug!("Getting all vehicle types");

    let mut conn = db_service.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut service = VehicleService::new(&mut conn);
    service.get_all_vehicle_types()
        .map_err(|e| format!("Database query failed: {}", e))
}

/// Get all vehicle sizes for filter dropdowns
#[tauri::command]
pub async fn get_vehicle_sizes_db(
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<Vec<String>, String> {
    debug!("Getting all vehicle sizes");

    let mut conn = db_service.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut service = VehicleService::new(&mut conn);
    service.get_all_sizes()
        .map_err(|e| format!("Database query failed: {}", e))
}

/// Get all vehicle terrains for filter dropdowns
#[tauri::command]
pub async fn get_vehicle_terrains_db(
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<Vec<String>, String> {
    debug!("Getting all vehicle terrains");

    let mut conn = db_service.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut service = VehicleService::new(&mut conn);
    service.get_all_terrains()
        .map_err(|e| format!("Database query failed: {}", e))
}

/// Get vehicle count by source for statistics
#[tauri::command]
pub async fn get_vehicle_statistics_db(
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<Vec<(String, i64)>, String> {
    debug!("Getting vehicle statistics");

    let mut conn = db_service.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut service = VehicleService::new(&mut conn);
    service.get_vehicle_count_by_source()
        .map_err(|e| format!("Database query failed: {}", e))
}