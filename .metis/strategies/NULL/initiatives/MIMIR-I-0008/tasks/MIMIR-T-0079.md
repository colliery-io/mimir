---
id: standardize-errors-equipment-and
level: task
title: "Standardize errors: Equipment and loot services"
short_code: "MIMIR-T-0079"
created_at: 2025-11-24T20:28:57.643104+00:00
updated_at: 2025-11-24T20:28:57.643104+00:00
parent: MIMIR-I-0008
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/todo"


exit_criteria_met: false
strategy_id: NULL
initiative_id: MIMIR-I-0008
---

# Standardize errors: Equipment and loot services

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0008]]

## Objective

Migrate equipment and loot catalog services from `Result<T, String>` to `Result<T, DbError>` for type-safe, consistent error handling.

## Services in Scope

- `item_service.rs` - Item catalog queries (weapons, armor, equipment)
- `vehicle_service.rs` - Vehicle catalog queries
- `object_service.rs` - Environmental object queries
- `trap_service.rs` - Trap catalog queries
- `reward_service.rs` - Reward/loot table queries

## Acceptance Criteria

- [ ] `item_service.rs` returns `Result<T, DbError>` for all public methods
- [ ] `vehicle_service.rs` returns `Result<T, DbError>` for all public methods
- [ ] `object_service.rs` returns `Result<T, DbError>` for all public methods
- [ ] `trap_service.rs` returns `Result<T, DbError>` for all public methods
- [ ] `reward_service.rs` returns `Result<T, DbError>` for all public methods
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

### Files to Modify
- `crates/mimir-dm-core/src/services/item_service.rs`
- `crates/mimir-dm-core/src/services/vehicle_service.rs`
- `crates/mimir-dm-core/src/services/object_service.rs`
- `crates/mimir-dm-core/src/services/trap_service.rs`
- `crates/mimir-dm-core/src/services/reward_service.rs`
- Corresponding command handlers in `crates/mimir-dm/src/commands/`

### Dependencies
- Follow same pattern established in MIMIR-T-0075

### Risk Considerations
- Medium risk: Changes affect API contracts
- Need to update all callers (commands, other services)
- Run full test suite after changes

## Status Updates **[REQUIRED]**

*To be added during implementation*