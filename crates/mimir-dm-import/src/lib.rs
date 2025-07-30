//! # Mimir Import
//! 
//! Bundle import functionality for the Mimir D&D campaign assistant.
//! 
//! This crate provides the ability to import standardized D&D rule bundles
//! (.tar.gz archives) into the Mimir database. It supports:
//! 
//! - Bundle validation (manifest, structure, integrity)
//! - Atomic database transactions (all-or-nothing imports)  
//! - Progress reporting with user feedback
//! - Comprehensive error handling and recovery
//! 
//! ## Usage
//! 
//! ```rust,no_run
//! use mimir_dm_import::BundleImporter;
//! use mimir_dm_db::connect_to_database;
//! 
//! # async fn example() -> anyhow::Result<()> {
//! let db = connect_to_database("mimir.db").await?;
//! let importer = BundleImporter::new(db);
//! 
//! importer.import_bundle("dnd5e-2014-core.tar.gz").await?;
//! # Ok(())
//! # }
//! ```

use std::path::Path;

mod bundle;
mod error;
mod importer;
mod manifest;
mod progress;

pub use bundle::Bundle;
pub use error::{ImportError, ImportResult};
pub use importer::BundleImporter;
pub use manifest::BundleManifest;
pub use progress::ImportProgress;

/// Import a bundle into the database with progress reporting
pub async fn import_bundle<P: AsRef<Path>>(
    database_url: &str,
    bundle_path: P,
) -> ImportResult<()> {
    let importer = BundleImporter::new(database_url.to_string());
    importer.import_bundle(bundle_path).await
}

/// Extract and parse a bundle manifest without importing
pub async fn extract_manifest<P: AsRef<Path>>(bundle_path: P) -> ImportResult<BundleManifest> {
    let bundle = Bundle::from_archive(bundle_path).await?;
    Ok(bundle.manifest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bundle_extraction_if_exists() {
        // Path to the real test bundle
        const TEST_BUNDLE_PATH: &str = "/Users/dstorey/Desktop/colliery/mimir/data/rule_sets/dnd5e-2014-core-v1.210.46.tar.gz";
        
        // Skip test if bundle doesn't exist
        if !Path::new(TEST_BUNDLE_PATH).exists() {
            eprintln!("Skipping test: bundle file not found at {}", TEST_BUNDLE_PATH);
            return;
        }

        // Test extraction
        let bundle = Bundle::from_archive(TEST_BUNDLE_PATH).await.unwrap();
        
        // Verify manifest
        assert_eq!(bundle.manifest.bundle_id, "dnd5e-2014-core");
        assert_eq!(bundle.manifest.bundle_name, "D&D 5e 2014 Core Ruleset");
        assert_eq!(bundle.manifest.rule_system, "dnd5e-2014");
        
        // Verify required files exist
        assert!(bundle.has_file("manifest.json"));
        assert!(bundle.has_file("version.json"));
        assert!(bundle.has_file("sources.json"));
    }
}