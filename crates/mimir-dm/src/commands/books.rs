//! Book library management commands

use crate::{
    types::ApiResponse,
    APP_PATHS,
};
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{error, info, debug, warn};
use base64::{engine::general_purpose::STANDARD, Engine};
use tar::Archive;
use flate2::read::GzDecoder;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use diesel::prelude::*;
use mimir_dm_core::models::catalog::{NewUploadedBook, UploadedBook};
use mimir_dm_core::schema::uploaded_books;
use mimir_dm_core::services::{SpellService, ActionService, ConditionService, LanguageService, RewardService, BackgroundService, FeatService, RaceService, ObjectService, TrapService, CultService, VariantRuleService, OptionalFeatureService, ItemService, MonsterService, DeityService, VehicleService, ClassService};
use chrono::Utc;

/// Upload and extract a book archive (tar.gz format from mimir-5esplit)
#[tauri::command]
pub async fn upload_book_archive(
    archive_path: String,
) -> Result<ApiResponse<BookInfo>, String> {
    info!("Uploading book archive from: {}", archive_path);
    
    // Get app paths
    let app_paths = APP_PATHS.get()
        .ok_or_else(|| "App paths not initialized".to_string())?;
    
    // Verify archive exists
    let archive_file = Path::new(&archive_path);
    if !archive_file.exists() {
        return Ok(ApiResponse::error(format!("Archive not found: {}", archive_path)));
    }
    
    // Open the archive
    let tar_gz = fs::File::open(archive_file)
        .map_err(|e| format!("Failed to open archive: {}", e))?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    
    // Create books directory if it doesn't exist
    let books_dir = app_paths.data_dir.join("books");
    if !books_dir.exists() {
        fs::create_dir_all(&books_dir)
            .map_err(|e| format!("Failed to create books directory: {}", e))?;
        info!("Created books directory at: {:?}", books_dir);
    }
    
    // Extract to a temporary directory first to validate structure
    let temp_dir = tempfile::TempDir::new()
        .map_err(|e| format!("Failed to create temp directory: {}", e))?;
    
    archive.unpack(temp_dir.path())
        .map_err(|e| format!("Failed to extract archive: {}", e))?;
    
    // Find the book directory (should be the only top-level directory)
    let mut book_id = None;
    let mut book_metadata = None;
    let mut book_data = None;
    
    for entry in fs::read_dir(temp_dir.path())
        .map_err(|e| format!("Failed to read temp directory: {}", e))? {
        
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        let path = entry.path();
        
        if path.is_dir() {
            // Found the book directory - use its name as the ID
            book_id = Some(entry.file_name().to_string_lossy().to_string());
            
            // Look for metadata.json from mimir-5esplit
            let metadata_path = path.join("metadata.json");
            if metadata_path.exists() {
                let metadata_content = fs::read_to_string(&metadata_path)
                    .map_err(|e| format!("Failed to read metadata: {}", e))?;
                book_metadata = serde_json::from_str::<serde_json::Value>(&metadata_content).ok();
            }
            
            // Look for the main book content
            let book_content_path = find_book_content_file(&path)?;
            if let Some(content_path) = book_content_path {
                let content = fs::read_to_string(&content_path)
                    .map_err(|e| format!("Failed to read book content: {}", e))?;
                book_data = serde_json::from_str::<serde_json::Value>(&content).ok();
            }
            
            break;
        }
    }
    
    let book_id = book_id
        .ok_or_else(|| "No book directory found in archive".to_string())?;
    
    // Check database for collision before doing any work
    match crate::db_connection::get_connection() {
        Ok(mut conn) => {
            let existing: Result<UploadedBook, _> = uploaded_books::table
                .filter(uploaded_books::id.eq(&book_id))
                .first(&mut conn);
                
            if existing.is_ok() {
                return Ok(ApiResponse::error(format!("Book '{}' is already uploaded. Please remove it first.", book_id)));
            }
        }
        Err(e) => {
            error!("Database connection error during collision check: {}", e);
            return Ok(ApiResponse::error("Database error - please try again".to_string()));
        }
    }
    
    // Extract book name from metadata or book data
    let book_name = book_metadata
        .as_ref()
        .and_then(|m| m.get("name"))
        .and_then(|n| n.as_str())
        .or_else(|| {
            book_data.as_ref()
                .and_then(|d| d.get("data"))
                .and_then(|d| d.get(0))
                .and_then(|d| d.get("name"))
                .and_then(|n| n.as_str())
        })
        .map(|s| s.to_string())
        .unwrap_or_else(|| book_id.clone());
    
    // Check if book already exists
    let final_book_dir = books_dir.join(&book_id);
    if final_book_dir.exists() {
        return Ok(ApiResponse::error(format!("Book '{}' already exists", book_name)));
    }
    
    // Create archives directory if it doesn't exist
    let archives_dir = app_paths.data_dir.join("archives");
    if !archives_dir.exists() {
        fs::create_dir_all(&archives_dir)
            .map_err(|e| format!("Failed to create archives directory: {}", e))?;
        info!("Created archives directory at: {:?}", archives_dir);
    }
    
    // Strategy: Do all database work first, then move files
    // This way if database fails, we haven't moved anything yet
    
    // Create database record first
    let archive_destination = archives_dir.join(format!("{}.tar.gz", book_id));
    let new_book = NewUploadedBook {
        id: book_id.clone(),
        name: book_name.clone(),
        location: final_book_dir.to_string_lossy().to_string(),
        archive_path: archive_destination.to_string_lossy().to_string(),
        uploaded_at: Utc::now().to_rfc3339(),
        metadata_json: book_metadata.map(|m| m.to_string()),
    };
    
    match crate::db_connection::get_connection() {
        Ok(mut conn) => {
            match diesel::insert_into(uploaded_books::table)
                .values(&new_book)
                .execute(&mut conn)
            {
                Ok(_) => {
                    info!("Successfully recorded book '{}' in database", book_name);
                }
                Err(e) => {
                    error!("Failed to insert book into database: {}", e);
                    return Ok(ApiResponse::error("Failed to record book in database".to_string()));
                }
            }
        }
        Err(e) => {
            error!("Database connection error during insert: {}", e);
            return Ok(ApiResponse::error("Database error - upload failed".to_string()));
        }
    }
    
    // Now that database is committed, move the files
    // If this fails, we have the database record so user can retry or we can implement repair
    
    // Copy archive to archives directory
    if let Err(e) = fs::copy(&archive_file, &archive_destination) {
        error!("Failed to copy archive after database insert: {}", e);
        // Clean up database record
        if let Ok(mut conn) = crate::db_connection::get_connection() {
            let _ = diesel::delete(uploaded_books::table.filter(uploaded_books::id.eq(&book_id)))
                .execute(&mut conn);
        }
        return Ok(ApiResponse::error("Failed to copy archive file".to_string()));
    }
    
    // Move the entire extracted book directory to its final location
    let temp_book_dir = temp_dir.path().join(&book_id);
    if let Err(e) = fs::rename(&temp_book_dir, &final_book_dir)
        .or_else(|_| copy_dir_recursive(&temp_book_dir, &final_book_dir))
    {
        error!("Failed to move book directory after database insert: {}", e);
        // Clean up archive and database record
        let _ = fs::remove_file(&archive_destination);
        if let Ok(mut conn) = crate::db_connection::get_connection() {
            let _ = diesel::delete(uploaded_books::table.filter(uploaded_books::id.eq(&book_id)))
                .execute(&mut conn);
        }
        return Ok(ApiResponse::error("Failed to move book directory".to_string()));
    }
    
    info!("Successfully imported book '{}'", book_name);
    
    // Import catalog content automatically
    match crate::db_connection::get_connection() {
        Ok(mut catalog_conn) => {
            // Import spells
            match SpellService::import_spells_from_book(&mut catalog_conn, &final_book_dir, &book_id) {
                Ok(spell_count) => {
                    info!("Imported {} spells from book '{}'", spell_count, book_name);
                }
                Err(e) => {
                    warn!("Book uploaded successfully but failed to import spells: {}", e);
                    // Don't fail the entire upload for catalog import errors
                }
            }
            
            // Import actions
            match ActionService::import_actions_from_book(&mut catalog_conn, &final_book_dir, &book_id) {
                Ok(action_count) => {
                    info!("Imported {} actions from book '{}'", action_count, book_name);
                }
                Err(e) => {
                    warn!("Book uploaded successfully but failed to import actions: {}", e);
                    // Don't fail the entire upload for catalog import errors
                }
            }
            
            // Import conditions
            match ConditionService::import_conditions_from_book(&mut catalog_conn, &final_book_dir, &book_id) {
                Ok(condition_count) => {
                    info!("Imported {} conditions from book '{}'", condition_count, book_name);
                }
                Err(e) => {
                    warn!("Book uploaded successfully but failed to import conditions: {}", e);
                    // Don't fail the entire upload for catalog import errors
                }
            }
            
            // Import languages
            match LanguageService::import_languages_from_book(&mut catalog_conn, &final_book_dir, &book_id) {
                Ok(language_count) => {
                    info!("Imported {} languages from book '{}'", language_count, book_name);
                }
                Err(e) => {
                    warn!("Book uploaded successfully but failed to import languages: {}", e);
                    // Don't fail the entire upload for catalog import errors
                }
            }
            
            // Import rewards
            match RewardService::import_rewards_from_book(&mut catalog_conn, &final_book_dir, &book_id) {
                Ok(reward_count) => {
                    info!("Imported {} rewards from book '{}'", reward_count, book_name);
                }
                Err(e) => {
                    warn!("Book uploaded successfully but failed to import rewards: {}", e);
                    // Don't fail the entire upload for catalog import errors
                }
            }
            
            // Import backgrounds
            match BackgroundService::import_backgrounds_from_book(&mut catalog_conn, &final_book_dir, &book_id) {
                Ok(background_count) => {
                    info!("Imported {} backgrounds from book '{}'", background_count, book_name);
                }
                Err(e) => {
                    warn!("Book uploaded successfully but failed to import backgrounds: {}", e);
                    // Don't fail the entire upload for catalog import errors
                }
            }
            
            // Import feats
            match FeatService::import_feats_from_book(&mut catalog_conn, &final_book_dir, &book_id) {
                Ok(feat_count) => {
                    info!("Imported {} feats from book '{}'", feat_count, book_name);
                }
                Err(e) => {
                    warn!("Book uploaded successfully but failed to import feats: {}", e);
                    // Don't fail the entire upload for catalog import errors
                }
            }
            
            // Import races
            match RaceService::import_races_from_book(&mut catalog_conn, &final_book_dir, &book_id) {
                Ok(race_count) => {
                    info!("Imported {} races from book '{}'", race_count, book_name);
                }
                Err(e) => {
                    warn!("Book uploaded successfully but failed to import races: {}", e);
                    // Don't fail the entire upload for catalog import errors
                }
            }

            // Import objects
            match ObjectService::import_objects_from_book(&mut catalog_conn, &final_book_dir, &book_id) {
                Ok(object_count) => {
                    info!("Imported {} objects from book '{}'", object_count, book_name);
                }
                Err(e) => {
                    warn!("Book uploaded successfully but failed to import objects: {}", e);
                    // Don't fail the entire upload for catalog import errors
                }
            }

            // Import traps and hazards
            match TrapService::import_traps_from_book(&mut catalog_conn, &final_book_dir, &book_id) {
                Ok(trap_count) => {
                    info!("Imported {} traps/hazards from book '{}'", trap_count, book_name);
                }
                Err(e) => {
                    warn!("Book uploaded successfully but failed to import traps/hazards: {}", e);
                    // Don't fail the entire upload for catalog import errors
                }
            }
            
            // Import cults and boons
            match CultService::import_cults_from_book(&mut catalog_conn, &final_book_dir, &book_id) {
                Ok(cult_count) => {
                    info!("Imported {} cults/boons from book '{}'", cult_count, book_name);
                }
                Err(e) => {
                    warn!("Book uploaded successfully but failed to import cults/boons: {}", e);
                    // Don't fail the entire upload for catalog import errors
                }
            }

            // Import variant rules
            match VariantRuleService::import_variant_rules_from_book(&mut catalog_conn, &final_book_dir, &book_id) {
                Ok(variant_rule_count) => {
                    info!("Imported {} variant rules from book '{}'", variant_rule_count, book_name);
                }
                Err(e) => {
                    warn!("Book uploaded successfully but failed to import variant rules: {}", e);
                    // Don't fail the entire upload for catalog import errors
                }
            }

            // Import optional features
            match OptionalFeatureService::import_optional_features_from_book(&mut catalog_conn, &final_book_dir, &book_id) {
                Ok(optional_feature_count) => {
                    info!("Imported {} optional features from book '{}'", optional_feature_count, book_name);
                }
                Err(e) => {
                    warn!("Book uploaded successfully but failed to import optional features: {}", e);
                    // Don't fail the entire upload for catalog import errors
                }
            }

            // Import items
            match ItemService::import_items_from_book(&mut catalog_conn, &final_book_dir, &book_id) {
                Ok(item_count) => {
                    info!("Imported {} items from book '{}'", item_count, book_name);
                }
                Err(e) => {
                    warn!("Book uploaded successfully but failed to import items: {}", e);
                    // Don't fail the entire upload for catalog import errors
                }
            }
            // Import monsters
            match MonsterService::import_monsters_from_book(&mut catalog_conn, &final_book_dir, &book_id) {
                Ok(monster_count) => {
                    info!("Imported {} monsters from book '{}'", monster_count, book_name);
                }
                Err(e) => {
                    warn!("Book uploaded successfully but failed to import monsters: {}", e);
                    // Don't fail the entire upload for catalog import errors
                }
            }
            // Import deities
            match DeityService::import_deities_from_book(&mut catalog_conn, &final_book_dir, &book_id) {
                Ok(deity_count) => {
                    info!("Imported {} deities from book '{}'", deity_count, book_name);
                }
                Err(e) => {
                    warn!("Book uploaded successfully but failed to import deities: {}", e);
                    // Don't fail the entire upload for catalog import errors
                }
            }
            // Import vehicles
            match VehicleService::import_vehicles_from_book(&mut catalog_conn, &final_book_dir, &book_id) {
                Ok(vehicle_count) => {
                    info!("Imported {} vehicles from book '{}'", vehicle_count, book_name);
                }
                Err(e) => {
                    warn!("Book uploaded successfully but failed to import vehicles: {}", e);
                    // Don't fail the entire upload for catalog import errors
                }
            }
            // Import classes
            match ClassService::import_classes_from_book(&mut catalog_conn, &final_book_dir, &book_id) {
                Ok(class_count) => {
                    info!("Imported {} classes from book '{}'", class_count, book_name);
                }
                Err(e) => {
                    warn!("Book uploaded successfully but failed to import classes: {}", e);
                    // Don't fail the entire upload for catalog import errors
                }
            }
        }
        Err(e) => {
            warn!("Book uploaded successfully but couldn't connect to database for catalog import: {}", e);
        }
    }
    
    // Return simple BookInfo 
    Ok(ApiResponse::success(BookInfo {
        id: book_id,
        name: book_name,
    }))
}

