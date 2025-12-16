//! Campaign Management Models
//!
//! Models for organizing and running campaigns, modules, and sessions.
//! These represent the story management layer, not game mechanics.

pub mod campaigns;
pub mod documents;
pub mod module_monsters;
pub mod modules;
pub mod sessions;
pub mod template_documents;
pub mod template_frontmatter;
pub mod workflow_cards;

// Re-export commonly used types
pub use campaigns::{Campaign, NewCampaign};
pub use documents::{Document, NewDocument};
pub use module_monsters::{
    EncounterGroup, ModuleMonster, ModuleMonsterWithData, NewModuleMonster, UpdateModuleMonster,
};
pub use modules::{Module, NewModule};
pub use sessions::{NewSession, Session};
pub use template_documents::TemplateDocument;
pub use template_frontmatter::TemplateFrontmatter;
pub use workflow_cards::{NewWorkflowCard, WorkflowCard};
