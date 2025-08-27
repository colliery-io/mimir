use std::fs;
use std::path::Path;
use tauri::State;
use tracing::{debug, error, info, warn};
use mimir_dm_core::models::catalog::{
    Condition, ConditionData, Disease, DiseaseData, ConditionSummary, 
    ConditionOrDisease, ConditionWithDetails, ConditionFluff, ConditionFluffData
};

/// In-memory catalog of conditions and diseases
pub struct ConditionCatalog {
    pub conditions: Vec<Condition>,
    pub diseases: Vec<Disease>,
    pub condition_fluff: Vec<ConditionFluff>,
}

impl ConditionCatalog {
    pub fn new() -> Self {
        Self {
            conditions: Vec::new(),
            diseases: Vec::new(),
            condition_fluff: Vec::new(),
        }
    }
    
    /// Load condition and disease data from the books directory
    pub fn load_from_books_directory(&mut self, books_dir: &Path) -> Result<(), String> {
        info!("Loading conditions and diseases from {:?}", books_dir);
        
        self.conditions.clear();
        self.diseases.clear();
        self.condition_fluff.clear();
        
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
            
            // Check for conditions directory
            let conditions_dir = book_path.join("conditions");
            if conditions_dir.exists() && conditions_dir.is_dir() {
                debug!("Found conditions directory for book: {}", book_id);
                
                // Load all JSON files in the conditions directory
                for cond_entry in fs::read_dir(&conditions_dir).map_err(|e| e.to_string())? {
                    let cond_entry = cond_entry.map_err(|e| e.to_string())?;
                    let cond_file = cond_entry.path();
                    
                    if cond_file.extension().and_then(|e| e.to_str()) != Some("json") {
                        continue;
                    }
                    
                    let filename = cond_file.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown");
                    
                    // Check if it's a fluff file
                    if filename.starts_with("fluff-") {
                        match fs::read_to_string(&cond_file) {
                            Ok(content) => {
                                match serde_json::from_str::<ConditionFluffData>(&content) {
                                    Ok(fluff_data) => {
                                        if let Some(fluff) = fluff_data.condition_fluff {
                                            info!("Loaded {} condition fluff entries from {}/{}", 
                                                  fluff.len(), book_id, filename);
                                            self.condition_fluff.extend(fluff);
                                        }
                                    }
                                    Err(e) => {
                                        warn!("Failed to parse condition fluff {}/{}: {}", book_id, filename, e);
                                    }
                                }
                            }
                            Err(e) => {
                                error!("Failed to read {}/{}: {}", book_id, filename, e);
                            }
                        }
                        continue;
                    }
                    
                    // Handle condition data files
                    match fs::read_to_string(&cond_file) {
                        Ok(content) => {
                            match serde_json::from_str::<ConditionData>(&content) {
                                Ok(cond_data) => {
                                    if let Some(conditions) = cond_data.condition {
                                        if !conditions.is_empty() {
                                            debug!("Loaded {} conditions from {}/{}", 
                                                    conditions.len(), book_id, filename);
                                            self.conditions.extend(conditions);
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
            
            // Check for diseases directory
            let diseases_dir = book_path.join("diseases");
            if diseases_dir.exists() && diseases_dir.is_dir() {
                debug!("Found diseases directory for book: {}", book_id);
                
                // Load all JSON files in the diseases directory
                for disease_entry in fs::read_dir(&diseases_dir).map_err(|e| e.to_string())? {
                    let disease_entry = disease_entry.map_err(|e| e.to_string())?;
                    let disease_file = disease_entry.path();
                    
                    if disease_file.extension().and_then(|e| e.to_str()) != Some("json") {
                        continue;
                    }
                    
                    let filename = disease_file.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown");
                    
                    // Handle disease data files
                    match fs::read_to_string(&disease_file) {
                        Ok(content) => {
                            match serde_json::from_str::<DiseaseData>(&content) {
                                Ok(disease_data) => {
                                    if let Some(diseases) = disease_data.disease {
                                        if !diseases.is_empty() {
                                            debug!("Loaded {} diseases from {}/{}", 
                                                    diseases.len(), book_id, filename);
                                            self.diseases.extend(diseases);
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
        
        info!("Total conditions loaded: {}", self.conditions.len());
        info!("Total diseases loaded: {}", self.diseases.len());
        info!("Total condition fluff loaded: {}", self.condition_fluff.len());
        Ok(())
    }
    
    /// Search conditions and diseases with filters
    pub fn search(&self, 
        query: Option<String>,
        sources: Vec<String>,
        type_filter: Option<String> // "condition", "disease", or None for both
    ) -> Vec<ConditionSummary> {
        info!("Searching conditions/diseases - query: {:?}, sources: {:?}, type: {:?}", 
                 query, sources, type_filter);
        
        let mut results: Vec<ConditionSummary> = Vec::new();
        
        // Search conditions if not filtered to diseases only
        if type_filter.as_deref() != Some("disease") {
            let condition_results: Vec<ConditionSummary> = self.conditions.iter()
                .filter(|cond| {
                    // Filter by query
                    if let Some(q) = &query {
                        if !q.is_empty() {
                            let name_match = cond.name.to_lowercase().contains(&q.to_lowercase());
                            // Also search in entries
                            let entry_match = cond.entries.iter().any(|e| {
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
                    if !sources.is_empty() && !sources.contains(&cond.source) {
                        return false;
                    }
                    
                    true
                })
                .map(ConditionSummary::from)
                .collect();
            
            results.extend(condition_results);
        }
        
        // Search diseases if not filtered to conditions only
        if type_filter.as_deref() != Some("condition") {
            let disease_results: Vec<ConditionSummary> = self.diseases.iter()
                .filter(|disease| {
                    // Filter by query
                    if let Some(q) = &query {
                        if !q.is_empty() {
                            let name_match = disease.name.to_lowercase().contains(&q.to_lowercase());
                            // Also search in entries
                            let entry_match = disease.entries.iter().any(|e| {
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
                    if !sources.is_empty() && !sources.contains(&disease.source) {
                        return false;
                    }
                    
                    true
                })
                .map(ConditionSummary::from)
                .collect();
            
            results.extend(disease_results);
        }
        
        // Sort by name
        results.sort_by(|a, b| a.name.cmp(&b.name));
        
        info!("Found {} conditions/diseases matching filters", results.len());
        results
    }
    
    /// Get detailed condition or disease information
    pub fn get_condition_details(&self, name: &str, source: &str) -> Option<ConditionWithDetails> {
        info!("Getting condition/disease details for {} from {}", name, source);
        
        // Check conditions first
        if let Some(condition) = self.conditions.iter()
            .find(|c| c.name == name && c.source == source) {
            
            // Find associated fluff
            let fluff = self.condition_fluff.iter()
                .find(|f| f.name == name && f.source == source)
                .cloned();
            
            return Some(ConditionWithDetails {
                item: ConditionOrDisease::Condition(condition.clone()),
                fluff,
            });
        }
        
        // Check diseases
        if let Some(disease) = self.diseases.iter()
            .find(|d| d.name == name && d.source == source) {
            
            // Diseases typically don't have fluff
            return Some(ConditionWithDetails {
                item: ConditionOrDisease::Disease(disease.clone()),
                fluff: None,
            });
        }
        
        None
    }
}

/// Initialize the condition catalog
#[tauri::command]
pub async fn init_condition_catalog(
    catalog: State<'_, std::sync::Mutex<ConditionCatalog>>
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

/// Search conditions and diseases with filters
#[tauri::command]
pub async fn search_conditions(
    query: Option<String>,
    sources: Option<Vec<String>>,
    type_filter: Option<String>,
    catalog: State<'_, std::sync::Mutex<ConditionCatalog>>
) -> Result<Vec<ConditionSummary>, String> {
    let catalog = catalog.lock().map_err(|e| e.to_string())?;
    
    Ok(catalog.search(
        query,
        sources.unwrap_or_default(),
        type_filter
    ))
}

/// Get detailed information about a specific condition or disease
#[tauri::command]
pub async fn get_condition_details(
    name: String,
    source: String,
    catalog: State<'_, std::sync::Mutex<ConditionCatalog>>
) -> Result<ConditionWithDetails, String> {
    let catalog = catalog.lock().map_err(|e| e.to_string())?;
    
    catalog.get_condition_details(&name, &source)
        .ok_or_else(|| format!("Condition/disease not found: {} from {}", name, source))
}