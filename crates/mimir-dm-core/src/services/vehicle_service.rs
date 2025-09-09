use diesel::prelude::*;
use crate::models::catalog::vehicle::{
    CatalogVehicle, VehicleSummary, VehicleFilters, Vehicle
};

pub struct VehicleService<'a> {
    pub conn: &'a mut SqliteConnection,
}

impl<'a> VehicleService<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }

    /// Search vehicles with filters
    pub fn search_vehicles(&mut self, filters: VehicleFilters) -> Result<Vec<VehicleSummary>, String> {
        use crate::schema::catalog_vehicles::dsl::*;
        
        let mut query = catalog_vehicles.into_boxed();
        
        // Filter by name (partial match)
        if let Some(name_filter) = &filters.name {
            if !name_filter.is_empty() {
                let search_pattern = format!("%{}%", name_filter.to_lowercase());
                query = query.filter(name.like(search_pattern));
            }
        }
        
        // Filter by sources
        if let Some(source_filters) = &filters.sources {
            if !source_filters.is_empty() {
                query = query.filter(source.eq_any(source_filters));
            }
        }
        
        // Filter by vehicle types
        if let Some(type_filters) = &filters.vehicle_types {
            if !type_filters.is_empty() {
                query = query.filter(vehicle_type.eq_any(type_filters));
            }
        }
        
        // Filter by sizes
        if let Some(size_filters) = &filters.sizes {
            if !size_filters.is_empty() {
                query = query.filter(size.eq_any(size_filters));
            }
        }
        
        // Filter by terrains (partial match in comma-separated string)
        if let Some(terrain_filters) = &filters.terrains {
            if !terrain_filters.is_empty() {
                for terrain in terrain_filters {
                    let search_pattern = format!("%{}%", terrain);
                    query = query.filter(terrain_text.like(search_pattern));
                }
            }
        }
        
        let vehicles = query
            .limit(1000) // Reasonable limit to prevent memory issues
            .load::<CatalogVehicle>(self.conn)
            .map_err(|e| format!("Failed to search vehicles: {}", e))?;
        
        Ok(vehicles.iter().map(VehicleSummary::from).collect())
    }

    /// Get vehicle by name and source
    pub fn get_vehicle_by_name_and_source(&mut self, vehicle_name: &str, vehicle_source: &str) -> Result<Option<Vehicle>, String> {
        use crate::schema::catalog_vehicles::dsl::*;
        
        let catalog_vehicle = catalog_vehicles
            .filter(name.eq(vehicle_name))
            .filter(source.eq(vehicle_source))
            .first::<CatalogVehicle>(self.conn)
            .optional()
            .map_err(|e| format!("Failed to get vehicle by name and source: {}", e))?;
        
        match catalog_vehicle {
            Some(vehicle_record) => {
                let parsed_vehicle: Vehicle = serde_json::from_str(&vehicle_record.full_vehicle_json)
                    .map_err(|e| format!("Failed to parse vehicle JSON: {}", e))?;
                Ok(Some(parsed_vehicle))
            },
            None => Ok(None),
        }
    }

    /// Get all unique vehicle types for filtering
    pub fn get_all_vehicle_types(&mut self) -> Result<Vec<String>, String> {
        use crate::schema::catalog_vehicles::dsl::*;
        
        let types: Vec<Option<String>> = catalog_vehicles
            .select(vehicle_type)
            .distinct()
            .filter(vehicle_type.is_not_null())
            .load(self.conn)
            .map_err(|e| format!("Failed to get vehicle types: {}", e))?;
        
        let mut result: Vec<String> = types
            .into_iter()
            .filter_map(|t| t)
            .filter(|t| !t.is_empty())
            .collect();
        
        result.sort();
        Ok(result)
    }

    /// Get all unique sizes for filtering
    pub fn get_all_sizes(&mut self) -> Result<Vec<String>, String> {
        use crate::schema::catalog_vehicles::dsl::*;
        
        let sizes: Vec<Option<String>> = catalog_vehicles
            .select(size)
            .distinct()
            .filter(size.is_not_null())
            .load(self.conn)
            .map_err(|e| format!("Failed to get vehicle sizes: {}", e))?;
        
        let mut result: Vec<String> = sizes
            .into_iter()
            .filter_map(|s| s)
            .filter(|s| !s.is_empty())
            .collect();
        
        result.sort();
        Ok(result)
    }

    /// Get all unique terrains for filtering
    pub fn get_all_terrains(&mut self) -> Result<Vec<String>, String> {
        use crate::schema::catalog_vehicles::dsl::*;
        
        let terrain_strings: Vec<Option<String>> = catalog_vehicles
            .select(terrain_text)
            .distinct()
            .filter(terrain_text.is_not_null())
            .load(self.conn)
            .map_err(|e| format!("Failed to get vehicle terrains: {}", e))?;
        
        let mut all_terrains = std::collections::HashSet::new();
        
        // Parse comma-separated terrains
        for terrain_str in terrain_strings.into_iter().filter_map(|t| t) {
            for terrain in terrain_str.split(',') {
                let trimmed = terrain.trim();
                if !trimmed.is_empty() {
                    all_terrains.insert(trimmed.to_string());
                }
            }
        }
        
        let mut result: Vec<String> = all_terrains.into_iter().collect();
        result.sort();
        Ok(result)
    }

    /// Get vehicle count by source for statistics
    pub fn get_vehicle_count_by_source(&mut self) -> Result<Vec<(String, i64)>, String> {
        use crate::schema::catalog_vehicles::dsl::*;
        
        let counts = catalog_vehicles
            .group_by(source)
            .select((source, diesel::dsl::count_star()))
            .load::<(String, i64)>(self.conn)
            .map_err(|e| format!("Failed to get vehicle counts: {}", e))?;
        
        Ok(counts)
    }
}