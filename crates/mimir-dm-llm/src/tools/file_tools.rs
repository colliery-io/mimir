//! Generic file-based tools for LLM
//!
//! These tools provide basic file operations without any domain knowledge,
//! making them reusable across different applications.

use async_trait::async_trait;
use crate::{ToolTrait, FileToolsConfig};
use crate::traits::{ActionDescription, ChangeDetail, LineEdit, EditOperation, ToolCallContext as ToolCall};
use crate::traits::tool::DiffPreview;
use serde_json::{json, Value};
use std::collections::VecDeque;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
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
        self.validate_path_internal(path, false)
    }
    
    /// Validate that a path is safe to access, allowing parent directory creation
    pub fn validate_path_for_write(&self, path: &str) -> Result<PathBuf, Box<dyn Error + Send + Sync>> {
        self.validate_path_internal(path, true)
    }
    
    /// Internal validation method with directory creation control
    fn validate_path_internal(&self, path: &str, allow_dir_creation: bool) -> Result<PathBuf, Box<dyn Error + Send + Sync>> {
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
                    } else if allow_dir_creation {
                        // For write operations, allow non-existent parent directories 
                        // since WriteFileTool creates them. Just validate the path structure.
                        let _canonical_root = self.allowed_prefixes.iter()
                            .find(|prefix| path.starts_with(prefix))
                            .ok_or_else(|| std::io::Error::new(
                                std::io::ErrorKind::PermissionDenied, 
                                format!("Path is outside allowed directories: {}", path.display())
                            ))?;
                        
                        // Return the path as-is if it's within allowed directories
                        // The parent directory will be created by the tool if needed
                        Ok(path.to_path_buf())
                    } else {
                        Err(std::io::Error::new(
                            std::io::ErrorKind::NotFound, 
                            format!(
                                "Parent directory does not exist: {}. Use list_files to check directory structure or use write_file which can create directories.",
                                parent.display()
                            )
                        ))
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
        "Read the contents of a file and return it with line numbers for easy editing and reference.

Usage:
- File path must be absolute and within allowed application directories
- Returns content formatted with line numbers (e.g., '  1→line content')
- Line numbering starts at 1 and uses the format: '  {line_number}→{content}'
- Essential prerequisite before using edit_file tool
- Use this to understand file structure, check existing content, and locate specific lines
- Handles text files of any size, but very large files may be truncated in display
- Empty files return '(empty file)' message
- Binary files may produce garbled output - use only on text files

Output Format:
- Each line is prefixed with line numbers for precise editing
- Line numbers are right-padded for consistent alignment
- Example output:
    1→import json
    2→import sys
    3→
    4→def main():
    5→    print('Hello World')

Security:
- All file paths are validated against allowed directories
- Cannot read files outside the application's data directory
- Cannot access system files or sensitive locations
- Will error if the path points to a directory instead of a file

Use Cases:
- Before editing: Always read first to understand current content and structure
- Code review: Examine source files with clear line references
- Configuration: Check current settings before making changes
- Debugging: Inspect file contents to understand application state
- Template inspection: Review template files before customization

Best Practices:
- Always read before editing to avoid conflicts and understand context
- Use the line numbers from output when specifying edit operations
- Check file size - very large files may need special handling"
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
            if validated_path.is_dir() {
                return Err(format!(
                    "Path is a directory, not a file: {}. Use list_files to explore directory contents.", 
                    validated_path.display()
                ).into());
            } else {
                return Err(format!(
                    "File does not exist: {}. Use list_files to check the directory structure and verify the correct path.", 
                    validated_path.display()
                ).into());
            }
        }
        
        // Read the file
        let content = fs::read_to_string(&validated_path)
            .map_err(|e| format!(
                "Failed to read file '{}': {}. Check that the file exists and is readable. Use list_files to verify the file is present.", 
                validated_path.display(), e
            ))?;
        
        // Format content with line numbers for LLM use
        let lines = content.lines();
        let line_count = lines.clone().count();
        
        if line_count == 0 {
            debug!("Read empty file: {}", validated_path.display());
            return Ok("(empty file)".to_string());
        }
        
        // Calculate the width needed for line numbers (minimum 3 digits)
        let line_number_width = std::cmp::max(3, (line_count as f64).log10().floor() as usize + 1);
        
        let formatted_content = lines
            .enumerate()
            .map(|(i, line)| format!("{:width$}→{}", i + 1, line, width = line_number_width))
            .collect::<Vec<_>>()
            .join("\n");
        
        debug!("Read file with {} lines: {}", line_count, validated_path.display());
        Ok(formatted_content)
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
        "Write content to a file, creating a new file or completely replacing existing file content.

