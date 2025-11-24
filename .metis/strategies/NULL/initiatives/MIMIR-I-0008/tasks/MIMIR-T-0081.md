---
id: standardize-errors-lore-and-rules
level: task
title: "Standardize errors: Lore and rules services"
short_code: "MIMIR-T-0081"
created_at: 2025-11-24T20:28:58.035872+00:00
updated_at: 2025-11-24T20:28:58.035872+00:00
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

# Standardize errors: Lore and rules services

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0008]]

## Objective

Migrate lore and rules catalog services from `Result<T, String>` to `Result<T, DbError>` for type-safe, consistent error handling.

## Services in Scope

- `deity_service.rs` - Deity catalog queries
- `cult_service.rs` - Cult catalog queries
- `language_service.rs` - Language catalog queries
- `psionic_service.rs` - Psionics system queries
- `variant_rule_service.rs` - Variant rule queries
- `optional_feature_service.rs` - Optional feature queries

## Acceptance Criteria

- [ ] `deity_service.rs` returns `Result<T, DbError>` for all public methods
- [ ] `cult_service.rs` returns `Result<T, DbError>` for all public methods
- [ ] `language_service.rs` returns `Result<T, DbError>` for all public methods
- [ ] `psionic_service.rs` returns `Result<T, DbError>` for all public methods
- [ ] `variant_rule_service.rs` returns `Result<T, DbError>` for all public methods
- [ ] `optional_feature_service.rs` returns `Result<T, DbError>` for all public methods
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
- `crates/mimir-dm-core/src/services/deity_service.rs`
- `crates/mimir-dm-core/src/services/cult_service.rs`
- `crates/mimir-dm-core/src/services/language_service.rs`
- `crates/mimir-dm-core/src/services/psionic_service.rs`
- `crates/mimir-dm-core/src/services/variant_rule_service.rs`
- `crates/mimir-dm-core/src/services/optional_feature_service.rs`
- Corresponding command handlers in `crates/mimir-dm/src/commands/`

### Dependencies
- Follow same pattern established in MIMIR-T-0075

### Risk Considerations
- Medium risk: Changes affect API contracts
- Need to update all callers (commands, other services)
- Run full test suite after changes

## Status Updates **[REQUIRED]**

*To be added during implementation*