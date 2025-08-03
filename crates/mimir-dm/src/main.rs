#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app_init;
mod commands;
mod db_connection;
mod seed_templates;
mod services;
mod types;

use app_init::{initialize_app, AppPaths};
use commands::*;
use services::database::DatabaseService;
use std::sync::{Arc, OnceLock};
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
            transition_campaign_stage,
            initialize_stage_documents
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}