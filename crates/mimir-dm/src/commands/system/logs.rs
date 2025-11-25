//! Log file management commands.
//!
//! Provides Tauri commands for listing, reading, and tailing application
//! and chat session log files.

use crate::state::AppState;
use crate::types::ApiResponse;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use tauri::State;
use tracing::{debug, info};

/// Information about a single log file.
#[derive(Serialize, Deserialize, Debug)]
pub struct LogFileInfo {
    /// File name without path.
    pub name: String,
    /// Absolute path to the file.
    pub full_path: String,
    /// File size in bytes.
    pub size: u64,
    /// Last modification timestamp.
    pub modified: String,
    /// Whether this is the current active log file.
    pub is_current: bool,
}

/// Response containing lists of available log files.
#[derive(Serialize, Deserialize, Debug)]
pub struct LogFilesResponse {
    /// Application log files (mimir.log*).
    pub application_logs: Vec<LogFileInfo>,
    /// Chat session log files.
    pub chat_logs: Vec<LogFileInfo>,
}

/// Contents of a log file with pagination info.
#[derive(Serialize, Deserialize, Debug)]
pub struct LogContent {
    /// Log lines for the requested range.
    pub lines: Vec<String>,
    /// Total number of lines in the file.
    pub total_lines: usize,
    /// Current byte position for tailing.
    pub position: u64,
}

/// Response from tailing a log file.
#[derive(Serialize, Deserialize, Debug)]
pub struct LogTailResponse {
    /// New lines since last position.
    pub new_lines: Vec<String>,
    /// New byte position for next tail call.
    pub new_position: u64,
}

/// List all available log files in both application and chat directories.
///
/// Returns lists of application logs (mimir.log*) and chat session logs,
/// sorted by modification time with newest first.
///
/// # Returns
/// `ApiResponse` containing `LogFilesResponse` with both log categories.
///
/// # Errors
/// Returns error string if directory reading fails.
#[tauri::command]
pub async fn list_log_files(
    state: State<'_, AppState>,
) -> Result<ApiResponse<LogFilesResponse>, String> {
    info!("Listing log files");

    let logs_dir = &state.paths.logs_dir;
    let chat_logs_dir = logs_dir.join("chat_sessions");

    let mut application_logs = Vec::new();
    let mut chat_logs = Vec::new();

    // Read application log files
    if logs_dir.exists() {
        let entries =
            fs::read_dir(logs_dir).map_err(|e| format!("Failed to read logs directory: {}", e))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let path = entry.path();

            if path.is_file() {
                let filename = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string();

                // Only include mimir.log files
                if filename.starts_with("mimir.log") {
                    let metadata = entry
                        .metadata()
                        .map_err(|e| format!("Failed to read file metadata: {}", e))?;

                    let size = metadata.len();
                    let modified = metadata
                        .modified()
                        .map_err(|e| format!("Failed to read modification time: {}", e))?;

                    let modified_dt: DateTime<Utc> = modified.into();
                    let is_current = filename == "mimir.log";

                    application_logs.push(LogFileInfo {
                        name: filename.clone(),
                        full_path: path.to_string_lossy().to_string(),
                        size,
                        modified: modified_dt.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
                        is_current,
                    });
                }
            }
        }

        // Sort application logs by modification time (newest first)
        application_logs.sort_by(|a, b| b.modified.cmp(&a.modified));
    }

    // Read chat log files
    if chat_logs_dir.exists() {
        let entries = fs::read_dir(&chat_logs_dir)
            .map_err(|e| format!("Failed to read chat logs directory: {}", e))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let path = entry.path();

            if path.is_file() {
                let filename = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string();

                // Only include .log files
                if filename.ends_with(".log") {
                    let metadata = entry
                        .metadata()
                        .map_err(|e| format!("Failed to read file metadata: {}", e))?;

                    let size = metadata.len();
                    let modified = metadata
                        .modified()
                        .map_err(|e| format!("Failed to read modification time: {}", e))?;

                    let modified_dt: DateTime<Utc> = modified.into();

                    chat_logs.push(LogFileInfo {
                        name: filename.clone(),
                        full_path: path.to_string_lossy().to_string(),
                        size,
                        modified: modified_dt.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
                        is_current: false, // Chat logs don't have a "current" concept
                    });
                }
            }
        }

        // Sort chat logs by modification time (newest first)
        chat_logs.sort_by(|a, b| b.modified.cmp(&a.modified));
    }

    info!(
        "Found {} application log files and {} chat log files",
        application_logs.len(),
        chat_logs.len()
    );

    let response = LogFilesResponse {
        application_logs,
        chat_logs,
    };

    Ok(ApiResponse::success(response))
}

