//! Rule system management commands

use crate::{
    services::database::DatabaseService,
    types::ApiResponse,
};
use diesel::prelude::*;
use mimir_dm_core::{
    models::rules::{
        rule_systems::RuleSystem,
        races::Race,
        classes::Class,
        spells::Spell,
        items::Item,
        backgrounds::Background,
        feats::Feat,
    },
    schema::{rule_systems, races, classes, spells, items, backgrounds, feats},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::State;
use tracing::{error, info};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleSystemInfo {
    pub id: String,
    pub name: String,
    pub short_name: Option<String>,
    pub publisher: Option<String>,
    pub version: Option<String>,
    pub is_active: bool,
}

impl From<RuleSystem> for RuleSystemInfo {
    fn from(rule_system: RuleSystem) -> Self {
        Self {
            id: rule_system.id,
            name: rule_system.name,
            short_name: rule_system.short_name,
            publisher: rule_system.publisher,
            version: rule_system.version,
            is_active: rule_system.is_active,
        }
    }
}

/// List all available rule systems
#[tauri::command]
pub async fn list_rule_systems(
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<ApiResponse<Vec<RuleSystemInfo>>, String> {
    info!("Listing available rule systems");
    
    let mut conn = db_service.get_connection().map_err(|e| e.to_string())?;
    
    match rule_systems::table
        .filter(rule_systems::is_active.eq(true))
        .load::<RuleSystem>(&mut *conn)
    {
        Ok(systems) => {
            let systems: Vec<RuleSystemInfo> = systems.into_iter()
                .map(RuleSystemInfo::from)
                .collect();
            info!("Found {} rule systems", systems.len());
            Ok(ApiResponse::success(systems))
        }
        Err(e) => {
            error!("Failed to list rule systems: {}", e);
            Ok(ApiResponse::error(format!("Failed to list rule systems: {}", e)))
        }
    }
}

/// Get a specific rule system by ID
#[tauri::command]
pub async fn get_rule_system(
    id: String,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<ApiResponse<RuleSystemInfo>, String> {
    info!("Getting rule system: {}", id);
    
    let mut conn = db_service.get_connection().map_err(|e| e.to_string())?;
    
    match rule_systems::table
        .find(&id)
        .first::<RuleSystem>(&mut *conn)
    {
        Ok(system) => {
            Ok(ApiResponse::success(RuleSystemInfo::from(system)))
        }
        Err(diesel::result::Error::NotFound) => {
            Ok(ApiResponse::error(format!("Rule system '{}' not found", id)))
        }
        Err(e) => {
            error!("Failed to get rule system '{}': {}", id, e);
            Ok(ApiResponse::error(format!("Failed to get rule system: {}", e)))
        }
    }
}