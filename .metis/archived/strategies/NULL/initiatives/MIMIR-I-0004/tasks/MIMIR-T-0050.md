---
id: implement-inventory-and-equipment
level: task
title: "Implement inventory and equipment management"
short_code: "MIMIR-T-0050"
created_at: 2025-11-10T18:57:00.567726+00:00
updated_at: 2025-11-18T01:19:27.414878+00:00
parent: MIMIR-I-0004
blocked_by: []
archived: true

tags:
  - "#task"
  - "#phase/completed"


exit_criteria_met: false
strategy_id: NULL
initiative_id: MIMIR-I-0004
---

# Implement inventory and equipment management

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0004]]

## Objective **[REQUIRED]**

Implement inventory and currency management for character sheets. Focus on tracking items and currency, not dynamic stat calculations (those belong in UI layer).

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria **[REQUIRED]**

- [x] add_item() method for adding items to inventory with quantity tracking
- [x] remove_item() method for removing or reducing item quantities
- [x] update_currency() method for adding/removing currency (gp, sp, cp, etc.)
- [x] Item validation against catalog_items database (similar to spell/class validation)
- [x] Unit tests for inventory operations and currency management

## Implementation Notes **[CONDITIONAL: Technical Task]**

### Technical Approach
- Extend CharacterService with inventory management methods
- Query catalog_items database to validate items exist before adding
- Track items in CharacterData.inventory with name, quantity, weight, value, notes
- Track currency separately (no automatic denomination conversion - player/DM responsibility)
- equipped field in CharacterData remains as simple strings (no complex slot validation)

### UI Layer Responsibilities (NOT in this task)
- Calculate AC based on equipped items
- Calculate encumbrance from inventory weights
- Display "what-if" scenarios (AC with/without shield, etc.)
- Attunement tracking (DM/player responsibility, not enforced in backend)

### Dependencies
- MIMIR-T-0044 (Inventory struct in CharacterData)
- MIMIR-T-0047 (CharacterService)
- catalog_items database table

### Risk Considerations
- Item database may have inconsistent weight/value data across sources
- Currency tracking should be simple - no automatic conversion complexity

## Status Updates **[REQUIRED]**

*To be added during implementation*