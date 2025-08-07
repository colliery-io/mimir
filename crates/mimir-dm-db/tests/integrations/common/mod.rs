//! Common test utilities

use mimir_dm_db::{DbConnection, DbError, Result};
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use tempfile::{TempDir, NamedTempFile};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

/// Test database fixture that handles setup and cleanup
pub struct TestDatabase {
    _temp_dir: Option<TempDir>,
    _temp_file: Option<NamedTempFile>,
    pub url: String,
}

impl TestDatabase {
    /// Create a file-based test database (for async tests or when persistence is needed)
    pub fn file_based() -> Result<Self> {
        let temp_file = NamedTempFile::new()?;
        let url = temp_file.path().to_string_lossy().to_string();
        
        // Set up the database
        let mut conn = DbConnection::establish(&url)?;
        Self::setup_database(&mut conn)?;
        drop(conn); // Close connection
        
        Ok(Self {
            _temp_dir: None,
            _temp_file: Some(temp_file),
            url,
        })
    }
    
    /// Get a new connection to this test database
    pub fn connection(&self) -> Result<DbConnection> {
        mimir_dm_db::establish_connection(&self.url)
    }
    
    /// Set up the database with migrations and test settings
    fn setup_database(conn: &mut DbConnection) -> Result<()> {
        // Run migrations
        conn.run_pending_migrations(MIGRATIONS).map_err(|e| {
            DbError::Migration(format!("Failed to run migrations: {}", e))
        })?;
        
        // Enable foreign keys
        diesel::sql_query("PRAGMA foreign_keys = ON")
            .execute(conn)?;
        
        // Optimize for tests
        diesel::sql_query("PRAGMA synchronous = OFF")
            .execute(conn)?;
        
        diesel::sql_query("PRAGMA journal_mode = MEMORY")
            .execute(conn)?;
        
        Ok(())
    }
}

/// Run a test with a fresh database (legacy function for compatibility)
pub fn with_test_db<F, R>(f: F) -> Result<R>
where
    F: FnOnce(&mut DbConnection) -> Result<R>,
{
    // For in-memory database, we need to set up after establishing connection
    let mut conn = mimir_dm_db::establish_connection(":memory:")?;
    TestDatabase::setup_database(&mut conn)?;
    f(&mut conn)
}