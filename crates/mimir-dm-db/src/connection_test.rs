use anyhow::Result;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use rusqlite::Connection as RusqliteConnection;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub struct TestDatabaseConnection {
    pub diesel_conn: SqliteConnection,
    pub rusqlite_conn: RusqliteConnection,
}

impl TestDatabaseConnection {
    pub fn establish(database_url: &str) -> Result<Self> {
        // Skip sqlite-vec for testing
        
        // Establish Diesel connection for schema operations
        let mut diesel_conn = SqliteConnection::establish(database_url)?;
        
        // Run only the first migration (core tables, skip search tables)
        diesel_conn.run_pending_migrations(MIGRATIONS)
            .map_err(|e| anyhow::anyhow!("Migration error: {}", e))?;

        // Establish rusqlite connection
        let rusqlite_conn = RusqliteConnection::open(database_url)?;
        
        // Enable foreign keys
        rusqlite_conn.execute_batch("PRAGMA foreign_keys = ON;")?;

        Ok(TestDatabaseConnection {
            diesel_conn,
            rusqlite_conn,
        })
    }

    pub fn in_memory() -> Result<Self> {
        Self::establish(":memory:")
    }
}