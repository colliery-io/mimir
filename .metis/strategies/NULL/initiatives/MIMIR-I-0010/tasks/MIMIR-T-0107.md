---
id: document-all-tauri-commands
level: task
title: "Document all Tauri commands"
short_code: "MIMIR-T-0107"
created_at: 2025-11-25T01:48:45.769883+00:00
updated_at: 2025-11-25T01:48:45.769883+00:00
parent: MIMIR-I-0010
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/todo"


exit_criteria_met: false
strategy_id: NULL
initiative_id: MIMIR-I-0010
---

# Document all Tauri commands

## Parent Initiative

[[MIMIR-I-0010]]

## Objective

Add comprehensive documentation comments to all Tauri command handlers, describing their purpose, parameters, return types, and error conditions for frontend developers.

## Acceptance Criteria

- [ ] All 100+ Tauri commands have `///` doc comments
- [ ] Each doc comment includes: purpose, parameters, return type, possible errors
- [ ] Doc comments follow consistent format across all command files
- [ ] `cargo doc` generates clean documentation for commands module
- [ ] Consider generating TypeScript types documentation from Rust docs

## Implementation Notes

### Technical Approach

Current state: Most commands have minimal or no documentation:
```rust
#[tauri::command]
pub async fn search_spells(...) -> Result<Vec<SpellSummary>, String> {
```

Target state:
```rust
/// Search the spell catalog with optional filters.
///
/// # Parameters
/// - `query`: Optional text to search spell names and descriptions
/// - `levels`: Optional list of spell levels to filter by (0-9)
/// - `schools`: Optional list of spell schools to filter by
///
/// # Returns
/// A list of spell summaries matching the search criteria.
///
/// # Errors
/// Returns an error string if the database query fails.
#[tauri::command]
pub async fn search_spells(...) -> Result<Vec<SpellSummary>, String> {
```

### Command Files to Document
- `commands/catalog_*.rs` (20 files) - Catalog search commands
- `commands/campaigns.rs` - Campaign management
- `commands/modules.rs` - Module management
- `commands/sessions.rs` - Session management
- `commands/character.rs` - Character operations
- `commands/chat_sessions.rs` - LLM chat commands
- `commands/documents.rs` - Document management
- `commands/player.rs` - Player management
- `commands/books/` - Book upload commands

### Dependencies
None - documentation task

### Risk Considerations
- Time-consuming but low risk
- Should be done incrementally, one file at a time

## Status Updates

*To be added during implementation*