/// List all books in the library
#[tauri::command]
pub async fn list_library_books() -> Result<ApiResponse<Vec<BookInfo>>, String> {
    info!("Listing library books from database");
    
    match crate::db_connection::get_connection() {
        Ok(mut conn) => {
            match uploaded_books::table.load::<UploadedBook>(&mut conn) {
                Ok(books) => {
                    let book_list: Vec<BookInfo> = books.into_iter()
                        .map(|book| BookInfo {
                            id: book.id,
                            name: book.name,
                        })
                        .collect();
                    
                    info!("Found {} books in library", book_list.len());
                    Ok(ApiResponse::success(book_list))
                }
                Err(e) => {
                    error!("Failed to query books from database: {}", e);
                    Ok(ApiResponse::error("Failed to load books from database".to_string()))
                }
            }
        }
        Err(e) => {
            error!("Database connection error when listing books: {}", e);
            Ok(ApiResponse::error("Database connection error".to_string()))
        }
    }
}

/// Remove a book from the library
#[tauri::command]
pub async fn remove_book_from_library(
    book_id: String,
) -> Result<ApiResponse<()>, String> {
    info!("Removing book from library: {}", book_id);
    
    // First, get book info from database to know what to clean up
    let book_record = match crate::db_connection::get_connection() {
        Ok(mut conn) => {
            match uploaded_books::table
                .filter(uploaded_books::id.eq(&book_id))
                .first::<UploadedBook>(&mut conn)
            {
                Ok(book) => Some(book),
                Err(diesel::NotFound) => {
                    return Ok(ApiResponse::error(format!("Book '{}' not found in database", book_id)));
                }
                Err(e) => {
                    error!("Database error when looking up book: {}", e);
                    return Ok(ApiResponse::error("Database error during lookup".to_string()));
                }
            }
        }
        Err(e) => {
            error!("Database connection error during lookup: {}", e);
            return Ok(ApiResponse::error("Database connection error".to_string()));
        }
    };
    
    if let Some(book) = book_record {
        // Use database transaction for atomic cleanup
        match crate::db_connection::get_connection() {
            Ok(mut conn) => {
                let transaction_result = conn.transaction::<_, diesel::result::Error, _>(|conn| {
                    // Delete from database first
                    diesel::delete(uploaded_books::table.filter(uploaded_books::id.eq(&book_id)))
                        .execute(conn)?;
                    
                    // Delete related catalog data
                    let _ = SpellService::remove_spells_by_source(conn, &book_id);
                    let _ = ActionService::remove_actions_by_source(conn, &book_id);
                    let _ = ConditionService::remove_conditions_by_source(conn, &book_id);
                    let _ = LanguageService::remove_languages_by_source(conn, &book_id);
                    let _ = RewardService::remove_rewards_by_source(conn, &book_id);
                    let _ = BackgroundService::remove_backgrounds_by_source(conn, &book_id);
                    let _ = FeatService::remove_feats_by_source(conn, &book_id);
                    let _ = RaceService::remove_races_by_source(conn, &book_id);
                    let _ = ObjectService::remove_objects_by_source(conn, &book_id);
                    let _ = TrapService::remove_traps_from_source(conn, &book_id);
                    let _ = ItemService::remove_items_from_source(conn, &book_id);
                    let _ = MonsterService::remove_monsters_from_source(conn, &book_id);
                    let _ = DeityService::remove_deities_from_source(conn, &book_id);
                    let _ = VehicleService::remove_vehicles_from_source(conn, &book_id);
                    let _ = ClassService::remove_classes_from_source(conn, &book_id);
                    // We don't want catalog cleanup errors to fail the book removal
                    
                    Ok(())
                });
                
                match transaction_result {
                    Ok(_) => {
                        info!("Successfully removed book '{}' from database", book_id);
                        
                        // Now clean up files
                        let mut cleanup_errors = Vec::new();
                        
                        // Remove extracted directory
                        let book_dir = Path::new(&book.location);
                        if book_dir.exists() {
                            if let Err(e) = fs::remove_dir_all(&book_dir) {
                                error!("Failed to remove book directory: {}", e);
                                cleanup_errors.push(format!("directory: {}", e));
                            }
                        }
                        
                        // Remove archive file
                        let archive_path = Path::new(&book.archive_path);
                        if archive_path.exists() {
                            if let Err(e) = fs::remove_file(&archive_path) {
                                error!("Failed to remove archive file: {}", e);
                                cleanup_errors.push(format!("archive: {}", e));
                            }
                        }
                        
                        if cleanup_errors.is_empty() {
                            info!("Successfully removed all files for book '{}'", book_id);
                            Ok(ApiResponse::success(()))
                        } else {
                            warn!("Book '{}' removed from database but some files couldn't be deleted: {}", 
                                  book_id, cleanup_errors.join(", "));
                            Ok(ApiResponse::success(())) // Still success since database is clean
                        }
                    }
                    Err(e) => {
                        error!("Failed to remove book from database: {}", e);
                        Ok(ApiResponse::error("Failed to remove book from database".to_string()))
                    }
                }
            }
            Err(e) => {
                error!("Database connection error during removal: {}", e);
                Ok(ApiResponse::error("Database connection error".to_string()))
            }
        }
    } else {
        Ok(ApiResponse::error(format!("Book '{}' not found", book_id)))
    }
}

