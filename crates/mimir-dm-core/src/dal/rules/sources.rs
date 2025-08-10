//! Data Access Layer for sources

use crate::{
    connection::DbConnection,
    dal::traits::{AsyncRepository, Repository},
    error::{DbError, Result},
    models::rules::sources::Source,
    schema::sources,
};
use async_trait::async_trait;
use diesel::prelude::*;
use tokio::task;

/// Repository for Source operations
pub struct SourceRepository {
    database_url: String,
}

impl SourceRepository {
    /// Create a new SourceRepository
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
impl Repository<Source> for DbConnection {
    fn create(&mut self, entity: Source) -> Result<Source> {
        diesel::insert_into(sources::table)
            .values(&entity)
            .get_result(self)
            .map_err(DbError::from)
    }

    fn find_by_id(&mut self, id: &str) -> Result<Option<Source>> {
        sources::table
            .find(id)
            .first(self)
            .optional()
            .map_err(DbError::from)
    }

    fn update(&mut self, id: &str, entity: Source) -> Result<Source> {
        diesel::update(sources::table.find(id))
            .set(&entity)
            .get_result(self)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => DbError::NotFound {
                    entity_type: "Source".to_string(),
                    id: id.to_string(),
                },
                _ => DbError::from(e),
            })
    }

    fn delete(&mut self, id: &str) -> Result<()> {
        let rows_affected = diesel::delete(sources::table.find(id))
            .execute(self)
            .map_err(DbError::from)?;

        if rows_affected == 0 {
            return Err(DbError::NotFound {
                entity_type: "Source".to_string(),
                id: id.to_string(),
            });
        }

        Ok(())
    }

    fn list(&mut self) -> Result<Vec<Source>> {
        sources::table
            .select(Source::as_select())
            .load(self)
            .map_err(DbError::from)
    }
}

/// Asynchronous repository implementation
#[async_trait]
impl AsyncRepository<Source> for SourceRepository {
    async fn create(&self, entity: Source) -> Result<Source> {
        self.with_transaction(move |conn| conn.create(entity)).await
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Source>> {
        let id = id.to_string();
        self.with_connection(move |conn| conn.find_by_id(&id)).await
    }

    async fn update(&self, id: &str, entity: Source) -> Result<Source> {
        let id = id.to_string();
        self.with_transaction(move |conn| conn.update(&id, entity)).await
    }

    async fn delete(&self, id: &str) -> Result<()> {
        let id = id.to_string();
        self.with_transaction(move |conn| Repository::<Source>::delete(conn, &id)).await
    }

    async fn list(&self) -> Result<Vec<Source>> {
        self.with_connection(|conn| conn.list()).await
    }
}

/// Source-specific queries
impl SourceRepository {
    /// Find all sources for a specific rule system
    pub async fn find_by_rule_system(&self, rule_system_id: &str) -> Result<Vec<Source>> {
        let rule_system_id = rule_system_id.to_string();
        self.with_connection(move |conn| {
            sources::table
                .filter(sources::rule_system_id.eq(&rule_system_id))
                .select(Source::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }

    /// Find all official sources
    pub async fn find_official(&self) -> Result<Vec<Source>> {
        self.with_connection(|conn| {
            sources::table
                .filter(sources::is_official.eq(true))
                .select(Source::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }

    /// Find all SRD sources
    pub async fn find_srd(&self) -> Result<Vec<Source>> {
        self.with_connection(|conn| {
            sources::table
                .filter(sources::is_srd.eq(true))
                .select(Source::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }
}