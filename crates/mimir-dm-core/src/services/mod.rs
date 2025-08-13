//! Service layer for business logic
//! 
//! This module contains services that orchestrate business logic,
//! combining DAL operations with domain rules.

pub mod campaign_service;
pub mod module_service;
pub mod template_service;

// Re-export services
pub use campaign_service::CampaignService;
pub use module_service::ModuleService;
pub use template_service::TemplateService;