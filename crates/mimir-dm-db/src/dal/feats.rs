//! Data Access Layer for feats

use crate::{
    connection::DbConnection,
    dal::traits::{AsyncRepository, Repository},
    error::{DbError, Result},
    models::feats::Feat,
    schema::feats,
};
use async_trait::async_trait;
use diesel::prelude::*;
use tokio::task;

/// Repository for Feat operations
pub struct FeatRepository {
    database_url: String,
}

impl FeatRepository {
    /// Create a new FeatRepository
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
impl Repository<Feat> for DbConnection {
    fn create(&mut self, entity: Feat) -> Result<Feat> {
        diesel::insert_into(feats::table)
            .values(&entity)
            .get_result(self)
            .map_err(DbError::from)
    }

    fn find_by_id(&mut self, id: &str) -> Result<Option<Feat>> {
        feats::table
            .find(id)
            .first(self)
            .optional()
            .map_err(DbError::from)
    }

    fn update(&mut self, id: &str, entity: Feat) -> Result<Feat> {
        diesel::update(feats::table.find(id))
            .set(&entity)
            .get_result(self)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => DbError::NotFound {
                    entity_type: "Feat".to_string(),
                    id: id.to_string(),
                },
                _ => DbError::from(e),
            })
    }

    fn delete(&mut self, id: &str) -> Result<()> {
        let rows_affected = diesel::delete(feats::table.find(id))
            .execute(self)
            .map_err(DbError::from)?;

        if rows_affected == 0 {
            return Err(DbError::NotFound {
                entity_type: "Feat".to_string(),
                id: id.to_string(),
            });
        }

        Ok(())
    }

    fn list(&mut self) -> Result<Vec<Feat>> {
        feats::table
            .select(Feat::as_select())
            .load(self)
            .map_err(DbError::from)
    }
}

