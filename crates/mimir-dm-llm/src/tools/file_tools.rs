//! Generic file-based tools for LLM
//!
//! These tools provide basic file operations without any domain knowledge,
//! making them reusable across different applications.

use async_trait::async_trait;
use crate::{ToolTrait, FileToolsConfig};
use crate::traits::ActionDescription;
use serde_json::{json, Value};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use similar::{ChangeTag, TextDiff};
use tracing::debug;

/// Path validator for security - ensures file access is restricted to allowed directories
#[derive(Debug, Clone)]
pub struct PathValidator {
    allowed_prefixes: Vec<PathBuf>,
    forbidden_patterns: Vec<String>,
}

impl PathValidator {
    /// Create a new path validator with allowed directory prefixes
    pub fn new(allowed_prefixes: Vec<PathBuf>) -> Self {
        Self {
            allowed_prefixes,
            forbidden_patterns: vec![
                "..".to_string(),
                "/etc".to_string(),
                "/var".to_string(),
                "/usr".to_string(),
                "/bin".to_string(),
                "/sys".to_string(),
                "/proc".to_string(),
            ],
        }
    }
    
    /// Validate that a path is safe to access
    pub fn validate_path(&self, path: &str) -> Result<PathBuf, Box<dyn Error + Send + Sync>> {
        let path = Path::new(path);
        
        // Check for forbidden patterns
        let path_str = path.to_string_lossy();
        for pattern in &self.forbidden_patterns {
            if path_str.contains(pattern) {
                return Err(format!("Path contains forbidden pattern: {}", pattern).into());
            }
        }
        
        // Canonicalize the path to resolve any symbolic links or relative components
        let canonical_path = path.canonicalize()
            .or_else(|_| {
                // If canonicalize fails (file doesn't exist), try with parent directory
                if let Some(parent) = path.parent() {
                    if parent.exists() {
                        let canonical_parent = parent.canonicalize()?;
                        if let Some(filename) = path.file_name() {
                            Ok(canonical_parent.join(filename))
                        } else {
                            Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid path"))
                        }
                    } else {
                        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Parent directory not found"))
                    }
                } else {
                    Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid path"))
                }
            })
            .map_err(|e| format!("Failed to canonicalize path: {}", e))?;
        
        // Check that the path is within allowed prefixes
        let is_allowed = self.allowed_prefixes.iter().any(|prefix| {
            canonical_path.starts_with(prefix)
        });
        
        if !is_allowed {
            return Err(format!("Path not within allowed directories: {}", canonical_path.display()).into());
        }
        
        Ok(canonical_path)
    }
}

/// Generate a diff display between current and new content
fn generate_diff_display(current_content: &str, new_content: &str) -> String {
    let diff = TextDiff::from_lines(current_content, new_content);
    let mut diff_output = String::new();
    let mut line_count = 0;
    const MAX_LINES: usize = 50;
    
    for change in diff.iter_all_changes() {
        if line_count >= MAX_LINES {
            diff_output.push_str(&format!("\n... ({} more lines changed) ...", 
                diff.iter_all_changes().count() - line_count));
            break;
        }
        
        let sign = match change.tag() {
            ChangeTag::Delete => "- ",
            ChangeTag::Insert => "+ ",
            ChangeTag::Equal => "  ",
        };
        
        // Only show changed lines and minimal context
        match change.tag() {
            ChangeTag::Delete | ChangeTag::Insert => {
                diff_output.push_str(&format!("{}{}", sign, change));
                line_count += 1;
            },
            ChangeTag::Equal => {
                // Show context lines (unchanged lines around changes)
                let line = change.to_string();
                if !line.trim().is_empty() && line.len() < 100 {
                    diff_output.push_str(&format!("{}{}", sign, change));
                    line_count += 1;
                }
            }
        }
    }
    
    // Check if there are actually any changes (insertions/deletions)
    let has_changes = diff.iter_all_changes().any(|change| {
        matches!(change.tag(), ChangeTag::Delete | ChangeTag::Insert)
    });
    
    if !has_changes {
        "No changes detected".to_string()
    } else if diff_output.trim().is_empty() {
        "No changes detected".to_string()
    } else {
        format!("```diff\n{}\n```", diff_output)
    }
}

