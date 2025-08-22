//! Embedded test books for development builds
//! Automatically loads all .tar.gz files from assets/dev/

use tar::Archive;
use flate2::read::GzDecoder;
use std::collections::HashMap;
use tracing::{info, error};

/// Structure to hold embedded test book data
pub struct EmbeddedTestBook {
    pub name: String,
    pub data: &'static [u8],
}

/// Macro to include all test books from the assets/dev directory
/// This is expanded at compile time
macro_rules! include_test_books {
    () => {{
        vec![
            EmbeddedTestBook {
                name: "PHB".to_string(),
                data: include_bytes!("../assets/dev/phb.tar.gz"),
            },
            EmbeddedTestBook {
                name: "MM".to_string(),
                data: include_bytes!("../assets/dev/mm.tar.gz"),
            },
            EmbeddedTestBook {
                name: "DMG".to_string(),
                data: include_bytes!("../assets/dev/dmg.tar.gz"),
            },
        ]
    }};
}

/// Check if we're in development mode
pub fn is_dev_build() -> bool {
    cfg!(debug_assertions) || std::env::var("MIMIR_DEV").is_ok()
}

/// Get all embedded test books
pub fn get_embedded_test_books() -> Vec<EmbeddedTestBook> {
    include_test_books!()
}

/// Extract all embedded test book archives
pub fn extract_all_test_books(target_dir: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let test_books = get_embedded_test_books();
    
    info!("Extracting {} embedded test books to {:?}", test_books.len(), target_dir);
    
    for book in test_books {
        match extract_single_book(&book, target_dir) {
            Ok(_) => info!("Successfully extracted test book: {}", book.name),
            Err(e) => error!("Failed to extract test book {}: {}", book.name, e),
        }
    }
    
    Ok(())
}

/// Extract a single test book archive
fn extract_single_book(book: &EmbeddedTestBook, target_dir: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    // Create a decoder from the embedded bytes
    let decoder = GzDecoder::new(book.data);
    let mut archive = Archive::new(decoder);
    
    // Extract the archive
    archive.unpack(target_dir)?;
    
    Ok(())
}

/// Get a map of test book names to their data
pub fn get_test_books_map() -> HashMap<String, &'static [u8]> {
    let mut map = HashMap::new();
    for book in get_embedded_test_books() {
        map.insert(book.name, book.data);
    }
    map
}

// Legacy functions for backwards compatibility
/// Extract the first test book archive (backwards compatibility)
pub fn extract_test_book(target_dir: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let books = get_embedded_test_books();
    if let Some(book) = books.first() {
        extract_single_book(book, target_dir)
    } else {
        Err("No test books found".into())
    }
}

/// Extract the second test book archive (backwards compatibility)
pub fn extract_test_book_two(target_dir: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let books = get_embedded_test_books();
    if books.len() > 1 {
        extract_single_book(&books[1], target_dir)
    } else {
        Err("Second test book not found".into())
    }
}