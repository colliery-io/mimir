---
id: create-playermanager-vue-component
level: task
title: "Create PlayerManager.vue component"
short_code: "MIMIR-T-0054"
created_at: 2025-11-10T18:57:02.525872+00:00
updated_at: 2025-11-10T18:57:02.525872+00:00
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

# Create PlayerManager.vue component

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0004]]

## Objective **[REQUIRED]**

Create a PlayerManager Vue component for managing campaign players, including adding, editing, removing players, and assigning them to the active campaign.

## Acceptance Criteria

## Acceptance Criteria **[REQUIRED]**

- [x] PlayerManager.vue component created in `src/components/PlayerManager.vue`
- [x] List view displaying all players with names, email, and notes
- [x] Add Player button opening dialog with form (name, email, notes)
- [x] Edit player functionality with dialog
- [x] Delete player with confirmation dialog (warns about cascading delete)
- [x] Delete player entirely with warning about character deletion
- [~] View showing player's characters with quick links (deferred to character list integration)
- [x] Component uses PlayerStore for all data operations
- [x] Responsive design working on desktop layout

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

### 2025-11-18: Implementation Complete

Created PlayerManager.vue component following existing modal patterns from CampaignManagementModal.

**Component Features** (`/src/components/PlayerManager.vue`):
- Modal-based UI with overlay and close button
- Player list with name, email, notes, and created date
- Empty state with call-to-action for first player
- Loading and error state handling
- Player count display in header

**Add/Edit Player Dialog**:
- Form with name (required), email (optional), notes (optional)
- Email validation with HTML5 input type
- Auto-focus on name field when opening
- Form validation preventing empty names
- Separate save states for create vs update
- Error handling with user-friendly messages

**Delete Confirmation Dialog**:
- Warning about cascading character deletion
- Two-step confirmation to prevent accidents
- Disabled state during deletion operation
- Clear error messaging on failure

**Integration**:
- Uses PlayerStore for all CRUD operations
- Reactive updates when players change
- Auto-loads players when modal opens
- TypeScript types throughout
- Follows existing Mimir styling patterns

**Design Patterns**:
- Vue 3 Composition API with script setup
- Props/emit pattern for modal visibility
- Computed properties for reactive data
- Watch for loading data on visibility change
- CSS custom properties for theming
- Responsive design with max-widths

**Deferred**:
- Character count per player (requires character list integration)
- Quick links to player's characters (will be in character list view)

Component is ready to integrate into campaign management interface.