/// Tool for reading file contents
pub struct ReadFileTool {
    config: Arc<FileToolsConfig>,
}

impl ReadFileTool {
    pub fn new(config: Arc<FileToolsConfig>) -> Self {
        Self { config }
    }
}

#[async_trait]
impl ToolTrait for ReadFileTool {
    fn name(&self) -> &str {
        "read_file"
    }
    
    fn description(&self) -> &str {
        "Read the contents of a file by its path. The file path must be within allowed directories for security."
    }
    
    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "file_path": {
                    "type": "string",
                    "description": "Absolute path to the file to read"
                }
            },
            "required": ["file_path"]
        })
    }
    
    async fn execute(&self, arguments: Value) -> Result<String, Box<dyn Error + Send + Sync>> {
        let file_path = arguments
            .get("file_path")
            .and_then(|v| v.as_str())
            .ok_or("Missing 'file_path' parameter")?;
        
        // Validate the path using PathValidator
        let path_validator = PathValidator::new(self.config.allowed_directories.clone());
        let validated_path = path_validator.validate_path(file_path)?;
        
        // Check that it's actually a file
        if !validated_path.is_file() {
            return Err(format!("Path is not a file: {}", validated_path.display()).into());
        }
        
        // Read the file
        let content = fs::read_to_string(&validated_path)
            .map_err(|e| format!("Failed to read file: {}", e))?;
        
        debug!("Read file: {}", validated_path.display());
        Ok(content)
    }
}

/// Tool for writing file contents
pub struct WriteFileTool {
    config: Arc<FileToolsConfig>,
}

impl WriteFileTool {
    pub fn new(config: Arc<FileToolsConfig>) -> Self {
        Self { config }
    }
}

#[async_trait]
impl ToolTrait for WriteFileTool {
    fn name(&self) -> &str {
        "write_file"
    }
    
    fn description(&self) -> &str {
        "Write content to a file. Requires user confirmation due to potential data loss."
    }
    
    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "file_path": {
                    "type": "string",
                    "description": "Absolute path to the file to write"
                },
                "content": {
                    "type": "string",
                    "description": "Content to write to the file"
                }
            },
            "required": ["file_path", "content"]
        })
    }
    
    fn requires_confirmation(&self) -> bool {
        true // Always require confirmation for file writes
    }
    
    fn describe_action(&self, arguments: &Value) -> Option<ActionDescription> {
        let file_path = arguments.get("file_path")?.as_str()?;
        let new_content = arguments.get("content")?.as_str()?;
        
        // Try to validate path and read current content for diff
        let path_validator = PathValidator::new(self.config.allowed_directories.clone());
        let changes_display = match path_validator.validate_path(file_path) {
            Ok(validated_path) => {
                if validated_path.exists() && validated_path.is_file() {
                    match fs::read_to_string(&validated_path) {
                        Ok(current_content) => {
                            generate_diff_display(&current_content, new_content)
                        },
                        Err(_) => {
                            format!("New content ({} characters):\n{}", 
                                new_content.len(),
                                if new_content.len() > 500 {
                                    format!("{}...\n\n[Content truncated for display]", &new_content[..500])
                                } else {
                                    new_content.to_string()
                                }
                            )
                        }
                    }
                } else {
                    format!("Creating new file with {} characters", new_content.len())
                }
            },
            Err(_) => {
                "Unable to validate file path for preview".to_string()
            }
        };
        
        Some(ActionDescription {
            title: "Write File".to_string(),
            description: format!("This will write content to file: {}", file_path),
            changes: vec![
                format!("File: {}", file_path),
                changes_display,
            ],
        })
    }
    
    async fn execute(&self, arguments: Value) -> Result<String, Box<dyn Error + Send + Sync>> {
        let file_path = arguments
            .get("file_path")
            .and_then(|v| v.as_str())
            .ok_or("Missing 'file_path' parameter")?;
        
        let content = arguments
            .get("content")
            .and_then(|v| v.as_str())
            .ok_or("Missing 'content' parameter")?;
        
        
        // Validate the path
        let path_validator = PathValidator::new(self.config.allowed_directories.clone());
        let validated_path = path_validator.validate_path(file_path)?;
        
        // Create parent directory if it doesn't exist
        if let Some(parent) = validated_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create parent directory: {}", e))?;
            }
        }
        
        
        // Write the file
        fs::write(&validated_path, content)
            .map_err(|e| format!("Failed to write file: {}", e))?;
        
        // Return success message
        let result = json!({
            "status": "success",
            "action": "file_written",
            "details": {
                "file_path": validated_path.to_string_lossy(),
                "content_length": content.len()
            },
            "message": format!("File successfully written to: {}", validated_path.display())
        });
        
        debug!("Wrote file: {}", validated_path.display());
        Ok(serde_json::to_string_pretty(&result).unwrap())
    }
}

