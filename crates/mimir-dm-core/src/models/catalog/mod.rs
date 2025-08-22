//! D&D 5e catalog models for game content
//!
//! This module contains all the data structures for static game content
//! like spells, items, monsters, classes, and more.

pub mod class;
pub mod feat;
pub mod item;
pub mod monster;
pub mod spell;

// Re-export commonly used types
pub use class::{
    Class, ClassData, ClassFeature, ClassFeatureData, ClassFluff, ClassFluffData,
    ClassSummary, HitDice, Multiclassing, MulticlassingProficiencies, StartingEquipment,
    StartingProficiencies, Subclass, SubclassFeature, SubclassFluff,
};

pub use item::{Item, ItemData, ItemSummary};

pub use monster::{
    ArmorClass, CreatureType, HitPoints, Monster, MonsterData, MonsterFluff,
    MonsterFluffData, MonsterSummary, Saves, Skills, Speed,
};

pub use spell::{
    CastingTime, ClassReference, Classes, Components, Distance, Duration,
    DurationValue, MaterialComponent, ScalingLevelDice, Spell, SpellData, SpellMeta,
    SpellRange, SpellSchool, SpellSummary, SubclassReference, SubclassReference2,
};

pub use feat::{Feat, FeatData, FeatSummary};