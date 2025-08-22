//! Development tools and commands

use crate::{
    embedded_test_book::{extract_all_test_books, get_embedded_test_books, is_dev_build},
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
    
    // Get list of embedded test books
    let test_books = get_embedded_test_books();
    info!("Found {} embedded test books to install", test_books.len());
    
    // ALWAYS remove and reinstall in dev to ensure latest version
    // Remove all existing test book directories
    for book in &test_books {
        let book_dir = books_dir.join(&book.name);
        if book_dir.exists() {
            info!("Removing existing test book at: {:?}", book_dir);
            fs::remove_dir_all(&book_dir)
                .map_err(|e| format!("Failed to remove old test book {}: {}", book.name, e))?;
            info!("Successfully removed old test book: {}", book.name);
        }
    }
    
    // Create books directory if needed
    if !books_dir.exists() {
        fs::create_dir_all(&books_dir)
            .map_err(|e| format!("Failed to create books directory: {}", e))?;
    }
    
    // Extract all embedded test book archives
    info!("Extracting all test book archives to: {:?}", books_dir);
    extract_all_test_books(&books_dir)
        .map_err(|e| format!("Failed to extract test books: {}", e))?;
    info!("All test book extractions completed");
    
    // Verify extraction of all test books
    for book in &test_books {
        let book_dir = books_dir.join(&book.name);
        if book_dir.exists() {
            info!("Test book '{}' directory confirmed at: {:?}", book.name, book_dir);
        
            // List contents
            if let Ok(entries) = fs::read_dir(&book_dir) {
                info!("Test book '{}' contents:", book.name);
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        info!("  - {:?} ({})", 
                            entry.file_name(),
                            if path.is_dir() { "dir" } else { "file" }
                        );
                        
                        // If it's the data dir, list its contents too
                        if path.is_dir() && entry.file_name() == "data" {
                            if let Ok(data_entries) = fs::read_dir(&path) {
                                info!("    Data subdirectory contents:");
                                for data_entry in data_entries {
                                    if let Ok(data_entry) = data_entry {
                                        info!("      - {:?}", data_entry.file_name());
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
            error!("Test book '{}' directory was not created!", book.name);
        }
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
    
    let books_dir = app_paths.data_dir.join("books");
    let test_books = get_embedded_test_books();
    
    // Remove all test book directories
    for book in &test_books {
        let book_dir = books_dir.join(&book.name);
        if book_dir.exists() {
            fs::remove_dir_all(&book_dir)
                .map_err(|e| format!("Failed to remove test book {}: {}", book.name, e))?;
            info!("Removed dev test book: {}", book.name);
        }
    }
    
    Ok(ApiResponse::success(()))
}