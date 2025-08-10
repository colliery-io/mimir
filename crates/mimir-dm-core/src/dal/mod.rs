//! Data Access Layer - Repository pattern for database operations
//! 
//! Organized into two domains matching the model structure:
//! - `rules`: Repositories for D&D reference data
//! - `campaign`: Repositories for campaign management

pub mod campaign;
pub mod rules;
pub mod traits;