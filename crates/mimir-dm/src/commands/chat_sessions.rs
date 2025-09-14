//! Chat session management commands
//! 
//! Handles persistent storage and retrieval of chat sessions as JSON files

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::State;
use tracing::{error, info};
use uuid::Uuid;

use crate::app_init::AppPaths;

/// Represents a chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub role: String, // 'user' | 'assistant' | 'system'
    pub content: String,
    pub timestamp: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_usage: Option<TokenUsage>,
}

/// Token usage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    pub prompt: u32,
    pub completion: u32,
    pub total: u32,
}

/// A complete chat session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSession {
    pub id: String,
    pub title: String,
    pub created_at: u64,
    pub updated_at: u64,
    pub messages: Vec<ChatMessage>,
}

/// Lightweight metadata for session list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSessionMetadata {
    pub id: String,
    pub title: String,
    pub created_at: u64,
    pub updated_at: u64,
    pub message_count: usize,
    pub preview: String, // First user message preview
}

/// Index file structure
#[derive(Debug, Serialize, Deserialize)]
pub struct SessionsIndex {
    pub sessions: Vec<ChatSessionMetadata>,
}

impl Default for SessionsIndex {
    fn default() -> Self {
        Self {
            sessions: Vec::new(),
        }
    }
}

/// Session manager handles file operations
pub struct SessionManager {
    sessions_dir: PathBuf,
    index_file: PathBuf,
}

impl SessionManager {
    pub fn new(app_paths: &AppPaths) -> Result<Self> {
        let sessions_dir = app_paths.data_dir.join("chat_sessions");
        let index_file = sessions_dir.join("sessions_index.json");
        
        // Create sessions directory if it doesn't exist
        fs::create_dir_all(&sessions_dir)
            .with_context(|| format!("Failed to create sessions directory: {}", sessions_dir.display()))?;
        
        info!("Session manager initialized: {}", sessions_dir.display());
        
        Ok(Self {
            sessions_dir,
            index_file,
        })
    }
    
    /// Load the sessions index, creating it if it doesn't exist
    fn load_index(&self) -> Result<SessionsIndex> {
        if self.index_file.exists() {
            let contents = fs::read_to_string(&self.index_file)
                .with_context(|| format!("Failed to read index file: {}", self.index_file.display()))?;
            
            serde_json::from_str(&contents)
                .with_context(|| "Failed to parse sessions index")
        } else {
            // Create empty index
            let index = SessionsIndex::default();
            self.save_index(&index)?;
            Ok(index)
        }
    }
    
    /// Save the sessions index
    fn save_index(&self, index: &SessionsIndex) -> Result<()> {
        let contents = serde_json::to_string_pretty(index)
            .context("Failed to serialize sessions index")?;
        
        fs::write(&self.index_file, contents)
            .with_context(|| format!("Failed to write index file: {}", self.index_file.display()))?;
        
        Ok(())
    }
    
    /// Get session file path
    fn get_session_path(&self, session_id: &str) -> PathBuf {
        self.sessions_dir.join(format!("{}.json", session_id))
    }
    
    /// Generate a preview string from session messages
    fn generate_preview(messages: &[ChatMessage]) -> String {
        messages
            .iter()
            .find(|msg| msg.role == "user")
            .map(|msg| {
                let content = msg.content.trim();
                if content.len() > 60 {
                    format!("{}...", &content[..57])
                } else {
                    content.to_string()
                }
            })
            .unwrap_or_else(|| "New chat".to_string())
    }
    
    /// Generate title from first user message
    fn generate_title(messages: &[ChatMessage]) -> String {
        messages
            .iter()
            .find(|msg| msg.role == "user")
            .map(|msg| {
                let content = msg.content.trim();
                if content.len() > 50 {
                    format!("{}...", &content[..47])
                } else {
                    content.to_string()
                }
            })
            .unwrap_or_else(|| "New chat".to_string())
    }
    
    /// List all sessions (returns metadata only)
    pub fn list_sessions(&self) -> Result<Vec<ChatSessionMetadata>> {
        let index = self.load_index()?;
        Ok(index.sessions)
    }
    
    /// Load a specific session
    pub fn load_session(&self, session_id: &str) -> Result<Option<ChatSession>> {
        let session_path = self.get_session_path(session_id);
        
        if !session_path.exists() {
            return Ok(None);
        }
        
        let contents = fs::read_to_string(&session_path)
            .with_context(|| format!("Failed to read session file: {}", session_path.display()))?;
        
        let session: ChatSession = serde_json::from_str(&contents)
            .with_context(|| "Failed to parse session file")?;
        
        Ok(Some(session))
    }
    
