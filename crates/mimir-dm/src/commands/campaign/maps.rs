//! Map management commands for Visual Display System.
//!
//! Provides Tauri commands for uploading, managing, and serving battle maps,
//! dungeon maps, and regional maps for visual display during in-person play sessions.

use crate::state::AppState;
use crate::types::{ApiError, ApiResponse};
use base64::{engine::general_purpose::STANDARD, Engine};
use mimir_dm_core::{
    models::campaign::{Map, MapSummary, NewMap, UpdateMap},
    services::MapService,
};
use serde::{Deserialize, Serialize};
use std::fs;
use tauri::State;
use tracing::{error, info};
use uuid::Uuid;

/// Request to upload a new map image.
#[derive(Debug, Serialize, Deserialize)]
pub struct UploadMapRequest {
    pub campaign_id: i32,
    pub module_id: Option<i32>,
    pub name: String,
    /// Base64-encoded image data
    pub image_data: String,
    /// Original filename for extension detection
    pub filename: String,
    pub width_px: i32,
    pub height_px: i32,
}

/// Request to update map properties.
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMapRequest {
    pub name: Option<String>,
    pub grid_type: Option<String>,
    pub grid_size_px: Option<Option<i32>>,
    pub grid_offset_x: Option<i32>,
    pub grid_offset_y: Option<i32>,
}

/// Request to list maps.
#[derive(Debug, Serialize, Deserialize)]
pub struct ListMapsRequest {
    pub campaign_id: i32,
    pub module_id: Option<i32>,
}

