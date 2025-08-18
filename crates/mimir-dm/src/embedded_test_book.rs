//! Embedded test book for development builds
//! This provides a comprehensive formatting test document

use tar::Archive;
use flate2::read::GzDecoder;

/// The embedded test book archives for development
const TEST_BOOK_ARCHIVE: &[u8] = include_bytes!("../assets/dev/test-book.tar.gz");
const TEST_BOOK_TWO_ARCHIVE: &[u8] = include_bytes!("../assets/dev/test-book-two.tar.gz");

/// Check if we're in development mode
pub fn is_dev_build() -> bool {
    cfg!(debug_assertions) || std::env::var("MIMIR_DEV").is_ok()
}

/// Extract the embedded test book archive
pub fn extract_test_book(target_dir: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    // Create a decoder from the embedded bytes
    let decoder = GzDecoder::new(TEST_BOOK_ARCHIVE);
    let mut archive = Archive::new(decoder);
    
    // Extract the archive
    archive.unpack(target_dir)?;
    
    Ok(())
}

/// Extract the second test book archive
pub fn extract_test_book_two(target_dir: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    // Create a decoder from the embedded bytes
    let decoder = GzDecoder::new(TEST_BOOK_TWO_ARCHIVE);
    let mut archive = Archive::new(decoder);
    
    // Extract the archive
    archive.unpack(target_dir)?;
    
    Ok(())
}