/// Tool for listing files in a directory
pub struct ListFilesTool {
    config: Arc<FileToolsConfig>,
}

impl ListFilesTool {
    pub fn new(config: Arc<FileToolsConfig>) -> Self {
        Self { config }
    }
}

#[async_trait]
impl ToolTrait for ListFilesTool {
    fn name(&self) -> &str {
        "list_files"
    }
    
    fn description(&self) -> &str {
        "List files in a directory, optionally filtered by a pattern. Returns file names and basic metadata."
    }
    
    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "directory_path": {
                    "type": "string",
                    "description": "Absolute path to the directory to list"
                },
                "pattern": {
                    "type": "string",
                    "description": "Optional glob pattern to filter files (e.g., '*.md', '*.txt')"
                },
                "recursive": {
                    "type": "boolean",
                    "description": "Whether to list files recursively (default: false)"
                }
            },
            "required": ["directory_path"]
        })
    }
    
    async fn execute(&self, arguments: Value) -> Result<String, Box<dyn Error + Send + Sync>> {
        let directory_path = arguments
            .get("directory_path")
            .or_else(|| arguments.get("path"))  // Accept both parameter names for compatibility
            .and_then(|v| v.as_str())
            .ok_or("Missing 'directory_path' parameter")?;
        
        let pattern = arguments
            .get("pattern")
            .and_then(|v| v.as_str());
        
        let recursive = arguments
            .get("recursive")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        
        // Validate the path
        let path_validator = PathValidator::new(self.config.allowed_directories.clone());
        let validated_path = path_validator.validate_path(directory_path)?;
        
        // Check that it's actually a directory
        if !validated_path.is_dir() {
            return Err(format!("Path is not a directory: {}", validated_path.display()).into());
        }
        
        // List files
        let mut files = Vec::new();
        
        if recursive {
            // Recursive listing using walkdir
            use std::path::Path;
            fn collect_files_recursive(dir: &Path, pattern: Option<&str>) -> Result<Vec<(PathBuf, std::fs::Metadata)>, Box<dyn Error + Send + Sync>> {
                let mut files = Vec::new();
                
                for entry in fs::read_dir(dir)? {
                    let entry = entry?;
                    let path = entry.path();
                    
                    if path.is_dir() {
                        // Recurse into subdirectory
                        files.extend(collect_files_recursive(&path, pattern)?);
                    } else if path.is_file() {
                        // Check pattern if provided
                        if let Some(pattern) = pattern {
                            if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                                if glob_match(pattern, filename) {
                                    files.push((path, entry.metadata()?));
                                }
                            }
                        } else {
                            files.push((path, entry.metadata()?));
                        }
                    }
                }
                
                Ok(files)
            }
            
            files = collect_files_recursive(&validated_path, pattern)?;
        } else {
            // Non-recursive listing
            for entry in fs::read_dir(&validated_path)
                .map_err(|e| format!("Failed to read directory: {}", e))? {
                let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
                let path = entry.path();
                
                if path.is_file() {
                    // Check pattern if provided
                    if let Some(pattern) = pattern {
                        if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                            if glob_match(pattern, filename) {
                                let metadata = entry.metadata()
                                    .map_err(|e| format!("Failed to read file metadata: {}", e))?;
                                files.push((path, metadata));
                            }
                        }
                    } else {
                        let metadata = entry.metadata()
                            .map_err(|e| format!("Failed to read file metadata: {}", e))?;
                        files.push((path, metadata));
                    }
                }
            }
        }
        
        // Sort files by name
        files.sort_by(|a, b| a.0.cmp(&b.0));
        
        if files.is_empty() {
            return Ok("No files found matching the criteria.".to_string());
        }
        
        // Format the file list
        let file_count = files.len();
        let mut output = format!("Found {} file(s) in {}:\n\n", file_count, validated_path.display());
        
        for (path, metadata) in &files {
            let relative_path = path.strip_prefix(&validated_path)
                .unwrap_or(&path);
            
            let size = metadata.len();
            let modified = metadata.modified()
                .ok()
                .and_then(|time| {
                    use std::time::SystemTime;
                    time.duration_since(SystemTime::UNIX_EPOCH).ok()
                })
                .map(|duration| {
                    use chrono::{DateTime, Utc};
                    let datetime = DateTime::<Utc>::from_timestamp(duration.as_secs() as i64, 0)
                        .unwrap_or_else(|| Utc::now());
                    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
                })
                .unwrap_or_else(|| "Unknown".to_string());
            
            output.push_str(&format!(
                "- {} ({} bytes, modified: {})\n",
                relative_path.display(),
                size,
                modified
            ));
        }
        
        debug!("Listed {} files in: {}", file_count, validated_path.display());
        Ok(output)
    }
}

