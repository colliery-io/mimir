---
id: create-playermanager-vue-component
level: task
title: "Create PlayerManager.vue component"
short_code: "MIMIR-T-0054"
created_at: 2025-11-10T18:57:02.525872+00:00
updated_at: 2025-11-10T18:57:02.525872+00:00
parent: MIMIR-I-0004
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/todo"


exit_criteria_met: false
strategy_id: NULL
initiative_id: MIMIR-I-0004
---

# Create PlayerManager.vue component

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0004]]

## Objective **[REQUIRED]**

Create a PlayerManager Vue component for managing campaign players, including adding, editing, removing players, and assigning them to the active campaign.

## Acceptance Criteria **[REQUIRED]**

- [ ] PlayerManager.vue component created in `src/components/campaign/PlayerManager.vue`
- [ ] List view displaying all players in current campaign with names and email
- [ ] Add Player button opening dialog with form (name, email, notes)
- [ ] Edit player functionality with inline editing or dialog
- [ ] Remove player from campaign with confirmation dialog
- [ ] Delete player entirely option (only if no characters associated)
- [ ] View showing player's characters with quick links
- [ ] Component uses PlayerStore for all data operations
- [ ] Responsive design working on desktop layout

## Implementation Notes **[CONDITIONAL: Technical Task]**

### Technical Approach
- Create `src/components/campaign/PlayerManager.vue` as Vue 3 SFC
- Use Composition API with setup() and script setup syntax
- Integrate PlayerStore via usePlayerStore() composable
- Use Vuetify or existing UI component library for tables, dialogs, forms
- Implement form validation for required fields (name)
- Show character count per player with navigation to character list

### Dependencies
- MIMIR-T-0053 (PlayerStore must exist)
- Existing Mimir UI components and styling
- Vue Router for navigation

### Risk Considerations
- Deleting a player with characters should be prevented or warn user
- Player email is optional but useful for contact
- Need to refresh player list after mutations
- Handle async operation loading states
- Form validation edge cases (duplicate names, etc.)

## Status Updates **[REQUIRED]**

*To be added during implementation*