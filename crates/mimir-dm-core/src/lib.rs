//! Mimir Core Business Logic
//! 
//! This crate provides the core domain models, business logic, and data persistence
//! for the Mimir D&D Campaign Assistant. It includes both the rules reference system
//! (D&D 5e data) and the campaign management system (modules, sessions, documents).

pub mod connection;
pub mod dal;
pub mod domain;
pub mod error;
pub mod models;
pub mod schema;
pub mod seed;
pub mod services;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

// Re-export commonly used types
pub use connection::{establish_connection, DbConnection};
pub use error::{DbError, Result};



// Re-export campaign models  
pub use models::campaign::{
    Campaign, Module, Session, Document,
    WorkflowCard, TemplateDocument
};

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