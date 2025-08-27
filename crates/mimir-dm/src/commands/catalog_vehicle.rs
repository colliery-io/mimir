use std::collections::HashMap;
use std::path::Path;
use tracing::{error, info, warn};

use mimir_dm_core::models::catalog::{Vehicle, VehicleData, VehicleSummary};

pub struct VehicleCatalog {
    vehicles: Vec<Vehicle>,
    by_type: HashMap<String, Vec<usize>>,
    by_source: HashMap<String, Vec<usize>>,
    by_terrain: HashMap<String, Vec<usize>>,
}

impl VehicleCatalog {
    pub fn new() -> Self {
        Self {
            vehicles: Vec::new(),
            by_type: HashMap::new(),
            by_source: HashMap::new(),
            by_terrain: HashMap::new(),
        }
    }

    pub async fn load(&mut self, data_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        self.vehicles.clear();
        self.by_type.clear();
        self.by_source.clear();
        self.by_terrain.clear();

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

            // Look for vehicles directory
            let vehicles_dir = path.join("vehicles");
            if !vehicles_dir.exists() {
                continue;
            }

            // Read all JSON files in vehicles directory
            let mut vehicle_entries = tokio::fs::read_dir(&vehicles_dir).await?;
            while let Some(vehicle_entry) = vehicle_entries.next_entry().await? {
                let vehicle_path = vehicle_entry.path();
                
                // Skip fluff files
                if let Some(name) = vehicle_path.file_name().and_then(|n| n.to_str()) {
                    if name.starts_with("fluff-") {
                        continue;
                    }
                }
                
                if vehicle_path.extension().and_then(|e| e.to_str()) != Some("json") {
                    continue;
                }

                match self.load_vehicles_file(&vehicle_path, book_name).await {
                    Ok(count) => {
                        info!("Loaded {} vehicles from {:?}", count, vehicle_path);
                    }
                    Err(e) => {
                        error!("Failed to parse vehicles from {:?}: {}", vehicle_path, e);
                    }
                }
            }
        }

        // Build indices
        for (idx, vehicle) in self.vehicles.iter().enumerate() {
            // By type
            let vehicle_type = vehicle.vehicle_type.clone().unwrap_or("Unknown".to_string());
            self.by_type
                .entry(vehicle_type)
                .or_insert_with(Vec::new)
                .push(idx);

            // By source
            self.by_source
                .entry(vehicle.source.clone())
                .or_insert_with(Vec::new)
                .push(idx);
                
            // By terrain
            if let Some(ref terrains) = vehicle.terrain {
                for terrain in terrains {
                    self.by_terrain
                        .entry(terrain.clone())
                        .or_insert_with(Vec::new)
                        .push(idx);
                }
            }
        }

