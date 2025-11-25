//! Database-backed class catalog commands

use crate::state::AppState;
use mimir_dm_core::models::catalog::class::{Class, ClassFilters, ClassSummary, Subclass};
use mimir_dm_core::services::ClassService;
use tauri::State;

/// Search classes with database backend
#[tauri::command]
pub async fn search_classes(
    state: State<'_, AppState>,
    filters: ClassFilters,
) -> Result<Vec<ClassSummary>, String> {
    let mut conn = state.db.get_connection()
        .map_err(|e| format!("Database connection failed: {}", e))?;
    
    let mut class_service = ClassService::new(&mut conn);
    class_service.search_classes(filters)
        .map_err(|e| e.to_string())
}

/// Get class details by name and source (for base classes)
#[tauri::command]
pub async fn get_class_details(
    state: State<'_, AppState>,
    #[allow(non_snake_case)]
    className: String,
    #[allow(non_snake_case)]
    classSource: String,
) -> Result<Option<Class>, String> {
    let mut conn = state.db.get_connection()
        .map_err(|e| format!("Database connection failed: {}", e))?;

    let mut class_service = ClassService::new(&mut conn);
    class_service.get_class_by_name_and_source(&className, &classSource)
        .map_err(|e| e.to_string())
}

/// Get subclass details by subclass name, class name and source
#[tauri::command]
pub async fn get_subclass_details(
    state: State<'_, AppState>,
    #[allow(non_snake_case)]
    subclassName: String,
    #[allow(non_snake_case)]
    className: String,
    #[allow(non_snake_case)]
    classSource: String,
) -> Result<Option<Subclass>, String> {
    let mut conn = state.db.get_connection()
        .map_err(|e| format!("Database connection failed: {}", e))?;

    let mut class_service = ClassService::new(&mut conn);
    class_service.get_subclass_by_name(&subclassName, &className, &classSource)
        .map_err(|e| e.to_string())
}

/// Get all subclasses for a class
#[tauri::command]
pub async fn get_class_subclasses(
    state: State<'_, AppState>,
    #[allow(non_snake_case)]
    className: String,
    #[allow(non_snake_case)]
    classSource: String,
) -> Result<Vec<Subclass>, String> {
    let mut conn = state.db.get_connection()
        .map_err(|e| format!("Database connection failed: {}", e))?;

    let mut class_service = ClassService::new(&mut conn);
    class_service.get_subclasses_for_class(&className, &classSource)
        .map_err(|e| e.to_string())
}

/// Get unique class sources
#[tauri::command]
pub async fn get_class_sources(
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    let mut conn = state.db.get_connection()
        .map_err(|e| format!("Database connection failed: {}", e))?;
    
    let mut class_service = ClassService::new(&mut conn);
    class_service.get_class_sources()
        .map_err(|e| e.to_string())
}

/// Get unique primary abilities
#[tauri::command]
pub async fn get_class_primary_abilities(
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    let mut conn = state.db.get_connection()
        .map_err(|e| format!("Database connection failed: {}", e))?;
    
    let mut class_service = ClassService::new(&mut conn);
    class_service.get_primary_abilities()
        .map_err(|e| e.to_string())
}

/// Get class statistics (count by source)
#[tauri::command]
pub async fn get_class_statistics(
    state: State<'_, AppState>,
) -> Result<Vec<(String, i64)>, String> {
    let mut conn = state.db.get_connection()
        .map_err(|e| format!("Database connection failed: {}", e))?;
    
    let mut class_service = ClassService::new(&mut conn);
    class_service.get_class_count_by_source()
        .map_err(|e| e.to_string())
}