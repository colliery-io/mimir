//! Data Access Layer for items

use crate::{
    connection::DbConnection,
    dal::traits::{AsyncRepository, Repository},
    error::{DbError, Result},
    models::rules::items::{Item, NewItem},
    schema::items,
};
use async_trait::async_trait;
use diesel::prelude::*;
use tokio::task;

/// Repository for Item operations
pub struct ItemRepository {
    database_url: String,
}

impl ItemRepository {
    /// Create a new ItemRepository
    pub fn new(database_url: String) -> Self {
        Self { database_url }
    }

    /// Execute a closure with a database connection
    async fn with_connection<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&mut DbConnection) -> Result<R> + Send + 'static,
        R: Send + 'static,
    {
        let db_url = self.database_url.clone();
        task::spawn_blocking(move || {
            let mut conn = crate::connection::establish_connection(&db_url)?;
            f(&mut conn)
        })
        .await
        .map_err(|e| DbError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?
    }

    /// Execute a closure within a transaction
    async fn with_transaction<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&mut DbConnection) -> Result<R> + Send + 'static,
        R: Send + 'static,
    {
        self.with_connection(|conn| conn.transaction(|conn| f(conn))).await
    }
}

/// Synchronous repository implementation for DbConnection
impl Repository<Item> for DbConnection {
    fn create(&mut self, entity: Item) -> Result<Item> {
        // Convert Item to NewItem for insertion (excluding generated columns)
        let new_item = NewItem {
            id: entity.id,
            name: entity.name,
            rule_system_id: entity.rule_system_id,
            source_id: entity.source_id,
            page: entity.page,
            base_item_id: entity.base_item_id,
            item_type: entity.item_type,
            weight_lb: entity.weight_lb,
            value_cp: entity.value_cp,
            armor_class: entity.armor_class,
            damage: entity.damage,
            properties: entity.properties,
            rarity: entity.rarity,
            requires_attunement: entity.requires_attunement,
            attunement_prereq: entity.attunement_prereq,
            magic_bonus: entity.magic_bonus,
            additional_properties: entity.additional_properties,
            entries: entity.entries,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        };
        
        diesel::insert_into(items::table)
            .values(&new_item)
            .get_result(self)
            .map_err(DbError::from)
    }

    fn find_by_id(&mut self, id: &str) -> Result<Option<Item>> {
        items::table
            .find(id)
            .first(self)
            .optional()
            .map_err(DbError::from)
    }

    fn update(&mut self, id: &str, entity: Item) -> Result<Item> {
        // Convert Item to NewItem for update (excluding generated columns)
        let new_item = NewItem {
            id: entity.id,
            name: entity.name,
            rule_system_id: entity.rule_system_id,
            source_id: entity.source_id,
            page: entity.page,
            base_item_id: entity.base_item_id,
            item_type: entity.item_type,
            weight_lb: entity.weight_lb,
            value_cp: entity.value_cp,
            armor_class: entity.armor_class,
            damage: entity.damage,
            properties: entity.properties,
            rarity: entity.rarity,
            requires_attunement: entity.requires_attunement,
            attunement_prereq: entity.attunement_prereq,
            magic_bonus: entity.magic_bonus,
            additional_properties: entity.additional_properties,
            entries: entity.entries,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        };
        
        diesel::update(items::table.find(id))
            .set(&new_item)
            .get_result(self)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => DbError::NotFound {
                    entity_type: "Item".to_string(),
                    id: id.to_string(),
                },
                _ => DbError::from(e),
            })
    }

    fn delete(&mut self, id: &str) -> Result<()> {
        let rows_affected = diesel::delete(items::table.find(id))
            .execute(self)
            .map_err(DbError::from)?;

        if rows_affected == 0 {
            return Err(DbError::NotFound {
                entity_type: "Item".to_string(),
                id: id.to_string(),
            });
        }

        Ok(())
    }

    fn list(&mut self) -> Result<Vec<Item>> {
        items::table
            .select(Item::as_select())
            .load(self)
            .map_err(DbError::from)
    }
}