/// Get book content from the archive structure
#[tauri::command]
pub async fn get_book_content(
    book_id: String,
) -> Result<ApiResponse<serde_json::Value>, String> {
    info!("Getting book content for: {}", book_id);
    
    // Get app paths
    let app_paths = APP_PATHS.get()
        .ok_or_else(|| "App paths not initialized".to_string())?;
    
    // Get book directory
    let book_dir = app_paths.data_dir
        .join("books")
        .join(&book_id);
    
    info!("Looking for book at: {:?}", book_dir);
    
    if !book_dir.exists() {
        error!("Book directory does not exist: {:?}", book_dir);
        return Ok(ApiResponse::error(format!("Book not found: {}", book_id)));
    }
    
    // List contents of book directory for debugging
    info!("Book directory contents:");
    if let Ok(entries) = fs::read_dir(&book_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                info!("  - {:?} ({})", 
                    entry.file_name(), 
                    if entry.path().is_dir() { "dir" } else { "file" }
                );
            }
        }
    }
    
    // Find the main book content file
    info!("Searching for book content file...");
    let book_content_path = find_book_content_file(&book_dir)?
        .ok_or_else(|| {
            error!("No book content file found in {:?}", book_dir);
            format!("No book content found for: {}", book_id)
        })?;
    
    // Read and parse JSON
    match fs::read_to_string(&book_content_path) {
        Ok(content) => {
            match serde_json::from_str::<serde_json::Value>(&content) {
                Ok(json) => Ok(ApiResponse::success(json)),
                Err(e) => {
                    error!("Failed to parse book JSON: {}", e);
                    Ok(ApiResponse::error(format!("Failed to parse book content: {}", e)))
                }
            }
        }
        Err(e) => {
            error!("Failed to read book file: {}", e);
            Ok(ApiResponse::error(format!("Failed to read book content: {}", e)))
        }
    }
}

