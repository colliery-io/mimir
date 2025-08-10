//! Data Access Layer for creatures

use crate::{
    connection::DbConnection,
    dal::traits::{AsyncRepository, Repository},
    error::{DbError, Result},
    models::rules::creatures::Creature,
    schema::creatures,
};
use async_trait::async_trait;
use diesel::prelude::*;
use tokio::task;

/// Repository for Creature operations
pub struct CreatureRepository {
    database_url: String,
}

impl CreatureRepository {
    /// Create a new CreatureRepository
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
impl Repository<Creature> for DbConnection {
    fn create(&mut self, entity: Creature) -> Result<Creature> {
        diesel::insert_into(creatures::table)
            .values(&entity)
            .get_result(self)
            .map_err(DbError::from)
    }

    fn find_by_id(&mut self, id: &str) -> Result<Option<Creature>> {
        creatures::table
            .find(id)
            .first(self)
            .optional()
            .map_err(DbError::from)
    }

    fn update(&mut self, _id: &str, _entity: Creature) -> Result<Creature> {
        unimplemented!("Update not implemented for creatures")
    }

    fn delete(&mut self, _id: &str) -> Result<()> {
        unimplemented!("Delete not implemented for creatures")
    }

    fn list(&mut self) -> Result<Vec<Creature>> {
        creatures::table
            .select(Creature::as_select())
            .load(self)
            .map_err(DbError::from)
    }
}

/// Asynchronous repository implementation
#[async_trait]
impl AsyncRepository<Creature> for CreatureRepository {
    async fn create(&self, entity: Creature) -> Result<Creature> {
        self.with_transaction(move |conn| conn.create(entity)).await
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Creature>> {
        let id = id.to_string();
        self.with_connection(move |conn| conn.find_by_id(&id)).await
    }

    async fn update(&self, _id: &str, _entity: Creature) -> Result<Creature> {
        unimplemented!("Update not implemented for creatures")
    }

    async fn delete(&self, _id: &str) -> Result<()> {
        unimplemented!("Delete not implemented for creatures")
    }

    async fn list(&self) -> Result<Vec<Creature>> {
        self.with_connection(|conn| conn.list()).await
    }
}