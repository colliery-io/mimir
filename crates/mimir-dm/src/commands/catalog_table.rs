use std::collections::HashMap;
use std::path::Path;
use tracing::{error, info, warn};

use mimir_dm_core::models::catalog::{Table, TableData, TableSummary};

pub struct TableCatalog {
    tables: Vec<Table>,
    by_category: HashMap<String, Vec<usize>>,
    by_source: HashMap<String, Vec<usize>>,
}

impl TableCatalog {
    pub fn new() -> Self {
        Self {
            tables: Vec::new(),
            by_category: HashMap::new(),
            by_source: HashMap::new(),
        }
    }

    pub async fn load(&mut self, data_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        self.tables.clear();
        self.by_category.clear();
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

            // Look for tables directory
            let tables_dir = path.join("tables");
            if !tables_dir.exists() {
                continue;
            }

            // Read all JSON files in tables directory
            let mut table_entries = tokio::fs::read_dir(&tables_dir).await?;
            while let Some(table_entry) = table_entries.next_entry().await? {
                let table_path = table_entry.path();
                if table_path.extension().and_then(|e| e.to_str()) != Some("json") {
                    continue;
                }

                match self.load_tables_file(&table_path).await {
                    Ok(count) => {
                        info!("Loaded {} tables from {:?}", count, table_path);
                    }
                    Err(e) => {
                        error!("Failed to parse tables from {:?}: {}", table_path, e);
                    }
                }
            }
        }

        // Build indices
        for (idx, table) in self.tables.iter().enumerate() {
            // By category (computed from name)
            let category = categorize_table(&table.name);
            self.by_category
                .entry(category)
                .or_insert_with(Vec::new)
                .push(idx);

            // By source
            self.by_source
                .entry(table.source.clone())
                .or_insert_with(Vec::new)
                .push(idx);
        }

        info!("Loaded {} total tables", self.tables.len());
        Ok(())
    }

    async fn load_tables_file(&mut self, path: &Path) -> Result<usize, Box<dyn std::error::Error>> {
        let content = tokio::fs::read_to_string(path).await?;
        let data: TableData = serde_json::from_str(&content)?;
        
        let mut count = 0;
        if let Some(tables) = data.table {
            count = tables.len();
            self.tables.extend(tables);
        }
        
        Ok(count)
    }

    pub fn search(&self, params: SearchParams) -> Vec<TableSummary> {
        let mut results: Vec<(usize, &Table)> = self.tables
            .iter()
            .enumerate()
            .filter(|(_, table)| {
                // Filter by query
                if let Some(ref query) = params.query {
                    let query_lower = query.to_lowercase();
                    if !table.name.to_lowercase().contains(&query_lower) &&
                       !table.caption.as_ref().map_or(false, |c| c.to_lowercase().contains(&query_lower)) {
                        return false;
                    }
                }

                // Filter by sources
                if let Some(ref sources) = params.sources {
                    if !sources.contains(&table.source) {
                        return false;
                    }
                }

                // Filter by category
                if let Some(ref categories) = params.categories {
                    let category = categorize_table(&table.name);
                    if !categories.contains(&category) {
                        return false;
                    }
                }

                // Filter by size (number of rows)
                if let Some(min_rows) = params.min_rows {
                    if table.rows.len() < min_rows {
                        return false;
                    }
                }
                if let Some(max_rows) = params.max_rows {
                    if table.rows.len() > max_rows {
                        return false;
                    }
                }

                true
            })
            .collect();

        // Sort by name
        results.sort_by(|a, b| a.1.name.cmp(&b.1.name));

        results.into_iter()
            .map(|(_, table)| TableSummary::from(table))
            .collect()
    }

    pub fn get_details(&self, name: &str, source: &str) -> Option<Table> {
        self.tables
            .iter()
            .find(|t| t.name.eq_ignore_ascii_case(name) && t.source == source)
            .cloned()
    }

    pub fn get_categories(&self) -> Vec<String> {
        let mut categories: Vec<String> = self.by_category.keys().cloned().collect();
        categories.sort();
        categories
    }

    pub fn get_sources(&self) -> Vec<String> {
        let mut sources: Vec<String> = self.by_source.keys().cloned().collect();
        sources.sort();
        sources
    }
}

