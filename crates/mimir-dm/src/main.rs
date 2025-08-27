#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app_init;
mod commands;
mod db_connection;
mod embedded_test_book;
mod seed_templates;
mod services;
mod types;

use app_init::{initialize_app, AppPaths};
use commands::*;
use commands::catalog_action::{init_action_catalog, search_actions, get_action_details};
use commands::catalog_background::{init_background_catalog, search_backgrounds, get_background_details};
use commands::catalog_condition::{init_condition_catalog, search_conditions, get_condition_details};
use commands::catalog_optionalfeature::{init_optional_feature_catalog, search_optional_features, get_optional_feature_details, get_feature_types};
use commands::catalog_deity::{init_deity_catalog, search_deities, get_deity_details, get_pantheons, get_domains};
use commands::catalog_object::{init_object_catalog, search_objects, get_object_details, get_object_types};
use commands::catalog_trap::{init_trap_catalog, search_traps, get_trap_details, get_trap_types};
use commands::catalog_language::{init_language_catalog, search_languages, get_language_details, get_language_types, get_language_scripts};
use services::database::DatabaseService;
use std::sync::{Arc, OnceLock, Mutex};
use tauri::Manager;
use tracing::{error, info};

// Global application state
pub static APP_PATHS: OnceLock<AppPaths> = OnceLock::new();

