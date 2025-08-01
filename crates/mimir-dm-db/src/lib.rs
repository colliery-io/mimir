//! Mimir Database Layer
//! 
//! This crate provides the database abstraction layer for Mimir,
//! including schema definitions, migrations, and CRUD operations
//! for all D&D entities.

pub mod connection;
pub mod dal;
pub mod error;
pub mod json_types;
pub mod models;
pub mod schema;
pub mod seed;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

// Re-export commonly used types
pub use connection::{establish_connection, DbConnection};
pub use error::{DbError, Result};

// Re-export models (only what's implemented)
pub use models::rule_systems::RuleSystem;
pub use models::sources::Source;
pub use models::races::Race;
pub use models::classes::Class;
pub use models::items::Item;
pub use models::creatures::Creature;

// Re-export DAL traits
pub use dal::traits::{Repository, AsyncRepository};

// Embed migrations
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

/// Run all pending migrations on the database
pub fn run_migrations(conn: &mut DbConnection) -> Result<()> {
    conn.run_pending_migrations(MIGRATIONS)
        .map_err(|e| DbError::Migration(format!("Failed to run migrations: {}", e)))?;
    Ok(())
}