/// Serve an image from a book's archive structure as base64
#[tauri::command]
pub async fn serve_book_image(
    book_id: String,
    image_path: String,
) -> Result<ApiResponse<String>, String> {
    info!("Serving image {} from book {}", image_path, book_id);
    
    let app_paths = APP_PATHS.get()
        .ok_or_else(|| "App paths not initialized".to_string())?;
    
    let books_dir = app_paths.data_dir.join("books");
    
    // Sanitize inputs to prevent directory traversal
    let sanitized_book = book_id.replace("..", "").replace("/", "").replace("\\", "");
    let sanitized_image = image_path.replace("..", "");
    
    // The image path from JSON is like "book/PHB/image.webp" but files are at "img/book/PHB/image.webp"
    // So we need to prepend "img/" if it's not already there
    let image_path_with_img = if sanitized_image.starts_with("img/") {
        sanitized_image.clone()
    } else {
        format!("img/{}", sanitized_image)
    };
    
    let full_image_path = books_dir
        .join(&sanitized_book)
        .join(&image_path_with_img);
    
    if !full_image_path.exists() {
        error!("Image not found: {:?}", full_image_path);
        return Ok(ApiResponse::error("Image not found".to_string()));
    }
    
    // Read the image file
    match fs::read(&full_image_path) {
        Ok(image_data) => {
            // Determine MIME type based on extension
            let mime_type = match full_image_path.extension().and_then(|ext| ext.to_str()) {
                Some("png") => "image/png",
                Some("jpg") | Some("jpeg") => "image/jpeg",
                Some("webp") => "image/webp",
                Some("gif") => "image/gif",
                _ => "image/png", // Default to PNG
            };
            
            // Encode as base64 data URL
            let base64_data = STANDARD.encode(&image_data);
            let data_url = format!("data:{};base64,{}", mime_type, base64_data);
            
            info!("Successfully served image: {} ({}KB)", image_path_with_img, image_data.len() / 1024);
            Ok(ApiResponse::success(data_url))
        }
        Err(e) => {
            error!("Failed to read image file: {}", e);
            Ok(ApiResponse::error(format!("Failed to read image: {}", e)))
        }
    }
}

