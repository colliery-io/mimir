//! Example of loading templates from the docs directory

use mimir_dm_db::{establish_connection, run_migrations};
use mimir_dm_db::seed::TemplateLoader;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up logging
    env_logger::init();
    
    // Connect to database (use a test database)
    let mut conn = establish_connection("templates_test.db")?;
    
    // Run migrations
    println!("Running migrations...");
    run_migrations(&mut conn)?;
    
    // Create template loader
    let mut loader = TemplateLoader::new();
    
    // Load templates from the docs directory
    let template_dir = Path::new("/Users/dstorey/Desktop/colliery/mimir/docs/src/campaign-framework/06-templates/templates");
    
    println!("Loading templates from: {}", template_dir.display());
    let summary = loader.load_directory(&mut conn, template_dir)?;
    
    // Print summary
    println!("\nLoad Summary:");
    println!("  Total files: {}", summary.total);
    println!("  Loaded: {}", summary.loaded);
    println!("  Skipped (already exist): {}", summary.skipped);
    println!("  Errors: {}", summary.errors);
    
    if !summary.loaded_ids.is_empty() {
        println!("\nLoaded templates:");
        for id in &summary.loaded_ids {
            println!("  - {}", id);
        }
    }
    
    if !summary.error_details.is_empty() {
        println!("\nErrors:");
        for (path, error) in &summary.error_details {
            println!("  - {}: {}", path.display(), error);
        }
    }
    
    Ok(())
}