use std::fs;
use std::path::Path;
use tauri::State;
use tracing::{debug, error, info, warn};
use mimir_dm_core::models::catalog::{Action, ActionData, ActionSummary};

/// Helper function to format action time
fn format_action_time(time: &[serde_json::Value]) -> String {
    if time.is_empty() {
        return "1 action".to_string();
    }
    
    let mut times = Vec::new();
    for t in time {
        if let Some(s) = t.as_str() {
            times.push(s.to_string());
        } else if let Some(obj) = t.as_object() {
            if let (Some(number), Some(unit)) = 
                (obj.get("number").and_then(|n| n.as_i64()),
                 obj.get("unit").and_then(|u| u.as_str())) {
                if number == 1 {
                    times.push(format!("1 {}", unit));
                } else {
                    times.push(format!("{} {}s", number, unit));
                }
            }
        }
    }
    
    if times.is_empty() {
        "1 action".to_string()
    } else {
        times.join(" or ")
    }
}

/// In-memory catalog of actions
pub struct ActionCatalog {
    pub actions: Vec<Action>,
}

impl ActionCatalog {
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }
    
    /// Load action data from the books directory
    pub fn load_from_books_directory(&mut self, books_dir: &Path) -> Result<(), String> {
        info!("Loading actions from {:?}", books_dir);
        
        self.actions.clear();
        
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
            
            // Check for actions directory
            let actions_dir = book_path.join("actions");
            if actions_dir.exists() && actions_dir.is_dir() {
                debug!("Found actions directory for book: {}", book_id);
                
                // Load all JSON files in the actions directory
                for action_entry in fs::read_dir(&actions_dir).map_err(|e| e.to_string())? {
                    let action_entry = action_entry.map_err(|e| e.to_string())?;
                    let action_file = action_entry.path();
                    
                    if action_file.extension().and_then(|e| e.to_str()) != Some("json") {
                        continue;
                    }
                    
                    let filename = action_file.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown");
                    
                    // Handle action data files
                    match fs::read_to_string(&action_file) {
                        Ok(content) => {
                            match serde_json::from_str::<ActionData>(&content) {
                                Ok(action_data) => {
                                    if let Some(actions) = action_data.action {
                                        if !actions.is_empty() {
                                            debug!("Loaded {} actions from {}/{}", 
                                                    actions.len(), book_id, filename);
                                            self.actions.extend(actions);
                                        }
                                    }
                                }
                                Err(e) => {
                                    warn!("Failed to parse {}/{}: {}", book_id, filename, e);
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
        
        info!("Total actions loaded: {}", self.actions.len());
        Ok(())
    }
    
    /// Search actions with filters
    pub fn search(&self, 
        query: Option<String>,
        sources: Vec<String>,
        time_filter: Option<String>
    ) -> Vec<ActionSummary> {
        info!("Searching actions - query: {:?}, sources: {:?}, time: {:?}", 
                 query, sources, time_filter);
        
        let results: Vec<ActionSummary> = self.actions.iter()
            .filter(|action| {
                // Filter by query (search in name and entries)
                if let Some(q) = &query {
                    if !q.is_empty() {
                        let name_match = action.name.to_lowercase().contains(&q.to_lowercase());
                        // Also search in entries
                        let entry_match = action.entries.iter().any(|e| {
                            if let Some(s) = e.as_str() {
                                s.to_lowercase().contains(&q.to_lowercase())
                            } else {
                                false
                            }
                        });
                        if !name_match && !entry_match {
                            return false;
                        }
                    }
                }
                
                // Filter by sources
                if !sources.is_empty() && !sources.contains(&action.source) {
                    return false;
                }
                
                // Filter by time
                if let Some(time) = &time_filter {
                    // Check if the action's time matches the filter
                    // We need to format the time to compare it
                    let formatted_time = format_action_time(&action.time);
                    if !formatted_time.to_lowercase().contains(&time.to_lowercase()) {
                        return false;
                    }
                }
                
                true
            })
            .map(ActionSummary::from)
            .collect();
        
        info!("Found {} actions matching filters", results.len());
        results
    }
    
    /// Get detailed action information
    pub fn get_action_details(&self, name: &str, source: &str) -> Option<Action> {
        info!("Getting action details for {} from {}", name, source);
        
        self.actions.iter()
            .find(|action| action.name == name && action.source == source)
            .cloned()
    }
}

/// Initialize the action catalog
#[tauri::command]
pub async fn init_action_catalog(
    catalog: State<'_, std::sync::Mutex<ActionCatalog>>
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

/// Search actions with filters
#[tauri::command]
pub async fn search_actions(
    query: Option<String>,
    sources: Option<Vec<String>>,
    time_filter: Option<String>,
    catalog: State<'_, std::sync::Mutex<ActionCatalog>>
) -> Result<Vec<ActionSummary>, String> {
    let catalog = catalog.lock().map_err(|e| e.to_string())?;
    
    Ok(catalog.search(
        query,
        sources.unwrap_or_default(),
        time_filter
    ))
}

/// Get detailed information about a specific action
#[tauri::command]
pub async fn get_action_details(
    name: String,
    source: String,
    catalog: State<'_, std::sync::Mutex<ActionCatalog>>
) -> Result<Action, String> {
    let catalog = catalog.lock().map_err(|e| e.to_string())?;
    
    catalog.get_action_details(&name, &source)
        .ok_or_else(|| format!("Action not found: {} from {}", name, source))
}