// Helper structures
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BookInfo {
    pub id: String,           // Book ID (e.g., "phb", "dmg")
    pub name: String,         // Display name (e.g., "Player's Handbook")
}

// Helper functions

/// Find the main book content file in the archive structure
fn find_book_content_file(dir: &Path) -> Result<Option<PathBuf>, String> {
    info!("find_book_content_file: searching in {:?}", dir);
    
    // Check for book directory with book-*.json files
    let book_dir = dir.join("book");
    info!("Checking for book subdirectory at: {:?}", book_dir);
    
    if book_dir.exists() {
        info!("Book subdirectory exists, listing contents:");
        for entry in fs::read_dir(&book_dir)
            .map_err(|e| format!("Failed to read book directory: {}", e))? {
            
            let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
            let path = entry.path();
            
            info!("  - Found: {:?} (is_file: {})", path, path.is_file());
            
            if path.is_file() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    info!("    Checking filename: {}", name);
                    if name.starts_with("book-") && name.ends_with(".json") {
                        info!("    ✓ Found book content file: {:?}", path);
                        return Ok(Some(path));
                    }
                }
            }
        }
        info!("No matching book-*.json files found in book subdirectory");
    } else {
        info!("Book subdirectory does not exist");
    }
    
    // Check for direct book.json in root
    let root_book = dir.join("book.json");
    info!("Checking for book.json in root: {:?}", root_book);
    if root_book.exists() {
        info!("Found book.json in root");
        return Ok(Some(root_book));
    }
    
    info!("No book content file found");
    Ok(None)
}


