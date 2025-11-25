//! Development tools and commands.
//!
//! Provides Tauri commands for development-only functionality such as
//! test data management and debugging features.

use crate::app_init::AppPaths;
use crate::{
    embedded_test_book::{get_embedded_test_books, is_dev_build},
    types::ApiResponse,
};
use std::fs;
use std::sync::Arc;
use tauri::State;
use tracing::info;

/// Check if the application is running in development mode.
///
/// Returns true if this is a development build.
///
/// # Returns
/// Boolean indicating development mode status.
#[tauri::command]
pub async fn is_dev_mode() -> Result<bool, String> {
    Ok(is_dev_build())
}

/// Remove all embedded development test books.
///
/// Deletes the test book directories created during development.
/// Only works when running in development mode.
///
/// # Returns
/// `ApiResponse` indicating success.
///
/// # Errors
/// Returns error response if not in development mode or deletion fails.
#[tauri::command]
pub async fn remove_dev_test_book(
    app_paths: State<'_, Arc<AppPaths>>
) -> Result<ApiResponse<()>, String> {
    if !is_dev_build() {
        return Ok(ApiResponse::error("Not in development mode".to_string()));
    }
    
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