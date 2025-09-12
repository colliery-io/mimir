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
pub mod logs;
pub mod catalog;
pub mod catalog_spell;
pub mod catalog_class;
// pub mod catalog_feat; // Replaced by catalog_feat_db
pub mod catalog_action;
// pub mod catalog_background; // Replaced by catalog_background_db
pub mod catalog_condition;
// pub mod catalog_race; // Replaced by catalog_race_db
pub mod catalog_race_db;
// pub mod catalog_optionalfeature; // Replaced by catalog_optional_feature_db
pub mod catalog_optional_feature_db;
pub mod catalog_deity;
// pub mod catalog_object; // Replaced by catalog_object_db
pub mod catalog_object_db;
// pub mod catalog_trap; // Replaced by catalog_trap_db
pub mod catalog_trap_db;
// pub mod catalog_language; // Replaced by catalog_language_db
// pub mod catalog_reward; // Replaced by catalog_reward_db
pub mod catalog_table;
// pub mod catalog_variant_rule; // Replaced by catalog_variant_rule_db
pub mod catalog_variant_rule_db;
// pub mod catalog_vehicle; // Replaced by catalog_vehicle_db
// pub mod catalog_cult; // Replaced by catalog_cult_db
pub mod catalog_cult_db;
// pub mod catalog_psionic; // Replaced by catalog_psionic_db
pub mod catalog_psionic_db;
pub mod catalog_language_db;
pub mod catalog_reward_db;
pub mod catalog_background_db;
pub mod catalog_feat_db;
pub mod catalog_item_db;
pub mod catalog_monster_db;
pub mod catalog_deity_db;
pub mod catalog_vehicle_db;
pub mod catalog_class_db;
pub mod context;
pub mod window_manager;
pub mod chat_sessions;
pub mod session_todos;

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
pub use catalog_optional_feature_db::*;
pub use catalog_object_db::*;
pub use catalog_trap_db::*;
pub use catalog_cult_db::*;
// Variant rule database commands are imported in main.rs directly
pub use catalog_item_db::*;
pub use catalog_monster_db::*;
pub use catalog_deity_db::*;
pub use context::*;
pub use window_manager::*;
pub use chat_sessions::*;
pub use session_todos::*;