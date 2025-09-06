//! Service layer for business logic
//! 
//! This module contains services that orchestrate business logic,
//! combining DAL operations with domain rules.

pub mod campaign_service;
pub mod module_service;
pub mod session_service;
pub mod template_service;
pub mod catalog_service;
pub mod spell_service;
pub mod action_service;
pub mod condition_service;
pub mod language_service;
pub mod reward_service;
pub mod background_service;
pub mod feat_service;
pub mod race_service;

// Re-export services
pub use campaign_service::CampaignService;
pub use module_service::ModuleService;
pub use session_service::SessionService;
pub use template_service::TemplateService;
pub use catalog_service::CatalogService;
pub use spell_service::SpellService;
pub use action_service::ActionService;
pub use condition_service::ConditionService;
pub use language_service::LanguageService;
pub use reward_service::RewardService;
pub use background_service::BackgroundService;
pub use feat_service::FeatService;
pub use race_service::RaceService;