//! Log file management commands

use crate::{
    types::ApiResponse,
    APP_PATHS,
};
use std::fs;
use std::path::Path;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use tracing::{error, info, debug};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug)]
pub struct LogFileInfo {
    pub name: String,
    pub full_path: String,
    pub size: u64,
    pub modified: String,
    pub is_current: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogFilesResponse {
    pub application_logs: Vec<LogFileInfo>,
    pub chat_logs: Vec<LogFileInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogContent {
    pub lines: Vec<String>,
    pub total_lines: usize,
    pub position: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogTailResponse {
    pub new_lines: Vec<String>,
    pub new_position: u64,
}

/// List all available log files in both application and chat directories
#[tauri::command]
pub async fn list_log_files() -> Result<ApiResponse<LogFilesResponse>, String> {
    info!("Listing log files");
    
    // Get app paths
    let app_paths = APP_PATHS.get()
        .ok_or_else(|| "App paths not initialized".to_string())?;
    
    let logs_dir = &app_paths.logs_dir;
    let chat_logs_dir = logs_dir.join("chat_sessions");
    
    let mut application_logs = Vec::new();
    let mut chat_logs = Vec::new();
    
    // Read application log files
    if logs_dir.exists() {
        let entries = fs::read_dir(logs_dir)
            .map_err(|e| format!("Failed to read logs directory: {}", e))?;
        
        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let path = entry.path();
            
            if path.is_file() {
                let filename = path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string();
                
                // Only include mimir.log files
                if filename.starts_with("mimir.log") {
                    let metadata = entry.metadata()
                        .map_err(|e| format!("Failed to read file metadata: {}", e))?;
                    
                    let size = metadata.len();
                    let modified = metadata.modified()
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
                let filename = path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string();
                
                // Only include .log files
                if filename.ends_with(".log") {
                    let metadata = entry.metadata()
                        .map_err(|e| format!("Failed to read file metadata: {}", e))?;
                    
                    let size = metadata.len();
                    let modified = metadata.modified()
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
    
    info!("Found {} application log files and {} chat log files", application_logs.len(), chat_logs.len());
    
    let response = LogFilesResponse {
        application_logs,
        chat_logs,
    };
    
    Ok(ApiResponse::success(response))
}

/// Read content from a log file with pagination
#[tauri::command]
pub async fn read_log_file(
    file_name: String,
    offset: usize,
    limit: usize,
) -> Result<ApiResponse<LogContent>, String> {
    debug!("Reading log file: {} (offset: {}, limit: {})", file_name, offset, limit);
    
    // Validate file name to prevent directory traversal
    if file_name.contains("..") || file_name.contains("/") || file_name.contains("\\") {
        return Ok(ApiResponse::error("Invalid file name".to_string()));
    }
    
    // Get app paths
    let app_paths = APP_PATHS.get()
        .ok_or_else(|| "App paths not initialized".to_string())?;
    
    // Determine file path - check application logs first, then chat logs
    let file_path = if file_name.starts_with("mimir.log") {
        app_paths.logs_dir.join(&file_name)
    } else if file_name.ends_with(".log") {
        app_paths.logs_dir.join("chat_sessions").join(&file_name)
    } else {
        return Ok(ApiResponse::error("Invalid log file type".to_string()));
    };
    
    if !file_path.exists() {
        return Ok(ApiResponse::error(format!("Log file not found: {}", file_name)));
    }
    
    // Read file content
    let file = fs::File::open(&file_path)
        .map_err(|e| format!("Failed to open log file: {}", e))?;
    
    let reader = BufReader::new(file);
    let all_lines: Result<Vec<String>, _> = reader.lines().collect();
    let all_lines = all_lines.map_err(|e| format!("Failed to read log file: {}", e))?;
    
    let total_lines = all_lines.len();
    
    // Apply pagination
    let start_idx = offset.min(total_lines);
    let end_idx = (offset + limit).min(total_lines);
    let lines = all_lines[start_idx..end_idx].to_vec();
    
    // Calculate position (for tail functionality)
    let position = file_path.metadata()
        .map_err(|e| format!("Failed to get file metadata: {}", e))?
        .len();
    
    debug!("Read {} lines from log file (total: {})", lines.len(), total_lines);
    
    Ok(ApiResponse::success(LogContent {
        lines,
        total_lines,
        position,
    }))
}

/// Tail a log file - get new content since last position
#[tauri::command]
pub async fn tail_log_file(
    file_name: String,
    last_position: u64,
) -> Result<ApiResponse<LogTailResponse>, String> {
    // Validate file name
    if file_name.contains("..") || file_name.contains("/") || file_name.contains("\\") {
        return Ok(ApiResponse::error("Invalid file name".to_string()));
    }
    
    // Get app paths
    let app_paths = APP_PATHS.get()
        .ok_or_else(|| "App paths not initialized".to_string())?;
    
    // Determine file path - check application logs first, then chat logs
    let file_path = if file_name.starts_with("mimir.log") {
        app_paths.logs_dir.join(&file_name)
    } else if file_name.ends_with(".log") {
        app_paths.logs_dir.join("chat_sessions").join(&file_name)
    } else {
        return Ok(ApiResponse::error("Invalid log file type".to_string()));
    };
    
    if !file_path.exists() {
        return Ok(ApiResponse::error(format!("Log file not found: {}", file_name)));
    }
    
    // Open file and seek to last position
    let mut file = fs::File::open(&file_path)
        .map_err(|e| format!("Failed to open log file: {}", e))?;
    
    // Check current file size
    let current_size = file.metadata()
        .map_err(|e| format!("Failed to get file metadata: {}", e))?
        .len();
    
    // If file is smaller than last position, it might have been rotated
    if current_size < last_position {
        debug!("File size ({}) is smaller than last position ({}), reading from beginning", current_size, last_position);
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