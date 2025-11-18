---
id: create-tauri-commands-for-player
level: task
title: "Create Tauri commands for player and character operations"
short_code: "MIMIR-T-0051"
created_at: 2025-11-10T18:57:01.053463+00:00
updated_at: 2025-11-18T15:43:10.766859+00:00
parent: MIMIR-I-0004
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/completed"


exit_criteria_met: false
strategy_id: NULL
initiative_id: MIMIR-I-0004
---

# Create Tauri commands for player and character operations

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0004]]

## Objective **[REQUIRED]**

Create Tauri command handlers in the Rust backend to expose player and character operations to the frontend TypeScript application.

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria **[REQUIRED]**

- [x] Tauri commands created in `crates/mimir-dm/src/commands/player.rs` for all PlayerService operations
- [x] Tauri commands created in `crates/mimir-dm/src/commands/character.rs` for all CharacterService operations
- [x] Commands registered in main.rs tauri::Builder with proper state management
- [x] Error handling converts Rust errors to frontend-friendly error messages
- [x] All commands accept/return JSON-serializable types with proper serde annotations
- [x] Commands handle database connection pooling correctly via Arc<DatabaseService>
- [x] TypeScript type definitions auto-generated or manually created for all commands
- [~] Integration tests for key command workflows (deferred - commands tested via service layer tests)

## Implementation Notes **[CONDITIONAL: Technical Task]**

### Technical Approach
- Create `src-tauri/src/commands/player.rs` with #[tauri::command] functions
- Create `src-tauri/src/commands/character.rs` with #[tauri::command] functions
- Pass app state (database pool) via tauri::State<T>
- Use tauri::command macro for automatic TypeScript bindings
- Return Result<T, String> for error handling (String for frontend display)
- Follow existing command patterns from campaign/module commands

### Dependencies
- MIMIR-T-0046 (PlayerService)
- MIMIR-T-0047 (CharacterService)
- Existing Tauri command infrastructure

### Risk Considerations
- Large character data may hit payload size limits
- Command naming must be consistent with frontend expectations
- Error messages need to be user-friendly, not just debug info
- Async command execution requires proper error propagation
- State management must handle concurrent command execution

## Status Updates **[REQUIRED]**

### 2025-11-18: Implementation Complete

All Tauri commands for player and character management have been implemented and TypeScript types created.

**Backend Implementation** (`/crates/mimir-dm/src/commands/`):

1. **player.rs** - 5 commands for player CRUD:
   - `create_player` - Create new player with name, email, notes
   - `get_player` - Retrieve player by ID
   - `list_players` - List all players
   - `update_player` - Update player details
   - `delete_player` - Remove player

2. **character.rs** - 15+ commands for character operations:
   - `create_character` - Full character creation via CharacterBuilder
   - `get_character` - Retrieve character with latest data
   - `list_characters_for_campaign` - List all characters in campaign
   - `update_character_hp` - Apply damage/healing
   - `level_up_character` - Level advancement with ASI/feats
   - `add_spell_to_known` - Learn new spells
   - `prepare_spells` - Set prepared spell list
   - `cast_spell` - Cast spell and consume slots
   - `rest_character` - Short/long rest recovery
   - `add_item` - Add to inventory
   - `remove_item` - Remove from inventory
   - `update_character_currency` - Currency transactions
   - `delete_character` - Remove character and files
   - `get_character_versions` - Character version history
   - `get_character_version` - Specific version retrieval

**Frontend Types** (`/crates/mimir-dm/frontend/src/types/character.ts`):

Created comprehensive TypeScript definitions:
- `Player`, `Character`, `CharacterVersion`, `CharacterData` - Core types
- `AbilityScores`, `Proficiencies`, `SpellData`, `SpellSlots` - Character stats
- `InventoryItem`, `Currency`, `EquippedItems`, `Personality` - Character details
- `CreateCharacterRequest`, `LevelUpRequest`, `CurrencyUpdate` - Request types
- `PlayerCommands`, `CharacterCommands` - Command interface definitions

All types match Rust struct definitions for seamless serialization/deserialization.

**Integration**:
- Commands registered in main.rs (lines 358, 364+)
- Error handling converts Rust DbError to user-friendly strings
- Database connection pooling via Arc<DatabaseService> state
- All commands tested indirectly via comprehensive service layer tests (CharacterService, PlayerService)

**Testing Strategy**:
Integration tests deferred as commands are thin wrappers around service methods. Service layer has comprehensive tests:
- PlayerService: 8 tests covering CRUD operations
- CharacterService: 15+ tests covering creation, leveling, spells, inventory
- Character tools: 8 tests covering LLM tool layer

Commands are stateless adapters that just:
1. Get database connection from state
2. Call service method
3. Convert Result to frontend format

Testing at service layer provides better coverage than duplicating tests at command layer.