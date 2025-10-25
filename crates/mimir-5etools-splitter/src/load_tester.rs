//! Load testing functionality for 5etools splitter archives
//!
//! This module provides functionality to test that generated tar.gz files
//! can be successfully parsed and imported into a database using the same
//! import logic as the main application.

use anyhow::{Context, Result};
use colored::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use flate2::read::GzDecoder;
use mimir_dm_core::{run_migrations, services::{CatalogService, ActionService, ConditionService, LanguageService, RewardService, BackgroundService, FeatService, RaceService, ObjectService, TrapService}};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use tar::Archive;
use tempfile::TempDir;
use tracing::{debug, error, info, warn};

/// Result of testing a single archive
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub archive_name: String,
    pub archive_path: String,
    pub extraction_ok: bool,
    pub metadata_valid: bool,
    pub metadata: Option<serde_json::Value>,
    pub import_results: HashMap<String, ImportResult>,
    pub overall_success: bool,
    pub errors: Vec<String>,
}

/// Result of importing a specific catalog type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportResult {
    pub catalog_type: String,
    pub total_found: i32,
    pub total_imported: usize,
    pub success: bool,
    pub errors: Vec<String>,
}

/// Summary of all test results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSummary {
    pub total_archives: usize,
    pub successful_archives: usize,
    pub failed_archives: usize,
    pub results: Vec<TestResult>,
}

impl TestSummary {
    pub fn new() -> Self {
        Self {
            total_archives: 0,
            successful_archives: 0,
            failed_archives: 0,
            results: Vec::new(),
        }
    }

    pub fn add_result(&mut self, result: TestResult) {
        if result.overall_success {
            self.successful_archives += 1;
        } else {
            self.failed_archives += 1;
        }
        self.total_archives += 1;
        self.results.push(result);
    }

    pub fn print_summary(&self, verbose: bool) {
        println!("\n{}", "=== LOAD TEST SUMMARY ===".bright_cyan().bold());
        println!("📦 Total archives tested: {}", self.total_archives);
        println!("✅ Successful: {}", self.successful_archives.to_string().green());
        println!("❌ Failed: {}", self.failed_archives.to_string().red());
        
        if self.failed_archives > 0 {
            println!("\n{}", "Failed archives:".red().bold());
            for result in &self.results {
                if !result.overall_success {
                    println!("  • {}: {}", result.archive_name.red(), 
                           result.errors.join(", "));
                }
            }
        }

        if verbose {
            println!("\n{}", "Detailed Results:".bright_yellow().bold());
            for result in &self.results {
                self.print_detailed_result(result);
            }
        }
    }

    fn print_detailed_result(&self, result: &TestResult) {
        let status_icon = if result.overall_success { "✅" } else { "❌" };
        println!("\n{} {}", status_icon, result.archive_name.bold());
        
        if !result.extraction_ok {
            println!("  {} Archive extraction failed", "❌".red());
        }
        
        if !result.metadata_valid {
            println!("  {} Metadata validation failed", "❌".red());
        }

        for (catalog_type, import_result) in &result.import_results {
            let status = if import_result.success { "✅" } else { "❌" };
            println!("  {} {}: {} imported", 
                   status, catalog_type, import_result.total_imported);
            
            if !import_result.errors.is_empty() {
                for error in &import_result.errors {
                    println!("    • {}", error.red());
                }
            }
        }
    }
}

/// Load tester for 5etools archives
pub struct LoadTester {
    verbose: bool,
}

impl LoadTester {
    pub fn new(verbose: bool) -> Self {
        Self { verbose }
    }