fn main() {
    // Initialize the application first
    let app_paths = match initialize_app() {
        Ok(paths) => {
            info!("Application initialized successfully");
            paths
        }
        Err(e) => {
            eprintln!("Failed to initialize application: {}", e);
            std::process::exit(1);
        }
    };

    // Store app paths globally
    if APP_PATHS.set(app_paths).is_err() {
        error!("Failed to set global app paths");
        std::process::exit(1);
    }

    // Start Tauri application
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            // Initialize database service
            let db_service = Arc::new(DatabaseService);
            app.manage(db_service);
            
            // Initialize catalogs
            let spell_catalog = Mutex::new(commands::catalog::SpellCatalog::new());
            let item_catalog = Mutex::new(commands::catalog::ItemCatalog::new());
            let monster_catalog = Mutex::new(commands::catalog::MonsterCatalog::new());
            let class_catalog = Mutex::new(commands::catalog_class::ClassCatalog::new());
            let feat_catalog = Mutex::new(commands::catalog_feat::FeatCatalog::new());
            let race_catalog = Mutex::new(commands::catalog_race::RaceCatalog::new());
            let background_catalog = Mutex::new(commands::catalog_background::BackgroundCatalog::new());
            let action_catalog = Mutex::new(commands::catalog_action::ActionCatalog::new());
            let condition_catalog = Mutex::new(commands::catalog_condition::ConditionCatalog::new());
            let optional_feature_catalog = Mutex::new(commands::catalog_optionalfeature::OptionalFeatureCatalog::new());
            let deity_catalog = Mutex::new(commands::catalog_deity::DeityCatalog::new());
            let object_catalog = Mutex::new(commands::catalog_object::ObjectCatalog::new());
            let trap_catalog = Mutex::new(commands::catalog_trap::TrapCatalog::new());
            let language_catalog = Mutex::new(commands::catalog_language::LanguageCatalog::new());
            app.manage(spell_catalog);
            app.manage(item_catalog);
            app.manage(monster_catalog);
            app.manage(class_catalog);
            app.manage(feat_catalog);
            app.manage(race_catalog);
            app.manage(background_catalog);
            app.manage(action_catalog);
            app.manage(condition_catalog);
            app.manage(optional_feature_catalog);
            app.manage(deity_catalog);
            app.manage(object_catalog);
            app.manage(trap_catalog);
            app.manage(language_catalog);
            
            let reward_catalog = std::sync::Mutex::new(commands::catalog_reward::RewardCatalog::new());
            app.manage(reward_catalog);
            
            let table_catalog = std::sync::Mutex::new(commands::catalog_table::TableCatalog::new());
            app.manage(table_catalog);
            
            let variant_rule_catalog = std::sync::Mutex::new(commands::catalog_variant_rule::VariantRuleCatalog::new());
            app.manage(variant_rule_catalog);
            
            let vehicle_catalog = std::sync::Mutex::new(commands::catalog_vehicle::VehicleCatalog::new());
            app.manage(vehicle_catalog);
            
            let cult_catalog = std::sync::Mutex::new(commands::catalog_cult::CultCatalog::new());
            app.manage(cult_catalog);
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            get_app_info,
            get_default_campaigns_directory,
            list_campaigns,
            create_campaign,
            get_campaign,
            generate_campaign_document,
            list_templates,
            get_campaign_documents,
            get_documents_by_level,
            create_document,
            update_document,
            complete_document,
            delete_document,
            get_incomplete_documents,
            get_completed_documents,
            create_document_from_template,
            read_document_file,
            save_document_file,
            check_campaign_stage_completion,
            transition_campaign_stage,
            initialize_stage_documents,
            get_board_configuration,
            get_next_stage,
            // Module commands
            create_module,
            get_module,
            list_campaign_modules,
            update_module,
            transition_module_stage,
            initialize_module_documents,
            get_module_documents,
            check_module_completion,
            find_modules_needing_next,
            increment_module_sessions,
            delete_module,
            // Session commands
            create_session,
            list_module_sessions,
            transition_session_status,
            get_session_board_config,
            // Book library commands
            upload_book_archive,
            list_library_books,
            remove_book_from_library,
            get_book_content,
            serve_book_image,
            lookup_reference,
            // Dev tools
            is_dev_mode,
            install_dev_test_book,
            remove_dev_test_book,
            // Catalog commands
            initialize_spell_catalog,
            search_spells,
            get_spell_details,
            initialize_item_catalog,
            search_items,
            get_item_details,
            initialize_monster_catalog,
            search_monsters,
            get_monster_details,
            // Class catalog commands
            initialize_class_catalog,
            search_classes,
            get_class_details,
            get_class_subclasses,
            get_class_sources,
            // Feat catalog commands
            initialize_feat_catalog,
            search_feats,
            get_feat_details,
            get_feat_sources,
            // Race catalog commands
            init_race_catalog,
            search_races,
            get_race_details,
            // Background catalog commands
            init_background_catalog,
            search_backgrounds,
            get_background_details,
            // Action catalog commands
            init_action_catalog,
            search_actions,
            get_action_details,
            // Condition catalog commands
            init_condition_catalog,
            search_conditions,
            get_condition_details,
            // Optional feature catalog commands
            init_optional_feature_catalog,
            search_optional_features,
            get_optional_feature_details,
            get_feature_types,
            // Deity catalog commands
            init_deity_catalog,
            search_deities,
            get_deity_details,
            get_pantheons,
            get_domains,
            // Object catalog commands
            init_object_catalog,
            search_objects,
            get_object_details,
            get_object_types,
            // Trap catalog commands
            init_trap_catalog,
            search_traps,
            get_trap_details,
            get_trap_types,
            // Language catalog commands
            init_language_catalog,
            search_languages,
            get_language_details,
            get_language_types,
            get_language_scripts,
            // Reward catalog commands
            commands::catalog_reward::initialize_reward_catalog,
            commands::catalog_reward::search_rewards,
            commands::catalog_reward::get_reward_details,
            commands::catalog_reward::get_reward_types,
            commands::catalog_reward::get_reward_sources,
            // Table catalog commands
            commands::catalog_table::init_table_catalog,
            commands::catalog_table::search_tables,
            commands::catalog_table::get_table_details,
            commands::catalog_table::get_table_categories,
            commands::catalog_table::get_table_sources,
            // Variant Rule catalog commands
            commands::catalog_variant_rule::init_variant_rule_catalog,
            commands::catalog_variant_rule::search_variant_rules,
            commands::catalog_variant_rule::get_variant_rule_details,
            commands::catalog_variant_rule::get_variant_rule_types,
            commands::catalog_variant_rule::get_variant_rule_sources,
            // Vehicle catalog commands
            commands::catalog_vehicle::init_vehicle_catalog,
            commands::catalog_vehicle::search_vehicles,
            commands::catalog_vehicle::get_vehicle_details,
            commands::catalog_vehicle::get_vehicle_types,
            commands::catalog_vehicle::get_vehicle_terrains,
            commands::catalog_vehicle::get_vehicle_sources,
            // Cult catalog commands
            commands::catalog_cult::init_cult_catalog,
            commands::catalog_cult::search_cults,
            commands::catalog_cult::get_cult_details,
            commands::catalog_cult::get_boon_details,
            commands::catalog_cult::get_cult_types,
            commands::catalog_cult::get_cult_sources
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}