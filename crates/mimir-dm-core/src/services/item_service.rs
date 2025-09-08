use diesel::prelude::*;
use crate::models::catalog::item::{
    CatalogItem, ItemSummary, ItemFilters, Item
};

pub struct ItemService<'a> {
    pub conn: &'a mut SqliteConnection,
}

impl<'a> ItemService<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }

    pub fn search_items(&mut self, filters: ItemFilters) -> QueryResult<Vec<ItemSummary>> {
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

    pub fn get_item_by_id(&mut self, item_id: i32) -> QueryResult<Option<Item>> {
        use crate::schema::catalog_items::dsl::*;
        
        let catalog_item = catalog_items
            .find(item_id)
            .first::<CatalogItem>(self.conn)
            .optional()?;
            
        if let Some(item) = catalog_item {
            let parsed_item: Result<Item, _> = serde_json::from_str(&item.full_item_json);
            match parsed_item {
                Ok(item) => Ok(Some(item)),
                Err(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }

    pub fn get_item_by_name_and_source(&mut self, item_name: &str, item_source: &str) -> QueryResult<Option<Item>> {
        use crate::schema::catalog_items::dsl::*;
        
        let catalog_item = catalog_items
            .filter(name.eq(item_name))
            .filter(source.eq(item_source))
            .first::<CatalogItem>(self.conn)
            .optional()?;
            
        if let Some(item) = catalog_item {
            let parsed_item: Result<Item, _> = serde_json::from_str(&item.full_item_json);
            match parsed_item {
                Ok(item) => Ok(Some(item)),
                Err(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }

    pub fn get_item_types(&mut self) -> QueryResult<Vec<String>> {
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

    pub fn get_item_rarities(&mut self) -> QueryResult<Vec<String>> {
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

    pub fn get_item_sources(&mut self) -> QueryResult<Vec<String>> {
        use crate::schema::catalog_items::dsl::*;
        
        let mut sources: Vec<String> = catalog_items
            .select(source)
            .distinct()
            .load(self.conn)?;
        sources.sort();
        Ok(sources)
    }
}