    /// Test a single archive file
    pub async fn test_archive(&self, archive_path: &Path) -> Result<TestResult> {
        let archive_name = archive_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        info!("Testing archive: {}", archive_name);

        let mut result = TestResult {
            archive_name: archive_name.clone(),
            archive_path: archive_path.to_string_lossy().to_string(),
            extraction_ok: false,
            metadata_valid: false,
            metadata: None,
            import_results: HashMap::new(),
            overall_success: false,
            errors: Vec::new(),
        };

        // Extract archive
        let temp_dir = match self.extract_archive(archive_path) {
            Ok(dir) => {
                result.extraction_ok = true;
                dir
            }
            Err(e) => {
                let error_msg = format!("Failed to extract archive: {}", e);
                result.errors.push(error_msg);
                return Ok(result);
            }
        };

        // Find and validate book directory
        let book_dir = match self.find_book_directory(&temp_dir) {
            Ok(dir) => dir,
            Err(e) => {
                let error_msg = format!("Failed to find book directory: {}", e);
                result.errors.push(error_msg);
                return Ok(result);
            }
        };

        // Validate metadata
        match self.validate_metadata(&book_dir) {
            Ok(metadata) => {
                result.metadata_valid = true;
                result.metadata = Some(metadata);
            }
            Err(e) => {
                let error_msg = format!("Metadata validation failed: {}", e);
                result.errors.push(error_msg);
            }
        }

        // Create in-memory database and run imports
        match self.test_imports(&book_dir, &archive_name).await {
            Ok(import_results) => {
                result.import_results = import_results;
            }
            Err(e) => {
                let error_msg = format!("Import testing failed: {}", e);
                result.errors.push(error_msg);
            }
        }

        // Determine overall success
        result.overall_success = result.extraction_ok 
            && result.metadata_valid 
            && result.import_results.values().all(|r| r.success)
            && result.errors.is_empty();

        if self.verbose {
            if result.overall_success {
                println!("✅ {} passed all tests", archive_name.green());
            } else {
                println!("❌ {} failed tests", archive_name.red());
                for error in &result.errors {
                    println!("  • {}", error.red());
                }
            }
        }

        Ok(result)
    }

