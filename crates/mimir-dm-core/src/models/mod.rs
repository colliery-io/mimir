//! Database Models
//! 
//! Split into two distinct domains:
//! - `rules`: Static D&D reference data (races, classes, spells, etc.)
//! - `campaign`: Campaign management and story organization

pub mod campaign;
pub mod rules;