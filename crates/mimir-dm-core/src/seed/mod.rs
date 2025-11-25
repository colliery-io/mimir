//! Database seeding utilities

// Seeding is internal implementation detail
#![allow(missing_docs)]

pub mod template_loader;
pub mod template_seeder;

pub use template_loader::{LoadSummary, TemplateLoader};