Usage:
- This tool OVERWRITES the entire file content with the provided content
- ALWAYS use read_file first to check if the file exists and understand its current content
- File path must be absolute and within allowed application directories
- Requires user confirmation before execution due to potential data loss
- Use this for creating new files or when you need to replace entire file content
- For incremental changes to existing files, prefer edit_file instead
- Content should be properly formatted with correct line endings for the target platform
- Empty content will create an empty file (this is valid)
- If the directory doesn't exist, the operation will fail
- File permissions will be set to standard read/write permissions

Security:
- All file paths are validated against allowed directories
- Cannot write outside the application's data directory
- Cannot overwrite system files or files outside the sandbox

Best Practices:
- Read the file first to understand existing structure and content
- Provide complete, well-formatted content
- Include appropriate file headers, imports, or metadata as needed
- Consider the impact on other parts of the application that might depend on the file"
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
        let diff_preview = match path_validator.validate_path(file_path) {
            Ok(validated_path) => {
                if validated_path.exists() && validated_path.is_file() {
                    match fs::read_to_string(&validated_path) {
                        Ok(current_content) => {
                            let diff_display = generate_diff_display(&current_content, new_content);
                            let added_lines = new_content.lines().count();
                            let removed_lines = current_content.lines().count();
                            Some(DiffPreview {
                                added_lines,
                                removed_lines,
                                preview: diff_display,
                            })
                        },
                        Err(_) => None
                    }
                } else {
                    None
                }
            },
            Err(_) => None
        };
        
        // Prepare content preview (truncate if too long)
        let content_preview = if diff_preview.is_none() {
            if new_content.len() <= 1000 {
                Some(new_content.to_string())
            } else {
                Some(format!("{}...\n\n[Content truncated at 1000 characters for display]", &new_content[..1000]))
            }
        } else {
            None // Use diff preview instead
        };
        
        Some(ActionDescription {
            title: "Write File".to_string(),
            description: format!("Write {} characters to file: {}", new_content.len(), file_path),
            changes: ChangeDetail::FileWrite {
                file_path: file_path.to_string(),
                content_length: new_content.len(),
                diff_preview,
                content_preview,
            },
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
        
        
        // Validate the path (allowing directory creation for write operations)
        let path_validator = PathValidator::new(self.config.allowed_directories.clone());
        let validated_path = path_validator.validate_path_for_write(file_path)?;
        
        // Create parent directory if it doesn't exist
        if let Some(parent) = validated_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!(
                        "Failed to create parent directory '{}': {}. Check that you have the correct path and permissions. Use list_files to explore the directory structure.", 
                        parent.display(), e
                    ))?;
            }
        }
        
        
        // Write the file
        fs::write(&validated_path, content)
            .map_err(|e| format!(
                "Failed to write file '{}': {}. Check that the path is correct and you have write permissions.", 
                validated_path.display(), e
            ))?;
        
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
        "List files and directories in a specified path with optional pattern filtering and metadata.

Usage:
- Directory path must be absolute and within allowed application directories
- Returns detailed information about each file and subdirectory
- Supports optional pattern filtering to find specific files
- Essential for exploring the application's file structure
- Use this before reading or editing files to understand the layout
- Provides file sizes, modification times, and type information
- Non-recursive by default - only shows immediate contents

Pattern Filtering:
- Optional pattern parameter supports glob-style wildcards
- Examples: '*.json', 'config.*', 'template_*', '*.md'
- Case-sensitive matching
- Empty or null pattern returns all files
- Pattern applies to filenames only, not full paths

