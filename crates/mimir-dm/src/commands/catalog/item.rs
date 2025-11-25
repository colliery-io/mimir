//! Database-backed item catalog commands.
//!
//! Provides Tauri commands for searching, filtering, and retrieving item data
//! from the 5e catalog database. Items include equipment, magic items, and
//! other gear.

use crate::state::AppState;
use mimir_dm_core::models::catalog::item::{Item, ItemFilters, ItemSummary};
use mimir_dm_core::services::item_service::ItemService;
use tauri::State;
use tracing::{debug, error, info};

/// Search the item catalog with optional filters.
///
/// Returns a list of item summaries matching the provided criteria.
/// All filter parameters are optional and can be combined.
///
/// # Parameters
/// - `name` - Text to search in item names (case-insensitive)
/// - `item_types` - Filter by item type (e.g., `["Weapon", "Armor", "Wondrous Item"]`)
/// - `rarities` - Filter by rarity (e.g., `["Common", "Uncommon", "Rare"]`)
/// - `sources` - Filter by source books
/// - `min_value` - Minimum item value in gold pieces
/// - `max_value` - Maximum item value in gold pieces
///
/// # Returns
/// List of `ItemSummary` objects containing basic item information.
///
/// # Errors
/// Returns an error string if the database connection or query fails.
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
    debug!(
        "Searching items with name: {:?}, item_types: {:?}, rarities: {:?}, sources: {:?}",
        name, item_types, rarities, sources
    );

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
    let results = service
        .search_items(filters)
        .map_err(|e| format!("Database query failed: {}", e))?;

    info!("Found {} items", results.len());
    Ok(results)
}

/// Get complete item details by database ID.
///
/// Retrieves the full item record including properties, description, and value.
///
/// # Parameters
/// - `item_id` - Database ID of the item
///
/// # Returns
/// The complete `Item` object if found, or `None` if no match exists.
///
/// # Errors
/// Returns an error string if the database connection or query fails.
#[tauri::command]
pub async fn get_item(item_id: i32, state: State<'_, AppState>) -> Result<Option<Item>, String> {
    debug!("Getting item details for ID: {}", item_id);

    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut service = ItemService::new(&mut conn);
    service
        .get_item_by_id(item_id)
        .map_err(|e| format!("Database query failed: {}", e))
}

/// Get complete item details by name and source.
///
/// Retrieves the full item record including properties, description, and value.
///
/// # Parameters
/// - `item_name` - Exact item name (case-sensitive)
/// - `item_source` - Source book abbreviation (e.g., "PHB", "DMG")
///
/// # Returns
/// The complete `Item` object if found, or `None` if no match exists.
///
/// # Errors
/// Returns an error string if the database connection or query fails.
#[tauri::command]
pub async fn get_item_details(
    item_name: String,
    item_source: String,
    state: State<'_, AppState>,
) -> Result<Option<Item>, String> {
    debug!(
        "Getting item details for name: {}, source: {}",
        item_name, item_source
    );

    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut service = ItemService::new(&mut conn);
    service
        .get_item_by_name_and_source(&item_name, &item_source)
        .map_err(|e| format!("Database query failed: {}", e))
}

/// Get all unique item types in the catalog.
///
/// Returns item type categories present in the item catalog.
/// Used to populate filter dropdowns in the UI.
///
/// # Returns
/// List of type names (e.g., `["Weapon", "Armor", "Potion", "Wondrous Item"]`).
///
/// # Errors
/// Returns an error string if the database connection or query fails.
#[tauri::command]
pub async fn get_item_types(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    debug!("Getting all item types");

    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut service = ItemService::new(&mut conn);
    let types = service
        .get_item_types()
        .map_err(|e| format!("Database query failed: {}", e))?;

    info!("Found {} item types", types.len());
    Ok(types)
}

/// Get all unique item rarities in the catalog.
///
/// Returns rarity values present in the item catalog.
/// Used to populate filter dropdowns in the UI.
///
/// # Returns
/// List of rarity names (e.g., `["Common", "Uncommon", "Rare", "Very Rare", "Legendary"]`).
///
/// # Errors
/// Returns an error string if the database connection or query fails.
#[tauri::command]
pub async fn get_item_rarities(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    debug!("Getting all item rarities");

    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut service = ItemService::new(&mut conn);
    let rarities = service
        .get_item_rarities()
        .map_err(|e| format!("Database query failed: {}", e))?;

    info!("Found {} rarities", rarities.len());
    Ok(rarities)
}

/// Get all unique source books containing items.
///
/// Returns source book abbreviations that contain at least one item.
/// Used to populate filter dropdowns in the UI.
///
/// # Returns
/// List of source abbreviations.
///
/// # Errors
/// Returns an error string if the database connection or query fails.
#[tauri::command]
pub async fn get_item_sources(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    debug!("Getting all item sources");

    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut service = ItemService::new(&mut conn);
    let sources = service
        .get_item_sources()
        .map_err(|e| format!("Database query failed: {}", e))?;

    info!("Found {} sources", sources.len());
    Ok(sources)
}
