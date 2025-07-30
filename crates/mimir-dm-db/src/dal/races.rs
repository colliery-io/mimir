//! Data Access Layer for races

use crate::{
    connection::DbConnection,
    dal::traits::{AsyncRepository, Repository},
    error::{DbError, Result},
    models::races::Race,
    schema::races,
};
use async_trait::async_trait;
use diesel::prelude::*;
use tokio::task;

/// Repository for Race operations
pub struct RaceRepository {
    database_url: String,
}

impl RaceRepository {
    /// Create a new RaceRepository
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
impl Repository<Race> for DbConnection {
    fn create(&mut self, entity: Race) -> Result<Race> {
        diesel::insert_into(races::table)
            .values(&entity)
            .get_result(self)
            .map_err(DbError::from)
    }

    fn find_by_id(&mut self, id: &str) -> Result<Option<Race>> {
        races::table
            .find(id)
            .first(self)
            .optional()
            .map_err(DbError::from)
    }

    fn update(&mut self, id: &str, entity: Race) -> Result<Race> {
        diesel::update(races::table.find(id))
            .set(&entity)
            .get_result(self)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => DbError::NotFound {
                    entity_type: "Race".to_string(),
                    id: id.to_string(),
                },
                _ => DbError::from(e),
            })
    }

    fn delete(&mut self, id: &str) -> Result<()> {
        let rows_affected = diesel::delete(races::table.find(id))
            .execute(self)
            .map_err(DbError::from)?;

        if rows_affected == 0 {
            return Err(DbError::NotFound {
                entity_type: "Race".to_string(),
                id: id.to_string(),
            });
        }

        Ok(())
    }

    fn list(&mut self) -> Result<Vec<Race>> {
        races::table
            .select(Race::as_select())
            .load(self)
            .map_err(DbError::from)
    }
}

/// Asynchronous repository implementation
#[async_trait]
impl AsyncRepository<Race> for RaceRepository {
    async fn create(&self, entity: Race) -> Result<Race> {
        self.with_transaction(move |conn| conn.create(entity)).await
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Race>> {
        let id = id.to_string();
        self.with_connection(move |conn| conn.find_by_id(&id)).await
    }

    async fn update(&self, id: &str, entity: Race) -> Result<Race> {
        let id = id.to_string();
        self.with_transaction(move |conn| conn.update(&id, entity)).await
    }

    async fn delete(&self, id: &str) -> Result<()> {
        let id = id.to_string();
        self.with_transaction(move |conn| Repository::<Race>::delete(conn, &id)).await
    }

    async fn list(&self) -> Result<Vec<Race>> {
        self.with_connection(|conn| conn.list()).await
    }
}

/// Race-specific queries
impl RaceRepository {
    /// Find all races for a specific rule system
    pub async fn find_by_rule_system(&self, rule_system_id: &str) -> Result<Vec<Race>> {
        let rule_system_id = rule_system_id.to_string();
        self.with_connection(move |conn| {
            races::table
                .filter(races::rule_system_id.eq(&rule_system_id))
                .select(Race::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }

    /// Find all races from a specific source
    pub async fn find_by_source(&self, source_id: &str) -> Result<Vec<Race>> {
        let source_id = source_id.to_string();
        self.with_connection(move |conn| {
            races::table
                .filter(races::source_id.eq(&source_id))
                .select(Race::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }

    /// Find all base races (not subraces)
    pub async fn find_base_races(&self) -> Result<Vec<Race>> {
        self.with_connection(|conn| {
            races::table
                .filter(races::race_type.eq("race"))
                .select(Race::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }

    /// Find all subraces for a specific base race
    pub async fn find_subraces(&self, parent_race_id: &str) -> Result<Vec<Race>> {
        let parent_race_id = parent_race_id.to_string();
        self.with_connection(move |conn| {
            races::table
                .filter(races::parent_race_id.eq(&parent_race_id))
                .select(Race::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }

    /// Find all races by type (race or subrace)
    pub async fn find_by_type(&self, race_type: &str) -> Result<Vec<Race>> {
        let race_type = race_type.to_string();
        self.with_connection(move |conn| {
            races::table
                .filter(races::race_type.eq(&race_type))
                .select(Race::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }

    /// Find races by size
    pub async fn find_by_size(&self, size: &str) -> Result<Vec<Race>> {
        let size = size.to_string();
        self.with_connection(move |conn| {
            races::table
                .filter(races::size.eq(&size))
                .select(Race::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }
}