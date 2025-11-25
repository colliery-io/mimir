---
id: add-timeout-handling-to-tool
level: task
title: "Add timeout handling to tool confirmation flow"
short_code: "MIMIR-T-0113"
created_at: 2025-11-25T01:48:54.215449+00:00
updated_at: 2025-11-25T01:48:54.215449+00:00
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

# Add timeout handling to tool confirmation flow

## Parent Initiative

[[MIMIR-I-0010]]

## Objective

Add timeout handling to the LLM tool confirmation flow, preventing indefinite hangs when the user doesn't respond to tool confirmation prompts.

## Acceptance Criteria

- [ ] Configurable timeout for tool confirmation (default: 60 seconds)
- [ ] Timeout triggers graceful cancellation of pending tool call
- [ ] User notified when timeout occurs
- [ ] LLM conversation can continue after timeout
- [ ] Timeout setting exposed in application settings UI
- [ ] No memory leaks from abandoned confirmation channels

## Implementation Notes

### Technical Approach

Current flow in `chat_sessions.rs`:
1. LLM requests tool use
2. Backend sends confirmation request to frontend
3. Backend waits indefinitely on `ConfirmationReceivers` channel
4. User approves/denies in UI
5. Backend proceeds with tool execution or skips

Problem: Step 3 has no timeout - if user closes window or ignores prompt, the backend hangs forever.

Solution:
```rust
// Add timeout to confirmation wait
use tokio::time::{timeout, Duration};

let confirmation_timeout = Duration::from_secs(settings.tool_confirmation_timeout);
match timeout(confirmation_timeout, receiver.recv()).await {
    Ok(Some(approved)) => { /* proceed */ },
    Ok(None) => { /* channel closed */ },
    Err(_) => {
        // Timeout - notify user and cancel
        emit_timeout_notification(app_handle);
        return Err(ToolError::Timeout);
    }
}
```

### Files to Modify
- `crates/mimir-dm/src/commands/chat_sessions.rs` - Add timeout logic
- `crates/mimir-dm/src/services/llm/mod.rs` - Timeout configuration
- Frontend settings component - Add timeout setting
- App settings store - Persist timeout value

### Dependencies
None - can be done independently

### Risk Considerations
- Need to handle partial tool execution states
- Ensure channels are properly cleaned up on timeout
- Test with various timeout values

## Status Updates

*To be added during implementation*