//! Catalog command handlers for 5e content browsing.
//!
//! This module contains Tauri commands for searching and retrieving
//! catalog data including spells, monsters, items, and other 5e content.
//!
//! ## Generic Command Macro
//!
//! The `catalog_commands!` macro can be used to generate standard command handlers
//! for services that implement the `CatalogService` trait. See the `generic` module.

#[macro_use]
pub mod generic;

pub mod action;
pub mod background;
pub mod class;
pub mod condition;
pub mod cult;
pub mod deity;
pub mod feat;
pub mod item;
pub mod language;
pub mod monster;
pub mod object;
pub mod optional_feature;
pub mod psionic;
pub mod race;
pub mod reward;
pub mod spell;
pub mod table;
pub mod trap;
pub mod variant_rule;
pub mod vehicle;

pub use action::*;
pub use background::*;
pub use class::*;
pub use condition::*;
pub use cult::*;
pub use deity::*;
pub use feat::*;
pub use item::*;
pub use language::*;
pub use monster::*;
pub use object::*;
pub use optional_feature::*;
pub use psionic::*;
pub use race::*;
pub use reward::*;
pub use spell::*;
pub use table::*;
pub use trap::*;
pub use variant_rule::*;
pub use vehicle::*;
