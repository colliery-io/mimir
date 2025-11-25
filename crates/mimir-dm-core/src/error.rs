//! Database error types

use thiserror::Error;

/// Result type alias for database operations
pub type Result<T> = std::result::Result<T, DbError>;

/// Database error types
#[derive(Error, Debug)]
pub enum DbError {
    /// Diesel connection error
    #[error("Database connection error: {0}")]
    Connection(#[from] diesel::ConnectionError),
    
    /// Diesel query/result error
    #[error("Database query error: {0}")]
    Query(#[from] diesel::result::Error),
    
    /// Entity not found
    #[error("Entity not found: {entity_type} with id '{id}'")]
    NotFound {
        /// Type of entity that was not found.
        entity_type: String,
        /// Identifier that was searched for.
        id: String,
    },

    /// Constraint violation
    #[error("Constraint violation: {field} - {message}")]
    ConstraintViolation {
        /// Field that violated the constraint.
        field: String,
        /// Description of the violation.
        message: String,
    },
    
    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    /// Invalid data
    #[error("Invalid data: {0}")]
    InvalidData(String),
    
    /// Transaction error
    #[error("Transaction error: {0}")]
    Transaction(String),
    
    /// Async runtime error
    #[error("Async runtime error: {0}")]
    Runtime(#[from] tokio::task::JoinError),
    
    /// Migration error
    #[error("Migration error: {0}")]
    Migration(String),
    
    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl DbError {
    /// Check if error is a unique constraint violation
    pub fn is_unique_violation(&self) -> bool {
        matches!(self, 
            DbError::Query(diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation, 
                _
            ))
        )
    }
    
    /// Check if error is a foreign key violation
    pub fn is_foreign_key_violation(&self) -> bool {
        matches!(self, 
            DbError::Query(diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::ForeignKeyViolation, 
                _
            ))
        )
    }
    
    /// Check if error is not found
    pub fn is_not_found(&self) -> bool {
        matches!(self, 
            DbError::NotFound { .. } | 
            DbError::Query(diesel::result::Error::NotFound)
        )
    }
}