//! D&D 5e catalog models for game content
//!
//! This module contains all the data structures for static game content
//! like spells, items, monsters, classes, and more.

pub mod action;
pub mod background;
pub mod book;
pub mod class;
pub mod condition;
pub mod deity;
pub mod feat;
pub mod item;
pub mod monster;
pub mod object;
pub mod optionalfeature;
pub mod race;
pub mod spell;
pub mod trap;
pub mod language;
pub mod reward;
pub mod table;
pub mod variant_rule;
pub mod vehicle;
pub mod cult;
pub mod psionic;

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

pub use book::{UploadedBook, NewUploadedBook};

pub use action::{Action, ActionData, ActionSummary};

pub use condition::{Condition, ConditionData, Disease, DiseaseData, ConditionSummary, ConditionOrDisease, ConditionWithDetails, ConditionFluff, ConditionFluffData};

pub use optionalfeature::{OptionalFeature, OptionalFeatureData, OptionalFeatureSummary};

pub use deity::{Deity, DeityData, DeitySummary};

pub use object::{DndObject, ObjectData, ObjectSummary};

pub use trap::{Trap, TrapData, Hazard, HazardData, TrapOrHazard, TrapSummary};

pub use language::{Language, LanguageData, LanguageSummary, LanguageFluff, LanguageFluffData};

pub use reward::{Reward, RewardData, RewardSummary, RewardFluff, RewardFluffData};

pub use table::{Table, TableData, TableSummary, TableFluff, TableFluffData};

pub use variant_rule::{VariantRule, VariantRuleData, VariantRuleSummary};

pub use vehicle::{Vehicle, VehicleData, VehicleSummary, VehicleWeapon, Speed as VehicleSpeed};

pub use cult::{Cult, CultData, Boon, BoonData, CultBoonSummary};

pub use psionic::{Psionic, PsionicSummary, PsionicMode, PsionicCost, ConcentrationDuration};