/// Upload a new map image.
///
/// Accepts base64-encoded image data, stores it in the app data directory,
/// and creates a database record for the map.
///
/// # Parameters
/// - `request` - Upload request with image data and metadata
/// - `state` - Application state
///
/// # Returns
/// `ApiResponse` containing the created `Map` record.
#[tauri::command]
pub async fn upload_map(
    request: UploadMapRequest,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Map>, ApiError> {
    info!(
        "Uploading map '{}' for campaign {} (module: {:?})",
        request.name, request.campaign_id, request.module_id
    );

    // Create maps directory if it doesn't exist
    let maps_dir = state.paths.data_dir.join("maps");
    if let Err(e) = fs::create_dir_all(&maps_dir) {
        error!("Failed to create maps directory: {}", e);
        return Ok(ApiResponse::error(format!(
            "Failed to create maps directory: {}",
            e
        )));
    }

    // Determine file extension from filename
    let extension = std::path::Path::new(&request.filename)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("png");

    // Generate unique filename
    let unique_id = Uuid::new_v4();
    let stored_filename = format!("{}.{}", unique_id, extension);
    let image_path = maps_dir.join(&stored_filename);

    // Decode and save the image
    let image_bytes = match STANDARD.decode(&request.image_data) {
        Ok(bytes) => bytes,
        Err(e) => {
            error!("Failed to decode base64 image: {}", e);
            return Ok(ApiResponse::error(format!(
                "Invalid image data: {}",
                e
            )));
        }
    };

    if let Err(e) = fs::write(&image_path, &image_bytes) {
        error!("Failed to write image file: {}", e);
        return Ok(ApiResponse::error(format!(
            "Failed to save image: {}",
            e
        )));
    }

    info!(
        "Saved map image to {:?} ({}KB)",
        image_path,
        image_bytes.len() / 1024
    );

    // Create database record
    let new_map = NewMap::new(
        request.campaign_id,
        request.name,
        stored_filename,
        request.width_px,
        request.height_px,
    );

    let new_map = if let Some(module_id) = request.module_id {
        new_map.with_module(module_id)
    } else {
        new_map
    };

    let mut conn = state.db.get_connection()?;
    let mut service = MapService::new(&mut conn);

    match service.create_map(new_map) {
        Ok(map) => {
            info!("Map created with ID: {}", map.id);
            Ok(ApiResponse::success(map))
        }
        Err(e) => {
            // Clean up the saved image on failure
            let _ = fs::remove_file(&image_path);
            error!("Failed to create map record: {}", e);
            Ok(ApiResponse::error(format!(
                "Failed to create map: {}",
                e
            )))
        }
    }
}

/// Get a map by ID.
///
/// # Parameters
/// - `id` - Database ID of the map
/// - `state` - Application state
///
/// # Returns
/// `ApiResponse` containing the `Map` if found.
#[tauri::command]
pub async fn get_map(
    id: i32,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Map>, ApiError> {
    info!("Getting map with ID: {}", id);

    let mut conn = state.db.get_connection()?;
    let mut service = MapService::new(&mut conn);

    match service.get_map(id) {
        Ok(Some(map)) => {
            info!("Map found: {}", map.name);
            Ok(ApiResponse::success(map))
        }
        Ok(None) => {
            info!("Map not found with ID: {}", id);
            Ok(ApiResponse::error(format!(
                "Map not found with ID: {}",
                id
            )))
        }
        Err(e) => {
            error!("Failed to get map: {}", e);
            Ok(ApiResponse::error(format!("Failed to get map: {}", e)))
        }
    }
}

/// List maps for a campaign or module.
///
/// If module_id is provided, returns only maps for that module.
/// Otherwise, returns all campaign-level maps (not tied to a module).
///
/// # Parameters
/// - `request` - Request with campaign_id and optional module_id
/// - `state` - Application state
///
/// # Returns
/// `ApiResponse` containing a vector of `Map` objects.
#[tauri::command]
pub async fn list_maps(
    request: ListMapsRequest,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<Map>>, ApiError> {
    info!(
        "Listing maps for campaign {} (module: {:?})",
        request.campaign_id, request.module_id
    );

    let mut conn = state.db.get_connection()?;
    let mut service = MapService::new(&mut conn);

    let maps = if let Some(module_id) = request.module_id {
        service.list_module_maps(module_id)
    } else {
        service.list_campaign_maps(request.campaign_id)
    };

    match maps {
        Ok(maps) => {
            info!("Found {} maps", maps.len());
            Ok(ApiResponse::success(maps))
        }
        Err(e) => {
            error!("Failed to list maps: {}", e);
            Ok(ApiResponse::error(format!("Failed to list maps: {}", e)))
        }
    }
}

/// List all maps for a campaign with module names.
///
/// Returns map summaries including which module each map belongs to.
///
/// # Parameters
/// - `campaign_id` - Database ID of the campaign
/// - `state` - Application state
///
/// # Returns
/// `ApiResponse` containing a vector of `MapSummary` objects.
#[tauri::command]
pub async fn list_map_summaries(
    campaign_id: i32,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<MapSummary>>, ApiError> {
    info!("Listing map summaries for campaign {}", campaign_id);

    let mut conn = state.db.get_connection()?;
    let mut service = MapService::new(&mut conn);

    match service.list_map_summaries(campaign_id) {
        Ok(summaries) => {
            info!("Found {} map summaries", summaries.len());
            Ok(ApiResponse::success(summaries))
        }
        Err(e) => {
            error!("Failed to list map summaries: {}", e);
            Ok(ApiResponse::error(format!(
                "Failed to list map summaries: {}",
                e
            )))
        }
    }
}

/// Update a map's properties.
///
/// # Parameters
/// - `id` - Database ID of the map
/// - `request` - Fields to update
/// - `state` - Application state
///
/// # Returns
/// `ApiResponse` containing the updated `Map`.
#[tauri::command]
pub async fn update_map(
    id: i32,
    request: UpdateMapRequest,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Map>, ApiError> {
    info!("Updating map with ID: {}", id);

    let mut conn = state.db.get_connection()?;
    let mut service = MapService::new(&mut conn);

    let update = UpdateMap {
        name: request.name,
        grid_type: request.grid_type,
        grid_size_px: request.grid_size_px,
        grid_offset_x: request.grid_offset_x,
        grid_offset_y: request.grid_offset_y,
        updated_at: None, // Service handles this
    };

    match service.update_map(id, update) {
        Ok(map) => {
            info!("Map updated successfully");
            Ok(ApiResponse::success(map))
        }
        Err(e) => {
            error!("Failed to update map: {}", e);
            Ok(ApiResponse::error(format!("Failed to update map: {}", e)))
        }
    }
}

/// Update a map's grid configuration.
///
/// Convenience command for updating just the grid settings.
///
/// # Parameters
/// - `id` - Database ID of the map
/// - `grid_type` - Grid type ("square", "hex", or "none")
/// - `grid_size_px` - Pixels per grid cell (None to remove grid)
/// - `offset_x` - Grid X offset for alignment
/// - `offset_y` - Grid Y offset for alignment
/// - `state` - Application state
///
/// # Returns
/// `ApiResponse` containing the updated `Map`.
#[tauri::command]
pub async fn update_map_grid(
    id: i32,
    grid_type: String,
    grid_size_px: Option<i32>,
    offset_x: i32,
    offset_y: i32,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Map>, ApiError> {
    info!(
        "Updating grid for map {}: type={}, size={:?}, offset=({}, {})",
        id, grid_type, grid_size_px, offset_x, offset_y
    );

    let mut conn = state.db.get_connection()?;
    let mut service = MapService::new(&mut conn);

    match service.update_grid_config(id, &grid_type, grid_size_px, offset_x, offset_y) {
        Ok(map) => {
            info!("Map grid updated successfully");
            Ok(ApiResponse::success(map))
        }
        Err(e) => {
            error!("Failed to update map grid: {}", e);
            Ok(ApiResponse::error(format!(
                "Failed to update map grid: {}",
                e
            )))
        }
    }
}

/// Delete a map.
///
/// Removes both the database record and the stored image file.
///
/// # Parameters
/// - `id` - Database ID of the map to delete
/// - `state` - Application state
///
/// # Returns
/// `ApiResponse` with success or error status.
#[tauri::command]
pub async fn delete_map(
    id: i32,
    state: State<'_, AppState>,
) -> Result<ApiResponse<()>, ApiError> {
    info!("Deleting map with ID: {}", id);

    let mut conn = state.db.get_connection()?;
    let mut service = MapService::new(&mut conn);

    // Get the map first to know the image path
    let map = match service.get_map(id) {
        Ok(Some(map)) => map,
        Ok(None) => {
            return Ok(ApiResponse::error(format!(
                "Map not found with ID: {}",
                id
            )));
        }
        Err(e) => {
            error!("Failed to get map for deletion: {}", e);
            return Ok(ApiResponse::error(format!(
                "Failed to get map: {}",
                e
            )));
        }
    };

    // Delete the database record
    if let Err(e) = service.delete_map(id) {
        error!("Failed to delete map record: {}", e);
        return Ok(ApiResponse::error(format!(
            "Failed to delete map: {}",
            e
        )));
    }

    // Delete the image file
    let image_path = state.paths.data_dir.join("maps").join(&map.image_path);
    if image_path.exists() {
        if let Err(e) = fs::remove_file(&image_path) {
            // Log but don't fail - the DB record is already deleted
            error!("Warning: Failed to delete image file {:?}: {}", image_path, e);
        } else {
            info!("Deleted image file: {:?}", image_path);
        }
    }

    info!("Map deleted successfully");
    Ok(ApiResponse::success(()))
}

/// Serve a map image as base64 data URL.
///
/// Reads the map image from storage and returns it as a base64-encoded
/// data URL suitable for display in the frontend.
///
/// # Parameters
/// - `id` - Database ID of the map
/// - `state` - Application state
///
/// # Returns
/// `ApiResponse` containing a base64 data URL (e.g., "data:image/png;base64,...")
#[tauri::command]
pub async fn serve_map_image(
    id: i32,
    state: State<'_, AppState>,
) -> Result<ApiResponse<String>, ApiError> {
    info!("Serving image for map {}", id);

    let mut conn = state.db.get_connection()?;
    let mut service = MapService::new(&mut conn);

    // Get the map to find the image path
    let map = match service.get_map(id) {
        Ok(Some(map)) => map,
        Ok(None) => {
            return Ok(ApiResponse::error(format!(
                "Map not found with ID: {}",
                id
            )));
        }
        Err(e) => {
            error!("Failed to get map: {}", e);
            return Ok(ApiResponse::error(format!(
                "Failed to get map: {}",
                e
            )));
        }
    };

    let image_path = state.paths.data_dir.join("maps").join(&map.image_path);

    if !image_path.exists() {
        error!("Map image not found: {:?}", image_path);
        return Ok(ApiResponse::error("Map image not found".to_string()));
    }

    // Read the image file
    match fs::read(&image_path) {
        Ok(image_data) => {
            // Determine MIME type based on extension
            let mime_type = match image_path.extension().and_then(|ext| ext.to_str()) {
                Some("png") => "image/png",
                Some("jpg") | Some("jpeg") => "image/jpeg",
                Some("webp") => "image/webp",
                Some("gif") => "image/gif",
                _ => "image/png", // Default to PNG
            };

            // Encode as base64 data URL
            let base64_data = STANDARD.encode(&image_data);
            let data_url = format!("data:{};base64,{}", mime_type, base64_data);

            info!(
                "Successfully served map image: {} ({}KB)",
                map.image_path,
                image_data.len() / 1024
            );
            Ok(ApiResponse::success(data_url))
        }
        Err(e) => {
            error!("Failed to read map image: {}", e);
            Ok(ApiResponse::error(format!(
                "Failed to read map image: {}",
                e
            )))
        }
    }
}
