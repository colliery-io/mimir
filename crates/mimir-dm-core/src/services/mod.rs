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

// Re-export services
pub use campaign_service::CampaignService;
pub use module_service::ModuleService;
pub use session_service::SessionService;
pub use template_service::TemplateService;
pub use catalog_service::CatalogService;
pub use spell_service::SpellService;
pub use action_service::ActionService;