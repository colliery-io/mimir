//! Data Access Layer for backgrounds

use crate::{
    connection::DbConnection,
    dal::traits::{AsyncRepository, Repository},
    error::{DbError, Result},
    models::backgrounds::Background,
    schema::backgrounds,
};
use async_trait::async_trait;
use diesel::prelude::*;
use tokio::task;

/// Repository for Background operations
pub struct BackgroundRepository {
    database_url: String,
}

impl BackgroundRepository {
    /// Create a new BackgroundRepository
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
impl Repository<Background> for DbConnection {
    fn create(&mut self, entity: Background) -> Result<Background> {
        diesel::insert_into(backgrounds::table)
            .values(&entity)
            .get_result(self)
            .map_err(DbError::from)
    }

    fn find_by_id(&mut self, id: &str) -> Result<Option<Background>> {
        backgrounds::table
            .find(id)
            .first(self)
            .optional()
            .map_err(DbError::from)
    }

    fn update(&mut self, id: &str, entity: Background) -> Result<Background> {
        diesel::update(backgrounds::table.find(id))
            .set(&entity)
            .get_result(self)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => DbError::NotFound {
                    entity_type: "Background".to_string(),
                    id: id.to_string(),
                },
                _ => DbError::from(e),
            })
    }

    fn delete(&mut self, id: &str) -> Result<()> {
        let rows_affected = diesel::delete(backgrounds::table.find(id))
            .execute(self)
            .map_err(DbError::from)?;

        if rows_affected == 0 {
            return Err(DbError::NotFound {
                entity_type: "Background".to_string(),
                id: id.to_string(),
            });
        }

        Ok(())
    }

    fn list(&mut self) -> Result<Vec<Background>> {
        backgrounds::table
            .select(Background::as_select())
            .load(self)
            .map_err(DbError::from)
    }
}

/// Asynchronous repository implementation
#[async_trait]
impl AsyncRepository<Background> for BackgroundRepository {
    async fn create(&self, entity: Background) -> Result<Background> {
        self.with_transaction(move |conn| conn.create(entity)).await
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Background>> {
        let id = id.to_string();
        self.with_connection(move |conn| conn.find_by_id(&id)).await
    }

    async fn update(&self, id: &str, entity: Background) -> Result<Background> {
        let id = id.to_string();
        self.with_transaction(move |conn| conn.update(&id, entity)).await
    }

    async fn delete(&self, id: &str) -> Result<()> {
        let id = id.to_string();
        self.with_transaction(move |conn| Repository::<Background>::delete(conn, &id)).await
    }

    async fn list(&self) -> Result<Vec<Background>> {
        self.with_connection(|conn| conn.list()).await
    }
}

/// Background-specific queries
impl BackgroundRepository {
    /// Find all backgrounds for a specific rule system
    pub async fn find_by_rule_system(&self, rule_system_id: &str) -> Result<Vec<Background>> {
        let rule_system_id = rule_system_id.to_string();
        self.with_connection(move |conn| {
            backgrounds::table
                .filter(backgrounds::rule_system_id.eq(&rule_system_id))
                .select(Background::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }

    /// Find all backgrounds from a specific source
    pub async fn find_by_source(&self, source_id: &str) -> Result<Vec<Background>> {
        let source_id = source_id.to_string();
        self.with_connection(move |conn| {
            backgrounds::table
                .filter(backgrounds::source_id.eq(&source_id))
                .select(Background::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }

    /// Find backgrounds that grant skill proficiencies
    pub async fn find_with_skill_proficiencies(&self) -> Result<Vec<Background>> {
        self.with_connection(|conn| {
            backgrounds::table
                .filter(backgrounds::skill_proficiencies.is_not_null())
                .select(Background::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }

    /// Find backgrounds that grant language proficiencies
    pub async fn find_with_language_proficiencies(&self) -> Result<Vec<Background>> {
        self.with_connection(|conn| {
            backgrounds::table
                .filter(backgrounds::language_proficiencies.is_not_null())
                .select(Background::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }

    /// Find backgrounds that grant tool proficiencies
    pub async fn find_with_tool_proficiencies(&self) -> Result<Vec<Background>> {
        self.with_connection(|conn| {
            backgrounds::table
                .filter(backgrounds::tool_proficiencies.is_not_null())
                .select(Background::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }

    /// Find backgrounds that provide starting equipment
    pub async fn find_with_starting_equipment(&self) -> Result<Vec<Background>> {
        self.with_connection(|conn| {
            backgrounds::table
                .filter(backgrounds::starting_equipment.is_not_null())
                .select(Background::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }

    /// Find backgrounds with a specific feature name
    pub async fn find_by_feature_name(&self, feature_name: &str) -> Result<Vec<Background>> {
        let feature_name = feature_name.to_string();
        self.with_connection(move |conn| {
            backgrounds::table
                .filter(backgrounds::feature_name.eq(&feature_name))
                .select(Background::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }

    /// Search backgrounds by feature text (case-insensitive)
    pub async fn search_by_feature_text(&self, search_term: &str) -> Result<Vec<Background>> {
        let search_pattern = format!("%{}%", search_term);
        self.with_connection(move |conn| {
            backgrounds::table
                .filter(backgrounds::feature_text.like(&search_pattern))
                .select(Background::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }

    /// Search backgrounds by name (case-insensitive)
    pub async fn search_by_name(&self, search_term: &str) -> Result<Vec<Background>> {
        let search_pattern = format!("%{}%", search_term);
        self.with_connection(move |conn| {
            backgrounds::table
                .filter(backgrounds::name.like(&search_pattern))
                .select(Background::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }
}