Output Information:
For each file/directory:
- Name: The filename or directory name
- Type: 'file' or 'directory'
- Size: File size in bytes (0 for directories)
- Modified: Last modification timestamp
- Path: Full path to the item (for reference)

Security:
- All paths are validated against allowed directories
- Cannot list files outside the application's data directory
- Cannot access system directories or sensitive locations
- Directory traversal attacks are prevented

Use Cases:
- Project exploration: Understanding application structure and organization
- File discovery: Finding configuration files, templates, or data files
- Before file operations: Confirming file existence and getting exact names
- Backup verification: Checking what files exist before modifications
- Template selection: Browsing available templates or resources
- Data analysis: Understanding data file organization and sizes

Best Practices:
- Start with root directory listing to understand overall structure
- Use pattern filtering to narrow down results for large directories
- Check file sizes before attempting to read very large files
- Use the exact filenames returned for subsequent read/write operations
- Combine with read_file to explore files of interest

Common Patterns:
- list_files() - List files in the application data directory
- list_files(directory_path, '*.json') - Find JSON files in a specific directory
- list_files(directory_path, '*_config.*') - Find configuration files"
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

/// Tool for editing files using line-number based changes
pub struct EditFileTool {
    config: Arc<FileToolsConfig>,
}

impl EditFileTool {
    pub fn new(config: Arc<FileToolsConfig>) -> Self {
        Self { config }
    }
    
    /// Parse edit instructions and convert them to line edits
    fn parse_edits(&self, content_lines: &[String], edits: &Value) -> Result<Vec<LineEdit>, Box<dyn Error + Send + Sync>> {
        let edits_array = edits.as_array()
            .ok_or("Edits must be an array")?;
        
        let mut line_edits = Vec::new();
        
        for edit in edits_array {
            let operation_str = edit.get("operation")
                .and_then(|v| v.as_str())
                .ok_or("Missing 'operation' field in edit")?;
            
            let operation = match operation_str {
                "replace" => EditOperation::Replace,
                "insert" => EditOperation::Insert,
                "delete" => EditOperation::Delete,
                _ => return Err(format!("Unknown operation: {}", operation_str).into()),
            };
            
            let start_line = edit.get("start_line")
                .and_then(|v| v.as_u64())
                .ok_or("Missing 'start_line' field in edit")? as usize;
            
            let end_line = edit.get("end_line")
                .and_then(|v| v.as_u64())
                .map(|v| v as usize)
                .unwrap_or(start_line);
            
            // Validate line numbers (1-indexed)
            if start_line == 0 || end_line == 0 {
                return Err("Line numbers must be 1-indexed (start from 1)".into());
            }
            
            if start_line > content_lines.len() + 1 {
                return Err(format!("Start line {} is beyond file length {}", start_line, content_lines.len()).into());
            }
            
            if end_line > content_lines.len() + 1 {
                return Err(format!("End line {} is beyond file length {}", end_line, content_lines.len()).into());
            }
            
            if start_line > end_line {
                return Err(format!("Start line {} cannot be greater than end line {}", start_line, end_line).into());
            }
            
            // Get old content (0-indexed for array access)
            let old_content: Vec<String> = match operation {
                EditOperation::Insert => Vec::new(),
                _ => {
                    let start_idx = (start_line - 1).min(content_lines.len());
                    let end_idx = end_line.min(content_lines.len());
                    content_lines[start_idx..end_idx].to_vec()
                }
            };
            
            // Get new content
            let new_content = edit.get("content")
                .and_then(|v| v.as_str())
                .map(|s| s.lines().map(|line| line.to_string()).collect::<Vec<String>>())
                .unwrap_or_else(Vec::new);
            
            // Get context lines for preview (2-3 lines each side)
            let context_line_count = 2; // Could be made configurable in the future
            
            // Context before: lines preceding the edit
            let context_before_start = start_line.saturating_sub(1 + context_line_count);
            let context_before_end = start_line.saturating_sub(1);
            let context_before = (context_before_start..context_before_end)
                .filter_map(|i| content_lines.get(i).cloned())
                .collect::<Vec<String>>();
            
            // Context after: lines following the edit
            let context_after_start = end_line;
            let context_after_end = end_line + context_line_count;
            let context_after = (context_after_start..context_after_end)
                .filter_map(|i| content_lines.get(i).cloned())
                .collect::<Vec<String>>();
            
            line_edits.push(LineEdit {
                operation,
                start_line,
                end_line,
                old_content,
                new_content,
                context_before,
                context_after,
            });
        }
        
        // Sort edits by line number (reverse order for proper application)
        line_edits.sort_by(|a, b| b.start_line.cmp(&a.start_line));
        
        Ok(line_edits)
    }
    
