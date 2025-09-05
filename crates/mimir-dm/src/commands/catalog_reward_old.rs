use std::collections::HashMap;
use std::path::Path;
use tracing::{error, info, warn};

use mimir_dm_core::models::catalog::{Reward, RewardData, RewardSummary};

pub struct RewardCatalog {
    rewards: Vec<Reward>,
    by_type: HashMap<String, Vec<usize>>,
    by_source: HashMap<String, Vec<usize>>,
}

impl RewardCatalog {
    pub fn new() -> Self {
        Self {
            rewards: Vec::new(),
            by_type: HashMap::new(),
            by_source: HashMap::new(),
        }
    }

    pub async fn load(&mut self, data_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        self.rewards.clear();
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

            let _book_name = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");

            // Look for rewards directory
            let rewards_dir = path.join("rewards");
            if !rewards_dir.exists() {
                continue;
            }

            // Read all JSON files in rewards directory
            let mut reward_entries = tokio::fs::read_dir(&rewards_dir).await?;
            while let Some(reward_entry) = reward_entries.next_entry().await? {
                let reward_path = reward_entry.path();
                if reward_path.extension().and_then(|e| e.to_str()) != Some("json") {
                    continue;
                }

                match self.load_rewards_file(&reward_path).await {
                    Ok(count) => {
                        info!("Loaded {} rewards from {:?}", count, reward_path);
                    }
                    Err(e) => {
                        error!("Failed to parse rewards from {:?}: {}", reward_path, e);
                    }
                }
            }
        }

        // Build indices
        for (idx, reward) in self.rewards.iter().enumerate() {
            // By type
            let reward_type = reward.reward_type.as_deref().unwrap_or("Unknown").to_string();
            self.by_type
                .entry(reward_type)
                .or_insert_with(Vec::new)
                .push(idx);

            // By source
            self.by_source
                .entry(reward.source.clone())
                .or_insert_with(Vec::new)
                .push(idx);
        }

        info!("Loaded {} total rewards", self.rewards.len());
        Ok(())
    }

    async fn load_rewards_file(&mut self, path: &Path) -> Result<usize, Box<dyn std::error::Error>> {
        let content = tokio::fs::read_to_string(path).await?;
        let data: RewardData = serde_json::from_str(&content)?;
        
        let mut count = 0;
        if let Some(rewards) = data.reward {
            count = rewards.len();
            self.rewards.extend(rewards);
        }
        
        Ok(count)
    }

    pub fn search(&self, params: SearchParams) -> Vec<RewardSummary> {
        let mut results: Vec<(usize, &Reward)> = self.rewards
            .iter()
            .enumerate()
            .filter(|(_, reward)| {
                // Filter by query
                if let Some(ref query) = params.query {
                    let query_lower = query.to_lowercase();
                    if !reward.name.to_lowercase().contains(&query_lower) &&
                       !reward.reward_type.as_ref().map_or(false, |t| t.to_lowercase().contains(&query_lower)) {
                        return false;
                    }
                }

                // Filter by sources
                if let Some(ref sources) = params.sources {
                    if !sources.contains(&reward.source) {
                        return false;
                    }
                }

                // Filter by type
                if let Some(ref types) = params.reward_types {
                    if !reward.reward_type.as_ref().map_or(false, |t| types.contains(t)) {
                        return false;
                    }
                }

                // Filter by prerequisites
                if params.has_prerequisites == Some(true) && reward.prerequisite.is_none() {
                    return false;
                }
                if params.has_prerequisites == Some(false) && reward.prerequisite.is_some() {
                    return false;
                }

                true
            })
            .collect();

        // Sort by name
        results.sort_by(|a, b| a.1.name.cmp(&b.1.name));

        results.into_iter()
            .map(|(_, reward)| RewardSummary::from(reward))
            .collect()
    }

    pub fn get_details(&self, name: &str, source: &str) -> Option<Reward> {
        self.rewards
            .iter()
            .find(|r| r.name.eq_ignore_ascii_case(name) && r.source == source)
            .cloned()
    }

    pub fn get_reward_types(&self) -> Vec<String> {
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
    pub sources: Option<Vec<String>>,
    pub reward_types: Option<Vec<String>>,
    pub has_prerequisites: Option<bool>,
}

// Tauri command interface
#[tauri::command]
pub async fn initialize_reward_catalog(
    catalog: tauri::State<'_, std::sync::Mutex<RewardCatalog>>
) -> Result<(), String> {
    use crate::APP_PATHS;
    
    // Get app paths to find the books directory
    let app_paths = APP_PATHS.get()
        .ok_or_else(|| "App paths not initialized".to_string())?;
    
    let data_dir = app_paths.data_dir.clone();
    
    // We need to clone the catalog to avoid holding the lock across await
    let mut new_catalog = RewardCatalog::new();
    new_catalog.load(&data_dir)
        .await
        .map_err(|e| format!("Failed to load reward catalog: {}", e))?;
    
    // Now update the shared catalog
    let mut cat = catalog.lock()
        .map_err(|e| format!("Failed to lock catalog: {}", e))?;
    *cat = new_catalog;
    
    Ok(())
}

#[tauri::command]
pub async fn search_rewards(
    catalog: tauri::State<'_, std::sync::Mutex<RewardCatalog>>,
    query: Option<String>,
    sources: Option<Vec<String>>,
    reward_types: Option<Vec<String>>,
    has_prerequisites: Option<bool>,
) -> Result<Vec<RewardSummary>, String> {
    let cat = catalog.lock()
        .map_err(|e| format!("Failed to lock catalog: {}", e))?;
    Ok(cat.search(SearchParams {
        query,
        sources,
        reward_types,
        has_prerequisites,
    }))
}

#[tauri::command]
pub async fn get_reward_details(
    catalog: tauri::State<'_, std::sync::Mutex<RewardCatalog>>,
    name: String, 
    source: String
) -> Result<Option<Reward>, String> {
    let cat = catalog.lock()
        .map_err(|e| format!("Failed to lock catalog: {}", e))?;
    Ok(cat.get_details(&name, &source))
}

#[tauri::command]
pub async fn get_reward_types(
    catalog: tauri::State<'_, std::sync::Mutex<RewardCatalog>>
) -> Result<Vec<String>, String> {
    let cat = catalog.lock()
        .map_err(|e| format!("Failed to lock catalog: {}", e))?;
    Ok(cat.get_reward_types())
}

#[tauri::command]
pub async fn get_reward_sources(
    catalog: tauri::State<'_, std::sync::Mutex<RewardCatalog>>
) -> Result<Vec<String>, String> {
    let cat = catalog.lock()
        .map_err(|e| format!("Failed to lock catalog: {}", e))?;
    Ok(cat.get_sources())
}