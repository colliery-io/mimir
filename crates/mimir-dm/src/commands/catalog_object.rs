use std::fs;
use std::path::Path;
use tauri::State;
use tracing::{debug, error, info, warn};
use mimir_dm_core::models::catalog::{DndObject, ObjectData, ObjectSummary};

/// In-memory catalog of objects
pub struct ObjectCatalog {
    pub objects: Vec<DndObject>,
}

impl ObjectCatalog {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
    
    /// Load object data from the books directory
    pub fn load_from_books_directory(&mut self, books_dir: &Path) -> Result<(), String> {
        info!("Loading objects from {:?}", books_dir);
        
        self.objects.clear();
        
        if !books_dir.exists() {
            warn!("Books directory does not exist: {:?}", books_dir);
            return Ok(());
        }
        
        // Iterate through each book directory
        for entry in fs::read_dir(books_dir).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let book_path = entry.path();
            
            if !book_path.is_dir() {
                continue;
            }
            
            let book_id = book_path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");
            
            // Check for objects directory
            let objects_dir = book_path.join("objects");
            if objects_dir.exists() && objects_dir.is_dir() {
                debug!("Found objects directory for book: {}", book_id);
                
                // Load all JSON files in the objects directory
                for object_entry in fs::read_dir(&objects_dir).map_err(|e| e.to_string())? {
                    let object_entry = object_entry.map_err(|e| e.to_string())?;
                    let object_file = object_entry.path();
                    
                    if object_file.extension().and_then(|e| e.to_str()) != Some("json") {
                        continue;
                    }
                    
                    let filename = object_file.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown");
                    
                    match fs::read_to_string(&object_file) {
                        Ok(content) => {
                            match serde_json::from_str::<ObjectData>(&content) {
                                Ok(data) => {
                                    if let Some(objects) = data.object {
                                        if !objects.is_empty() {
                                            info!("Loaded {} objects from {}/{}", 
                                                  objects.len(), book_id, filename);
                                            self.objects.extend(objects);
                                        }
                                    }
                                }
                                Err(e) => {
                                    error!("Failed to parse objects from {}/{}: {}", book_id, filename, e);
                                }
                            }
                        }
                        Err(e) => {
                            error!("Failed to read {}/{}: {}", book_id, filename, e);
                        }
                    }
                }
            }
        }
        
        info!("Total objects loaded: {}", self.objects.len());
        Ok(())
    }
    
    /// Search objects with filters
    pub fn search(&self, 
        query: Option<String>,
        sources: Vec<String>,
        object_types: Vec<String>,
        sizes: Vec<String>
    ) -> Vec<ObjectSummary> {
        info!("Searching objects - query: {:?}, sources: {:?}, types: {:?}, sizes: {:?}", 
                 query, sources, object_types, sizes);
        
        let results: Vec<ObjectSummary> = self.objects.iter()
            .filter(|obj| {
                // Filter by query
                if let Some(q) = &query {
                    if !q.is_empty() {
                        let name_match = obj.name.to_lowercase().contains(&q.to_lowercase());
                        let entry_match = if let Some(entries) = &obj.entries {
                            entries.iter().any(|e| {
                                if let Some(s) = e.as_str() {
                                    s.to_lowercase().contains(&q.to_lowercase())
                                } else {
                                    false
                                }
                            })
                        } else {
                            false
                        };
                        if !name_match && !entry_match {
                            return false;
                        }
                    }
                }
                
                // Filter by sources
                if !sources.is_empty() && !sources.contains(&obj.source) {
                    return false;
                }
                
                // Filter by object types
                if !object_types.is_empty() {
                    if let Some(obj_type) = &obj.object_type {
                        if !object_types.contains(obj_type) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                
                // Filter by sizes
                if !sizes.is_empty() {
                    if let Some(obj_sizes) = &obj.size {
                        let has_matching_size = obj_sizes.iter()
                            .any(|s| sizes.contains(s));
                        if !has_matching_size {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                
                true
            })
            .map(ObjectSummary::from)
            .collect();
        
        info!("Found {} objects matching filters", results.len());
        results
    }
    
    /// Get detailed object information
    pub fn get_object_details(&self, name: &str, source: &str) -> Option<DndObject> {
        self.objects.iter()
            .find(|o| o.name == name && o.source == source)
            .cloned()
    }
    
    /// Get all unique object types in the catalog
    pub fn get_object_types(&self) -> Vec<String> {
        let mut types = std::collections::HashSet::new();
        for obj in &self.objects {
            if let Some(obj_type) = &obj.object_type {
                types.insert(obj_type.clone());
            }
        }
        let mut sorted: Vec<String> = types.into_iter().collect();
        sorted.sort();
        sorted
    }
}

/// Initialize the object catalog
#[tauri::command]
pub async fn init_object_catalog(
    catalog: State<'_, std::sync::Mutex<ObjectCatalog>>
) -> Result<(), String> {
    use crate::APP_PATHS;
    
    // Get app paths to find the books directory
    let app_paths = APP_PATHS.get()
        .ok_or_else(|| "App paths not initialized".to_string())?;
    
    let books_dir = app_paths.data_dir.join("books");
    
    let mut catalog = catalog.lock().map_err(|e| e.to_string())?;
    catalog.load_from_books_directory(&books_dir)?;
    
    Ok(())
}

/// Search objects with filters
#[tauri::command]
pub async fn search_objects(
    query: Option<String>,
    sources: Option<Vec<String>>,
    object_types: Option<Vec<String>>,
    sizes: Option<Vec<String>>,
    catalog: State<'_, std::sync::Mutex<ObjectCatalog>>
) -> Result<Vec<ObjectSummary>, String> {
    let catalog = catalog.lock().map_err(|e| e.to_string())?;
    
    Ok(catalog.search(
        query,
        sources.unwrap_or_default(),
        object_types.unwrap_or_default(),
        sizes.unwrap_or_default()
    ))
}

/// Get detailed information about a specific object
#[tauri::command]
pub async fn get_object_details(
    name: String,
    source: String,
    catalog: State<'_, std::sync::Mutex<ObjectCatalog>>
) -> Result<DndObject, String> {
    let catalog = catalog.lock().map_err(|e| e.to_string())?;
    
    catalog.get_object_details(&name, &source)
        .ok_or_else(|| format!("Object not found: {} from {}", name, source))
}

/// Get all available object types
#[tauri::command]
pub async fn get_object_types(
    catalog: State<'_, std::sync::Mutex<ObjectCatalog>>
) -> Result<Vec<String>, String> {
    let catalog = catalog.lock().map_err(|e| e.to_string())?;
    Ok(catalog.get_object_types())
}