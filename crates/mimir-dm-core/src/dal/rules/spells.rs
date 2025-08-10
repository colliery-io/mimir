//! Data Access Layer for spells

use crate::{
    connection::DbConnection,
    dal::traits::{AsyncRepository, Repository},
    error::{DbError, Result},
    models::rules::spells::Spell,
    schema::spells,
};
use async_trait::async_trait;
use diesel::prelude::*;
use tokio::task;

/// Repository for Spell operations
pub struct SpellRepository {
    database_url: String,
}

impl SpellRepository {
    /// Create a new SpellRepository
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
impl Repository<Spell> for DbConnection {
    fn create(&mut self, entity: Spell) -> Result<Spell> {
        diesel::insert_into(spells::table)
            .values(&entity)
            .get_result(self)
            .map_err(DbError::from)
    }

    fn find_by_id(&mut self, id: &str) -> Result<Option<Spell>> {
        spells::table
            .find(id)
            .first(self)
            .optional()
            .map_err(DbError::from)
    }

    fn update(&mut self, _id: &str, _entity: Spell) -> Result<Spell> {
        unimplemented!("Update not implemented for spells")
    }

    fn delete(&mut self, _id: &str) -> Result<()> {
        unimplemented!("Delete not implemented for spells")
    }

    fn list(&mut self) -> Result<Vec<Spell>> {
        spells::table
            .select(Spell::as_select())
            .load(self)
            .map_err(DbError::from)
    }
}

/// Asynchronous repository implementation
#[async_trait]
impl AsyncRepository<Spell> for SpellRepository {
    async fn create(&self, entity: Spell) -> Result<Spell> {
        self.with_transaction(move |conn| conn.create(entity)).await
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Spell>> {
        let id = id.to_string();
        self.with_connection(move |conn| conn.find_by_id(&id)).await
    }

    async fn update(&self, _id: &str, _entity: Spell) -> Result<Spell> {
        unimplemented!("Update not implemented for spells")
    }

    async fn delete(&self, _id: &str) -> Result<()> {
        unimplemented!("Delete not implemented for spells")
    }

    async fn list(&self) -> Result<Vec<Spell>> {
        self.with_connection(|conn| conn.list()).await
    }
}