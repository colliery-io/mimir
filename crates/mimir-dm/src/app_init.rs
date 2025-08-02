//! Application initialization and directory management

use anyhow::{Context, Result};
use directories::ProjectDirs;
use mimir_dm_db::run_migrations;
use std::fs;
use std::path::PathBuf;
use tracing::{info, warn};

pub struct AppPaths {
    pub app_dir: PathBuf,
    pub config_dir: PathBuf,
    pub data_dir: PathBuf,
    pub database_path: PathBuf,
    pub is_memory_db: bool,
}

impl AppPaths {
    /// Get application directories, creating them if they don't exist
    pub fn init() -> Result<Self> {
        // Check if we're in development mode
        let is_dev = cfg!(debug_assertions) || std::env::var("MIMIR_DEV").is_ok();
        let use_memory_db = is_dev && std::env::var("MIMIR_USE_FILE_DB").is_err();
        
        if use_memory_db {
            info!("Development mode: Using in-memory database");
            // Still create directories for campaign files
            let project_dirs = ProjectDirs::from("com", "mimir", "mimir")
                .context("Failed to determine application directories")?;
            
            let app_dir = project_dirs.data_dir().to_path_buf();
            let config_dir = project_dirs.config_dir().to_path_buf();
            let data_dir = app_dir.join("data");
            
            fs::create_dir_all(&app_dir).ok();
            fs::create_dir_all(&config_dir).ok();
            fs::create_dir_all(&data_dir).ok();
            
            return Ok(AppPaths {
                app_dir,
                config_dir,
                data_dir,
                database_path: PathBuf::from(":memory:"),
                is_memory_db: true,
            });
        }
        
        let project_dirs = ProjectDirs::from("com", "mimir", "mimir")
            .context("Failed to determine application directories")?;

        let app_dir = project_dirs.data_dir().to_path_buf();
        let config_dir = project_dirs.config_dir().to_path_buf();
        let data_dir = app_dir.join("data");
        let database_path = data_dir.join("mimir.db");

        info!("Initializing application directories:");
        info!("  App dir: {}", app_dir.display());
        info!("  Config dir: {}", config_dir.display());
        info!("  Data dir: {}", data_dir.display());
        info!("  Database: {}", database_path.display());

        // Create directories if they don't exist
        fs::create_dir_all(&app_dir)
            .with_context(|| format!("Failed to create app directory: {}", app_dir.display()))?;
        
        fs::create_dir_all(&config_dir)
            .with_context(|| format!("Failed to create config directory: {}", config_dir.display()))?;
        
        fs::create_dir_all(&data_dir)
            .with_context(|| format!("Failed to create data directory: {}", data_dir.display()))?;

        Ok(AppPaths {
            app_dir,
            config_dir,
            data_dir,
            database_path,
            is_memory_db: false,
        })
    }

    /// Initialize the database, running migrations if needed
    pub fn init_database(&self) -> Result<()> {
        let db_path = self.database_path.to_string_lossy();
        let is_new_db = self.is_memory_db || !self.database_path.exists();

        if self.is_memory_db {
            info!("Using in-memory database");
        } else if is_new_db {
            info!("Creating new database at: {}", db_path);
        } else {
            info!("Using existing database at: {}", db_path);
        }

        // Initialize the connection pool
        let pool = crate::db_connection::init_db_pool(&db_path, self.is_memory_db)
            .context("Failed to initialize database pool")?;
        
        // For file-based databases, run migrations
        if !self.is_memory_db {
            let mut conn = pool.get()
                .context("Failed to get connection from pool")?;
            
            info!("Running database migrations...");
            match run_migrations(&mut *conn) {
                Ok(_) => {
                    info!("Database migrations completed successfully");
                }
                Err(e) => {
                    warn!("Database migration warning: {}", e);
                    // Don't fail on migration warnings - database might already be up to date
                }
            }
        }
        
        // Store the pool globally
        crate::db_connection::DB_POOL
            .set(pool)
            .map_err(|_| anyhow::anyhow!("Failed to set global database pool"))?;

        if is_new_db {
            info!("Database initialized successfully");
            
            // Seed templates for new databases
            info!("Seeding initial templates...");
            if let Err(e) = crate::seed_templates::seed_templates() {
                warn!("Failed to seed templates: {}", e);
                // Don't fail app init if seeding fails
            }
        }

        Ok(())
    }

    /// Get the database path as a string
    pub fn database_path_str(&self) -> String {
        self.database_path.to_string_lossy().to_string()
    }
}

/// Initialize the application on startup
pub fn initialize_app() -> Result<AppPaths> {
    // Initialize simple logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("Starting Mimir application initialization...");

    // Initialize directories
    let app_paths = AppPaths::init()
        .context("Failed to initialize application directories")?;

    // Initialize database
    app_paths.init_database()
        .context("Failed to initialize database")?;

    info!("Application initialization completed successfully");
    Ok(app_paths)
}