/// Asynchronous repository implementation
#[async_trait]
impl AsyncRepository<Item> for ItemRepository {
    async fn create(&self, entity: Item) -> Result<Item> {
        self.with_transaction(move |conn| conn.create(entity)).await
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Item>> {
        let id = id.to_string();
        self.with_connection(move |conn| conn.find_by_id(&id)).await
    }

    async fn update(&self, id: &str, entity: Item) -> Result<Item> {
        let id = id.to_string();
        self.with_transaction(move |conn| conn.update(&id, entity)).await
    }

    async fn delete(&self, id: &str) -> Result<()> {
        let id = id.to_string();
        self.with_transaction(move |conn| Repository::<Item>::delete(conn, &id)).await
    }

    async fn list(&self) -> Result<Vec<Item>> {
        self.with_connection(|conn| conn.list()).await
    }
}

/// Item-specific queries
impl ItemRepository {
    /// Find all items for a specific rule system
    pub async fn find_by_rule_system(&self, rule_system_id: &str) -> Result<Vec<Item>> {
        let rule_system_id = rule_system_id.to_string();
        self.with_connection(move |conn| {
            items::table
                .filter(items::rule_system_id.eq(&rule_system_id))
                .select(Item::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }

    /// Find all items from a specific source
    pub async fn find_by_source(&self, source_id: &str) -> Result<Vec<Item>> {
        let source_id = source_id.to_string();
        self.with_connection(move |conn| {
            items::table
                .filter(items::source_id.eq(&source_id))
                .select(Item::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }

    /// Find all magic items
    pub async fn find_magic_items(&self) -> Result<Vec<Item>> {
        self.with_connection(|conn| {
            items::table
                .filter(items::is_magic.eq(true))
                .select(Item::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }

    /// Find all mundane items
    pub async fn find_mundane_items(&self) -> Result<Vec<Item>> {
        self.with_connection(|conn| {
            items::table
                .filter(items::is_magic.eq(false))
                .select(Item::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }

    /// Find items by type
    pub async fn find_by_type(&self, item_type: &str) -> Result<Vec<Item>> {
        let item_type = item_type.to_string();
        self.with_connection(move |conn| {
            items::table
                .filter(items::item_type.eq(&item_type))
                .select(Item::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }

    /// Find items by rarity
    pub async fn find_by_rarity(&self, rarity: &str) -> Result<Vec<Item>> {
        let rarity = rarity.to_string();
        self.with_connection(move |conn| {
            items::table
                .filter(items::rarity.eq(&rarity))
                .select(Item::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }

    /// Find items that require attunement
    pub async fn find_attunement_items(&self) -> Result<Vec<Item>> {
        self.with_connection(|conn| {
            items::table
                .filter(items::requires_attunement.eq(true))
                .select(Item::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }

    /// Find weapons
    pub async fn find_weapons(&self) -> Result<Vec<Item>> {
        self.with_connection(|conn| {
            items::table
                .filter(items::item_type.like("%weapon%"))
                .select(Item::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }

    /// Find armor
    pub async fn find_armor(&self) -> Result<Vec<Item>> {
        self.with_connection(|conn| {
            items::table
                .filter(items::item_type.like("%armor%").or(items::item_type.eq("shield")))
                .select(Item::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }

    /// Find base items (not variants)
    pub async fn find_base_items(&self) -> Result<Vec<Item>> {
        self.with_connection(|conn| {
            items::table
                .filter(items::base_item_id.is_null())
                .select(Item::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }

    /// Find variants of a specific base item
    pub async fn find_variants(&self, base_item_id: &str) -> Result<Vec<Item>> {
        let base_item_id = base_item_id.to_string();
        self.with_connection(move |conn| {
            items::table
                .filter(items::base_item_id.eq(&base_item_id))
                .select(Item::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }

    /// Find items with magic bonus
    pub async fn find_with_magic_bonus(&self, bonus: i32) -> Result<Vec<Item>> {
        self.with_connection(move |conn| {
            items::table
                .filter(items::magic_bonus.eq(bonus))
                .select(Item::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }

    /// Find items by value range (in copper pieces)
    pub async fn find_by_value_range(&self, min_cp: i32, max_cp: i32) -> Result<Vec<Item>> {
        self.with_connection(move |conn| {
            items::table
                .filter(items::value_cp.between(min_cp, max_cp))
                .select(Item::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }

    /// Find items by weight range (in pounds)
    pub async fn find_by_weight_range(&self, min_lb: f32, max_lb: f32) -> Result<Vec<Item>> {
        self.with_connection(move |conn| {
            items::table
                .filter(items::weight_lb.between(min_lb, max_lb))
                .select(Item::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }
}