/// Asynchronous repository implementation
#[async_trait]
impl AsyncRepository<Feat> for FeatRepository {
    async fn create(&self, entity: Feat) -> Result<Feat> {
        self.with_transaction(move |conn| conn.create(entity)).await
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Feat>> {
        let id = id.to_string();
        self.with_connection(move |conn| conn.find_by_id(&id)).await
    }

    async fn update(&self, id: &str, entity: Feat) -> Result<Feat> {
        let id = id.to_string();
        self.with_transaction(move |conn| conn.update(&id, entity)).await
    }

    async fn delete(&self, id: &str) -> Result<()> {
        let id = id.to_string();
        self.with_transaction(move |conn| Repository::<Feat>::delete(conn, &id)).await
    }

    async fn list(&self) -> Result<Vec<Feat>> {
        self.with_connection(|conn| conn.list()).await
    }
}

/// Feat-specific queries
impl FeatRepository {
    /// Find all feats for a specific rule system
    pub async fn find_by_rule_system(&self, rule_system_id: &str) -> Result<Vec<Feat>> {
        let rule_system_id = rule_system_id.to_string();
        self.with_connection(move |conn| {
            feats::table
                .filter(feats::rule_system_id.eq(&rule_system_id))
                .select(Feat::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }

    /// Find all feats from a specific source
    pub async fn find_by_source(&self, source_id: &str) -> Result<Vec<Feat>> {
        let source_id = source_id.to_string();
        self.with_connection(move |conn| {
            feats::table
                .filter(feats::source_id.eq(&source_id))
                .select(Feat::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }

    /// Find feats by type
    pub async fn find_by_type(&self, feat_type: &str) -> Result<Vec<Feat>> {
        let feat_type = feat_type.to_string();
        self.with_connection(move |conn| {
            feats::table
                .filter(feats::feat_type.eq(&feat_type))
                .select(Feat::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }

    /// Find origin feats (available at character creation)
    pub async fn find_origin_feats(&self) -> Result<Vec<Feat>> {
        self.find_by_type("origin").await
    }

    /// Find general feats
    pub async fn find_general_feats(&self) -> Result<Vec<Feat>> {
        self.find_by_type("general").await
    }

    /// Find fighting style feats
    pub async fn find_fighting_style_feats(&self) -> Result<Vec<Feat>> {
        self.find_by_type("fighting-style").await
    }

    /// Find epic feats (20th level+)
    pub async fn find_epic_feats(&self) -> Result<Vec<Feat>> {
        self.find_by_type("epic").await
    }

    /// Find feats that have prerequisites
    pub async fn find_with_prerequisites(&self) -> Result<Vec<Feat>> {
        self.with_connection(|conn| {
            feats::table
                .filter(feats::prerequisites.is_not_null())
                .select(Feat::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }

    /// Find feats without prerequisites
    pub async fn find_without_prerequisites(&self) -> Result<Vec<Feat>> {
        self.with_connection(|conn| {
            feats::table
                .filter(feats::prerequisites.is_null())
                .select(Feat::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }

    /// Find feats that provide ability increases
    pub async fn find_with_ability_increases(&self) -> Result<Vec<Feat>> {
        self.with_connection(|conn| {
            feats::table
                .filter(feats::ability_increases.is_not_null())
                .select(Feat::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }

    /// Search feats by name (case-insensitive)
    pub async fn search_by_name(&self, search_term: &str) -> Result<Vec<Feat>> {
        let search_pattern = format!("%{}%", search_term);
        self.with_connection(move |conn| {
            feats::table
                .filter(feats::name.like(&search_pattern))
                .select(Feat::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }

    /// Find feats available for multiclassing (no specific class prerequisites)
    pub async fn find_multiclass_compatible(&self) -> Result<Vec<Feat>> {
        self.with_connection(|conn| {
            // This is a simplified version - in practice, you'd need to parse JSON
            // to check if prerequisites contain class requirements
            feats::table
                .filter(feats::prerequisites.is_null().or(
                    feats::prerequisites.not_like("%\"classes\"%")
                ))
                .select(Feat::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }

    /// Find feats by rule system and type
    pub async fn find_by_rule_system_and_type(&self, rule_system_id: &str, feat_type: &str) -> Result<Vec<Feat>> {
        let rule_system_id = rule_system_id.to_string();
        let feat_type = feat_type.to_string();
        self.with_connection(move |conn| {
            feats::table
                .filter(feats::rule_system_id.eq(&rule_system_id))
                .filter(feats::feat_type.eq(&feat_type))
                .select(Feat::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }

    /// Count feats by type in a rule system
    pub async fn count_by_type(&self, rule_system_id: &str, feat_type: &str) -> Result<i64> {
        let rule_system_id = rule_system_id.to_string();
        let feat_type = feat_type.to_string();
        self.with_connection(move |conn| {
            feats::table
                .filter(feats::rule_system_id.eq(&rule_system_id))
                .filter(feats::feat_type.eq(&feat_type))
                .count()
                .get_result(conn)
                .map_err(DbError::from)
        })
        .await
    }

    /// Get feat statistics for a rule system
    pub async fn get_feat_statistics(&self, rule_system_id: &str) -> Result<FeatStatistics> {
        let rule_system_id = rule_system_id.to_string();
        
        // Get counts for each feat type
        let total = self.with_connection({
            let rule_system_id = rule_system_id.clone();
            move |conn| {
                feats::table
                    .filter(feats::rule_system_id.eq(&rule_system_id))
                    .count()
                    .get_result::<i64>(conn)
                    .map_err(DbError::from)
            }
        }).await?;

        let origin = self.count_by_type(&rule_system_id, "origin").await.unwrap_or(0);
        let general = self.count_by_type(&rule_system_id, "general").await.unwrap_or(0);
        let fighting_style = self.count_by_type(&rule_system_id, "fighting-style").await.unwrap_or(0);
        let epic = self.count_by_type(&rule_system_id, "epic").await.unwrap_or(0);

        let with_prerequisites = self.with_connection({
            let rule_system_id = rule_system_id.clone();
            move |conn| {
                feats::table
                    .filter(feats::rule_system_id.eq(&rule_system_id))
                    .filter(feats::prerequisites.is_not_null())
                    .count()
                    .get_result::<i64>(conn)
                    .map_err(DbError::from)
            }
        }).await.unwrap_or(0);

        let with_ability_increases = self.with_connection({
            let rule_system_id = rule_system_id.clone();
            move |conn| {
                feats::table
                    .filter(feats::rule_system_id.eq(&rule_system_id))
                    .filter(feats::ability_increases.is_not_null())
                    .count()
                    .get_result::<i64>(conn)
                    .map_err(DbError::from)
            }
        }).await.unwrap_or(0);

        Ok(FeatStatistics {
            total,
            origin,
            general,
            fighting_style,
            epic,
            with_prerequisites,
            with_ability_increases,
        })
    }
}

/// Statistics about feats in a rule system
#[derive(Debug, Clone)]
pub struct FeatStatistics {
    pub total: i64,
    pub origin: i64,
    pub general: i64,
    pub fighting_style: i64,
    pub epic: i64,
    pub with_prerequisites: i64,
    pub with_ability_increases: i64,
}