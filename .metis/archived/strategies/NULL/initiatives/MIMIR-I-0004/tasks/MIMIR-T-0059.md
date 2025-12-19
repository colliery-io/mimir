---
id: create-inventorymanager-vue
level: task
title: "Create InventoryManager.vue component"
short_code: "MIMIR-T-0059"
created_at: 2025-11-10T18:57:05.386464+00:00
updated_at: 2025-11-23T02:14:02.595462+00:00
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

# Create InventoryManager.vue component

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0004]]

## Objective **[REQUIRED]**

Create an InventoryManager Vue component for managing character inventory, including adding/removing items, equipping gear, tracking currency, and calculating encumbrance.

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria **[REQUIRED]**

- [ ] InventoryManager.vue component created in `src/components/character/InventoryManager.vue`
- [ ] Inventory list showing all items with quantity, weight, and description
- [ ] Add Item button with item search/select from equipment database
- [ ] Remove/reduce quantity functionality for inventory items
- [ ] Equipment slots UI showing equipped items (main hand, off hand, armor, etc.)
- [ ] Equip/unequip buttons with slot validation (can't equip two armor pieces)
- [ ] Attunement tracker showing attuned items with 3-item limit enforcement
- [ ] Currency manager for platinum, gold, silver, copper with conversion
- [ ] Encumbrance display showing current weight vs capacity with warning indicators

## Implementation Notes **[CONDITIONAL: Technical Task]**

### Technical Approach
- Create `src/components/character/InventoryManager.vue` with tabs or sections
- Use CharacterStore inventory management actions
- Display equipment slots as card layout with drag-and-drop support (optional)
- Load item database from D&D 5e SRD for item details and weights
- Calculate total weight reactively from all carried items
- Visual indicators for encumbrance levels (normal, encumbered, heavily encumbered)
- Currency converter helper for denomination exchanges

### Dependencies
- MIMIR-T-0053 (CharacterStore with inventory actions)
- D&D 5e SRD item/equipment database
- Drag-and-drop library (optional, for better UX)

### Risk Considerations
- Item database needs weights, costs, and properties for all equipment
- Some items have variable weight (e.g., containers, bag of holding)
- Magic item attunement rules vary by item
- Encumbrance is optional rule in D&D 5e (some tables ignore it)
- Need to handle custom/homebrew items not in SRD

## Status Updates **[REQUIRED]**

*To be added during implementation*