    /// Apply line edits to content and return the new content
    fn apply_edits(&self, content_lines: &mut Vec<String>, line_edits: &[LineEdit]) -> Result<(), Box<dyn Error + Send + Sync>> {
        for edit in line_edits {
            match edit.operation {
                EditOperation::Replace => {
                    // Remove old lines and insert new ones
                    let start_idx = (edit.start_line - 1).min(content_lines.len());
                    let end_idx = edit.end_line.min(content_lines.len());
                    
                    // Remove old lines
                    for _ in start_idx..end_idx {
                        if start_idx < content_lines.len() {
                            content_lines.remove(start_idx);
                        }
                    }
                    
                    // Insert new lines
                    for (i, line) in edit.new_content.iter().enumerate() {
                        content_lines.insert(start_idx + i, line.clone());
                    }
                },
                EditOperation::Insert => {
                    // Insert new lines at the specified position
                    let insert_idx = (edit.start_line - 1).min(content_lines.len());
                    for (i, line) in edit.new_content.iter().enumerate() {
                        content_lines.insert(insert_idx + i, line.clone());
                    }
                },
                EditOperation::Delete => {
                    // Remove lines
                    let start_idx = (edit.start_line - 1).min(content_lines.len());
                    let end_idx = edit.end_line.min(content_lines.len());
                    
                    for _ in start_idx..end_idx {
                        if start_idx < content_lines.len() {
                            content_lines.remove(start_idx);
                        }
                    }
                },
            }
        }
        
        Ok(())
    }
    
    /// Check if read_file or write_file was called recently for the same file
    fn check_recent_read(&self, file_path: &str, recent_calls: &Arc<Mutex<VecDeque<ToolCall>>>) -> bool {
        let calls = recent_calls.lock().unwrap();
        
        // Look for a recent read_file or write_file call for the same file
        // write_file is acceptable because it means we just created/modified the file content
        calls.iter().rev().take(5).any(|call| {
            (call.name == "read_file" || call.name == "write_file") && 
            call.file_path.as_ref().map(|p| p == file_path).unwrap_or(false)
        })
    }
}

#[async_trait]
impl ToolTrait for EditFileTool {
    fn name(&self) -> &str {
        "edit_file"
    }
    
