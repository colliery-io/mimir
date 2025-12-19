//! Common repository traits for the Data Access Layer

use crate::error::Result;

/// Synchronous repository trait for basic CRUD operations
pub trait Repository<T> {
    /// Create a new entity
    fn create(&mut self, entity: T) -> Result<T>;

    /// Find an entity by ID
    fn find_by_id(&mut self, id: &str) -> Result<Option<T>>;

    /// Update an entity
    fn update(&mut self, id: &str, entity: T) -> Result<T>;

    /// Delete an entity
    fn delete(&mut self, id: &str) -> Result<()>;

    /// List all entities
    fn list(&mut self) -> Result<Vec<T>>;
}

/// Trait for batch operations
pub trait BatchOperations<T> {
    /// Insert multiple entities in a single transaction
    fn batch_insert(&mut self, entities: Vec<T>) -> Result<usize>;

    /// Delete multiple entities by IDs
    fn batch_delete(&mut self, ids: Vec<String>) -> Result<usize>;
}

/// Trait for entities that belong to a rule system
pub trait RuleSystemScoped {
    /// Find all entities for a specific rule system
    fn find_by_rule_system(&mut self, rule_system_id: &str) -> Result<Vec<Self>>
    where
        Self: Sized;
}

/// Trait for entities that have a source
pub trait SourceScoped {
    /// Find all entities from a specific source
    fn find_by_source(&mut self, source_id: &str) -> Result<Vec<Self>>
    where
        Self: Sized;
}
