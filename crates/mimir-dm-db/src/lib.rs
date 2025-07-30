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

// Re-export commonly used types
pub use connection::{establish_connection, DbConnection};
pub use error::{DbError, Result};

// Re-export models (only what's implemented)
pub use models::rule_systems::RuleSystem;
pub use models::sources::Source;
pub use models::races::Race;
pub use models::classes::Class;

// Re-export DAL traits
pub use dal::traits::{Repository, AsyncRepository};