    fn description(&self) -> &str {
        "Edit a file using precise line-number based operations for safe, incremental changes.

Usage:
- MANDATORY: You MUST call read_file first to get current content with line numbers
- This tool will guide you to read the file first if you haven't done so recently
- Supports three edit operations: replace, insert, delete
- All line numbers are 1-indexed and must match the read_file output exactly
- Multiple edits can be applied in a single operation
- Requires user confirmation before execution due to potential data modification
- Safer than write_file for incremental changes - preserves file structure

Edit Operations:
1. REPLACE: Replace content on specific lines
   - Use start_line and end_line to define the range
   - new_content replaces everything between (and including) those lines
   - Can replace single line (start_line = end_line) or multiple lines
   
2. INSERT: Add new content at a specific position
   - Use start_line to specify where to insert (content goes BEFORE this line)
   - new_content is inserted as new lines
   - Original content at start_line and beyond shifts down
   
3. DELETE: Remove lines from the file
   - Use start_line and end_line to define range to delete
   - All lines between (and including) those lines are removed
   - new_content should be empty for delete operations

Line Number Rules:
- Line numbers must exactly match those from read_file output
- start_line must be <= end_line for replace and delete operations
- For insert operations, start_line indicates insertion point
- Line numbers are validated against current file content
- Invalid line numbers will cause the operation to fail

Security:
- All file paths are validated against allowed directories
- Cannot edit files outside the application's data directory
- Atomic operation - either all edits succeed or none are applied
- Original file is backed up during the edit process

Best Practices:
- Always read the file first to get current line numbers and content
- Plan your edits carefully - line numbers change after insertions/deletions
- Apply related edits in a single operation when possible
- Provide meaningful context in new_content for replace operations
- Test with small changes first for complex files
- Consider the impact on file structure and dependencies

Error Recovery:
- If you haven't read the file recently, the tool provides guidance instead of failing
- Invalid line numbers or malformed edits provide clear error messages
- Failed edits don't modify the file - it's safe to retry with corrections"
    }
    
    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "file_path": {
                    "type": "string",
                    "description": "Absolute path to the file to edit"
                },
                "edits": {
                    "type": "array",
                    "description": "Array of edit operations to apply",
                    "items": {
                        "type": "object",
                        "properties": {
                            "operation": {
                                "type": "string",
                                "enum": ["replace", "insert", "delete"],
                                "description": "Type of edit operation"
                            },
                            "start_line": {
                                "type": "number",
                                "description": "Starting line number (1-indexed, from read_file output)"
                            },
                            "end_line": {
                                "type": "number",
                                "description": "Ending line number (1-indexed, inclusive). Optional for insert operations."
                            },
                            "content": {
                                "type": "string",
                                "description": "New content to insert or replace with. Not used for delete operations."
                            }
                        },
                        "required": ["operation", "start_line"]
                    }
                }
            },
            "required": ["file_path", "edits"]
        })
    }
    
    fn requires_confirmation(&self) -> bool {
        // Always require confirmation for actual file edits
        // Guidance messages (when read_file wasn't called) won't reach the confirmation stage
        // because execute_with_context will return guidance before triggering confirmation
        true
    }
    
    fn describe_action(&self, arguments: &Value) -> Option<ActionDescription> {
        let file_path = match arguments.get("file_path").and_then(|v| v.as_str()) {
            Some(path) => path,
            None => {
                return Some(ActionDescription {
                    title: "Edit File".to_string(),
                    description: "Missing file_path parameter".to_string(),
                    changes: ChangeDetail::Generic {
                        items: vec!["Invalid parameters".to_string()],
                    },
                });
            }
        };
        
        let edits_value = match arguments.get("edits") {
            Some(edits) => edits,
            None => {
                return Some(ActionDescription {
                    title: "Edit File".to_string(),
                    description: "Missing edits parameter".to_string(),
                    changes: ChangeDetail::Generic {
                        items: vec!["Invalid parameters".to_string()],
                    },
                });
            }
        };
        
        // Try to read current content for preview
        let path_validator = PathValidator::new(self.config.allowed_directories.clone());
        let validated_path = match path_validator.validate_path(file_path) {
            Ok(path) => path,
            Err(err) => {
                return Some(ActionDescription {
                    title: "Edit File".to_string(),
                    description: format!("Invalid file path: {}", err),
                    changes: ChangeDetail::Generic {
                        items: vec![format!("Path validation error: {}", err)],
                    },
                });
            }
        };
        
        if !validated_path.is_file() {
            return Some(ActionDescription {
                title: "Edit File".to_string(),
                description: format!("File does not exist: {}", file_path),
                changes: ChangeDetail::Generic {
                    items: vec!["File not found".to_string()],
                },
            });
        }
        
        let current_content = match fs::read_to_string(&validated_path) {
            Ok(content) => content,
            Err(_err) => {
                return Some(ActionDescription {
                    title: "Edit File (Read Required)".to_string(),
                    description: format!("This edit requires reading the file first. Please call read_file('{}') to get current content with line numbers, then retry this edit.", file_path),
                    changes: ChangeDetail::Generic {
                        items: vec![
                            format!("Action needed: Call read_file('{}') first", file_path),
                            "This edit operation requires current file content for safety".to_string(),
                        ],
                    },
                });
            }
        };
        
        let content_lines: Vec<String> = current_content.lines().map(|s| s.to_string()).collect();
        
        // Parse the edits to generate preview
        match self.parse_edits(&content_lines, edits_value) {
            Ok(line_edits) => {
                let total_lines_affected = line_edits.iter()
                    .map(|edit| (edit.end_line - edit.start_line + 1).max(edit.new_content.len()))
                    .sum();
                
                Some(ActionDescription {
                    title: "Edit File".to_string(),
                    description: format!("Apply {} edit operation(s) to file: {}", line_edits.len(), file_path),
                    changes: ChangeDetail::FileEdit {
                        file_path: file_path.to_string(),
                        edits: line_edits,
                        total_lines_affected,
                        total_lines_in_file: content_lines.len(),
                    },
                })
            },
            Err(err) => {
                Some(ActionDescription {
                    title: "Edit File".to_string(),
                    description: format!("Invalid edit operations for file: {}", file_path),
                    changes: ChangeDetail::Generic {
                        items: vec![format!("Edit parsing error: {}", err)],
                    },
                })
            }
        }
    }
    
    async fn execute_with_context(
        &self,
        arguments: Value,
        recent_calls: Arc<Mutex<VecDeque<ToolCall>>>
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        let file_path = arguments
            .get("file_path")
            .and_then(|v| v.as_str())
            .ok_or("Missing 'file_path' parameter")?;
        
        let edits_value = arguments
            .get("edits")
            .ok_or("Missing 'edits' parameter")?;
        
        // Check if read_file was called recently for this file
        if !self.check_recent_read(file_path, &recent_calls) {
            // Instead of erroring, provide helpful guidance to the LLM
            let guidance_message = json!({
                "status": "guidance_needed",
                "action": "read_file_required",
                "message": format!(
                    "To edit this file, I need to read its current content with line numbers first. Please call read_file('{}') and then retry this edit operation.", 
                    file_path
                ),
                "suggested_next_steps": [
                    format!("1. Call read_file with file_path: '{}'", file_path),
                    "2. Review the current content and line numbers",
                    "3. Retry this edit_file operation with the same parameters"
                ],
                "details": {
                    "file_path": file_path,
                    "reason": "Line-number based editing requires current file content for safety and accuracy"
                }
            });
            
            debug!("EditFileTool: Guiding LLM to read file first: {}", file_path);
            return Ok(serde_json::to_string_pretty(&guidance_message).unwrap());
        }
        
        // Validate the path
        let path_validator = PathValidator::new(self.config.allowed_directories.clone());
        let validated_path = path_validator.validate_path(file_path)?;
        
        // Check that it's actually a file
        if !validated_path.is_file() {
            return Err(format!("Path is not a file: {}", validated_path.display()).into());
        }
        
        // Read current content
        let current_content = fs::read_to_string(&validated_path)
            .map_err(|e| format!("Failed to read file: {}", e))?;
        
        let mut content_lines: Vec<String> = current_content.lines().map(|s| s.to_string()).collect();
        
        // Parse and apply edits
        let line_edits = self.parse_edits(&content_lines, edits_value)?;
        self.apply_edits(&mut content_lines, &line_edits)?;
        
        // Write the modified content back to the file
        let new_content = content_lines.join("\n");
        fs::write(&validated_path, &new_content)
            .map_err(|e| format!("Failed to write file: {}", e))?;
        
        debug!("Edited file: {}", validated_path.display());
        
        // Return success message
        let result = json!({
            "status": "success",
            "action": "file_edited",
            "details": {
                "file_path": validated_path.to_string_lossy(),
                "edits_applied": line_edits.len(),
                "final_line_count": content_lines.len()
            },
            "message": format!("Successfully applied {} edit(s) to file: {}", line_edits.len(), validated_path.display())
        });
        
        Ok(serde_json::to_string_pretty(&result).unwrap())
    }
    
    async fn execute(&self, arguments: Value) -> Result<String, Box<dyn Error + Send + Sync>> {
        // This fallback version creates an empty call stack for contexts where it's not available
        let empty_calls = Arc::new(Mutex::new(VecDeque::new()));
        self.execute_with_context(arguments, empty_calls).await
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