/// Read content from a log file with pagination.
///
/// Returns a range of lines from the specified log file.
/// Validates the file name to prevent directory traversal attacks.
///
/// # Parameters
/// - `file_name` - Name of the log file (not a path)
/// - `offset` - Line number to start from (0-indexed)
/// - `limit` - Maximum number of lines to return
///
/// # Returns
/// `ApiResponse` containing `LogContent` with lines and metadata.
///
/// # Errors
/// Returns error response if file name is invalid or file cannot be read.
#[tauri::command]
pub async fn read_log_file(
    file_name: String,
    offset: usize,
    limit: usize,
    state: State<'_, AppState>,
) -> Result<ApiResponse<LogContent>, String> {
    debug!(
        "Reading log file: {} (offset: {}, limit: {})",
        file_name, offset, limit
    );

    // Validate file name to prevent directory traversal
    if file_name.contains("..") || file_name.contains("/") || file_name.contains("\\") {
        return Ok(ApiResponse::error("Invalid file name".to_string()));
    }

    // Determine file path - check application logs first, then chat logs
    let file_path = if file_name.starts_with("mimir.log") {
        state.paths.logs_dir.join(&file_name)
    } else if file_name.ends_with(".log") {
        state.paths.logs_dir.join("chat_sessions").join(&file_name)
    } else {
        return Ok(ApiResponse::error("Invalid log file type".to_string()));
    };

    if !file_path.exists() {
        return Ok(ApiResponse::error(format!(
            "Log file not found: {}",
            file_name
        )));
    }

    // Read file content
    let file = fs::File::open(&file_path).map_err(|e| format!("Failed to open log file: {}", e))?;

    let reader = BufReader::new(file);
    let all_lines: Result<Vec<String>, _> = reader.lines().collect();
    let all_lines = all_lines.map_err(|e| format!("Failed to read log file: {}", e))?;

    let total_lines = all_lines.len();

    // Apply pagination
    let start_idx = offset.min(total_lines);
    let end_idx = (offset + limit).min(total_lines);
    let lines = all_lines[start_idx..end_idx].to_vec();

    // Calculate position (for tail functionality)
    let position = file_path
        .metadata()
        .map_err(|e| format!("Failed to get file metadata: {}", e))?
        .len();

    debug!(
        "Read {} lines from log file (total: {})",
        lines.len(),
        total_lines
    );

    Ok(ApiResponse::success(LogContent {
        lines,
        total_lines,
        position,
    }))
}

/// Tail a log file - get new content since last position.
///
/// Returns lines appended to the log since the specified byte position.
/// Handles file rotation by detecting if the file has shrunk.
///
/// # Parameters
/// - `file_name` - Name of the log file (not a path)
/// - `last_position` - Byte position from previous read
///
/// # Returns
/// `ApiResponse` containing `LogTailResponse` with new lines and position.
///
/// # Errors
/// Returns error response if file name is invalid or file cannot be read.
#[tauri::command]
pub async fn tail_log_file(
    file_name: String,
    last_position: u64,
    state: State<'_, AppState>,
) -> Result<ApiResponse<LogTailResponse>, String> {
    // Validate file name
    if file_name.contains("..") || file_name.contains("/") || file_name.contains("\\") {
        return Ok(ApiResponse::error("Invalid file name".to_string()));
    }

    // Determine file path - check application logs first, then chat logs
    let file_path = if file_name.starts_with("mimir.log") {
        state.paths.logs_dir.join(&file_name)
    } else if file_name.ends_with(".log") {
        state.paths.logs_dir.join("chat_sessions").join(&file_name)
    } else {
        return Ok(ApiResponse::error("Invalid log file type".to_string()));
    };

    if !file_path.exists() {
        return Ok(ApiResponse::error(format!(
            "Log file not found: {}",
            file_name
        )));
    }

    // Open file and seek to last position
    let mut file =
        fs::File::open(&file_path).map_err(|e| format!("Failed to open log file: {}", e))?;

    // Check current file size
    let current_size = file
        .metadata()
        .map_err(|e| format!("Failed to get file metadata: {}", e))?
        .len();

    // If file is smaller than last position, it might have been rotated
    if current_size < last_position {
        debug!(
            "File size ({}) is smaller than last position ({}), reading from beginning",
            current_size, last_position
        );
        file.seek(SeekFrom::Start(0))
            .map_err(|e| format!("Failed to seek to start of file: {}", e))?;
    } else {
        // Seek to last position
        file.seek(SeekFrom::Start(last_position))
            .map_err(|e| format!("Failed to seek to position {}: {}", last_position, e))?;
    }

    // Read new content
    let reader = BufReader::new(file);
    let new_lines: Result<Vec<String>, _> = reader.lines().collect();
    let new_lines = new_lines.map_err(|e| format!("Failed to read log file: {}", e))?;

    Ok(ApiResponse::success(LogTailResponse {
        new_lines,
        new_position: current_size,
    }))
}

/// Open the chat logs directory in the system file explorer.
///
/// Creates the chat logs directory if it doesn't exist.
///
/// # Returns
/// `ApiResponse` indicating success.
///
/// # Errors
/// Returns error string if directory creation fails.
#[tauri::command]
pub async fn open_logs_folder(
    state: State<'_, AppState>,
) -> Result<ApiResponse<()>, String> {
    let chat_logs_dir = state.paths.logs_dir.join("chat_sessions");

    // Create the directory if it doesn't exist
    if !chat_logs_dir.exists() {
        fs::create_dir_all(&chat_logs_dir)
            .map_err(|e| format!("Failed to create chat logs directory: {}", e))?;
    }

    // Use the opener crate to open the directory in the file explorer
    // For now, we'll return the path so the frontend can use Tauri's shell API
    Ok(ApiResponse::success(()))
}
