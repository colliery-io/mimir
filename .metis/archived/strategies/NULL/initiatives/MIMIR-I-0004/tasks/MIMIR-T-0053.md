---
id: create-frontend-typescript
level: task
title: "Create frontend TypeScript interfaces and Pinia stores"
short_code: "MIMIR-T-0053"
created_at: 2025-11-10T18:57:02.026766+00:00
updated_at: 2025-11-10T18:57:02.026766+00:00
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

# Create frontend TypeScript interfaces and Pinia stores

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0004]]

## Objective **[REQUIRED]**

Create TypeScript interfaces matching Rust backend types and Pinia stores for reactive state management of players, characters, and related data in the Vue frontend.

## Acceptance Criteria

## Acceptance Criteria **[REQUIRED]**

- [x] TypeScript interfaces created in `src/types/character.ts` matching Rust CharacterData structures
- [x] TypeScript interfaces for Player included in character.ts (combined for cohesion)
- [x] PlayerStore created in `src/stores/players.ts` with Pinia
- [x] CharacterStore created in `src/stores/characters.ts` with Pinia
- [x] Stores include actions for all CRUD operations calling Tauri commands
- [x] Stores maintain reactive state for current players and characters
- [x] Stores handle loading states and error states appropriately
- [x] Stores follow existing Mimir store patterns (campaignStore, moduleStore)

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

### 2025-11-18: Implementation Complete

Created Pinia stores for player and character state management following existing patterns from campaignStore.

**Player Store** (`/src/stores/players.ts`):
- State: `players` (list), `currentPlayer`, loading, error
- Computed: `playerCount`, `getPlayerById`
- Actions:
  - `fetchPlayers()` - Load all players
  - `getPlayer(id)` - Load specific player
  - `createPlayer(name, email?, notes?)` - Create new player
  - `updatePlayer(id, updates)` - Update player details
  - `deletePlayer(id)` - Remove player
  - `setCurrentPlayer()` - Set active player
  - `reset()` - Clear state

**Character Store** (`/src/stores/characters.ts`):
- State: `characters` (list), `currentCharacter`, `characterVersions`, loading, error
- Computed: `characterCount`, `getCharacterById`, `currentCharacterLevel`, `currentCharacterProficiencyBonus`
- Actions (16 total):
  - **CRUD**: `fetchCharactersForCampaign()`, `getCharacter()`, `createCharacter()`, `deleteCharacter()`
  - **Combat**: `updateCharacterHp()`, `rest()`
  - **Leveling**: `levelUpCharacter()`
  - **Spells**: `addSpellToKnown()`, `prepareSpells()`, `castSpell()`
  - **Inventory**: `addItem()`, `removeItem()`, `updateCurrency()`
  - **History**: `getCharacterVersions()`, `getCharacterVersion()`
  - **Utility**: `setCurrentCharacter()`, `reset()`

**Design Patterns**:
- Vue 3 Composition API with `defineStore()`
- Ref/computed pattern for reactive state
- Async/await for Tauri invoke calls
- Error handling with try/catch and error state
- Loading state management
- Auto-refresh after mutations (e.g., after casting spell, refresh character data)
- Follows existing patterns from campaignStore and moduleStore

**Type Safety**:
- All actions typed with proper input/output types
- Leverages TypeScript types from `../types/character`
- TypeScript compilation passing without errors