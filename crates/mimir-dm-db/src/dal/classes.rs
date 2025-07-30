//! Data Access Layer for classes

use crate::{
    connection::DbConnection,
    dal::traits::{AsyncRepository, Repository},
    error::{DbError, Result},
    models::classes::Class,
    schema::classes,
};
use async_trait::async_trait;
use diesel::prelude::*;
use tokio::task;

/// Repository for Class operations
pub struct ClassRepository {
    database_url: String,
}

impl ClassRepository {
    /// Create a new ClassRepository
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
impl Repository<Class> for DbConnection {
    fn create(&mut self, entity: Class) -> Result<Class> {
        diesel::insert_into(classes::table)
            .values(&entity)
            .get_result(self)
            .map_err(DbError::from)
    }

    fn find_by_id(&mut self, id: &str) -> Result<Option<Class>> {
        classes::table
            .find(id)
            .first(self)
            .optional()
            .map_err(DbError::from)
    }

    fn update(&mut self, id: &str, entity: Class) -> Result<Class> {
        diesel::update(classes::table.find(id))
            .set(&entity)
            .get_result(self)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => DbError::NotFound {
                    entity_type: "Class".to_string(),
                    id: id.to_string(),
                },
                _ => DbError::from(e),
            })
    }

    fn delete(&mut self, id: &str) -> Result<()> {
        let rows_affected = diesel::delete(classes::table.find(id))
            .execute(self)
            .map_err(DbError::from)?;

        if rows_affected == 0 {
            return Err(DbError::NotFound {
                entity_type: "Class".to_string(),
                id: id.to_string(),
            });
        }

        Ok(())
    }

    fn list(&mut self) -> Result<Vec<Class>> {
        classes::table
            .select(Class::as_select())
            .load(self)
            .map_err(DbError::from)
    }
}

/// Asynchronous repository implementation
#[async_trait]
impl AsyncRepository<Class> for ClassRepository {
    async fn create(&self, entity: Class) -> Result<Class> {
        self.with_transaction(move |conn| conn.create(entity)).await
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Class>> {
        let id = id.to_string();
        self.with_connection(move |conn| conn.find_by_id(&id)).await
    }

    async fn update(&self, id: &str, entity: Class) -> Result<Class> {
        let id = id.to_string();
        self.with_transaction(move |conn| conn.update(&id, entity)).await
    }

    async fn delete(&self, id: &str) -> Result<()> {
        let id = id.to_string();
        self.with_transaction(move |conn| Repository::<Class>::delete(conn, &id)).await
    }

    async fn list(&self) -> Result<Vec<Class>> {
        self.with_connection(|conn| conn.list()).await
    }
}

/// Class-specific queries
impl ClassRepository {
    /// Find all classes for a specific rule system
    pub async fn find_by_rule_system(&self, rule_system_id: &str) -> Result<Vec<Class>> {
        let rule_system_id = rule_system_id.to_string();
        self.with_connection(move |conn| {
            classes::table
                .filter(classes::rule_system_id.eq(&rule_system_id))
                .select(Class::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }

    /// Find all classes from a specific source
    pub async fn find_by_source(&self, source_id: &str) -> Result<Vec<Class>> {
        let source_id = source_id.to_string();
        self.with_connection(move |conn| {
            classes::table
                .filter(classes::source_id.eq(&source_id))
                .select(Class::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }

    /// Find all base classes (not subclasses)
    pub async fn find_base_classes(&self) -> Result<Vec<Class>> {
        self.with_connection(|conn| {
            classes::table
                .filter(classes::class_type.eq("class"))
                .select(Class::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }

    /// Find all subclasses for a specific base class
    pub async fn find_subclasses(&self, parent_class_id: &str) -> Result<Vec<Class>> {
        let parent_class_id = parent_class_id.to_string();
        self.with_connection(move |conn| {
            classes::table
                .filter(classes::parent_class_id.eq(&parent_class_id))
                .select(Class::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }

    /// Find all classes by type (class or subclass)
    pub async fn find_by_type(&self, class_type: &str) -> Result<Vec<Class>> {
        let class_type = class_type.to_string();
        self.with_connection(move |conn| {
            classes::table
                .filter(classes::class_type.eq(&class_type))
                .select(Class::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }

    /// Find all spellcasting classes
    pub async fn find_spellcasters(&self) -> Result<Vec<Class>> {
        self.with_connection(|conn| {
            classes::table
                .filter(classes::spell_ability.is_not_null().or(classes::caster_progression.is_not_null()))
                .select(Class::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }

    /// Find classes by caster progression type
    pub async fn find_by_caster_progression(&self, progression: &str) -> Result<Vec<Class>> {
        let progression = progression.to_string();
        self.with_connection(move |conn| {
            classes::table
                .filter(classes::caster_progression.eq(&progression))
                .select(Class::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }

    /// Find classes by hit die
    pub async fn find_by_hit_die(&self, hit_die: i32) -> Result<Vec<Class>> {
        self.with_connection(move |conn| {
            classes::table
                .filter(classes::hit_die.eq(hit_die))
                .select(Class::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }

    /// Find classes by spell ability
    pub async fn find_by_spell_ability(&self, spell_ability: &str) -> Result<Vec<Class>> {
        let spell_ability = spell_ability.to_string();
        self.with_connection(move |conn| {
            classes::table
                .filter(classes::spell_ability.eq(&spell_ability))
                .select(Class::as_select())
                .load(conn)
                .map_err(DbError::from)
        })
        .await
    }
}