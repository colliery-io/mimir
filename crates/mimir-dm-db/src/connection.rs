use anyhow::Result;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use rusqlite::Connection as RusqliteConnection;
use sqlite_vec::sqlite3_vec_init;
use rusqlite::ffi::sqlite3_auto_extension;
use std::path::Path;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub struct DatabaseConnection {
    pub diesel_conn: SqliteConnection,
    pub rusqlite_conn: RusqliteConnection,
}

impl DatabaseConnection {
    pub fn establish(database_url: &str) -> Result<Self> {
        // Register sqlite-vec extension
        unsafe {
            sqlite3_auto_extension(Some(std::mem::transmute(sqlite3_vec_init as *const ())));
        }

        // Create directory if it doesn't exist
        if let Some(parent) = Path::new(database_url).parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Establish Diesel connection for schema operations
        let mut diesel_conn = SqliteConnection::establish(database_url)?;
        
        // Run migrations
        diesel_conn.run_pending_migrations(MIGRATIONS)
            .map_err(|e| anyhow::anyhow!("Migration error: {}", e))?;

        // Establish rusqlite connection for vector operations
        let rusqlite_conn = RusqliteConnection::open(database_url)?;
        
        // Enable foreign keys
        rusqlite_conn.execute_batch("PRAGMA foreign_keys = ON;")?;

        Ok(DatabaseConnection {
            diesel_conn,
            rusqlite_conn,
        })
    }

    pub fn in_memory() -> Result<Self> {
        Self::establish(":memory:")
    }
}