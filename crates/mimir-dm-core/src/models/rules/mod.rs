//! D&D Rules Reference Models
//! 
//! Static reference data from rulebooks and source materials.
//! These represent the rules as written, not campaign-specific instances.

pub mod backgrounds;
pub mod classes;
pub mod creatures;
pub mod feats;
pub mod items;
pub mod races;
pub mod rule_systems;
pub mod sources;
pub mod spells;

// Re-export commonly used types
pub use backgrounds::Background;
pub use classes::Class;
pub use creatures::Creature;
pub use feats::Feat;
pub use items::Item;
pub use races::Race;
pub use rule_systems::RuleSystem;
pub use sources::Source;
pub use spells::Spell;