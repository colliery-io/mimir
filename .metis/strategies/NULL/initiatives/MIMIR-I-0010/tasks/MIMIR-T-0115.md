---
id: reorganize-commands-directory-by
level: task
title: "Reorganize commands directory by responsibility"
short_code: "MIMIR-T-0115"
created_at: 2025-11-25T01:48:54.321114+00:00
updated_at: 2025-11-25T01:48:54.321114+00:00
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

# Reorganize commands directory by responsibility

## Parent Initiative

[[MIMIR-I-0010]]

## Objective

Reorganize the flat commands directory (35+ files) into logical subdirectories grouped by domain responsibility, improving code navigation and maintainability.

## Acceptance Criteria

- [ ] Commands grouped into subdirectories by domain
- [ ] mod.rs files properly export all commands
- [ ] main.rs updated to use new module structure
- [ ] All Tauri command registrations still work
- [ ] No breaking changes to frontend invocations
- [ ] Build and tests pass

## Implementation Notes

### Technical Approach

Current structure (flat, 35+ files):
```
commands/
  mod.rs
  app_info.rs
  boards.rs
  campaigns.rs
  catalog_action.rs
  catalog_background.rs
  ... (20 more catalog_*.rs)
  character.rs
  chat_sessions.rs
  documents.rs
  logs.rs
  modules.rs
  player.rs
  sessions.rs
  books/
    mod.rs
    ...
```

Proposed structure (grouped by domain):
```
commands/
  mod.rs                    # Re-exports all submodules
  catalog/
    mod.rs
    action.rs
    background.rs
    class.rs
    ... (all catalog commands)
  campaign/
    mod.rs
    campaigns.rs
    modules.rs
    sessions.rs
  character/
    mod.rs
    character.rs
    player.rs
  chat/
    mod.rs
    chat_sessions.rs
  content/
    mod.rs
    documents.rs
    books/
  system/
    mod.rs
    app_info.rs
    logs.rs
    dev_tools.rs
    window_manager.rs
```

### Migration Steps
1. Create subdirectory structure
2. Move files into appropriate directories
3. Update mod.rs files with proper exports
4. Update main.rs imports
5. Verify all command registrations work
6. Run full test suite

### Dependencies
None - can be done independently

### Risk Considerations
- Need to ensure Tauri command names stay the same
- Watch for import path changes breaking other code
- Do in small commits for easy rollback

## Status Updates

*To be added during implementation*