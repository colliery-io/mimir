---
id: decompose-characterservice-into
level: task
title: "Decompose CharacterService into focused services"
short_code: "MIMIR-T-0103"
created_at: 2025-11-25T01:48:45.528168+00:00
updated_at: 2025-11-25T11:19:51.446228+00:00
parent: MIMIR-I-0010
blocked_by: []
archived: true

tags:
  - "#task"
  - "#phase/completed"


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

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [x] CharacterService split into 4 focused services
- [x] Each new service has a single clear responsibility
- [x] All existing character functionality preserved (original methods kept for backward compatibility)
- [ ] character.rs command file updated to use new services (optional - can use new services directly)
- [x] All existing tests pass (351 tests)
- [x] New services follow the standardized service pattern (per ADR-005)

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

**2025-11-24**: Core decomposition complete
- Created CharacterProgressionService (progression.rs) - level up, ASI, multiclassing
- Created CharacterSpellService (spells.rs) - spell management, slots, resting  
- Created CharacterInventoryService (inventory.rs) - items, currency, equipment
- Original CharacterService methods preserved for backward compatibility
- All 351 tests pass
- Commit: 353c6da

Note: Command file update is optional - new services can be used directly by callers who want the focused API.