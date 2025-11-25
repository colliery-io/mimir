use crate::state::AppState;
use mimir_dm_core::models::catalog::item::{Item, ItemFilters, ItemSummary};
use mimir_dm_core::services::item_service::ItemService;
use tauri::State;
use tracing::{debug, error, info};

#[tauri::command]
pub async fn search_items(
    name: Option<String>,
    item_types: Option<Vec<String>>,
    rarities: Option<Vec<String>>,
    sources: Option<Vec<String>>,
    min_value: Option<f64>,
    max_value: Option<f64>,
    state: State<'_, AppState>,
) -> Result<Vec<ItemSummary>, String> {
    debug!("Searching items with name: {:?}, item_types: {:?}, rarities: {:?}, sources: {:?}",
           name, item_types, rarities, sources);

    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let filters = ItemFilters {
        name,
        item_types,
        rarities,
        sources,
        min_value,
        max_value,
    };

    let mut service = ItemService::new(&mut conn);
    let results = service.search_items(filters)
        .map_err(|e| format!("Database query failed: {}", e))?;

    info!("Found {} items", results.len());
    Ok(results)
}

#[tauri::command]
pub async fn get_item(
    item_id: i32,
    state: State<'_, AppState>,
) -> Result<Option<Item>, String> {
    debug!("Getting item details for ID: {}", item_id);

    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut service = ItemService::new(&mut conn);
    service.get_item_by_id(item_id)
        .map_err(|e| format!("Database query failed: {}", e))
}

#[tauri::command]
pub async fn get_item_details(
    item_name: String,
    item_source: String,
    state: State<'_, AppState>,
) -> Result<Option<Item>, String> {
    debug!("Getting item details for name: {}, source: {}", item_name, item_source);

    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut service = ItemService::new(&mut conn);
    service.get_item_by_name_and_source(&item_name, &item_source)
        .map_err(|e| format!("Database query failed: {}", e))
}

#[tauri::command]
pub async fn get_item_types(
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    debug!("Getting all item types");

    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut service = ItemService::new(&mut conn);
    let types = service.get_item_types()
        .map_err(|e| format!("Database query failed: {}", e))?;

    info!("Found {} item types", types.len());
    Ok(types)
}

#[tauri::command]
pub async fn get_item_rarities(
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    debug!("Getting all item rarities");

    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut service = ItemService::new(&mut conn);
    let rarities = service.get_item_rarities()
        .map_err(|e| format!("Database query failed: {}", e))?;

    info!("Found {} rarities", rarities.len());
    Ok(rarities)
}

#[tauri::command]
pub async fn get_item_sources(
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    debug!("Getting all item sources");

    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut service = ItemService::new(&mut conn);
    let sources = service.get_item_sources()
        .map_err(|e| format!("Database query failed: {}", e))?;

    info!("Found {} sources", sources.len());
    Ok(sources)
}