---
id: create-backend-commands-for-tables
level: task
title: "Create backend commands for tables catalog"
short_code: "MIMIR-T-0097"
created_at: 2025-11-24T20:29:01.517390+00:00
updated_at: 2025-11-25T01:42:00.283021+00:00
parent: MIMIR-I-0008
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/completed"


exit_criteria_met: false
strategy_id: NULL
initiative_id: MIMIR-I-0008
---

# Create backend commands for tables catalog

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0008]]

## Objective

Create Tauri backend commands for the tables catalog to expose table data to the frontend.

*Note: This task was migrated from archived initiative MIMIR-I-0006*

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [ ] `search_tables` Tauri command created
- [ ] `get_table_by_id` Tauri command created
- [ ] `get_table_by_name` Tauri command created
- [ ] Commands are registered in main.rs
- [ ] Commands return proper `ApiResponse<T>` types
- [ ] Error handling follows established patterns

## Implementation Notes

### Files to Create/Modify
- `crates/mimir-dm/src/commands/catalog_table.rs` - New command file
- `crates/mimir-dm/src/commands/mod.rs` - Register module
- `crates/mimir-dm/src/main.rs` - Register commands

### Command Signatures
```rust
#[tauri::command]
pub async fn search_tables(
    db_service: State<'_, Arc<DatabaseService>>,
    filters: TableFilters,
) -> Result<ApiResponse<Vec<TableSummary>>, ApiError> {
    // Implementation
}

#[tauri::command]
pub async fn get_table_by_id(
    db_service: State<'_, Arc<DatabaseService>>,
    id: String,
) -> Result<ApiResponse<CatalogTable>, ApiError> {
    // Implementation
}

#[tauri::command]
pub async fn get_table_by_name(
    db_service: State<'_, Arc<DatabaseService>>,
    name: String,
    source: Option<String>,
) -> Result<ApiResponse<CatalogTable>, ApiError> {
    // Implementation
}
```

### Pattern to Follow
Reference existing catalog commands:
- `crates/mimir-dm/src/commands/catalog_spell.rs`
- `crates/mimir-dm/src/commands/catalog_monster.rs`

### Service Layer
May need to create `table_service.rs` in mimir-dm-core if it doesn't exist:
- `crates/mimir-dm-core/src/services/table_service.rs`

### Dependencies
- Requires MIMIR-T-0095 (Seed missing catalog data) for tables data
- Table model and schema must exist

### Risk Considerations
- Low risk: Following established patterns
- Ensure table schema/model exists before implementing

## Status Updates **[REQUIRED]**

*To be added during implementation*