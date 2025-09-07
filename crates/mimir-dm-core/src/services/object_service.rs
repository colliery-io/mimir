use diesel::prelude::*;
use tracing::{debug, error};
use crate::models::catalog::{ObjectFilters, ObjectSummary, CatalogObject};
use crate::schema::catalog_objects;

pub struct ObjectService;

impl ObjectService {
    pub fn search_objects(
        conn: &mut SqliteConnection,
        filters: ObjectFilters,
    ) -> Result<Vec<ObjectSummary>, String> {
        debug!("Searching objects with filters: {:?}", filters);

        let mut query = catalog_objects::table.into_boxed();

        // Apply search pattern filter
        if let Some(search_pattern) = filters.search_pattern.clone() {
            if !search_pattern.is_empty() {
                let pattern = format!("%{}%", search_pattern.to_lowercase());
                query = query.filter(
                    catalog_objects::name.like(pattern.clone())
                        .or(catalog_objects::object_type.like(pattern.clone()))
                        .or(catalog_objects::size.like(pattern))
                );
            }
        }

        // Apply source filter
        if let Some(sources) = filters.sources {
            if !sources.is_empty() {
                query = query.filter(catalog_objects::source.eq_any(sources));
            }
        }

        // Apply object type filter
        if let Some(object_types) = filters.object_types {
            if !object_types.is_empty() {
                query = query.filter(catalog_objects::object_type.eq_any(object_types));
            }
        }

        // Apply size filter
        if let Some(sizes) = filters.sizes {
            if !sizes.is_empty() {
                query = query.filter(catalog_objects::size.eq_any(sizes));
            }
        }


        let objects = query
            .order_by(catalog_objects::name.asc())
            .select(CatalogObject::as_select())
            .load::<CatalogObject>(conn)
            .map_err(|e| {
                error!("Failed to search objects: {}", e);
                format!("Database error: {}", e)
            })?;

        debug!("Found {} objects", objects.len());

        let summaries: Vec<ObjectSummary> = objects
            .iter()
            .map(|o| ObjectSummary::from(o))
            .collect();

        Ok(summaries)
    }

    pub fn get_object_details(
        conn: &mut SqliteConnection,
        name: &str,
        source: &str,
    ) -> Result<Option<String>, String> {
        debug!("Getting object details for: {} ({})", name, source);

        let result = catalog_objects::table
            .filter(catalog_objects::name.eq(name))
            .filter(catalog_objects::source.eq(source))
            .select(catalog_objects::full_object_json)
            .first::<String>(conn)
            .optional()
            .map_err(|e| {
                error!("Failed to get object details: {}", e);
                format!("Database error: {}", e)
            })?;

        Ok(result)
    }

    pub fn get_object_sources(conn: &mut SqliteConnection) -> Result<Vec<String>, String> {
        debug!("Getting distinct object sources");

        let sources = catalog_objects::table
            .select(catalog_objects::source)
            .distinct()
            .order_by(catalog_objects::source.asc())
            .load::<String>(conn)
            .map_err(|e| {
                error!("Failed to get object sources: {}", e);
                format!("Database error: {}", e)
            })?;

        debug!("Found {} object sources", sources.len());
        Ok(sources)
    }

    pub fn get_object_count(conn: &mut SqliteConnection) -> Result<i64, String> {
        debug!("Getting total object count");

        let count = catalog_objects::table
            .count()
            .get_result(conn)
            .map_err(|e| {
                error!("Failed to get object count: {}", e);
                format!("Database error: {}", e)
            })?;

        debug!("Total objects: {}", count);
        Ok(count)
    }

    pub fn get_object_types(conn: &mut SqliteConnection) -> Result<Vec<String>, String> {
        debug!("Getting distinct object types");

        let types: Vec<String> = catalog_objects::table
            .select(catalog_objects::object_type)
            .distinct()
            .filter(catalog_objects::object_type.is_not_null())
            .order_by(catalog_objects::object_type.asc())
            .load::<Option<String>>(conn)
            .map_err(|e| {
                error!("Failed to get object types: {}", e);
                format!("Database error: {}", e)
            })?
            .into_iter()
            .filter_map(|t| t)
            .collect();

        debug!("Found {} distinct object types", types.len());
        Ok(types)
    }

    pub fn get_object_sizes(conn: &mut SqliteConnection) -> Result<Vec<String>, String> {
        debug!("Getting distinct object sizes");

        let sizes: Vec<String> = catalog_objects::table
            .select(catalog_objects::size)
            .distinct()
            .filter(catalog_objects::size.is_not_null())
            .order_by(catalog_objects::size.asc())
            .load::<Option<String>>(conn)
            .map_err(|e| {
                error!("Failed to get object sizes: {}", e);
                format!("Database error: {}", e)
            })?
            .into_iter()
            .filter_map(|s| s)
            .collect();

        debug!("Found {} distinct object sizes", sizes.len());
        Ok(sizes)
    }
}