/// Copy directory recursively
fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<(), std::io::Error> {
    fs::create_dir_all(dst)?;
    
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dst_path = dst.join(entry.file_name());
        
        if path.is_dir() {
            copy_dir_recursive(&path, &dst_path)?;
        } else {
            fs::copy(&path, &dst_path)?;
        }
    }
    
    Ok(())
}

/// Reference lookup data structure
#[derive(Debug, Serialize, Deserialize)]
pub struct ReferenceData {
    pub ref_type: String,
    pub name: String,
    pub source: Option<String>,
    pub data: Value,
    pub preview: String,
}

/// Look up a cross-reference in the book data
#[tauri::command]
pub async fn lookup_reference(
    ref_type: String,
    ref_name: String,
    ref_source: Option<String>,
) -> Result<ApiResponse<ReferenceData>, String> {
    info!("Looking up reference: {} '{}' from {:?}", ref_type, ref_name, ref_source);
    
    let app_paths = APP_PATHS.get()
        .ok_or_else(|| "App paths not initialized".to_string())?;
    
    let books_dir = app_paths.data_dir.join("books");
    
    // Determine which book to search in
    let source_book = ref_source.as_deref().unwrap_or("PHB").to_lowercase();
    
    // Map source codes to book IDs
    let book_id = match source_book.as_str() {
        "phb" => "PHB",
        "dmg" => "DMG",
        "mm" => "MM",
        "test-book" => "test-book",
        "test-book-two" => "test-book-two",
        "tb2" => "test-book-two",
        "test" => "test-book",
        _ => &source_book,
    };
    
    // Try to find the reference in the specified book
    let book_dir = books_dir.join(book_id);
    if !book_dir.exists() {
        // If specific book not found, search all books
        return search_all_books_for_reference(&books_dir, &ref_type, &ref_name).await;
    }
    
    // Search in the specific book
    match search_book_for_reference(&book_dir, &ref_type, &ref_name).await {
        Ok(Some(data)) => Ok(ApiResponse::success(data)),
        Ok(None) => {
            // Not found in specified book, search all
            search_all_books_for_reference(&books_dir, &ref_type, &ref_name).await
        }
        Err(e) => Err(e),
    }
}

