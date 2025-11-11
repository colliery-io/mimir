---
id: create-frontend-typescript
level: task
title: "Create frontend TypeScript interfaces and Pinia stores"
short_code: "MIMIR-T-0053"
created_at: 2025-11-10T18:57:02.026766+00:00
updated_at: 2025-11-10T18:57:02.026766+00:00
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

# Create frontend TypeScript interfaces and Pinia stores

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0004]]

## Objective **[REQUIRED]**

Create TypeScript interfaces matching Rust backend types and Pinia stores for reactive state management of players, characters, and related data in the Vue frontend.

## Acceptance Criteria **[REQUIRED]**

- [ ] TypeScript interfaces created in `src/types/character.ts` matching Rust CharacterData structures
- [ ] TypeScript interfaces in `src/types/player.ts` for Player and CampaignPlayer
- [ ] PlayerStore created in `src/stores/playerStore.ts` with Pinia
- [ ] CharacterStore created in `src/stores/characterStore.ts` with Pinia
- [ ] Stores include actions for all CRUD operations calling Tauri commands
- [ ] Stores maintain reactive state for current players and characters
- [ ] Stores handle loading states and error states appropriately
- [ ] Stores follow existing Mimir store patterns (campaignStore, moduleStore)

## Implementation Notes **[CONDITIONAL: Technical Task]**

### Technical Approach
- Create `src/types/player.ts` and `src/types/character.ts` with interface definitions
- Use Pinia defineStore() for reactive state management
- Store actions wrap Tauri invoke() calls to backend commands
- Maintain separate state for list views (all characters) vs detail views (single character)
- Use computed properties for derived values (character total level, AC, etc.)
- Follow Vue 3 Composition API patterns

### Dependencies
- MIMIR-T-0051 (Tauri commands must be implemented)
- Existing Pinia store infrastructure
- Vue 3 and Tauri invoke API

### Risk Considerations
- TypeScript types must exactly match Rust serde serialization
- State synchronization between stores (e.g., character belongs to player)
- Handling stale data when multiple views are open
- Large character lists may need pagination or virtualization
- Error handling must surface user-friendly messages

## Status Updates **[REQUIRED]**

*To be added during implementation*