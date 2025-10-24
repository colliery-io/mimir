//! # Session Management for GGUF Provider
//!
//! Handles session pooling and management for concurrent inference requests.

use std::sync::{Arc, Mutex};
use tracing::{debug, warn};

use llama_cpp_rs::{LlamaModel, LlamaSession, SessionParams};
use crate::traits::LlmError;

/// RAII guard for session management
pub struct SessionGuard {
    pub session: Option<LlamaSession>,
    pool: Arc<Mutex<Vec<LlamaSession>>>,
}

impl SessionGuard {
    /// Get a mutable reference to the session
    pub fn session_mut(&mut self) -> &mut LlamaSession {
        self.session.as_mut().expect("Session should be available")
    }
    
    /// Get a reference to the session
    pub fn session(&self) -> &LlamaSession {
        self.session.as_ref().expect("Session should be available")
    }
}

impl Drop for SessionGuard {
    fn drop(&mut self) {
        if let Some(session) = self.session.take() {
            let mut pool = self.pool.lock().unwrap();
            
            // Return session to pool if not at capacity
            if pool.len() < 8 {  // Max pool size
                debug!("Returning session to pool (size: {})", pool.len());
                pool.push(session);
            } else {
                debug!("Pool at capacity, dropping session");
            }
        }
    }
}

/// Session pool for managing concurrent inference requests
pub struct SessionPool {
    model: Arc<LlamaModel>,
    pool: Arc<Mutex<Vec<LlamaSession>>>,
    session_params: SessionParams,
}

impl SessionPool {
    /// Create a new session pool
    pub fn new(model: Arc<LlamaModel>, initial_size: usize) -> Result<Self, LlmError> {
        let session_params = SessionParams::default();
        let mut sessions = Vec::new();
        
        // Pre-create initial sessions
        for i in 0..initial_size {
            let session = model.create_session(session_params.clone())
                .map_err(|e| LlmError::ProviderError(
                    format!("Failed to create initial session {}: {}", i, e)
                ))?;
            sessions.push(session);
        }
        
        debug!("Created session pool with {} initial sessions", initial_size);
        
        Ok(Self {
            model,
            pool: Arc::new(Mutex::new(sessions)),
            session_params,
        })
    }
    
    /// Acquire a session from the pool
    pub fn acquire(&self) -> Result<SessionGuard, LlmError> {
        let mut pool = self.pool.lock().unwrap();
        
        let session = if let Some(session) = pool.pop() {
            debug!("Acquired existing session from pool (remaining: {})", pool.len());
            session
        } else {
            debug!("Pool empty, creating new session");
            // Pool is empty, create new session
            drop(pool); // Release lock before potentially slow operation
            
            self.model.create_session(self.session_params.clone())
                .map_err(|e| LlmError::ProviderError(
                    format!("Failed to create new session: {}", e)
                ))?
        };
        
        Ok(SessionGuard {
            session: Some(session),
            pool: Arc::clone(&self.pool),
        })
    }
    
    /// Get current pool statistics
    pub fn stats(&self) -> SessionPoolStats {
        let pool = self.pool.lock().unwrap();
        SessionPoolStats {
            available_sessions: pool.len(),
            max_sessions: 8,
        }
    }
    
    /// Clear all sessions in the pool
    pub fn clear(&self) {
        let mut pool = self.pool.lock().unwrap();
        let cleared = pool.len();
        pool.clear();
        debug!("Cleared {} sessions from pool", cleared);
    }
}

/// Statistics about the session pool
#[derive(Debug, Clone)]
pub struct SessionPoolStats {
    pub available_sessions: usize,
    pub max_sessions: usize,
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_session_pool_stats() {
        let stats = SessionPoolStats {
            available_sessions: 3,
            max_sessions: 8,
        };
        
        assert_eq!(stats.available_sessions, 3);
        assert_eq!(stats.max_sessions, 8);
    }
}