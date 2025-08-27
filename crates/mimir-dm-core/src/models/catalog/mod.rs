//! D&D 5e catalog models for game content
//!
//! This module contains all the data structures for static game content
//! like spells, items, monsters, classes, and more.

pub mod action;
pub mod background;
pub mod class;
pub mod condition;
pub mod feat;
pub mod item;
pub mod monster;
pub mod optionalfeature;
pub mod race;
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

pub use race::{Race, RaceData, RaceSummary, Subrace, RaceFluff, RaceFluffData};

pub use background::{Background, BackgroundData, BackgroundSummary, BackgroundFluff, BackgroundFluffData, BackgroundWithDetails};

pub use action::{Action, ActionData, ActionSummary};

pub use condition::{Condition, ConditionData, Disease, DiseaseData, ConditionSummary, ConditionOrDisease, ConditionWithDetails, ConditionFluff, ConditionFluffData};

pub use optionalfeature::{OptionalFeature, OptionalFeatureData, OptionalFeatureSummary};