    /// Save a session and update the index
    pub fn save_session(&self, mut session: ChatSession) -> Result<()> {
        // Update the session timestamp
        session.updated_at = chrono::Utc::now().timestamp() as u64;
        
        // If no title is set, generate one
        if session.title.is_empty() || session.title == "New chat" {
            session.title = Self::generate_title(&session.messages);
        }
        
        // Save session file
        let session_path = self.get_session_path(&session.id);
        let contents = serde_json::to_string_pretty(&session)
            .context("Failed to serialize session")?;
        
        fs::write(&session_path, contents)
            .with_context(|| format!("Failed to write session file: {}", session_path.display()))?;
        
        // Update index
        let mut index = self.load_index()?;
        
        let metadata = ChatSessionMetadata {
            id: session.id.clone(),
            title: session.title.clone(),
            created_at: session.created_at,
            updated_at: session.updated_at,
            message_count: session.messages.len(),
            preview: Self::generate_preview(&session.messages),
        };
        
        // Update or insert metadata
        if let Some(existing) = index.sessions.iter_mut().find(|s| s.id == session.id) {
            *existing = metadata;
        } else {
            index.sessions.push(metadata);
        }
        
        // Sort by updated_at descending (most recent first)
        index.sessions.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        
        self.save_index(&index)?;
        
        info!("Saved session: {} ({})", session.title, session.id);
        Ok(())
    }
    
    /// Create a new session
    pub fn create_session(&self) -> Result<ChatSession> {
        let now = chrono::Utc::now().timestamp() as u64;
        
        let session = ChatSession {
            id: Uuid::new_v4().to_string(),
            title: "New chat".to_string(),
            created_at: now,
            updated_at: now,
            messages: Vec::new(),
        };
        
        info!("Created new session: {}", session.id);
        Ok(session)
    }
    
    /// Delete a session
    pub fn delete_session(&self, session_id: &str) -> Result<bool> {
        let session_path = self.get_session_path(session_id);
        
        if !session_path.exists() {
            return Ok(false);
        }
        
        // Remove session file
        fs::remove_file(&session_path)
            .with_context(|| format!("Failed to delete session file: {}", session_path.display()))?;
        
        // Update index
        let mut index = self.load_index()?;
        index.sessions.retain(|s| s.id != session_id);
        self.save_index(&index)?;
        
        info!("Deleted session: {}", session_id);
        Ok(true)
    }
}

/// Global session manager state
pub type SessionManagerState<'a> = State<'a, SessionManager>;

/// Initialize session manager
pub fn init_session_manager(app_paths: &AppPaths) -> Result<SessionManager> {
    SessionManager::new(app_paths)
}

/// Tauri command: List all chat sessions
#[tauri::command]
pub async fn list_chat_sessions(
    session_manager: SessionManagerState<'_>,
) -> Result<Vec<ChatSessionMetadata>, String> {
    session_manager
        .list_sessions()
        .map_err(|e| {
            error!("Failed to list chat sessions: {}", e);
            format!("Failed to list sessions: {}", e)
        })
}

/// Tauri command: Load a specific chat session
#[tauri::command]
pub async fn load_chat_session(
    session_manager: SessionManagerState<'_>,
    session_id: String,
) -> Result<Option<ChatSession>, String> {
    session_manager
        .load_session(&session_id)
        .map_err(|e| {
            error!("Failed to load chat session {}: {}", session_id, e);
            format!("Failed to load session: {}", e)
        })
}

/// Tauri command: Save a chat session
#[tauri::command]
pub async fn save_chat_session(
    session_manager: SessionManagerState<'_>,
    session: ChatSession,
) -> Result<(), String> {
    session_manager
        .save_session(session)
        .map_err(|e| {
            error!("Failed to save chat session: {}", e);
            format!("Failed to save session: {}", e)
        })
}

/// Tauri command: Create a new chat session
#[tauri::command]
pub async fn create_chat_session(
    session_manager: SessionManagerState<'_>,
    llm_service: tauri::State<'_, Arc<tokio::sync::Mutex<Option<crate::services::llm_service::LlmService>>>>,
) -> Result<ChatSession, String> {
    let session = session_manager
        .create_session()
        .map_err(|e| {
            error!("Failed to create chat session: {}", e);
            format!("Failed to create session: {}", e)
        })?;
    
    // Initialize chat logger for this session
    if let Some(llm) = llm_service.lock().await.as_ref() {
        match llm.get_chat_logger(&session.id).await {
            Ok(logger) => {
                logger.log_session_info("session_created", serde_json::json!({
                    "session_id": session.id,
                    "created_at": session.created_at,
                    "title": session.title
                }));
                info!("Initialized chat logger for session: {}", session.id);
            }
            Err(e) => {
                error!("Failed to initialize chat logger for session {}: {}", session.id, e);
                // Don't fail session creation if logging fails
            }
        }
    } else {
        info!("LLM service not initialized, skipping chat logger setup for session: {}", session.id);
    }
    
    Ok(session)
}

/// Tauri command: Delete a chat session
#[tauri::command]
pub async fn delete_chat_session(
    session_manager: SessionManagerState<'_>,
    session_id: String,
) -> Result<bool, String> {
    session_manager
        .delete_session(&session_id)
        .map_err(|e| {
            error!("Failed to delete chat session {}: {}", session_id, e);
            format!("Failed to delete session: {}", e)
        })
}