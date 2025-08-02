//! Database connection management with support for in-memory databases

use anyhow::{Context, Result};
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use r2d2::{self, Pool};
use std::sync::OnceLock;
use tracing::{info, warn};

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

// Global connection pool for persistent in-memory database support
pub static DB_POOL: OnceLock<DbPool> = OnceLock::new();

pub fn init_db_pool(database_url: &str, is_memory_db: bool) -> Result<DbPool> {
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    
    let mut pool_builder = r2d2::Pool::builder();
    
    if is_memory_db {
        // For in-memory database, we need to maintain at least one connection
        // to prevent the database from being destroyed
        info!("Configuring connection pool for in-memory database");
        pool_builder = pool_builder
            .min_idle(Some(1))  // Always keep at least 1 connection
            .max_size(1);       // Use only 1 connection for in-memory DB
    }
    
    let pool = pool_builder
        .build(manager)
        .context("Failed to create connection pool")?;
    
    // For in-memory databases, run migrations on the pool connection
    if is_memory_db {
        let mut conn = pool.get().context("Failed to get connection from pool")?;
        info!("Running migrations on in-memory database...");
        match mimir_dm_db::run_migrations(&mut conn) {
            Ok(_) => info!("Migrations completed successfully"),
            Err(e) => warn!("Migration warning: {}", e),
        }
    }
    
    Ok(pool)
}

pub fn get_connection() -> Result<r2d2::PooledConnection<ConnectionManager<SqliteConnection>>> {
    DB_POOL
        .get()
        .context("Database pool not initialized")?
        .get()
        .context("Failed to get connection from pool")
}