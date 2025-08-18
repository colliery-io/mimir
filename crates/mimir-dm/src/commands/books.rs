//! Book library management commands

use crate::{
    types::ApiResponse,
    APP_PATHS,
};
use std::fs;
use std::path::Path;
use tracing::{error, info};
use base64::{engine::general_purpose::STANDARD, Engine};

/// Add a book file to the library with a custom name
#[tauri::command]
pub async fn add_book_to_library(
    book_name: String,
    file_path: String,
) -> Result<ApiResponse<String>, String> {
    info!("Adding book '{}' from: {}", book_name, file_path);
    
    // Get app paths
    let app_paths = APP_PATHS.get()
        .ok_or_else(|| "App paths not initialized".to_string())?;
    
    // Sanitize book name for folder creation
    let sanitized_name = sanitize_folder_name(&book_name);
    
    // Create books directory if it doesn't exist
    let books_dir = app_paths.data_dir.join("books");
    if !books_dir.exists() {
        fs::create_dir_all(&books_dir)
            .map_err(|e| format!("Failed to create books directory: {}", e))?;
        info!("Created books directory at: {:?}", books_dir);
    }
    
    // Create book folder
    let book_dir = books_dir.join(&sanitized_name);
    if book_dir.exists() {
        return Ok(ApiResponse::error(format!("Book '{}' already exists", book_name)));
    }
    
    fs::create_dir_all(&book_dir)
        .map_err(|e| format!("Failed to create book directory: {}", e))?;
    
    // Create images folder
    let images_dir = book_dir.join("images");
    fs::create_dir_all(&images_dir)
        .map_err(|e| format!("Failed to create images directory: {}", e))?;
    
    // Get source file path
    let source_path = Path::new(&file_path);
    if !source_path.exists() {
        return Ok(ApiResponse::error(format!("File not found: {}", file_path)));
    }
    
    // Copy book JSON as book.json
    let dest_path = book_dir.join("book.json");
    match fs::copy(&source_path, &dest_path) {
        Ok(bytes_copied) => {
            info!("Successfully copied {} bytes to {:?}", bytes_copied, dest_path);
            
            // Save metadata file with the display name
            let metadata_path = book_dir.join("metadata.json");
            let metadata = BookMetadata {
                display_name: book_name.clone(),
                folder_name: sanitized_name.clone(),
                added_date: chrono::Utc::now().to_rfc3339(),
            };
            
            let metadata_json = serde_json::to_string_pretty(&metadata)
                .map_err(|e| format!("Failed to serialize metadata: {}", e))?;
            fs::write(&metadata_path, metadata_json)
                .map_err(|e| format!("Failed to write metadata: {}", e))?;
            
            Ok(ApiResponse::success(sanitized_name))
        }
        Err(e) => {
            // Clean up created directory on failure
            let _ = fs::remove_dir_all(&book_dir);
            error!("Failed to copy file: {}", e);
            Ok(ApiResponse::error(format!("Failed to copy file: {}", e)))
        }
    }
}

/// List all books in the library
#[tauri::command]
pub async fn list_library_books() -> Result<ApiResponse<Vec<BookInfo>>, String> {
    info!("Listing library books");
    
    // Get app paths
    let app_paths = APP_PATHS.get()
        .ok_or_else(|| "App paths not initialized".to_string())?;
    
    // Get books directory
    let books_dir = app_paths.data_dir.join("books");
    if !books_dir.exists() {
        // No books directory yet, return empty list
        return Ok(ApiResponse::success(vec![]));
    }
    
    // Read directory and collect book info
    let mut books = Vec::new();
    
    match fs::read_dir(&books_dir) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    // Look for directories (each book is in its own folder)
                    if path.is_dir() {
                        // Check if it has a book.json file
                        let book_json_path = path.join("book.json");
                        if book_json_path.exists() {
                            // Read metadata to get display name
                            let metadata_path = path.join("metadata.json");
                            let display_name = if metadata_path.exists() {
                                match fs::read_to_string(&metadata_path) {
                                    Ok(content) => {
                                        serde_json::from_str::<BookMetadata>(&content)
                                            .map(|m| m.display_name)
                                            .unwrap_or_else(|_| {
                                                path.file_name()
                                                    .map(|n| n.to_string_lossy().to_string())
                                                    .unwrap_or_else(|| "Unknown".to_string())
                                            })
                                    }
                                    Err(_) => {
                                        path.file_name()
                                            .map(|n| n.to_string_lossy().to_string())
                                            .unwrap_or_else(|| "Unknown".to_string())
                                    }
                                }
                            } else {
                                path.file_name()
                                    .map(|n| n.to_string_lossy().to_string())
                                    .unwrap_or_else(|| "Unknown".to_string())
                            };
                            
                            // Get book.json size
                            let size = fs::metadata(&book_json_path)
                                .map(|m| m.len())
                                .unwrap_or(0);
                            
                            // Count images in images folder
                            let images_dir = path.join("images");
                            let image_count = if images_dir.exists() {
                                fs::read_dir(&images_dir)
                                    .map(|entries| entries.filter(|e| e.is_ok()).count())
                                    .unwrap_or(0)
                            } else {
                                0
                            };
                            
                            books.push(BookInfo {
                                name: display_name,
                                folder_name: path.file_name()
                                    .map(|n| n.to_string_lossy().to_string())
                                    .unwrap_or_else(|| "unknown".to_string()),
                                size_bytes: size,
                                image_count,
                            });
                        }
                    }
                }
            }
            
            info!("Found {} books in library", books.len());
            Ok(ApiResponse::success(books))
        }
        Err(e) => {
            error!("Failed to read books directory: {}", e);
            Ok(ApiResponse::error(format!("Failed to read books directory: {}", e)))
        }
    }
}

