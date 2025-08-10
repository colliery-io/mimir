//! Database connection management

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use crate::error::Result;

/// Type alias for our database connection
pub type DbConnection = SqliteConnection;

/// Establish a connection to the SQLite database
pub fn establish_connection(database_url: &str) -> Result<DbConnection> {
    let mut conn = DbConnection::establish(database_url)?;
    
    // Enable foreign key constraints
    diesel::sql_query("PRAGMA foreign_keys = ON")
        .execute(&mut conn)?;
    
    // Enable WAL mode for better concurrency
    diesel::sql_query("PRAGMA journal_mode = WAL")
        .execute(&mut conn)?;
    
    // Set busy timeout to 5 seconds
    diesel::sql_query("PRAGMA busy_timeout = 5000")
        .execute(&mut conn)?;
    
    Ok(conn)
}

/// Create an in-memory database for testing
#[cfg(test)]
pub fn establish_test_connection() -> Result<DbConnection> {
    establish_connection(":memory:")
}

/// Run a function within a database transaction
pub async fn with_transaction<F, R>(database_url: String, f: F) -> Result<R>
where
    F: FnOnce(&mut DbConnection) -> Result<R> + Send + 'static,
    R: Send + 'static,
{
    tokio::task::spawn_blocking(move || {
        let mut conn = establish_connection(&database_url)?;
        conn.transaction(|conn| f(conn))
    })
    .await?
}

/// Run a function with a database connection (async wrapper)
pub async fn with_connection<F, R>(database_url: String, f: F) -> Result<R>
where
    F: FnOnce(&mut DbConnection) -> Result<R> + Send + 'static,
    R: Send + 'static,
{
    tokio::task::spawn_blocking(move || {
        let mut conn = establish_connection(&database_url)?;
        f(&mut conn)
    })
    .await?
}