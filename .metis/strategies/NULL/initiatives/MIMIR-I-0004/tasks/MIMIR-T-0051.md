---
id: create-tauri-commands-for-player
level: task
title: "Create Tauri commands for player and character operations"
short_code: "MIMIR-T-0051"
created_at: 2025-11-10T18:57:01.053463+00:00
updated_at: 2025-11-18T01:25:25.612210+00:00
parent: MIMIR-I-0004
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/active"


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

## Acceptance Criteria **[REQUIRED]**

- [x] Tauri commands created in `crates/mimir-dm/src/commands/player.rs` for all PlayerService operations
- [x] Tauri commands created in `crates/mimir-dm/src/commands/character.rs` for all CharacterService operations
- [x] Commands registered in main.rs tauri::Builder with proper state management
- [x] Error handling converts Rust errors to frontend-friendly error messages
- [x] All commands accept/return JSON-serializable types with proper serde annotations
- [x] Commands handle database connection pooling correctly via Arc<DatabaseService>
- [ ] TypeScript type definitions auto-generated or manually created for all commands
- [ ] Integration tests for key command workflows

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

*To be added during implementation*