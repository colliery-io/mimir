//! Domain logic module
//! 
//! This module contains the business domain logic including:
//! - Board workflow definitions
//! - Business rules
//! - Domain services

pub mod boards;

// Re-export commonly used types
pub use boards::{BoardDefinition, BoardCompletionStatus, StageMetadata, BoardRegistry};
pub use boards::campaign_board::CampaignBoard;
pub use boards::module_board::ModuleBoard;
pub use boards::session_board::SessionBoard;