/// Remove a book from the library
#[tauri::command]
pub async fn remove_book_from_library(
    folder_name: String,
) -> Result<ApiResponse<()>, String> {
    info!("Removing book from library: {}", folder_name);
    
    // Get app paths
    let app_paths = APP_PATHS.get()
        .ok_or_else(|| "App paths not initialized".to_string())?;
    
    // Get book directory path
    let book_dir = app_paths.data_dir.join("books").join(&folder_name);
    
    if !book_dir.exists() {
        return Ok(ApiResponse::error(format!("Book not found: {}", folder_name)));
    }
    
    // Delete the entire directory
    match fs::remove_dir_all(&book_dir) {
        Ok(_) => {
            info!("Successfully removed book: {}", folder_name);
            Ok(ApiResponse::success(()))
        }
        Err(e) => {
            error!("Failed to remove book: {}", e);
            Ok(ApiResponse::error(format!("Failed to remove book: {}", e)))
        }
    }
}

/// Get book content from JSON file
#[tauri::command]
pub async fn get_book_content(
    folder_name: String,
) -> Result<ApiResponse<serde_json::Value>, String> {
    info!("Getting book content for: {}", folder_name);
    
    // Get app paths
    let app_paths = APP_PATHS.get()
        .ok_or_else(|| "App paths not initialized".to_string())?;
    
    // Get book.json path
    let book_json_path = app_paths.data_dir
        .join("books")
        .join(&folder_name)
        .join("book.json");
    
    if !book_json_path.exists() {
        return Ok(ApiResponse::error(format!("Book content not found: {}", folder_name)));
    }
    
    // Read and parse JSON
    match fs::read_to_string(&book_json_path) {
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

/// Add images to an existing book
#[tauri::command]
pub async fn add_book_images(
    folder_name: String,
    image_paths: Vec<String>,
) -> Result<ApiResponse<usize>, String> {
    info!("Adding {} images to book: {}", image_paths.len(), folder_name);
    
    // Get app paths
    let app_paths = APP_PATHS.get()
        .ok_or_else(|| "App paths not initialized".to_string())?;
    
    // Get images directory
    let images_dir = app_paths.data_dir
        .join("books")
        .join(&folder_name)
        .join("images");
    
    if !images_dir.exists() {
        return Ok(ApiResponse::error(format!("Book not found: {}", folder_name)));
    }
    
    let mut copied_count = 0;
    
    for image_path in image_paths {
        let source = Path::new(&image_path);
        if source.exists() {
            if let Some(file_name) = source.file_name() {
                let dest = images_dir.join(file_name);
                if let Ok(_) = fs::copy(&source, &dest) {
                    copied_count += 1;
                    info!("Copied image: {:?}", file_name);
                }
            }
        }
    }
    
    info!("Successfully copied {} images", copied_count);
    Ok(ApiResponse::success(copied_count))
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BookInfo {
    pub name: String,           // Display name
    pub folder_name: String,    // Folder name on disk
    pub size_bytes: u64,
    pub image_count: usize,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct BookMetadata {
    display_name: String,
    folder_name: String,
    added_date: String,
}

/// Sanitize a name to be safe for folder creation
fn sanitize_folder_name(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '-',
            _ => c,
        })
        .collect::<String>()
        .trim()
        .to_lowercase()
        .replace(' ', "-")
}

/// Serve an image from a book's folder as base64
#[tauri::command]
pub async fn serve_book_image(
    folder_name: String,
    image_name: String,
) -> Result<ApiResponse<String>, String> {
    info!("Serving image {} from book {}", image_name, folder_name);
    
    let app_paths = APP_PATHS.get()
        .ok_or_else(|| "App paths not initialized".to_string())?;
    
    let books_dir = app_paths.data_dir.join("books");
    
    // Sanitize the folder name to prevent directory traversal
    let sanitized_folder = folder_name.replace("..", "").replace("/", "").replace("\\", "");
    let sanitized_image = image_name.replace("..", "").replace("/", "").replace("\\", "");
    
    let image_path = books_dir
        .join(&sanitized_folder)
        .join("images")
        .join(&sanitized_image);
    
    if !image_path.exists() {
        error!("Image not found: {:?}", image_path);
        return Ok(ApiResponse::error("Image not found".to_string()));
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
            
            info!("Successfully served image: {} ({}KB)", sanitized_image, image_data.len() / 1024);
            Ok(ApiResponse::success(data_url))
        }
        Err(e) => {
            error!("Failed to read image file: {}", e);
            Ok(ApiResponse::error(format!("Failed to read image: {}", e)))
        }
    }
}