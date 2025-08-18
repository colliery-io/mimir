//! Development tools and commands

use crate::{
    embedded_test_book::{extract_test_book, extract_test_book_two, is_dev_build},
    types::ApiResponse,
    APP_PATHS,
};
use std::fs;
use tracing::{info, error};

/// Check if we're in development mode
#[tauri::command]
pub async fn is_dev_mode() -> Result<bool, String> {
    Ok(is_dev_build())
}

/// Install the development test book if in dev mode
#[tauri::command]
pub async fn install_dev_test_book() -> Result<ApiResponse<String>, String> {
    if !is_dev_build() {
        return Ok(ApiResponse::error("Not in development mode".to_string()));
    }
    
    info!("Installing development test book");
    
    // Get app paths
    let app_paths = APP_PATHS.get()
        .ok_or_else(|| "App paths not initialized".to_string())?;
    
    // Get books directory
    let books_dir = app_paths.data_dir.join("books");
    let test_book_dir = books_dir.join("test-book");
    let test_book_two_dir = books_dir.join("test-book-two");
    
    // ALWAYS remove and reinstall in dev to ensure latest version
    if test_book_dir.exists() {
        info!("Removing existing dev test book at: {:?}", test_book_dir);
        fs::remove_dir_all(&test_book_dir)
            .map_err(|e| format!("Failed to remove old test book: {}", e))?;
        info!("Successfully removed old test book");
    }
    
    if test_book_two_dir.exists() {
        info!("Removing existing test book two at: {:?}", test_book_two_dir);
        fs::remove_dir_all(&test_book_two_dir)
            .map_err(|e| format!("Failed to remove old test book two: {}", e))?;
        info!("Successfully removed old test book two");
    }
    
    // Create books directory if needed
    if !books_dir.exists() {
        fs::create_dir_all(&books_dir)
            .map_err(|e| format!("Failed to create books directory: {}", e))?;
    }
    
    // Extract the embedded test book archives
    info!("Extracting test book archive to: {:?}", books_dir);
    extract_test_book(&books_dir)
        .map_err(|e| format!("Failed to extract test book: {}", e))?;
    info!("Test book one extraction completed");
    
    info!("Extracting test book two archive to: {:?}", books_dir);
    extract_test_book_two(&books_dir)
        .map_err(|e| format!("Failed to extract test book two: {}", e))?;
    info!("Test book two extraction completed");
    
    // Verify extraction
    if test_book_dir.exists() {
        info!("Test book directory confirmed at: {:?}", test_book_dir);
        
        // List contents
        if let Ok(entries) = fs::read_dir(&test_book_dir) {
            info!("Test book contents:");
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    info!("  - {:?} ({})", 
                        entry.file_name(),
                        if path.is_dir() { "dir" } else { "file" }
                    );
                    
                    // If it's the book dir, list its contents too
                    if path.is_dir() && entry.file_name() == "book" {
                        if let Ok(book_entries) = fs::read_dir(&path) {
                            info!("    Book subdirectory contents:");
                            for book_entry in book_entries {
                                if let Ok(book_entry) = book_entry {
                                    info!("      - {:?}", book_entry.file_name());
                                }
                            }
                        }
                    }
                    
                    // Check metadata.json
                    if entry.file_name() == "metadata.json" {
                        if let Ok(content) = fs::read_to_string(&path) {
                            info!("    Metadata content: {}", content);
                        }
                    }
                }
            }
        }
    } else {
        error!("Test book directory was not created!");
    }
    
    if test_book_two_dir.exists() {
        info!("Test book two directory confirmed at: {:?}", test_book_two_dir);
    } else {
        error!("Test book two directory was not created!");
    }
    
    info!("Successfully installed dev test books");
    Ok(ApiResponse::success("Test books installed successfully".to_string()))
}

/// Remove the development test book
#[tauri::command]
pub async fn remove_dev_test_book() -> Result<ApiResponse<()>, String> {
    if !is_dev_build() {
        return Ok(ApiResponse::error("Not in development mode".to_string()));
    }
    
    let app_paths = APP_PATHS.get()
        .ok_or_else(|| "App paths not initialized".to_string())?;
    
    let test_book_dir = app_paths.data_dir.join("books").join("test-book");
    
    if test_book_dir.exists() {
        fs::remove_dir_all(&test_book_dir)
            .map_err(|e| format!("Failed to remove test book: {}", e))?;
        info!("Removed dev test book");
    }
    
    Ok(ApiResponse::success(()))
}