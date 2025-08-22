//! Database Models
//! 
//! Split into distinct domains:
//! - `catalog`: Static D&D reference data (races, classes, spells, items, monsters, etc.)
//! - `campaign`: Campaign management and story organization

pub mod campaign;
pub mod catalog;
pub mod rules_extended;