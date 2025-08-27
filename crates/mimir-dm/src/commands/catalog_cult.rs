use std::collections::HashMap;
use std::path::Path;
use tracing::{error, info, warn};

use mimir_dm_core::models::catalog::{Cult, CultData, Boon, BoonData, CultBoonSummary};

pub struct CultCatalog {
    cults: Vec<Cult>,
    boons: Vec<Boon>,
    by_type: HashMap<String, Vec<(bool, usize)>>, // (is_boon, index)
    by_source: HashMap<String, Vec<(bool, usize)>>,
}

impl CultCatalog {
    pub fn new() -> Self {
        Self {
            cults: Vec::new(),
            boons: Vec::new(),
            by_type: HashMap::new(),
            by_source: HashMap::new(),
        }
    }

    pub async fn load(&mut self, data_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        self.cults.clear();
        self.boons.clear();
        self.by_type.clear();
        self.by_source.clear();

        let books_path = data_path.join("books");
        if !books_path.exists() {
            warn!("Books directory not found at: {:?}", books_path);
            return Ok(());
        }

        // Read all book directories
        let mut entries = tokio::fs::read_dir(&books_path).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }

            let book_name = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");

            // Look for cults directory
            let cults_dir = path.join("cults");
            if cults_dir.exists() {
                let mut cult_entries = tokio::fs::read_dir(&cults_dir).await?;
                while let Some(cult_entry) = cult_entries.next_entry().await? {
                    let cult_path = cult_entry.path();
                    if cult_path.extension().and_then(|e| e.to_str()) != Some("json") {
                        continue;
                    }

                    match self.load_cults_file(&cult_path, book_name).await {
                        Ok(count) => {
                            info!("Loaded {} cults from {:?}", count, cult_path);
                        }
                        Err(e) => {
                            error!("Failed to parse cults from {:?}: {}", cult_path, e);
                        }
                    }
                }
            }

            // Look for boons directory
            let boons_dir = path.join("boons");
            if boons_dir.exists() {
                let mut boon_entries = tokio::fs::read_dir(&boons_dir).await?;
                while let Some(boon_entry) = boon_entries.next_entry().await? {
                    let boon_path = boon_entry.path();
                    if boon_path.extension().and_then(|e| e.to_str()) != Some("json") {
                        continue;
                    }

                    match self.load_boons_file(&boon_path, book_name).await {
                        Ok(count) => {
                            info!("Loaded {} boons from {:?}", count, boon_path);
                        }
                        Err(e) => {
                            error!("Failed to parse boons from {:?}: {}", boon_path, e);
                        }
                    }
                }
            }
        }

        // Build indices for cults
        for (idx, cult) in self.cults.iter().enumerate() {
            // By type
            let cult_type = cult.cult_type.clone().unwrap_or("Unknown".to_string());
            self.by_type
                .entry(cult_type)
                .or_insert_with(Vec::new)
                .push((false, idx));

            // By source
            self.by_source
                .entry(cult.source.clone())
                .or_insert_with(Vec::new)
                .push((false, idx));
        }

        // Build indices for boons
        for (idx, boon) in self.boons.iter().enumerate() {
            // By type
            let boon_type = boon.boon_type.clone().unwrap_or("Unknown".to_string());
            self.by_type
                .entry(boon_type)
                .or_insert_with(Vec::new)
                .push((true, idx));

            // By source
            self.by_source
                .entry(boon.source.clone())
                .or_insert_with(Vec::new)
                .push((true, idx));
        }

        info!("Loaded {} cults and {} boons", self.cults.len(), self.boons.len());
        Ok(())
    }

    async fn load_cults_file(&mut self, path: &Path, source: &str) -> Result<usize, Box<dyn std::error::Error>> {
        let content = tokio::fs::read_to_string(path).await?;
        let data: CultData = serde_json::from_str(&content)?;
        
        let mut count = 0;
        if let Some(cults) = data.cult {
            for mut cult in cults {
                // Ensure source is set
                if cult.source.is_empty() {
                    cult.source = source.to_string();
                }
                self.cults.push(cult);
                count += 1;
            }
        }
        
        Ok(count)
    }

    async fn load_boons_file(&mut self, path: &Path, source: &str) -> Result<usize, Box<dyn std::error::Error>> {
        let content = tokio::fs::read_to_string(path).await?;
        let data: BoonData = serde_json::from_str(&content)?;
        
        let mut count = 0;
        if let Some(boons) = data.boon {
            for mut boon in boons {
                // Ensure source is set
                if boon.source.is_empty() {
                    boon.source = source.to_string();
                }
                self.boons.push(boon);
                count += 1;
            }
        }
        
        Ok(count)
    }

    pub fn search(&self, params: SearchParams) -> Vec<CultBoonSummary> {
        let mut results = Vec::new();

        // Search cults
        for (idx, cult) in self.cults.iter().enumerate() {
            if self.matches_cult(cult, &params) {
                results.push((false, idx, CultBoonSummary::from(cult)));
            }
        }

        // Search boons
        for (idx, boon) in self.boons.iter().enumerate() {
            if self.matches_boon(boon, &params) {
                results.push((true, idx, CultBoonSummary::from(boon)));
            }
        }

        // Sort by name
        results.sort_by(|a, b| a.2.name.cmp(&b.2.name));

        results.into_iter().map(|(_, _, summary)| summary).collect()
    }

    fn matches_cult(&self, cult: &Cult, params: &SearchParams) -> bool {
        // Filter by query
        if let Some(ref query) = params.query {
            let query_lower = query.to_lowercase();
            if !cult.name.to_lowercase().contains(&query_lower) {
                return false;
            }
        }

        // Filter by item type
        if let Some(ref item_types) = params.item_types {
            if !item_types.contains(&"cult".to_string()) {
                return false;
            }
        }

        // Filter by subtype
        if let Some(ref subtypes) = params.subtypes {
            let cult_type = cult.cult_type.as_ref().map(|s| s.as_str()).unwrap_or("Unknown");
            if !subtypes.iter().any(|t| t == cult_type) {
                return false;
            }
        }

        // Filter by sources
        if let Some(ref sources) = params.sources {
            if !sources.contains(&cult.source) {
                return false;
            }
        }

        true
    }

    fn matches_boon(&self, boon: &Boon, params: &SearchParams) -> bool {
        // Filter by query
        if let Some(ref query) = params.query {
            let query_lower = query.to_lowercase();
            if !boon.name.to_lowercase().contains(&query_lower) {
                return false;
            }
        }

        // Filter by item type
        if let Some(ref item_types) = params.item_types {
            if !item_types.contains(&"boon".to_string()) {
                return false;
            }
        }

        // Filter by subtype
        if let Some(ref subtypes) = params.subtypes {
            let boon_type = boon.boon_type.as_ref().map(|s| s.as_str()).unwrap_or("Unknown");
            if !subtypes.iter().any(|t| t == boon_type) {
                return false;
            }
        }

        // Filter by sources
        if let Some(ref sources) = params.sources {
            if !sources.contains(&boon.source) {
                return false;
            }
        }

        true
    }

    pub fn get_cult_details(&self, name: &str, source: &str) -> Option<Cult> {
        self.cults
            .iter()
            .find(|c| c.name.eq_ignore_ascii_case(name) && c.source == source)
            .cloned()
    }

    pub fn get_boon_details(&self, name: &str, source: &str) -> Option<Boon> {
        self.boons
            .iter()
            .find(|b| b.name.eq_ignore_ascii_case(name) && b.source == source)
            .cloned()
    }

    pub fn get_types(&self) -> Vec<String> {
        let mut types: Vec<String> = self.by_type.keys().cloned().collect();
        types.sort();
        types
    }

    pub fn get_sources(&self) -> Vec<String> {
        let mut sources: Vec<String> = self.by_source.keys().cloned().collect();
        sources.sort();
        sources
    }
}

