//! 5etools Repository Splitter
//!
//! This crate processes 5etools repositories and splits them into individual
//! book archives for import into Mimir.

#![warn(missing_docs)]

pub mod archive;
pub mod collector;
pub mod filter;
pub mod images;
pub mod input;
pub mod load_tester;
pub mod magic_variants;
pub mod parallel;
pub mod parser;
pub mod repo;
pub mod srd_filter;
pub mod srd_collector;

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

/// Extract SRD content from a 5etools repository
pub async fn extract_srd(
    input: InputSource,
    output_dir: &Path,
) -> Result<SrdResults> {
    use crate::srd_collector::collect_srd_content;
    
    // Setup repository (clone if needed)
    let repo_path = repo::setup_repository(input).await?;
    
    // Collect all SRD content
    let content = collect_srd_content(&repo_path)?;
    
    // Create output directory
    std::fs::create_dir_all(output_dir)?;
    
    // Generate summary
    let summary = srd_collector::generate_srd_summary(&content);
    
    // Create archive
    let archive_path = output_dir.join("srd.tar.gz");
    archive::create_tar_gz(&content.files, &archive_path)?;
    
    Ok(SrdResults {
        archive_path: archive_path.to_string_lossy().to_string(),
        total_items: content.metadata.total_items,
        content_summary: content.metadata.content_summary,
        summary,
    })
}

#[derive(Debug)]
pub struct SplitResults {
    pub successful: Vec<String>,
    pub failed: Vec<(String, String)>, // (book_id, error_message)
    pub total_processed: usize,
}

#[derive(Debug)]
pub struct SrdResults {
    pub archive_path: String,
    pub total_items: usize,
    pub content_summary: std::collections::HashMap<String, usize>,
    pub summary: String,
}