/// Search a specific book for a reference
async fn search_book_for_reference(
    book_dir: &Path,
    ref_type: &str,
    ref_name: &str,
) -> Result<Option<ReferenceData>, String> {
    debug!("Searching book {:?} for {} '{}'", book_dir, ref_type, ref_name);
    
    // Map reference types to data file patterns
    let file_patterns = match ref_type {
        "spell" => vec!["spells-*.json", "*.json", "book-*.json"],
        "item" => vec!["items-*.json", "*.json", "book-*.json"],
        "creature" | "monster" => vec!["bestiary-*.json", "*.json", "book-*.json"],
        "class" => vec!["class-*.json", "*.json", "book-*.json"],
        "race" => vec!["race-*.json", "*.json", "book-*.json"],
        "feat" => vec!["feats-*.json", "*.json", "book-*.json"],
        "background" => vec!["backgrounds-*.json", "*.json", "book-*.json"],
        _ => vec!["book-*.json"],
    };
    
    // Check data subdirectory first, then type-specific directories
    let data_dir = book_dir.join("data");
    let mut search_dirs = if data_dir.exists() {
        vec![data_dir, book_dir.join("book")]
    } else {
        vec![book_dir.join("book")]
    };
    
    // Add type-specific directories
    match ref_type {
        "spell" => search_dirs.push(book_dir.join("spells")),
        "item" => search_dirs.push(book_dir.join("items")),
        "creature" | "monster" => search_dirs.push(book_dir.join("bestiary")),
        "class" => search_dirs.push(book_dir.join("class")),
        "race" => search_dirs.push(book_dir.join("races")),
        "feat" => search_dirs.push(book_dir.join("feats")),
        "background" => search_dirs.push(book_dir.join("backgrounds")),
        _ => {}
    }
    
    for dir in search_dirs {
        if !dir.exists() {
            continue;
        }
        
        // Search through relevant JSON files
        for pattern in &file_patterns {
            if let Ok(entries) = fs::read_dir(&dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    let file_name = path.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("");
                    
                    // Check if filename matches pattern
                    if matches_pattern(file_name, pattern) {
                        if let Ok(content) = fs::read_to_string(&path) {
                            if let Ok(json) = serde_json::from_str::<Value>(&content) {
                                if let Some(data) = find_reference_in_json(&json, ref_type, ref_name) {
                                    return Ok(Some(data));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    Ok(None)
}

/// Search all books for a reference
async fn search_all_books_for_reference(
    books_dir: &Path,
    ref_type: &str,
    ref_name: &str,
) -> Result<ApiResponse<ReferenceData>, String> {
    debug!("Searching all books for {} '{}'", ref_type, ref_name);
    
    if let Ok(entries) = fs::read_dir(books_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Ok(Some(data)) = search_book_for_reference(&path, ref_type, ref_name).await {
                    return Ok(ApiResponse::success(data));
                }
            }
        }
    }
    
    Ok(ApiResponse::error(format!("Reference not found: {} '{}'", ref_type, ref_name)))
}

/// Check if a filename matches a pattern (simple glob)
fn matches_pattern(filename: &str, pattern: &str) -> bool {
    if pattern.contains('*') {
        let parts: Vec<&str> = pattern.split('*').collect();
        if parts.len() == 2 {
            return filename.starts_with(parts[0]) && filename.ends_with(parts[1]);
        }
    }
    filename == pattern
}

/// Find a reference in a JSON data structure
fn find_reference_in_json(json: &Value, ref_type: &str, ref_name: &str) -> Option<ReferenceData> {
    let ref_name_lower = ref_name.to_lowercase();
    
    // Check for typed arrays (spell, item, creature, etc.)
    let type_key = match ref_type {
        "creature" | "monster" => "monster",
        other => other,
    };
    
    if let Some(array) = json.get(type_key).and_then(|v| v.as_array()) {
        for item in array {
            if let Some(name) = item.get("name").and_then(|v| v.as_str()) {
                if name.to_lowercase() == ref_name_lower {
                    return Some(create_reference_data(ref_type, name, item));
                }
            }
        }
    }
    
    // Check in book data structure
    if let Some(data_array) = json.get("data").and_then(|v| v.as_array()) {
        for section in data_array {
            if let Some(found) = search_entries_for_reference(section, ref_type, &ref_name_lower) {
                return Some(found);
            }
        }
    }
    
    None
}

/// Search through entries recursively for references
fn search_entries_for_reference(entry: &Value, ref_type: &str, ref_name_lower: &str) -> Option<ReferenceData> {
    // Check if this entry is the reference we're looking for
    if let Some(entry_type) = entry.get("type").and_then(|v| v.as_str()) {
        if entry_type == ref_type || (ref_type == "spell" && entry_type == "spellList") {
            if let Some(name) = entry.get("name").and_then(|v| v.as_str()) {
                if name.to_lowercase() == *ref_name_lower {
                    return Some(create_reference_data(ref_type, name, entry));
                }
            }
        }
    }
    
    // Search in entries array
    if let Some(entries) = entry.get("entries").and_then(|v| v.as_array()) {
        for sub_entry in entries {
            if let Some(found) = search_entries_for_reference(sub_entry, ref_type, ref_name_lower) {
                return Some(found);
            }
        }
    }
    
    None
}

/// Create a ReferenceData object from JSON data
fn create_reference_data(ref_type: &str, name: &str, data: &Value) -> ReferenceData {
    let preview = generate_preview(ref_type, data);
    
    ReferenceData {
        ref_type: ref_type.to_string(),
        name: name.to_string(),
        source: data.get("source").and_then(|v| v.as_str()).map(|s| s.to_string()),
        data: data.clone(),
        preview,
    }
}

/// Generate a preview string for tooltips
fn generate_preview(ref_type: &str, data: &Value) -> String {
    match ref_type {
        "spell" => {
            let level = data.get("level").and_then(|v| v.as_u64()).unwrap_or(0);
            let school = data.get("school").and_then(|v| v.as_str())
                .map(|s| get_spell_school_name(s))
                .unwrap_or("Unknown");
            let range = format_spell_range(data.get("range"));
            
            if level == 0 {
                format!("Cantrip • {}<br/>{}", school, range)
            } else {
                format!("Level {} • {}<br/>{}", level, school, range)
            }
        }
        "item" => {
            let item_type = data.get("type").and_then(|v| v.as_str()).unwrap_or("Item");
            let rarity = data.get("rarity").and_then(|v| v.as_str()).unwrap_or("");
            let value = data.get("value").and_then(|v| v.as_u64()).map(|v| format!("{} gp", v)).unwrap_or_default();
            
            format!("{}{}<br/>{}", 
                item_type,
                if !rarity.is_empty() { format!(" • {}", rarity) } else { String::new() },
                value
            )
        }
        "creature" | "monster" => {
            let cr = data.get("cr").and_then(|v| {
                if let Some(s) = v.as_str() {
                    Some(s.to_string())
                } else if let Some(obj) = v.as_object() {
                    obj.get("cr").and_then(|c| c.as_str()).map(|s| s.to_string())
                } else {
                    None
                }
            }).unwrap_or_else(|| "?".to_string());
            
            let type_str = data.get("type").and_then(|v| {
                if let Some(s) = v.as_str() {
                    Some(s.to_string())
                } else if let Some(obj) = v.as_object() {
                    obj.get("type").and_then(|t| t.as_str()).map(|s| s.to_string())
                } else {
                    None
                }
            }).unwrap_or_else(|| "creature".to_string());
            
            let ac = data.get("ac").and_then(|v| {
                if let Some(n) = v.as_u64() {
                    Some(n.to_string())
                } else if let Some(arr) = v.as_array() {
                    arr.first().and_then(|a| {
                        if let Some(n) = a.as_u64() {
                            Some(n.to_string())
                        } else if let Some(obj) = a.as_object() {
                            obj.get("ac").and_then(|ac| ac.as_u64()).map(|n| n.to_string())
                        } else {
                            None
                        }
                    })
                } else {
                    None
                }
            }).unwrap_or_else(|| "?".to_string());
            
            let hp = data.get("hp").and_then(|v| {
                if let Some(obj) = v.as_object() {
                    obj.get("average").and_then(|a| a.as_u64()).map(|n| n.to_string())
                } else {
                    None
                }
            }).unwrap_or_else(|| "?".to_string());
            
            format!("{} • CR {}<br/>AC {}, HP {}", type_str, cr, ac, hp)
        }
        "class" => {
            let hd = data.get("hd").and_then(|v| v.as_object())
                .and_then(|o| o.get("faces").and_then(|f| f.as_u64()))
                .map(|d| format!("d{}", d))
                .unwrap_or_else(|| "d?".to_string());
            
            format!("Class • {} Hit Die", hd)
        }
        _ => format!("{}: {}", ref_type, data.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown"))
    }
}

/// Get spell school full name from abbreviation
fn get_spell_school_name(abbr: &str) -> &'static str {
    match abbr {
        "A" => "Abjuration",
        "C" => "Conjuration", 
        "D" => "Divination",
        "E" => "Enchantment",
        "V" => "Evocation",
        "I" => "Illusion",
        "N" => "Necromancy",
        "T" => "Transmutation",
        _ => "Unknown",
    }
}

/// Format spell range for display
fn format_spell_range(range: Option<&Value>) -> String {
    if let Some(range_val) = range {
        if let Some(range_type) = range_val.get("type").and_then(|v| v.as_str()) {
            match range_type {
                "point" => {
                    if let Some(distance) = range_val.get("distance") {
                        if let Some(dist_type) = distance.get("type").and_then(|v| v.as_str()) {
                            if let Some(amount) = distance.get("amount").and_then(|v| v.as_u64()) {
                                return format!("Range: {} {}", amount, dist_type);
                            }
                        }
                    }
                }
                "self" => return "Range: Self".to_string(),
                "touch" => return "Range: Touch".to_string(),
                "sight" => return "Range: Sight".to_string(),
                "unlimited" => return "Range: Unlimited".to_string(),
                _ => {}
            }
        }
    }
    "Range: Varies".to_string()
}