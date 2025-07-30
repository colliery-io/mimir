//! Error types for bundle import operations

use std::path::PathBuf;
use thiserror::Error;

/// Result type for import operations
pub type ImportResult<T> = Result<T, ImportError>;

/// Errors that can occur during bundle import operations
#[derive(Error, Debug)]
pub enum ImportError {
    /// Bundle file not found or inaccessible
    #[error("Bundle file not found: {path}")]
    BundleNotFound { path: PathBuf },

    /// Bundle archive is corrupted or invalid
    #[error("Invalid bundle archive: {reason}")]
    InvalidArchive { reason: String },

    /// Bundle manifest is missing or invalid
    #[error("Invalid bundle manifest: {reason}")]
    InvalidManifest { reason: String },

    /// Bundle structure doesn't match expected format
    #[error("Invalid bundle structure: {reason}")]
    InvalidStructure { reason: String },

    /// Required entity file is missing from bundle
    #[error("Missing required file: {filename}")]
    MissingFile { filename: String },

    /// Entity data is malformed or invalid
    #[error("Invalid entity data in {filename}: {reason}")]
    InvalidEntityData { filename: String, reason: String },

    /// Database operation failed
    #[error("Database error: {0}")]
    Database(#[from] mimir_dm_db::DbError),

    /// JSON parsing error
    #[error("JSON parsing error in {filename}: {source}")]
    JsonParsing {
        filename: String,
        source: serde_json::Error,
    },

    /// IO operation failed
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Rule system already exists in database
    #[error("Rule system '{rule_system_id}' already exists")]
    RuleSystemExists { rule_system_id: String },

    /// Foreign key constraint violation
    #[error("Foreign key constraint violation: {reason}")]
    ForeignKeyViolation { reason: String },

    /// Generic bundle validation error
    #[error("Bundle validation failed: {reason}")]
    ValidationFailed { reason: String },
}

impl ImportError {
    /// Create a new InvalidManifest error
    pub fn invalid_manifest<S: Into<String>>(reason: S) -> Self {
        Self::InvalidManifest {
            reason: reason.into(),
        }
    }

    /// Create a new InvalidStructure error
    pub fn invalid_structure<S: Into<String>>(reason: S) -> Self {
        Self::InvalidStructure {
            reason: reason.into(),
        }
    }

    /// Create a new InvalidEntityData error
    pub fn invalid_entity_data<S: Into<String>>(filename: S, reason: S) -> Self {
        Self::InvalidEntityData {
            filename: filename.into(),
            reason: reason.into(),
        }
    }

    /// Create a new ValidationFailed error
    pub fn validation_failed<S: Into<String>>(reason: S) -> Self {
        Self::ValidationFailed {
            reason: reason.into(),
        }
    }
}