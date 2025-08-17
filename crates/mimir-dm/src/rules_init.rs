//! Rules initialization module
//! 
//! Handles checking for and importing embedded D&D rules on first run

use anyhow::{Context, Result};
use mimir_dm_core::dal::{rules::rule_systems::RuleSystemRepository, traits::AsyncRepository};
use mimir_dm_import::{Bundle, BundleImporter};
use tracing::{info, warn};

use crate::embedded_rules::{CORE_BUNDLE_ID, CORE_RULES_BUNDLE, CORE_RULES_ID};

/// Check if the core rules have already been imported
pub async fn check_rules_imported(db_url: &str) -> Result<bool> {
    let repo = RuleSystemRepository::new(db_url.to_string());
    
    match repo.find_by_id(CORE_RULES_ID).await {
        Ok(rule_system) => Ok(rule_system.is_some()),
        Err(e) => {
            warn!("Error checking for rule system: {}", e);
            // If we can't check, assume they're not imported
            Ok(false)
        }
    }
}

/// Import the embedded core rules bundle
pub async fn import_embedded_rules(db_url: &str) -> Result<()> {
    info!("Importing embedded core D&D rules...");
    
    // Extract bundle from embedded bytes
    let bundle = Bundle::from_bytes(CORE_RULES_BUNDLE)
        .await
        .context("Failed to extract embedded bundle")?;
    
    // Verify this is the expected bundle
    if bundle.manifest.bundle_id != CORE_BUNDLE_ID {
        return Err(anyhow::anyhow!(
            "Unexpected bundle ID: expected '{}', got '{}'",
            CORE_BUNDLE_ID,
            bundle.manifest.bundle_id
        ));
    }
    
    info!(
        "Extracted bundle '{}' version {}",
        bundle.manifest.bundle_name,
        bundle.manifest.bundle_version
    );
    
    // Import using the importer
    let importer = BundleImporter::new(db_url.to_string());
    importer
        .import_bundle_direct(bundle)
        .await
        .context("Failed to import bundle")?;
    
    info!("Core rules imported successfully");
    Ok(())
}