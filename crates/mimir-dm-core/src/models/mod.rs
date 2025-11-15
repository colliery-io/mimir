//! Database Models
//!
//! Split into distinct domains:
//! - `catalog`: Static D&D reference data (races, classes, spells, items, monsters, etc.)
//! - `campaign`: Campaign management and story organization
//! - `player`: Player management and campaign associations
//! - `character`: Character data and version tracking

pub mod campaign;
pub mod catalog;
pub mod character;
pub mod player;