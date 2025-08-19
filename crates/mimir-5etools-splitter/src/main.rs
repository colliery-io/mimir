use anyhow::Result;
use clap::Parser;
use mimir_5etools_splitter::{split_repository, InputSource};
use std::path::PathBuf;
use tracing_subscriber;

/// Split 5etools repository into book-specific archives
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input source: local directory path or GitHub URL[@tag]
    /// 
    /// Examples:
    ///   /path/to/5etools-src
    ///   https://github.com/5etools-mirror-3/5etools-2014-src.git
    ///   https://github.com/5etools-mirror-3/5etools-2014-src.git@v1.210.46
    #[arg(value_name = "INPUT")]
    input: String,
    
    /// Output directory for archive files
    #[arg(value_name = "OUTPUT_DIR")]
    output_dir: PathBuf,
    
    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    // Setup logging
    let log_level = if args.verbose { "debug" } else { "info" };
    tracing_subscriber::fmt()
        .with_env_filter(log_level)
        .init();
    
    // Parse input source
    let input_source = InputSource::parse(&args.input)?;
    
    // Display what we're doing
    match &input_source {
        InputSource::LocalPath(path) => {
            println!("📁 Processing local repository: {:?}", path);
        }
        InputSource::GitHub { url, reference } => {
            if let Some(ref_str) = reference {
                println!("🌐 Cloning from GitHub: {} @ {}", url, ref_str);
            } else {
                println!("🌐 Cloning from GitHub: {} @ latest", url);
            }
        }
    }
    
    println!("📦 Output directory: {:?}", args.output_dir);
    
    // Process the repository
    println!("\n🚀 Starting processing...\n");
    let results = split_repository(input_source, &args.output_dir).await?;
    
    // Display results
    println!("\n✨ Processing complete!");
    println!("📊 Total books processed: {}", results.total_processed);
    println!("✅ Successful: {}", results.successful.len());
    
    if !results.successful.is_empty() {
        println!("\nSuccessfully created archives for:");
        for book_id in &results.successful {
            println!("  • {}.tar.gz", book_id.to_lowercase());
        }
    }
    
    if !results.failed.is_empty() {
        println!("\n❌ Failed: {}", results.failed.len());
        println!("\nFailed to process:");
        for (book_id, error) in &results.failed {
            println!("  • {}: {}", book_id, error);
        }
    }
    
    Ok(())
}