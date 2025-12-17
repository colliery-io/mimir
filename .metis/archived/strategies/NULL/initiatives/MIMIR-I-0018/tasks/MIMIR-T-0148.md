---
id: implement-quick-access-sidebar
level: task
title: "Implement Quick Access Sidebar with monster/NPC display"
short_code: "MIMIR-T-0148"
created_at: 2025-12-16T16:23:42.719662+00:00
updated_at: 2025-12-16T16:23:42.719662+00:00
parent: MIMIR-I-0018
blocked_by: []
archived: true

tags:
  - "#task"
  - "#phase/todo"


exit_criteria_met: false
strategy_id: NULL
initiative_id: MIMIR-I-0018
---

# Implement Quick Access Sidebar with monster/NPC display

## Parent Initiative

[[MIMIR-I-0018]] - Module Play Mode (DM Screen)

## Objective

Build the Quick Access Sidebar for Play Mode that displays module monsters (grouped by encounter), NPCs, and locations for fast reference during gameplay.

## Acceptance Criteria

## Acceptance Criteria

- [ ] Sidebar component integrated into Play Mode layout
- [ ] Collapsible sidebar (toggle button or drag to collapse)
- [ ] Sidebar sections:
  - **Monsters** - grouped by encounter tag, shows name + quantity
  - **NPCs** - list of module/campaign NPCs
  - **Locations** - key locations from module
  - **Plot Points** - secrets/hooks (future, can be placeholder)
- [ ] Click on monster → expands stat block in main area
- [ ] Click on NPC → expands NPC card in main area
- [ ] Visual indicator for "currently selected" item
- [ ] Smooth expand/collapse animations
- [ ] Keyboard navigation support (arrow keys, enter to select)

## Implementation Notes

### Sidebar Layout

```
+------------------------+
|  [<<]  QUICK ACCESS    |
+------------------------+
|  v MONSTERS            |
|    Cave Entrance       |
|      Goblin (2)        |
|    Wolf Den            |
|      Wolf (3)          |
|      Goblin (1)        |
|    Boss Room           |
|      Bugbear (1)       |
+------------------------+
|  > NPCs                |
+------------------------+
|  > Locations           |
+------------------------+
|  > Plot Points         |
+------------------------+
```

### Card Display Integration

When sidebar item clicked:
1. Fetch full data (monster stats, NPC details)
2. Render appropriate card component in main area
3. Support pinning multiple cards
4. Reuse print template rendering where possible

### Files to Create/Modify

- `crates/mimir-dm/frontend/src/features/campaigns/components/play/QuickAccessSidebar.vue` (new)
- `crates/mimir-dm/frontend/src/features/campaigns/components/play/SidebarSection.vue` (new)
- `crates/mimir-dm/frontend/src/features/campaigns/components/play/MonsterCard.vue` (new, or reuse print)
- `crates/mimir-dm/frontend/src/features/campaigns/views/ModulePlayView.vue` (integrate sidebar)

### Dependencies

- MIMIR-T-0145 (module_monsters backend)
- MIMIR-T-0146 (play mode layout shell)
- Existing NPC and monster catalog services

## Status Updates

*To be added during implementation*