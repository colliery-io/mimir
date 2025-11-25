---
id: decompose-characterservice-into
level: task
title: "Decompose CharacterService into focused services"
short_code: "MIMIR-T-0103"
created_at: 2025-11-25T01:48:45.528168+00:00
updated_at: 2025-11-25T01:48:45.528168+00:00
parent: MIMIR-I-0010
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/todo"


exit_criteria_met: false
strategy_id: NULL
initiative_id: MIMIR-I-0010
---

# Decompose CharacterService into focused services

## Parent Initiative

[[MIMIR-I-0010]]

## Objective

Decompose the monolithic CharacterService into focused, single-responsibility services to improve maintainability, testability, and code organization.

## Acceptance Criteria

- [ ] CharacterService split into 4-5 focused services
- [ ] Each new service has a single clear responsibility
- [ ] All existing character functionality preserved
- [ ] character.rs command file updated to use new services
- [ ] All existing tests pass
- [ ] New services follow the standardized service pattern (per ADR)

## Implementation Notes

### Technical Approach

Current CharacterService responsibilities (in `character/` module):
1. **Character CRUD**: Create, read, update, delete characters
2. **Version Management**: Snapshots, version history, rollback
3. **Level Up Logic**: ASI, multiclassing, HP calculations
4. **Spell Management**: Known spells, spell slots, prepared spells
5. **Inventory Management**: Items, equipment, encumbrance

Proposed decomposition:
- `CharacterService` - Core CRUD and character queries
- `CharacterVersionService` - Snapshots and version management
- `CharacterProgressionService` - Level up, ASI, multiclassing
- `CharacterSpellService` - Spell slot calculation, spell management
- `CharacterInventoryService` - Equipment and inventory

### Files to Modify
- `crates/mimir-dm-core/src/services/character/` - Split module
- `crates/mimir-dm/src/commands/character.rs` - Update imports
- Tests in `tests/integrations/`

### Dependencies
- MIMIR-T-0101 (ADR) should be decided first to establish pattern

### Risk Considerations
- Large refactoring with many call sites
- Need careful testing to ensure no regressions
- May need to update frontend TypeScript interfaces

## Status Updates

*To be added during implementation*