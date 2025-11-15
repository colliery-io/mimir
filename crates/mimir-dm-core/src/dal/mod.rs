//! Data Access Layer - Repository pattern for database operations
//!
//! Organized into domains matching the model structure:
//! - `campaign`: Repositories for campaign management
//! - `player`: Repositories for player management

pub mod campaign;
pub mod player;
pub mod traits;