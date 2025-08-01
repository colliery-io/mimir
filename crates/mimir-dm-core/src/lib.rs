//! Core types and business logic for Mimir D&D Campaign Assistant

pub mod models;
pub mod traits;
pub mod types;

// Re-export core workflow types
pub use models::{
    board::*,
    campaign::{Campaign, CampaignStatus},
    ids::{CampaignId, ModuleId, SessionId},
    module::Module,
    session::Session,
};