    /// Test all archives in a directory
    pub async fn test_directory(&self, dir_path: &Path) -> Result<TestSummary> {
        let mut summary = TestSummary::new();

        info!("Scanning directory for tar.gz files: {:?}", dir_path);

        let entries = fs::read_dir(dir_path)
            .with_context(|| format!("Failed to read directory: {:?}", dir_path))?;

        let mut archive_paths = Vec::new();
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "gz") {
                if let Some(file_name) = path.file_name() {
                    if file_name.to_string_lossy().ends_with(".tar.gz") {
                        archive_paths.push(path);
                    }
                }
            }
        }

        info!("Found {} tar.gz files to test", archive_paths.len());

        for archive_path in archive_paths {
            match self.test_archive(&archive_path).await {
                Ok(result) => {
                    summary.add_result(result);
                }
                Err(e) => {
                    error!("Failed to test archive {:?}: {}", archive_path, e);
                    let result = TestResult {
                        archive_name: archive_path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("unknown")
                            .to_string(),
                        archive_path: archive_path.to_string_lossy().to_string(),
                        extraction_ok: false,
                        metadata_valid: false,
                        metadata: None,
                        import_results: HashMap::new(),
                        overall_success: false,
                        errors: vec![format!("Test execution failed: {}", e)],
                    };
                    summary.add_result(result);
                }
            }
        }

        Ok(summary)
    }

    /// Extract archive to temporary directory
    fn extract_archive(&self, archive_path: &Path) -> Result<TempDir> {
        debug!("Extracting archive: {:?}", archive_path);

        let file = fs::File::open(archive_path)
            .with_context(|| format!("Failed to open archive: {:?}", archive_path))?;

        let tar_gz = GzDecoder::new(file);
        let mut archive = Archive::new(tar_gz);

        let temp_dir = TempDir::new()
            .context("Failed to create temporary directory")?;

        archive.unpack(temp_dir.path())
            .context("Failed to extract archive")?;

        Ok(temp_dir)
    }

    /// Find the book directory within the extracted archive
    fn find_book_directory(&self, temp_dir: &TempDir) -> Result<PathBuf> {
        let entries = fs::read_dir(temp_dir.path())
            .context("Failed to read temporary directory")?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                debug!("Found book directory: {:?}", path);
                return Ok(path);
            }
        }

        anyhow::bail!("No book directory found in archive");
    }

    /// Validate metadata.json exists and is parseable
    fn validate_metadata(&self, book_dir: &Path) -> Result<serde_json::Value> {
        let metadata_path = book_dir.join("metadata.json");
        
        if !metadata_path.exists() {
            anyhow::bail!("metadata.json not found");
        }

        let metadata_content = fs::read_to_string(&metadata_path)
            .context("Failed to read metadata.json")?;

        let metadata: serde_json::Value = serde_json::from_str(&metadata_content)
            .context("Failed to parse metadata.json")?;

        debug!("Metadata validated successfully");
        Ok(metadata)
    }

    /// Test all catalog imports against in-memory database
    async fn test_imports(&self, book_dir: &Path, source: &str) -> Result<HashMap<String, ImportResult>> {
        debug!("Setting up in-memory database for import testing");

        let mut conn = SqliteConnection::establish(":memory:")
            .context("Failed to create in-memory database")?;

        // Enable foreign keys and other pragmas
        diesel::sql_query("PRAGMA foreign_keys = ON").execute(&mut conn)?;
        diesel::sql_query("PRAGMA journal_mode = WAL").execute(&mut conn)?;

        // Run migrations
        run_migrations(&mut conn)
            .context("Failed to run database migrations")?;

        debug!("Database setup complete, running catalog imports");

        let mut results = HashMap::new();

        // Test each catalog type
        self.test_catalog_import(
            &mut results,
            "spells",
            || CatalogService::import_spells_from_book(&mut conn, book_dir, source),
        );

        self.test_catalog_import(
            &mut results,
            "actions",
            || ActionService::import_actions_from_book(&mut conn, book_dir, source),
        );

        self.test_catalog_import(
            &mut results,
            "conditions",
            || ConditionService::import_conditions_from_book(&mut conn, book_dir, source),
        );

        self.test_catalog_import(
            &mut results,
            "languages",
            || LanguageService::import_languages_from_book(&mut conn, book_dir, source),
        );

        self.test_catalog_import(
            &mut results,
            "rewards",
            || RewardService::import_rewards_from_book(&mut conn, book_dir, source),
        );

        self.test_catalog_import(
            &mut results,
            "backgrounds",
            || BackgroundService::import_backgrounds_from_book(&mut conn, book_dir, source),
        );

        self.test_catalog_import(
            &mut results,
            "feats",
            || FeatService::import_feats_from_book(&mut conn, book_dir, source),
        );

        self.test_catalog_import(
            &mut results,
            "races",
            || RaceService::import_races_from_book(&mut conn, book_dir, source),
        );

        self.test_catalog_import(
            &mut results,
            "objects",
            || ObjectService::import_objects_from_book(&mut conn, book_dir, source),
        );

        self.test_catalog_import(
            &mut results,
            "traps",
            || TrapService::import_traps_from_book(&mut conn, book_dir, source),
        );

        self.test_catalog_import(
            &mut results,
            "cults",
            || CatalogService::import_cults_from_book(&mut conn, book_dir, source),
        );

        self.test_catalog_import(
            &mut results,
            "variant_rules",
            || CatalogService::import_variant_rules_from_book(&mut conn, book_dir, source),
        );

        self.test_catalog_import(
            &mut results,
            "optional_features",
            || CatalogService::import_optional_features_from_book(&mut conn, book_dir, source),
        );

        self.test_catalog_import(
            &mut results,
            "items",
            || CatalogService::import_items_from_book(&mut conn, book_dir, source),
        );

        self.test_catalog_import(
            &mut results,
            "monsters",
            || CatalogService::import_monsters_from_book(&mut conn, book_dir, source),
        );

        self.test_catalog_import(
            &mut results,
            "deities",
            || CatalogService::import_deities_from_book(&mut conn, book_dir, source),
        );

        self.test_catalog_import(
            &mut results,
            "vehicles",
            || CatalogService::import_vehicles_from_book(&mut conn, book_dir, source),
        );

        self.test_catalog_import(
            &mut results,
            "classes",
            || CatalogService::import_classes_from_book(&mut conn, book_dir, source),
        );

        Ok(results)
    }

    /// Test a single catalog import
    fn test_catalog_import<F>(
        &self,
        results: &mut HashMap<String, ImportResult>,
        catalog_type: &str,
        import_fn: F,
    ) where
        F: FnOnce() -> Result<usize, String>,
    {
        debug!("Testing {} import", catalog_type);

        let mut import_result = ImportResult {
            catalog_type: catalog_type.to_string(),
            total_found: 0,
            total_imported: 0,
            success: false,
            errors: Vec::new(),
        };

        match import_fn() {
            Ok(count) => {
                import_result.total_imported = count;
                import_result.success = true;
                debug!("Successfully imported {} {}", count, catalog_type);
            }
            Err(e) => {
                import_result.errors.push(e.clone());
                warn!("Failed to import {}: {}", catalog_type, e);
            }
        }

        results.insert(catalog_type.to_string(), import_result);
    }
}