/// Simple glob pattern matching (supports * and ? wildcards)
fn glob_match(pattern: &str, text: &str) -> bool {
    // Convert glob pattern to regex
    let regex_pattern = pattern
        .replace(".", "\\.")
        .replace("*", ".*")
        .replace("?", ".");
    
    if let Ok(regex) = regex::Regex::new(&format!("^{}$", regex_pattern)) {
        regex.is_match(text)
    } else {
        // Fallback to simple contains check if regex compilation fails
        text.contains(&pattern.replace("*", "").replace("?", ""))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_path_validator_allows_valid_paths() {
        let temp_dir = TempDir::new().unwrap();
        let validator = PathValidator::new(vec![temp_dir.path().to_path_buf()]);
        
        let test_file = temp_dir.path().join("test.txt");
        std::fs::write(&test_file, "test content").unwrap();
        
        let result = validator.validate_path(test_file.to_str().unwrap());
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_path_validator_rejects_forbidden_paths() {
        let temp_dir = TempDir::new().unwrap();
        let validator = PathValidator::new(vec![temp_dir.path().to_path_buf()]);
        
        let result = validator.validate_path("/etc/passwd");
        assert!(result.is_err());
        
        let result = validator.validate_path("../../../etc/passwd");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_generate_diff_display_simple_change() {
        let current = "# Test File\n\nOriginal content.\nStay the same.";
        let new_content = "# Test File\n\nNew content.\nStay the same.";
        
        let result = generate_diff_display(current, new_content);
        
        assert!(result.contains("```diff"));
        assert!(result.contains("- Original content."));
        assert!(result.contains("+ New content."));
        assert!(result.contains("  Stay the same."));
    }
    
    #[test]
    fn test_glob_match() {
        assert!(glob_match("*.txt", "file.txt"));
        assert!(glob_match("test.*", "test.md"));
        assert!(glob_match("file?.txt", "file1.txt"));
        assert!(!glob_match("*.txt", "file.md"));
    }
}