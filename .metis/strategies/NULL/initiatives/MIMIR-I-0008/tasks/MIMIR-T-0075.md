---
id: standardize-errors-core-catalog
level: task
title: "Standardize errors: Core catalog services"
short_code: "MIMIR-T-0075"
created_at: 2025-11-24T20:28:57.084089+00:00
updated_at: 2025-11-24T22:30:59.862665+00:00
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

# Standardize errors: Core catalog services

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0008]]

## Objective

Migrate core catalog services from `Result<T, String>` to `Result<T, DbError>` for type-safe, consistent error handling.

## Services in Scope

- `spell_service.rs` (462 lines) - Spell search and filtering
- `class_service.rs` (785 lines) - Class and subclass queries
- `race_service.rs` (168 lines) - Race catalog queries
- `background_service.rs` - Background catalog queries
- `feat_service.rs` - Feat catalog queries

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [ ] `spell_service.rs` returns `Result<T, DbError>` for all public methods
- [ ] `class_service.rs` returns `Result<T, DbError>` for all public methods
- [ ] `race_service.rs` returns `Result<T, DbError>` for all public methods
- [ ] `background_service.rs` returns `Result<T, DbError>` for all public methods
- [ ] `feat_service.rs` returns `Result<T, DbError>` for all public methods
- [ ] All calling code updated to handle `DbError` instead of `String`
- [ ] Tauri command handlers updated with proper error conversion

## Implementation Notes

### Technical Approach

1. Update function signatures from `Result<T, String>` to `Result<T, DbError>`
2. Replace `.map_err(|e| e.to_string())` with `.map_err(DbError::from)` or specific variants
3. Use appropriate `DbError` variants:
   - `DbError::Query` for diesel query errors
   - `DbError::NotFound` for missing entities
   - `DbError::InvalidData` for validation failures
4. Update Tauri commands to convert `DbError` to `ApiError`

### Example Transformation
```rust
// Before
pub fn search_spells(
    conn: &mut SqliteConnection,
    filters: SpellFilters,
) -> Result<Vec<SpellSummary>, String> {
    // ...
    .load::<CatalogSpell>(conn)
    .map_err(|e| e.to_string())?;
}

// After
pub fn search_spells(
    conn: &mut SqliteConnection,
    filters: SpellFilters,
) -> Result<Vec<SpellSummary>, DbError> {
    // ...
    .load::<CatalogSpell>(conn)
    .map_err(DbError::Query)?;
}
```

### Dependencies
None - can be done independently

### Risk Considerations
- Medium risk: Changes affect API contracts
- Need to update all callers (commands, other services)
- Run full test suite after changes

## Status Updates **[REQUIRED]**

*To be added during implementation*