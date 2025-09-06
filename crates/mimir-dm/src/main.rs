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
use commands::catalog_action::{search_actions, get_action, get_action_time_types, get_action_sources, get_action_count};
// use commands::catalog_background::{init_background_catalog, search_backgrounds, get_background_details}; // Replaced by catalog_background_db
use commands::catalog_condition::{search_conditions, get_condition, get_condition_item_types, get_condition_sources, get_condition_count};
use commands::catalog_optionalfeature::{init_optional_feature_catalog, search_optional_features, get_optional_feature_details, get_feature_types};
use commands::catalog_deity::{init_deity_catalog, search_deities, get_deity_details, get_pantheons, get_domains};
use commands::catalog_object::{init_object_catalog, search_objects, get_object_details, get_object_types};
use commands::catalog_trap::{init_trap_catalog, search_traps, get_trap_details, get_trap_types};
use commands::catalog_language_db::{search_languages, get_language_details, get_language_types, get_language_scripts, get_language_sources, get_language_count};
use commands::catalog_reward_db::{search_rewards, get_reward_details, get_reward_types, get_reward_sources, get_reward_count};
use commands::catalog_background_db::{search_backgrounds, get_background_details, get_background_sources, get_background_count};
use commands::catalog_feat_db::{search_feats, get_feat_details, get_feat_sources, get_feat_count};
use services::database::DatabaseService;
use services::context_service::ContextState;
use services::llm_service::{self, LlmService, ConfirmationReceivers};
use std::collections::HashMap;
use std::sync::{Arc, OnceLock, Mutex};
use tauri::Manager;
use tracing::{error, info, warn};

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
            let db_service_clone = Arc::clone(&db_service);
            app.manage(db_service);
            
            // Initialize context service
            let context_state = ContextState::new();
            app.manage(context_state);
            
            // Initialize session manager
            let app_paths = APP_PATHS.get().expect("App paths should be initialized");
            let session_manager = commands::chat_sessions::init_session_manager(app_paths)
                .map_err(|e| {
                    error!("Failed to initialize session manager: {}", e);
                    e
                })?;
            app.manage(session_manager);
            
            // Create shared confirmation receivers for LLM tools
            let confirmation_receivers: ConfirmationReceivers = Arc::new(tokio::sync::Mutex::new(HashMap::new()));
            let confirmation_receivers_clone = Arc::clone(&confirmation_receivers);
            app.manage(confirmation_receivers);
            
            // Initialize LLM service
            let app_handle = app.handle().clone();
            let llm_service = Arc::new(tokio::sync::Mutex::new(None::<LlmService>));
            let llm_service_clone = Arc::clone(&llm_service);
            
            // Spawn async task to initialize LLM
            tauri::async_runtime::spawn(async move {
                info!("Starting LLM service initialization...");
                match llm_service::initialize_llm(app_handle, db_service_clone, confirmation_receivers_clone).await {
                    Ok(service) => {
                        info!("LLM service initialized successfully");
                        let mut llm = llm_service_clone.lock().await;
                        *llm = Some(service);
                    }
                    Err(e) => {
                        error!("Failed to initialize LLM service: {}", e);
                        warn!("Application will continue without LLM functionality");
                        // Don't fail app startup if LLM fails to initialize
                    }
                }
            });
            
            app.manage(llm_service);
            
            // Initialize catalogs
            let item_catalog = Mutex::new(commands::catalog::ItemCatalog::new());
            let monster_catalog = Mutex::new(commands::catalog::MonsterCatalog::new());
            let class_catalog = Mutex::new(commands::catalog_class::ClassCatalog::new());
            // Feat catalog now uses database-backed service
            // Race catalog now uses database-backed service
            // Background catalog now uses database-backed service
            // Action catalog now uses database-backed service
            // Condition catalog now uses database-backed service
            let optional_feature_catalog = Mutex::new(commands::catalog_optionalfeature::OptionalFeatureCatalog::new());
            let deity_catalog = Mutex::new(commands::catalog_deity::DeityCatalog::new());
            let object_catalog = Mutex::new(commands::catalog_object::ObjectCatalog::new());
            let trap_catalog = Mutex::new(commands::catalog_trap::TrapCatalog::new());
            // Language catalog now uses database-backed service
            app.manage(item_catalog);
            app.manage(monster_catalog);
            app.manage(class_catalog);
            // Feat catalog now uses database-backed service (no state needed)
            // Race catalog now uses database-backed service (no state needed)
            // Background catalog now uses database-backed service (no state needed)
            // Action catalog now uses database-backed service (no state needed)
            // Condition catalog now uses database-backed service (no state needed)
            app.manage(optional_feature_catalog);
            app.manage(deity_catalog);
            app.manage(object_catalog);
            app.manage(trap_catalog);
            // Language catalog now uses database-backed service (no state needed)
            
            // Reward catalog now uses database-backed service (no state needed)
            
            let table_catalog = std::sync::Mutex::new(commands::catalog_table::TableCatalog::new());
            app.manage(table_catalog);
            
            let variant_rule_catalog = std::sync::Mutex::new(commands::catalog_variant_rule::VariantRuleCatalog::new());
            app.manage(variant_rule_catalog);
            
            let vehicle_catalog = std::sync::Mutex::new(commands::catalog_vehicle::VehicleCatalog::new());
            app.manage(vehicle_catalog);
            
            let cult_catalog = std::sync::Mutex::new(commands::catalog_cult::CultCatalog::new());
            app.manage(cult_catalog);
            
            let psionic_catalog = std::sync::Mutex::new(commands::catalog_psionic::PsionicCatalog::new());
            app.manage(psionic_catalog);
            
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
            archive_campaign,
            unarchive_campaign,
            delete_campaign,
            list_archived_campaigns,
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
            search_spells,
            get_spell_details,
            get_spell_sources,
            get_spell_schools,
            get_spell_statistics,
            get_spell_count,
            // Action catalog commands
            search_actions,
            get_action,
            get_action_time_types,
            get_action_sources,
            get_action_count,
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
            search_feats,
            get_feat_details,
            get_feat_sources,
            get_feat_count,
            // Race catalog commands
            search_races,
            get_race_details,
            get_race_sources,
            get_race_count,
            get_race_sizes,
            // Background catalog commands
            search_backgrounds,
            get_background_details,
            get_background_sources,
            get_background_count,
            // Feat catalog commands
            search_feats,
            get_feat_details,
            get_feat_sources,
            get_feat_count,
            // Action catalog commands
            search_actions,
            get_action,
            // Condition catalog commands
            search_conditions,
            get_condition,
            get_condition_item_types,
            get_condition_sources,
            get_condition_count,
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
            search_languages,
            get_language_details,
            get_language_types,
            get_language_scripts,
            get_language_sources,
            get_language_count,
            // Reward catalog commands
            search_rewards,
            get_reward_details,
            get_reward_types,
            get_reward_sources,
            get_reward_count,
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
            commands::catalog_cult::get_cult_sources,
            commands::catalog_psionic::search_psionics,
            commands::catalog_psionic::get_psionic_details,
            commands::catalog_psionic::get_psionic_orders,
            commands::catalog_psionic::get_psionic_sources,
            // Context commands
            update_context,
            get_full_context,
            register_window,
            unregister_window,
            clear_shared_context,
            get_context_for_llm,
            update_context_usage,
            // Window management commands
            open_context_debug_window,
            open_chat_window,
            // LLM commands
            llm_service::check_llm_status,
            llm_service::get_llm_model_info,
            llm_service::send_chat_message,
            llm_service::get_model_context_info,
            llm_service::confirm_tool_action,
            llm_service::list_available_models,
            // Chat session commands
            list_chat_sessions,
            load_chat_session,
            save_chat_session,
            create_chat_session,
            delete_chat_session,
            // Todo commands
            get_todos,
            clear_todos
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}