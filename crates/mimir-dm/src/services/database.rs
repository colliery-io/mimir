//! Database service for managing database connections

use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::SqliteConnection;
use crate::db_connection::get_db_pool;

pub type DbConnection = PooledConnection<ConnectionManager<SqliteConnection>>;

pub struct DatabaseService;

impl DatabaseService {
    /// Get a database connection from the pool
    pub fn get_connection(&self) -> Result<DbConnection, String> {
        get_db_pool()
            .map_err(|e| format!("Failed to get db pool: {}", e))?
            .get()
            .map_err(|e| format!("Failed to get connection from pool: {}", e))
    }
}