---
id: create-play-mode-route-and-layout
level: task
title: "Create Play Mode route and layout shell"
short_code: "MIMIR-T-0146"
created_at: 2025-12-16T16:23:42.351764+00:00
updated_at: 2025-12-16T16:23:42.351764+00:00
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

# Create Play Mode route and layout shell

## Parent Initiative

[[MIMIR-I-0018]] - Module Play Mode (DM Screen)

## Objective

Create the frontend route and basic layout structure for Play Mode, establishing the visual foundation that subsequent tasks will build upon.

## Acceptance Criteria

- [ ] Route `/campaigns/:campaignId/modules/:moduleId/play` is registered
- [ ] `ModulePlayView.vue` component created with basic layout:
  - Header with module name, "Back to Module" link, "End Session" button
  - Left sidebar (collapsible) for quick access panels
  - Main content area with placeholder sections
- [ ] "Play Mode" button added to module prep view that navigates to play route
- [ ] Visual distinction from prep mode (different accent color or theme class)
- [ ] Clear "PLAY MODE" indicator visible in header
- [ ] Responsive layout that works on desktop (mobile deferred)
- [ ] Module data loads correctly (name, basic info)

## Implementation Notes

### Layout Structure

```
+------------------------------------------------------------------+
|  [< Back to Module]       MODULE NAME            [End Session]   |
|                         PLAY MODE                                |
+------------------------------------------------------------------+
|                    |                                             |
|  SIDEBAR           |              MAIN AREA                      |
|  (240px)           |                                             |
|                    |   [Placeholder for session notes]           |
|  [Placeholder]     |                                             |
|                    |   [Placeholder for card display]            |
|                    |                                             |
+------------------------------------------------------------------+
```

### Files to Create/Modify

- `crates/mimir-dm/frontend/src/features/campaigns/views/ModulePlayView.vue` (new)
- `crates/mimir-dm/frontend/src/router/index.ts` (add route)
- `crates/mimir-dm/frontend/src/features/campaigns/views/ModuleView.vue` (add Play Mode button)
- `crates/mimir-dm/frontend/src/assets/styles/` (play mode theme variables if needed)

### Design Notes

- Use existing design system components where possible
- Sidebar should be resizable or collapsible
- Main area uses flex layout for future panel arrangement
- Consider darker/warmer color scheme for play mode distinction

## Status Updates

*To be added during implementation*