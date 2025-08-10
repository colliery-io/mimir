//! Rule systems repository implementation

use diesel::prelude::*;
use async_trait::async_trait;

use crate::connection::{with_connection, with_transaction, DbConnection};
use crate::dal::traits::{AsyncRepository, Repository};
use crate::error::{DbError, Result};
use crate::models::rules::rule_systems::RuleSystem;
use crate::schema::rule_systems;

/// Repository for rule system operations
pub struct RuleSystemRepository {
    database_url: String,
}

impl RuleSystemRepository {
    /// Create a new repository instance
    pub fn new(database_url: String) -> Self {
        Self { database_url }
    }
}

impl Repository<RuleSystem> for DbConnection {
    fn create(&mut self, entity: RuleSystem) -> Result<RuleSystem> {
        diesel::insert_into(rule_systems::table)
            .values(&entity)
            .execute(self)?;
        
        self.find_by_id(&entity.id)?
            .ok_or_else(|| DbError::NotFound {
                entity_type: "RuleSystem".to_string(),
                id: entity.id.clone(),
            })
    }
    
    fn find_by_id(&mut self, id: &str) -> Result<Option<RuleSystem>> {
        rule_systems::table
            .find(id)
            .first::<RuleSystem>(self)
            .optional()
            .map_err(Into::into)
    }
    
    fn update(&mut self, id: &str, entity: RuleSystem) -> Result<RuleSystem> {
        diesel::update(rule_systems::table.find(id))
            .set(&entity)
            .execute(self)?;
        
        self.find_by_id(id)?
            .ok_or_else(|| DbError::NotFound {
                entity_type: "RuleSystem".to_string(),
                id: id.to_string(),
            })
    }
    
    fn delete(&mut self, id: &str) -> Result<()> {
        let rows_affected = diesel::delete(rule_systems::table.find(id))
            .execute(self)?;
        
        if rows_affected == 0 {
            return Err(DbError::NotFound {
                entity_type: "RuleSystem".to_string(),
                id: id.to_string(),
            });
        }
        
        Ok(())
    }
    
    fn list(&mut self) -> Result<Vec<RuleSystem>> {
        rule_systems::table
            .load::<RuleSystem>(self)
            .map_err(Into::into)
    }
}

#[async_trait]
impl AsyncRepository<RuleSystem> for RuleSystemRepository {
    async fn create(&self, entity: RuleSystem) -> Result<RuleSystem> {
        let db_url = self.database_url.clone();
        with_transaction(db_url, move |conn| conn.create(entity)).await
    }
    
    async fn find_by_id(&self, id: &str) -> Result<Option<RuleSystem>> {
        let id = id.to_string();
        let db_url = self.database_url.clone();
        with_connection(db_url, move |conn| conn.find_by_id(&id)).await
    }
    
    async fn update(&self, id: &str, entity: RuleSystem) -> Result<RuleSystem> {
        let id = id.to_string();
        let db_url = self.database_url.clone();
        with_transaction(db_url, move |conn| conn.update(&id, entity)).await
    }
    
    async fn delete(&self, id: &str) -> Result<()> {
        let id = id.to_string();
        let db_url = self.database_url.clone();
        with_transaction(db_url, move |conn| Repository::<RuleSystem>::delete(conn, &id)).await
    }
    
    async fn list(&self) -> Result<Vec<RuleSystem>> {
        let db_url = self.database_url.clone();
        with_connection(db_url, |conn| conn.list()).await
    }
}

