---
id: add-monster-tagging-ui-to-module
level: task
title: "Add monster tagging UI to module prep view"
short_code: "MIMIR-T-0147"
created_at: 2025-12-16T16:23:42.515366+00:00
updated_at: 2025-12-16T16:23:42.515366+00:00
parent: MIMIR-I-0018
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/todo"


exit_criteria_met: false
strategy_id: NULL
initiative_id: MIMIR-I-0018
---

# Add monster tagging UI to module prep view

## Parent Initiative

[[MIMIR-I-0018]] - Module Play Mode (DM Screen)

## Objective

Create the UI in the module prep view that allows DMs to add monsters from the catalog to a module, with optional encounter grouping.

## Acceptance Criteria

- [ ] "Monsters" section/tab added to module prep view
- [ ] "Add Monster" button opens monster search dialog
- [ ] Monster search dialog:
  - Search by name with autocomplete from catalog
  - Show monster CR, type, source in results
  - Select monster adds it to module
- [ ] Monster list displays:
  - Monster name, CR, type
  - Quantity (editable, default 1)
  - Encounter tag (editable, optional)
  - Remove button
- [ ] Monsters grouped by encounter tag in display
- [ ] Changes persist via Tauri commands to backend
- [ ] Frontend service methods for CRUD operations

## Implementation Notes

### UI Components

```
+------------------------------------------+
|  MONSTERS                    [+ Add]     |
+------------------------------------------+
|  Cave Entrance                           |
|    Goblin (CR 1/4)              [2] [x]  |
|                                          |
|  Wolf Den                                |
|    Wolf (CR 1/4)                [3] [x]  |
|    Goblin (CR 1/4)              [1] [x]  |
|                                          |
|  Unassigned                              |
|    Bugbear (CR 1)               [1] [x]  |
+------------------------------------------+
```

### Files to Create/Modify

- `crates/mimir-dm/frontend/src/features/campaigns/components/ModuleMonsters.vue` (new)
- `crates/mimir-dm/frontend/src/features/campaigns/components/MonsterSearchDialog.vue` (new)
- `crates/mimir-dm/frontend/src/services/ModuleMonsterService.ts` (new)
- `crates/mimir-dm/frontend/src/features/campaigns/views/ModuleView.vue` (integrate component)

### Dependencies

- MIMIR-T-0145 (backend module_monsters service)
- Existing monster catalog and search functionality

## Status Updates

*To be added during implementation*