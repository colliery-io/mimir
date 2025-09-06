//! Tauri command handlers

pub mod campaigns;
pub mod app_info;
pub mod documents;
pub mod stage_transitions;
pub mod boards;
pub mod modules;
pub mod sessions;
pub mod books;
pub mod dev_tools;
pub mod catalog;
pub mod catalog_spell;
pub mod catalog_class;
// pub mod catalog_feat; // Replaced by catalog_feat_db
pub mod catalog_action;
// pub mod catalog_background; // Replaced by catalog_background_db
pub mod catalog_condition;
// pub mod catalog_race; // Replaced by catalog_race_db
pub mod catalog_race_db;
pub mod catalog_optionalfeature;
pub mod catalog_deity;
// pub mod catalog_object; // Replaced by catalog_object_db
pub mod catalog_object_db;
pub mod catalog_trap;
// pub mod catalog_language; // Replaced by catalog_language_db
// pub mod catalog_reward; // Replaced by catalog_reward_db
pub mod catalog_table;
pub mod catalog_variant_rule;
pub mod catalog_vehicle;
pub mod catalog_cult;
pub mod catalog_psionic;
pub mod catalog_language_db;
pub mod catalog_reward_db;
pub mod catalog_background_db;
pub mod catalog_feat_db;
pub mod context;
pub mod window_manager;
pub mod chat_sessions;
pub mod todos;

pub use campaigns::*;
pub use app_info::*;
pub use documents::*;
pub use stage_transitions::*;
pub use boards::*;
pub use modules::*;
pub use sessions::*;
pub use books::*;
pub use dev_tools::*;
pub use catalog::*;
pub use catalog_spell::*;
pub use catalog_class::*;
// pub use catalog_feat::*; // Replaced by catalog_feat_db
// pub use catalog_race::*; // Replaced by catalog_race_db
pub use catalog_race_db::*;
pub use catalog_object_db::*;
pub use context::*;
pub use window_manager::*;
pub use chat_sessions::*;
pub use todos::*;