fn categorize_table(name: &str) -> String {
    let name_lower = name.to_lowercase();
    
    if name_lower.contains("madness") || name_lower.contains("insanity") {
        "Madness".to_string()
    } else if name_lower.contains("treasure") || name_lower.contains("loot") || name_lower.contains("hoard") {
        "Treasure".to_string()
    } else if name_lower.contains("encounter") || name_lower.contains("random") {
        "Encounters".to_string()
    } else if name_lower.contains("trinket") {
        "Trinkets".to_string()
    } else if name_lower.contains("wild magic") || name_lower.contains("surge") {
        "Wild Magic".to_string()
    } else if name_lower.contains("damage") || name_lower.contains("critical") {
        "Combat".to_string()
    } else if name_lower.contains("npc") || name_lower.contains("name") || name_lower.contains("personality") {
        "NPCs".to_string()
    } else if name_lower.contains("quest") || name_lower.contains("adventure") || name_lower.contains("plot") {
        "Adventures".to_string()
    } else if name_lower.contains("magic item") || name_lower.contains("artifact") {
        "Magic Items".to_string()
    } else {
        "Miscellaneous".to_string()
    }
}

#[derive(Debug, Clone)]
pub struct SearchParams {
    pub query: Option<String>,
    pub sources: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
    pub min_rows: Option<usize>,
    pub max_rows: Option<usize>,
}

// Tauri command interface
#[tauri::command]
pub async fn init_table_catalog(
    catalog: tauri::State<'_, std::sync::Mutex<TableCatalog>>
) -> Result<(), String> {
    use crate::APP_PATHS;
    
    // Get app paths to find the books directory
    let app_paths = APP_PATHS.get()
        .ok_or_else(|| "App paths not initialized".to_string())?;
    
    let data_dir = app_paths.data_dir.clone();
    
    // We need to clone the catalog to avoid holding the lock across await
    let mut new_catalog = TableCatalog::new();
    new_catalog.load(&data_dir)
        .await
        .map_err(|e| format!("Failed to load table catalog: {}", e))?;
    
    // Now update the shared catalog
    let mut cat = catalog.lock()
        .map_err(|e| format!("Failed to lock catalog: {}", e))?;
    *cat = new_catalog;
    
    Ok(())
}

#[tauri::command]
pub async fn search_tables(
    catalog: tauri::State<'_, std::sync::Mutex<TableCatalog>>,
    query: Option<String>,
    sources: Option<Vec<String>>,
    categories: Option<Vec<String>>,
    min_rows: Option<usize>,
    max_rows: Option<usize>,
) -> Result<Vec<TableSummary>, String> {
    let cat = catalog.lock()
        .map_err(|e| format!("Failed to lock catalog: {}", e))?;
    Ok(cat.search(SearchParams {
        query,
        sources,
        categories,
        min_rows,
        max_rows,
    }))
}

#[tauri::command]
pub async fn get_table_details(
    catalog: tauri::State<'_, std::sync::Mutex<TableCatalog>>,
    name: String, 
    source: String
) -> Result<Option<Table>, String> {
    let cat = catalog.lock()
        .map_err(|e| format!("Failed to lock catalog: {}", e))?;
    Ok(cat.get_details(&name, &source))
}

#[tauri::command]
pub async fn get_table_categories(
    catalog: tauri::State<'_, std::sync::Mutex<TableCatalog>>
) -> Result<Vec<String>, String> {
    let cat = catalog.lock()
        .map_err(|e| format!("Failed to lock catalog: {}", e))?;
    Ok(cat.get_categories())
}

#[tauri::command]
pub async fn get_table_sources(
    catalog: tauri::State<'_, std::sync::Mutex<TableCatalog>>
) -> Result<Vec<String>, String> {
    let cat = catalog.lock()
        .map_err(|e| format!("Failed to lock catalog: {}", e))?;
    Ok(cat.get_sources())
}