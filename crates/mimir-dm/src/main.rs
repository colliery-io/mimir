#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app_init;
mod commands;
mod embedded_test_book;
mod seed_templates;
mod services;
mod types;

use app_init::initialize_app;
use commands::{logs, *};
use commands::catalog_action::{search_actions, get_action, get_action_time_types, get_action_sources, get_action_count};
// use commands::catalog_background::{init_background_catalog, search_backgrounds, get_background_details}; // Replaced by catalog_background_db
use commands::catalog_condition::{search_conditions, get_condition, get_condition_item_types, get_condition_sources, get_condition_count};
// use commands::catalog_optionalfeature::{init_optional_feature_catalog, search_optional_features, get_optional_feature_details, get_feature_types}; // Replaced by catalog_optional_feature_db
// Deity catalog now uses database-backed commands only
// Object catalog now uses database-backed service
// Trap catalog now uses database-backed service
use commands::catalog_language_db::{search_languages, get_language_details, get_language_types, get_language_scripts, get_language_sources, get_language_count};
use commands::catalog_reward_db::{search_rewards, get_reward_details, get_reward_types, get_reward_sources, get_reward_count};
use commands::catalog_background_db::{search_backgrounds, get_background_details, get_background_sources, get_background_count};
use commands::catalog_feat_db::{search_feats, get_feat_details, get_feat_sources, get_feat_count};
use commands::catalog_psionic_db::{search_psionics, get_psionic_details, get_psionic_types, get_psionic_orders, get_psionic_sources};
use commands::catalog_vehicle_db::{search_vehicles_db, get_vehicle_details_db, get_vehicle_types_db, get_vehicle_sizes_db, get_vehicle_terrains_db, get_vehicle_statistics_db};
// Class catalog now uses database-backed commands only
use mimir_dm_core::{DatabaseService, run_migrations};
use services::context_service::ContextState;
use services::llm::{self, LlmService, ConfirmationReceivers, CancellationTokens};
use std::collections::HashMap;
use std::sync::Arc;
use tauri::Manager;
use tracing::{error, info, warn};

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

    let is_new_db = app_paths.is_new_database();

    // Start Tauri application
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(move |app| {
            // Initialize database service from core

            let db_service = DatabaseService::new(
                &app_paths.database_path_str(),
                app_paths.is_memory_db
            ).expect("Failed to initialize database service");

            // Run migrations
            info!("Running database migrations...");
            let mut conn = db_service.get_connection()
                .expect("Failed to get connection for migrations");
            match run_migrations(&mut *conn) {
                Ok(_) => info!("Database migrations completed successfully"),
                Err(e) => warn!("Database migration warning: {}", e),
            }
            // Seed templates for new databases (reuse migration connection)
            if is_new_db {
                info!("Seeding initial templates...");
                if let Err(e) = seed_templates::seed_templates(&mut conn) {
                    warn!("Failed to seed templates: {}", e);
                }
            }
            drop(conn); // Release connection after seeding

            let db_service = Arc::new(db_service);
            let db_service_clone = Arc::clone(&db_service);
            app.manage(db_service);

            // Register AppPaths as Tauri state
            let app_paths_state = Arc::new(app_paths);
            app.manage(app_paths_state.clone());

            // Initialize context service
            let context_state = ContextState::new();
            app.manage(context_state);

            // Initialize session manager
            let session_manager = commands::chat_sessions::init_session_manager(&app_paths_state)
                .map_err(|e| {
                    error!("Failed to initialize session manager: {}", e);
                    e
                })?;
            app.manage(session_manager);
            
            // Create shared confirmation receivers for LLM tools
            let confirmation_receivers: ConfirmationReceivers = Arc::new(tokio::sync::Mutex::new(HashMap::new()));
            let cancellation_tokens: CancellationTokens = Arc::new(tokio::sync::Mutex::new(HashMap::new()));
            let confirmation_receivers_clone = Arc::clone(&confirmation_receivers);
            app.manage(confirmation_receivers);
            app.manage(cancellation_tokens);
            
            // Initialize LLM service
            let app_handle = app.handle().clone();
            let llm_service = Arc::new(tokio::sync::Mutex::new(None::<LlmService>));
            let llm_service_clone = Arc::clone(&llm_service);
            let app_paths_clone = Arc::clone(&app_paths_state);

            // Spawn async task to initialize LLM
            tauri::async_runtime::spawn(async move {
                info!("Starting LLM service initialization...");
                match llm::initialize_llm(app_handle, db_service_clone, confirmation_receivers_clone, app_paths_clone).await {
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

            // All catalogs now use database-backed services (no in-memory state needed)
            
            // Vehicle catalog now uses database-backed service (no state needed)
            
            // Cult catalog now uses database-backed system
            
            // Psionic catalog now uses database-backed service (no state needed)
            
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
            // Condition catalog commands
            search_conditions,
            get_condition,
            get_condition_item_types,
            get_condition_sources,
            get_condition_count,
            // Optional feature catalog commands (database-backed)
            search_optional_features_db,
            get_optional_feature_db,
            get_optional_feature_details,
            get_optional_feature_types,
            get_optional_feature_sources,
            // Item catalog commands (database-backed)
            search_items_db,
            get_item_db,
            get_item_details_db,
            get_item_types_db,
            get_item_rarities_db,
            get_item_sources_db,
            // Monster catalog commands (database-backed)
            search_monsters_db,
            get_monster_details_db,
            get_monster_sizes_db,
            get_monster_types_db,
            get_monster_alignments_db,
            get_monster_cr_range_db,
            get_monster_statistics_db,
            // Object catalog commands
            search_objects,
            get_object_details,
            get_object_sources,
            get_object_count,
            get_object_types,
            get_object_sizes,
            // Trap catalog commands
            search_traps,
            get_trap_details,
            get_trap_sources,
            get_trap_count,
            get_trap_types,
            get_trap_categories,
            // Cult catalog commands
            search_cults,
            get_cult_details,
            get_cult_sources,
            get_cult_count,
            get_cult_types,
            get_cult_categories,
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
            // Variant Rule catalog commands (database-backed)
            commands::catalog_variant_rule_db::search_variant_rules,
            commands::catalog_variant_rule_db::get_variant_rule,
            commands::catalog_variant_rule_db::get_variant_rule_details,
            commands::catalog_variant_rule_db::get_variant_rule_types,
            commands::catalog_variant_rule_db::get_variant_rule_sources,
            // Psionic catalog commands
            search_psionics,
            get_psionic_details,
            get_psionic_types,
            get_psionic_orders,
            get_psionic_sources,
            // Deity catalog commands (database-backed)
            search_deities_db,
            get_deity_details_db,
            get_deity_pantheons_db,
            get_deity_domains_db,
            get_deity_alignments_db,
            get_deity_statistics_db,
            // Vehicle catalog commands (database-backed)
            search_vehicles_db,
            get_vehicle_details_db,
            get_vehicle_types_db,
            get_vehicle_sizes_db,
            get_vehicle_terrains_db,
            get_vehicle_statistics_db,
            // Class catalog commands (database-backed)
            commands::catalog_class_db::search_classes_db,
            commands::catalog_class_db::get_class_details_db,
            commands::catalog_class_db::get_subclass_details_db,
            commands::catalog_class_db::get_class_subclasses_db,
            commands::catalog_class_db::get_class_sources_db,
            commands::catalog_class_db::get_class_primary_abilities_db,
            commands::catalog_class_db::get_class_statistics_db,
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
            open_log_viewer_window,
            // LLM commands
            llm::commands::check_llm_status,
            llm::commands::get_llm_model_info,
            llm::commands::send_chat_message,
            llm::commands::cancel_chat_message,
            llm::commands::get_model_context_info,
            llm::commands::confirm_tool_action,
            llm::commands::list_available_models,
            // Chat session commands
            list_chat_sessions,
            load_chat_session,
            save_chat_session,
            create_chat_session,
            delete_chat_session,
            // Session todo commands
            get_session_todos,
            configure_todo_storage,
            // Log management commands
            logs::list_log_files,
            logs::read_log_file,
            logs::tail_log_file,
            logs::open_logs_folder
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}