        info!("Loaded {} total vehicles", self.vehicles.len());
        Ok(())
    }

    async fn load_vehicles_file(&mut self, path: &Path, source: &str) -> Result<usize, Box<dyn std::error::Error>> {
        let content = tokio::fs::read_to_string(path).await?;
        let data: VehicleData = serde_json::from_str(&content)?;
        
        let mut count = 0;
        if let Some(vehicles) = data.vehicle {
            for mut vehicle in vehicles {
                // Ensure source is set
                if vehicle.source.is_empty() {
                    vehicle.source = source.to_string();
                }
                self.vehicles.push(vehicle);
                count += 1;
            }
        }
        
        Ok(count)
    }

    pub fn search(&self, params: SearchParams) -> Vec<VehicleSummary> {
        let mut results: Vec<(usize, &Vehicle)> = self.vehicles
            .iter()
            .enumerate()
            .filter(|(_, vehicle)| {
                // Filter by query
                if let Some(ref query) = params.query {
                    let query_lower = query.to_lowercase();
                    if !vehicle.name.to_lowercase().contains(&query_lower) {
                        return false;
                    }
                }

                // Filter by types
                if let Some(ref types) = params.types {
                    let vehicle_type = vehicle.vehicle_type.as_ref().map(|s| s.as_str()).unwrap_or("Unknown");
                    if !types.iter().any(|t| t == vehicle_type) {
                        return false;
                    }
                }

                // Filter by sources
                if let Some(ref sources) = params.sources {
                    if !sources.contains(&vehicle.source) {
                        return false;
                    }
                }
                
                // Filter by terrain
                if let Some(ref terrains) = params.terrains {
                    if let Some(ref vehicle_terrains) = vehicle.terrain {
                        if !terrains.iter().any(|t| vehicle_terrains.contains(t)) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }

                // Filter by size
                if let Some(ref sizes) = params.sizes {
                    if let Some(ref vehicle_size) = vehicle.size {
                        if !sizes.contains(vehicle_size) {
                            return false;
                        }
                    }
                }

                true
            })
            .collect();

        // Sort by name
        results.sort_by(|a, b| a.1.name.cmp(&b.1.name));

        results.into_iter()
            .map(|(_, vehicle)| VehicleSummary::from(vehicle))
            .collect()
    }

    pub fn get_details(&self, name: &str, source: &str) -> Option<Vehicle> {
        self.vehicles
            .iter()
            .find(|v| v.name.eq_ignore_ascii_case(name) && v.source == source)
            .cloned()
    }

    pub fn get_types(&self) -> Vec<String> {
        let mut types: Vec<String> = self.by_type.keys().cloned().collect();
        types.sort();
        types
    }

    pub fn get_terrains(&self) -> Vec<String> {
        let mut terrains: Vec<String> = self.by_terrain.keys().cloned().collect();
        terrains.sort();
        terrains
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
    pub types: Option<Vec<String>>,
    pub sources: Option<Vec<String>>,
    pub terrains: Option<Vec<String>>,
    pub sizes: Option<Vec<String>>,
}

// Tauri command interface
#[tauri::command]
pub async fn init_vehicle_catalog(
    catalog: tauri::State<'_, std::sync::Mutex<VehicleCatalog>>
) -> Result<(), String> {
    use crate::APP_PATHS;
    
    // Get app paths to find the books directory
    let app_paths = APP_PATHS.get()
        .ok_or_else(|| "App paths not initialized".to_string())?;
    
    let data_dir = app_paths.data_dir.clone();
    
    // We need to clone the catalog to avoid holding the lock across await
    let mut new_catalog = VehicleCatalog::new();
    new_catalog.load(&data_dir)
        .await
        .map_err(|e| format!("Failed to load vehicle catalog: {}", e))?;
    
    // Now update the shared catalog
    let mut cat = catalog.lock()
        .map_err(|e| format!("Failed to lock catalog: {}", e))?;
    *cat = new_catalog;
    
    Ok(())
}

#[tauri::command]
pub async fn search_vehicles(
    catalog: tauri::State<'_, std::sync::Mutex<VehicleCatalog>>,
    query: Option<String>,
    types: Option<Vec<String>>,
    sources: Option<Vec<String>>,
    terrains: Option<Vec<String>>,
    sizes: Option<Vec<String>>,
) -> Result<Vec<VehicleSummary>, String> {
    let cat = catalog.lock()
        .map_err(|e| format!("Failed to lock catalog: {}", e))?;
    Ok(cat.search(SearchParams {
        query,
        types,
        sources,
        terrains,
        sizes,
    }))
}

#[tauri::command]
pub async fn get_vehicle_details(
    catalog: tauri::State<'_, std::sync::Mutex<VehicleCatalog>>,
    name: String, 
    source: String
) -> Result<Option<Vehicle>, String> {
    let cat = catalog.lock()
        .map_err(|e| format!("Failed to lock catalog: {}", e))?;
    Ok(cat.get_details(&name, &source))
}

#[tauri::command]
pub async fn get_vehicle_types(
    catalog: tauri::State<'_, std::sync::Mutex<VehicleCatalog>>
) -> Result<Vec<String>, String> {
    let cat = catalog.lock()
        .map_err(|e| format!("Failed to lock catalog: {}", e))?;
    Ok(cat.get_types())
}

#[tauri::command]
pub async fn get_vehicle_terrains(
    catalog: tauri::State<'_, std::sync::Mutex<VehicleCatalog>>
) -> Result<Vec<String>, String> {
    let cat = catalog.lock()
        .map_err(|e| format!("Failed to lock catalog: {}", e))?;
    Ok(cat.get_terrains())
}

#[tauri::command]
pub async fn get_vehicle_sources(
    catalog: tauri::State<'_, std::sync::Mutex<VehicleCatalog>>
) -> Result<Vec<String>, String> {
    let cat = catalog.lock()
        .map_err(|e| format!("Failed to lock catalog: {}", e))?;
    Ok(cat.get_sources())
}