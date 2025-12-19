---
id: add-integration-tests-equipment
level: task
title: "Add integration tests: Equipment and loot services"
short_code: "MIMIR-T-0087"
created_at: 2025-11-24T20:28:59.263467+00:00
updated_at: 2025-11-25T00:09:27.866815+00:00
parent: MIMIR-I-0008
blocked_by: []
archived: true

tags:
  - "#task"
  - "#phase/completed"


exit_criteria_met: false
strategy_id: NULL
initiative_id: MIMIR-I-0008
---

# Add integration tests: Equipment and loot services

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0008]]

## Objective

Add integration tests for equipment and loot catalog services used in character equipment and DM loot generation.

## Services in Scope

- `item_service.rs` - Item catalog (weapons, armor, equipment)
- `vehicle_service.rs` - Vehicle catalog
- `object_service.rs` - Environmental object queries
- `trap_service.rs` - Trap catalog
- `reward_service.rs` - Reward/loot table queries

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [ ] `item_service` has tests for search by item type (weapon, armor, etc.)
- [ ] `item_service` has tests for filtering by properties (magical, rarity)
- [ ] `vehicle_service` has integration tests
- [ ] `object_service` has integration tests
- [ ] `trap_service` has integration tests
- [ ] `reward_service` has integration tests
- [ ] All tests use the existing `TestDatabase` fixture pattern
- [ ] Tests include both success and error scenarios

## Implementation Notes

### Test File Locations
- `crates/mimir-dm-core/tests/integrations/services/item_service.rs`
- `crates/mimir-dm-core/tests/integrations/services/vehicle_service.rs`
- `crates/mimir-dm-core/tests/integrations/services/object_service.rs`
- `crates/mimir-dm-core/tests/integrations/services/trap_service.rs`
- `crates/mimir-dm-core/tests/integrations/services/reward_service.rs`

### Key Test Cases

**Item Service:**
- Search by item category (weapon, armor, adventuring gear)
- Search by rarity (common, uncommon, rare, etc.)
- Search by magical property
- Search by price range
- Combined filters
- Get item by name

**Vehicle Service:**
- List all vehicles
- Search by vehicle type
- Get vehicle details

**Object/Trap/Reward Services:**
- Basic CRUD operations
- Search functionality
- Filter edge cases

### Dependencies
- Follow patterns established in MIMIR-T-0083
- Requires item/vehicle/object data seeded in test DB

### Risk Considerations
- Low risk: Adding tests doesn't affect production code

## Status Updates **[REQUIRED]**

*To be added during implementation*