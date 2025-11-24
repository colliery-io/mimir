---
id: standardize-errors-combat-and
level: task
title: "Standardize errors: Combat and encounter services"
short_code: "MIMIR-T-0077"
created_at: 2025-11-24T20:28:57.321541+00:00
updated_at: 2025-11-24T22:31:10.080614+00:00
parent: MIMIR-I-0008
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/active"


exit_criteria_met: false
strategy_id: NULL
initiative_id: MIMIR-I-0008
---

# Standardize errors: Combat and encounter services

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0008]]

## Objective

Migrate combat and encounter catalog services from `Result<T, String>` to `Result<T, DbError>` for type-safe, consistent error handling.

## Services in Scope

- `monster_service.rs` - Monster catalog queries and CR filtering
- `action_service.rs` - Combat action lookups
- `condition_service.rs` - Game condition queries

## Acceptance Criteria

## Acceptance Criteria

- [ ] `monster_service.rs` returns `Result<T, DbError>` for all public methods
- [ ] `action_service.rs` returns `Result<T, DbError>` for all public methods
- [ ] `condition_service.rs` returns `Result<T, DbError>` for all public methods
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
- `crates/mimir-dm-core/src/services/monster_service.rs`
- `crates/mimir-dm-core/src/services/action_service.rs`
- `crates/mimir-dm-core/src/services/condition_service.rs`
- Corresponding command handlers in `crates/mimir-dm/src/commands/`

### Dependencies
- Follow same pattern established in MIMIR-T-0075

### Risk Considerations
- Medium risk: Changes affect API contracts
- Need to update all callers (commands, other services)
- Run full test suite after changes

## Status Updates **[REQUIRED]**

*To be added during implementation*