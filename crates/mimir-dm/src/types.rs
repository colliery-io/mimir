//! Shared type definitions

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
        }
    }
}

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Database error: {0}")]
    Database(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Bad request: {0}")]
    BadRequest(String),
    
    #[error("Internal server error: {0}")]
    InternalServerError(String),
}

impl From<mimir_dm_db::error::DbError> for ApiError {
    fn from(err: mimir_dm_db::error::DbError) -> Self {
        match err {
            mimir_dm_db::error::DbError::NotFound { entity_type, id } => {
                ApiError::NotFound(format!("{} with id '{}' not found", entity_type, id))
            }
            _ => ApiError::Database(err.to_string()),
        }
    }
}