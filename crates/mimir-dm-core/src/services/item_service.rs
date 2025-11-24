use diesel::prelude::*;
use crate::error::Result;
use crate::models::catalog::item::{
    CatalogItem, ItemSummary, ItemFilters, Item, NewCatalogItem, ItemData
};
use crate::schema::catalog_items;
use std::fs;
use std::path::Path;
use tracing::{error, info, debug};

pub struct ItemService<'a> {
    pub conn: &'a mut SqliteConnection,
}

impl<'a> ItemService<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }

    pub fn search_items(&mut self, filters: ItemFilters) -> Result<Vec<ItemSummary>> {
        use crate::schema::catalog_items::dsl::*;
        
        let mut query = catalog_items.into_boxed();
        
        // Filter by name
        if let Some(search_name) = &filters.name {
            if !search_name.is_empty() {
                query = query.filter(name.like(format!("%{}%", search_name)));
            }
        }
        
        // Filter by sources
        if let Some(sources) = &filters.sources {
            if !sources.is_empty() {
                query = query.filter(source.eq_any(sources));
            }
        }
        
        // Filter by item types
        if let Some(types) = &filters.item_types {
            if !types.is_empty() {
                query = query.filter(item_type.eq_any(types));
            }
        }
        
        // Filter by rarities
        if let Some(rarities) = &filters.rarities {
            if !rarities.is_empty() {
                query = query.filter(rarity.eq_any(rarities));
            }
        }
        
        // Filter by value range
        if let Some(min_val) = filters.min_value {
            query = query.filter(value.ge(min_val));
        }
        if let Some(max_val) = filters.max_value {
            query = query.filter(value.le(max_val));
        }
        
        let items = query
            .order(name.asc())
            .load::<CatalogItem>(self.conn)?;
            
        Ok(items.iter().map(ItemSummary::from).collect())
    }

    pub fn get_item_by_id(&mut self, item_id: i32) -> Result<Option<Item>> {
        use crate::schema::catalog_items::dsl::*;
        
        let catalog_item = catalog_items
            .find(item_id)
            .first::<CatalogItem>(self.conn)
            .optional()?;
            
        match catalog_item {
            Some(item) => {
                let parsed_item = serde_json::from_str(&item.full_item_json)?;
                Ok(Some(parsed_item))
            }
            None => Ok(None),
        }
    }

    pub fn get_item_by_name_and_source(&mut self, item_name: &str, item_source: &str) -> Result<Option<Item>> {
        use crate::schema::catalog_items::dsl::*;

        let catalog_item = catalog_items
            .filter(name.eq(item_name))
            .filter(source.eq(item_source))
            .first::<CatalogItem>(self.conn)
            .optional()?;

        match catalog_item {
            Some(item) => {
                let parsed_item = serde_json::from_str(&item.full_item_json)?;
                Ok(Some(parsed_item))
            }
            None => Ok(None),
        }
    }

    pub fn get_item_types(&mut self) -> Result<Vec<String>> {
        use crate::schema::catalog_items::dsl::*;
        
        let types: Vec<Option<String>> = catalog_items
            .select(item_type)
            .distinct()
            .load(self.conn)?;
        
        // Filter out None values and collect into Vec<String>
        let mut result: Vec<String> = types.into_iter().flatten().collect();
        result.sort();
        Ok(result)
    }

    pub fn get_item_rarities(&mut self) -> Result<Vec<String>> {
        use crate::schema::catalog_items::dsl::*;
        
        let rarities: Vec<Option<String>> = catalog_items
            .select(rarity)
            .distinct()
            .load(self.conn)?;
        
        // Filter out None values and collect into Vec<String>
        let mut result: Vec<String> = rarities.into_iter().flatten().collect();
        result.sort();
        Ok(result)
    }

    pub fn get_item_sources(&mut self) -> Result<Vec<String>> {
        use crate::schema::catalog_items::dsl::*;

        let mut sources: Vec<String> = catalog_items
            .select(source)
            .distinct()
            .load(self.conn)?;
        sources.sort();
        Ok(sources)
    }

    /// Import all item data from an uploaded book directory
    pub fn import_items_from_book(
        conn: &mut SqliteConnection,
        book_dir: &Path,
        source: &str
    ) -> Result<usize> {
        info!("Importing items from book directory: {:?} (source: {})", book_dir, source);

        let mut total_imported = 0;
        let item_files = Self::find_item_files(book_dir)?;

        if item_files.is_empty() {
            info!("No item files found in book directory");
            return Ok(0);
        }

        info!("Found {} item files to process", item_files.len());

        for item_file in item_files {
            debug!("Processing item file: {:?}", item_file);

            match Self::import_items_from_file(conn, &item_file, source) {
                Ok(count) => {
                    info!("Imported {} items from {:?}", count, item_file);
                    total_imported += count;
                }
                Err(e) => {
                    error!("Failed to import items from {:?}: {}", item_file, e);
                    // Continue processing other files instead of failing completely
                }
            }
        }

        info!("Total items imported: {}", total_imported);
        Ok(total_imported)
    }

    /// Find item files in a book directory (reusing existing logic from catalog.rs)
    fn find_item_files(book_dir: &Path) -> Result<Vec<std::path::PathBuf>> {
        let mut files = Vec::new();

        // Check the items directory
        let items_dir = book_dir.join("items");
        if items_dir.exists() && items_dir.is_dir() {
            let entries = fs::read_dir(&items_dir)?;

            for entry in entries {
                let entry = entry?;
                let path = entry.path();

                if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("json") {
                    let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

                    // Skip fluff files, index files, and foundry files
                    if filename.starts_with("fluff-") || filename == "index.json" || filename == "foundry.json" {
                        continue;
                    }

                    debug!("Found item file: {:?}", path);
                    files.push(path);
                }
            }
        }

        Ok(files)
    }

    /// Import items from a single JSON file
    fn import_items_from_file(
        conn: &mut SqliteConnection,
        file_path: &Path,
        source: &str
    ) -> Result<usize> {
        debug!("Reading items from file: {:?}", file_path);

        let content = fs::read_to_string(file_path)?;

        let data: ItemData = serde_json::from_str(&content)?;

        if !data.item.is_empty() {
            let new_items: Vec<NewCatalogItem> = data.item.iter().map(|item| {
                let mut new_item = NewCatalogItem::from(item);
                // Always override the source with the book source to ensure consistency
                new_item.source = source.to_string();

                // Also update the source in the full_item_json to maintain consistency
                if let Ok(mut item_json) = serde_json::from_str::<serde_json::Value>(&new_item.full_item_json) {
                    if let Some(obj) = item_json.as_object_mut() {
                        obj.insert("source".to_string(), serde_json::Value::String(source.to_string()));
                        if let Ok(updated_json) = serde_json::to_string(&item_json) {
                            new_item.full_item_json = updated_json;
                        }
                    }
                }

                new_item
            }).collect();

            debug!("Inserting {} items individually (SQLite limitation)", new_items.len());

            for item in &new_items {
                diesel::insert_into(catalog_items::table)
                    .values(item)
                    .on_conflict((catalog_items::name, catalog_items::source))
                    .do_nothing()
                    .execute(conn)?;
            }

            info!("Successfully imported {} items into database", new_items.len());
            Ok(new_items.len())
        } else {
            Ok(0)
        }
    }

    /// Remove all items from a specific source
    pub fn remove_items_from_source(
        conn: &mut SqliteConnection,
        source: &str
    ) -> Result<usize> {
        info!("Removing items from source: {}", source);

        let deleted = diesel::delete(catalog_items::table.filter(catalog_items::source.eq(source)))
            .execute(conn)?;

        info!("Removed {} items from source: {}", deleted, source);
        Ok(deleted)
    }
}