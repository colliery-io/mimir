---
id: add-integration-tests-combat-and
level: task
title: "Add integration tests: Combat and encounter services"
short_code: "MIMIR-T-0085"
created_at: 2025-11-24T20:28:58.792544+00:00
updated_at: 2025-11-24T20:28:58.792544+00:00
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

# Add integration tests: Combat and encounter services

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0008]]

## Objective

Add integration tests for combat and encounter catalog services used in DM encounter building.

## Services in Scope

- `monster_service.rs` - Monster catalog with CR filtering
- `action_service.rs` - Combat action lookups
- `condition_service.rs` - Game condition queries

## Acceptance Criteria

- [ ] `monster_service` has tests for search with CR range filters
- [ ] `monster_service` has tests for type/environment filtering
- [ ] `monster_service` has tests for name search
- [ ] `action_service` has integration tests for action lookups
- [ ] `condition_service` has integration tests for condition queries
- [ ] All tests use the existing `TestDatabase` fixture pattern
- [ ] Tests include both success and error scenarios

## Implementation Notes

### Test File Locations
- `crates/mimir-dm-core/tests/integrations/services/monster_service.rs`
- `crates/mimir-dm-core/tests/integrations/services/action_service.rs`
- `crates/mimir-dm-core/tests/integrations/services/condition_service.rs`

### Key Test Cases

**Monster Service:**
- Search by CR range (e.g., CR 1-5)
- Search by creature type (e.g., "undead", "dragon")
- Search by environment
- Search by name pattern
- Combined filters
- Empty results handling

**Action Service:**
- List all actions
- Search by action name
- Filter by action type

**Condition Service:**
- List all conditions
- Get condition by name
- Search conditions

### Dependencies
- Follow patterns established in MIMIR-T-0083
- Requires monster/action/condition data seeded in test DB

### Risk Considerations
- Low risk: Adding tests doesn't affect production code

## Status Updates **[REQUIRED]**

*To be added during implementation*