#[derive(Debug, Clone)]
pub struct SearchParams {
    pub query: Option<String>,
    pub item_types: Option<Vec<String>>, // "cult", "boon", or both
    pub subtypes: Option<Vec<String>>, // Diabolical, Demonic, Elder Evil, etc.
    pub sources: Option<Vec<String>>,
}

// Tauri command interface
#[tauri::command]
pub async fn init_cult_catalog(
    catalog: tauri::State<'_, std::sync::Mutex<CultCatalog>>
) -> Result<(), String> {
    use crate::APP_PATHS;
    
    // Get app paths to find the books directory
    let app_paths = APP_PATHS.get()
        .ok_or_else(|| "App paths not initialized".to_string())?;
    
    let data_dir = app_paths.data_dir.clone();
    
    // We need to clone the catalog to avoid holding the lock across await
    let mut new_catalog = CultCatalog::new();
    new_catalog.load(&data_dir)
        .await
        .map_err(|e| format!("Failed to load cult catalog: {}", e))?;
    
    // Now update the shared catalog
    let mut cat = catalog.lock()
        .map_err(|e| format!("Failed to lock catalog: {}", e))?;
    *cat = new_catalog;
    
    Ok(())
}

#[tauri::command]
pub async fn search_cults(
    catalog: tauri::State<'_, std::sync::Mutex<CultCatalog>>,
    query: Option<String>,
    item_types: Option<Vec<String>>,
    subtypes: Option<Vec<String>>,
    sources: Option<Vec<String>>,
) -> Result<Vec<CultBoonSummary>, String> {
    let cat = catalog.lock()
        .map_err(|e| format!("Failed to lock catalog: {}", e))?;
    Ok(cat.search(SearchParams {
        query,
        item_types,
        subtypes,
        sources,
    }))
}

#[tauri::command]
pub async fn get_cult_details(
    catalog: tauri::State<'_, std::sync::Mutex<CultCatalog>>,
    name: String, 
    source: String
) -> Result<Option<Cult>, String> {
    let cat = catalog.lock()
        .map_err(|e| format!("Failed to lock catalog: {}", e))?;
    Ok(cat.get_cult_details(&name, &source))
}

#[tauri::command]
pub async fn get_boon_details(
    catalog: tauri::State<'_, std::sync::Mutex<CultCatalog>>,
    name: String, 
    source: String
) -> Result<Option<Boon>, String> {
    let cat = catalog.lock()
        .map_err(|e| format!("Failed to lock catalog: {}", e))?;
    Ok(cat.get_boon_details(&name, &source))
}

#[tauri::command]
pub async fn get_cult_types(
    catalog: tauri::State<'_, std::sync::Mutex<CultCatalog>>
) -> Result<Vec<String>, String> {
    let cat = catalog.lock()
        .map_err(|e| format!("Failed to lock catalog: {}", e))?;
    Ok(cat.get_types())
}

#[tauri::command]
pub async fn get_cult_sources(
    catalog: tauri::State<'_, std::sync::Mutex<CultCatalog>>
) -> Result<Vec<String>, String> {
    let cat = catalog.lock()
        .map_err(|e| format!("Failed to lock catalog: {}", e))?;
    Ok(cat.get_sources())
}