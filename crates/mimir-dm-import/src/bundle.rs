//! Bundle archive handling and extraction

use crate::error::{ImportError, ImportResult};
use crate::manifest::BundleManifest;
use flate2::read::GzDecoder;
use std::collections::HashMap;
use std::io::Read;
use std::path::Path;
use tar::Archive;
use tracing::{debug, info};

/// Represents an extracted bundle with all its files
#[derive(Debug)]
pub struct Bundle {
    /// Bundle manifest
    pub manifest: BundleManifest,
    
    /// Root directory name in the archive
    pub root_dir: String,
    
    /// All files from the bundle, keyed by filename
    pub files: HashMap<String, Vec<u8>>,
}

impl Bundle {
    /// Extract and parse a bundle from a .tar.gz file
    pub async fn from_archive<P: AsRef<Path>>(bundle_path: P) -> ImportResult<Self> {
        let bundle_path = bundle_path.as_ref();
        info!("Extracting bundle from: {}", bundle_path.display());

        // Open and decompress the archive
        let file = std::fs::File::open(bundle_path).map_err(|_| ImportError::BundleNotFound {
            path: bundle_path.to_path_buf(),
        })?;

        let gz_decoder = GzDecoder::new(file);
        let mut archive = Archive::new(gz_decoder);

        let mut files = HashMap::new();
        let mut root_dir = String::new();
        let mut manifest_content = None;

        // Extract all files
        for entry in archive.entries()? {
            let mut entry = entry?;
            let path = entry.path()?.into_owned();
            let path_str = path.to_string_lossy();

            debug!("Extracting: {}", path_str);

            // Determine root directory from first entry
            if root_dir.is_empty() {
                if let Some(first_component) = path.components().next() {
                    root_dir = first_component.as_os_str().to_string_lossy().to_string();
                }
            }

            // Read file contents
            let mut contents = Vec::new();
            entry.read_to_end(&mut contents)?;

            // Store file, using relative path from root
            let relative_path = path
                .strip_prefix(&root_dir)
                .unwrap_or(&path)
                .to_string_lossy()
                .to_string();

            // Special handling for manifest
            if relative_path == "manifest.json" {
                manifest_content = Some(contents.clone());
            }

            files.insert(relative_path, contents);
        }

        // Parse manifest
        let manifest_bytes = manifest_content.ok_or_else(|| ImportError::MissingFile {
            filename: "manifest.json".to_string(),
        })?;

        let manifest: BundleManifest = serde_json::from_slice(&manifest_bytes)
            .map_err(|e| ImportError::JsonParsing {
                filename: "manifest.json".to_string(),
                source: e,
            })?;

        info!(
            "Extracted bundle '{}' with {} files",
            manifest.bundle_name,
            files.len()
        );

        Ok(Bundle {
            manifest,
            root_dir,
            files,
        })
    }

    /// Get the contents of a specific file
    pub fn get_file(&self, filename: &str) -> Option<&[u8]> {
        self.files.get(filename).map(|v| v.as_slice())
    }

    /// Get the contents of a file as a UTF-8 string
    pub fn get_file_string(&self, filename: &str) -> ImportResult<Option<String>> {
        match self.get_file(filename) {
            Some(bytes) => {
                let content = String::from_utf8(bytes.to_vec()).map_err(|_| {
                    ImportError::InvalidEntityData {
                        filename: filename.to_string(),
                        reason: "File contains invalid UTF-8".to_string(),
                    }
                })?;
                Ok(Some(content))
            }
            None => Ok(None),
        }
    }

    /// Parse a JSON file from the bundle
    pub fn parse_json_file<T>(&self, filename: &str) -> ImportResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let contents = self.get_file(filename).ok_or_else(|| ImportError::MissingFile {
            filename: filename.to_string(),
        })?;

        serde_json::from_slice(contents).map_err(|e| ImportError::JsonParsing {
            filename: filename.to_string(),
            source: e,
        })
    }

    /// List all files in the bundle
    pub fn list_files(&self) -> Vec<&str> {
        self.files.keys().map(|s| s.as_str()).collect()
    }

    /// Check if a required file exists
    pub fn has_file(&self, filename: &str) -> bool {
        self.files.contains_key(filename)
    }
}