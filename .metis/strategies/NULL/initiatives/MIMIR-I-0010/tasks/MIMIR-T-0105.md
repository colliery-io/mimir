---
id: consolidate-tauri-state-into
level: task
title: "Consolidate Tauri state into AppState struct"
short_code: "MIMIR-T-0105"
created_at: 2025-11-25T01:48:45.656685+00:00
updated_at: 2025-11-25T02:52:10.782457+00:00
parent: MIMIR-I-0010
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/completed"


exit_criteria_met: false
strategy_id: NULL
initiative_id: MIMIR-I-0010
---

# Consolidate Tauri state into AppState struct

## Parent Initiative

[[MIMIR-I-0010]]

## Objective

Consolidate the multiple individual Tauri state registrations in main.rs into a single `AppState` struct, improving code organization and making state dependencies explicit.

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [x] Create `AppState` struct containing all shared application state
- [x] Single `app.manage(app_state)` call in main.rs setup
- [x] All command handlers updated to extract from `AppState`
- [x] Remove individual `State<Arc<DatabaseService>>`, `State<ContextState>`, etc.
- [x] All existing functionality preserved
- [x] Cleaner, more maintainable main.rs setup code

## Implementation Notes

### Technical Approach

Current state in main.rs (multiple manage calls):
```rust
app.manage(db_service);
app.manage(app_paths_state);
app.manage(context_state);
app.manage(llm_service);
app.manage(confirmation_receivers);
app.manage(cancellation_tokens);
```

Proposed consolidated state:
```rust
pub struct AppState {
    pub db: Arc<DatabaseService>,
    pub paths: Arc<AppPaths>,
    pub context: ContextState,
    pub llm: Arc<LlmService>,
    pub confirmations: ConfirmationReceivers,
    pub cancellations: CancellationTokens,
}

// Single registration
app.manage(Arc::new(AppState { ... }));
```

Command handlers change from:
```rust
pub async fn some_command(
    db: State<'_, Arc<DatabaseService>>,
    paths: State<'_, Arc<AppPaths>>,
) -> Result<...>
```

To:
```rust
pub async fn some_command(
    state: State<'_, Arc<AppState>>,
) -> Result<...> {
    let db = &state.db;
    let paths = &state.paths;
    ...
}
```

### Files to Modify
- `crates/mimir-dm/src/main.rs` - Create AppState, single registration
- `crates/mimir-dm/src/types.rs` or new `state.rs` - Define AppState struct
- All command files in `crates/mimir-dm/src/commands/` - Update State extraction

### Dependencies
None - can be done independently

### Risk Considerations
- Many files to update (all command handlers)
- Careful testing needed to ensure no regressions

## Status Updates

*To be added during implementation*