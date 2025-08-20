pub mod archive;
pub mod collector;
pub mod filter;
pub mod images;
pub mod input;
pub mod magic_variants;
pub mod parallel;
pub mod parser;
pub mod repo;

use anyhow::Result;
use std::path::Path;

pub use input::InputSource;
pub use parser::Book;

/// Process a 5etools repository and split it into book archives
pub async fn split_repository(
    input: InputSource,
    output_dir: &Path,
) -> Result<SplitResults> {
    // Setup repository (clone if needed)
    let repo_path = repo::setup_repository(input).await?;
    
    // Load all books
    let books = parser::load_all_books(&repo_path)?;
    
    // Process books in parallel
    let results = parallel::process_all_books(books, &repo_path, output_dir)?;
    
    Ok(results)
}

#[derive(Debug)]
pub struct SplitResults {
    pub successful: Vec<String>,
    pub failed: Vec<(String, String)>, // (book_id, error_message)
    pub total_processed: usize,
}