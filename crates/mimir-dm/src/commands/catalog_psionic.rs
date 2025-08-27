use mimir_dm_core::models::catalog::{Psionic, PsionicSummary};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tauri::State;

pub struct PsionicCatalog {
    psionics: Vec<Psionic>,
    by_name_and_source: HashMap<(String, String), usize>,
}

impl PsionicCatalog {
    pub fn new() -> Self {
        let psionics = Self::load_psionics();
        let by_name_and_source = psionics
            .iter()
            .enumerate()
            .map(|(i, p)| ((p.name.clone(), p.source.clone()), i))
            .collect();

        Self {
            psionics,
            by_name_and_source,
        }
    }

    fn load_psionics() -> Vec<Psionic> {
        let data_path = PathBuf::from("data/5etools-2014-src-v1.210.46/data/psionics.json");
        
        if !data_path.exists() {
            eprintln!("Warning: psionics.json not found at {:?}", data_path);
            return Vec::new();
        }

        let content = match fs::read_to_string(&data_path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Failed to read psionics.json: {}", e);
                return Vec::new();
            }
        };

        let data: serde_json::Value = match serde_json::from_str(&content) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Failed to parse psionics.json: {}", e);
                return Vec::new();
            }
        };

        let mut psionics = Vec::new();

        if let Some(psionic_array) = data["psionic"].as_array() {
            for psionic_json in psionic_array {
                match serde_json::from_value(psionic_json.clone()) {
                    Ok(psionic) => psionics.push(psionic),
                    Err(e) => {
                        eprintln!(
                            "Failed to deserialize psionic {}: {}",
                            psionic_json.get("name").and_then(|n| n.as_str()).unwrap_or("unknown"),
                            e
                        );
                    }
                }
            }
        }

        println!("Loaded {} psionics", psionics.len());
        psionics
    }

    pub fn search(&self, params: SearchPsionicsParams) -> Vec<PsionicSummary> {
        self.psionics
            .iter()
            .filter(|p| {
                // Filter by query
                if let Some(query) = &params.query {
                    let query_lower = query.to_lowercase();
                    if !p.name.to_lowercase().contains(&query_lower) {
                        return false;
                    }
                }

                // Filter by type (D = Discipline, T = Talent)
                if let Some(psionic_types) = &params.psionic_types {
                    if !psionic_types.is_empty() && !psionic_types.contains(&p.psionic_type) {
                        return false;
                    }
                }

                // Filter by order
                if let Some(orders) = &params.orders {
                    if !orders.is_empty() {
                        match &p.order {
                            Some(order) => {
                                if !orders.contains(order) {
                                    return false;
                                }
                            },
                            None => return false,
                        }
                    }
                }

                // Filter by sources
                if let Some(sources) = &params.sources {
                    if !sources.is_empty() && !sources.contains(&p.source) {
                        return false;
                    }
                }

                true
            })
            .map(|p| PsionicSummary {
                name: p.name.clone(),
                source: p.source.clone(),
                psionic_type: p.psionic_type.clone(),
                order: p.order.clone(),
                page: p.page,
            })
            .collect()
    }

    pub fn get_details(&self, name: &str, source: &str) -> Option<Psionic> {
        self.by_name_and_source
            .get(&(name.to_string(), source.to_string()))
            .and_then(|&idx| self.psionics.get(idx))
            .cloned()
    }

    pub fn get_orders(&self) -> Vec<String> {
        let mut orders = std::collections::HashSet::new();
        for psionic in &self.psionics {
            if let Some(order) = &psionic.order {
                orders.insert(order.clone());
            }
        }
        let mut result: Vec<String> = orders.into_iter().collect();
        result.sort();
        result
    }

    pub fn get_sources(&self) -> Vec<String> {
        let mut sources = std::collections::HashSet::new();
        for psionic in &self.psionics {
            sources.insert(psionic.source.clone());
        }
        let mut result: Vec<String> = sources.into_iter().collect();
        result.sort();
        result
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchPsionicsParams {
    pub query: Option<String>,
    pub psionic_types: Option<Vec<String>>, // "D" for Discipline, "T" for Talent
    pub orders: Option<Vec<String>>,
    pub sources: Option<Vec<String>>,
}

#[tauri::command]
pub async fn search_psionics(
    params: SearchPsionicsParams,
    catalog: State<'_, std::sync::Mutex<PsionicCatalog>>,
) -> Result<Vec<PsionicSummary>, String> {
    let catalog = catalog.lock().map_err(|e| e.to_string())?;
    Ok(catalog.search(params))
}

#[tauri::command]
pub async fn get_psionic_details(
    name: String,
    source: String,
    catalog: State<'_, std::sync::Mutex<PsionicCatalog>>,
) -> Result<Option<Psionic>, String> {
    let catalog = catalog.lock().map_err(|e| e.to_string())?;
    Ok(catalog.get_details(&name, &source))
}

#[tauri::command]
pub async fn get_psionic_orders(
    catalog: State<'_, std::sync::Mutex<PsionicCatalog>>,
) -> Result<Vec<String>, String> {
    let catalog = catalog.lock().map_err(|e| e.to_string())?;
    Ok(catalog.get_orders())
}

#[tauri::command]
pub async fn get_psionic_sources(
    catalog: State<'_, std::sync::Mutex<PsionicCatalog>>,
) -> Result<Vec<String>, String> {
    let catalog = catalog.lock().map_err(|e| e.to_string())?;
    Ok(catalog.get_sources())
}