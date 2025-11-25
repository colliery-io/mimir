---
id: add-timeout-handling-to-tool
level: task
title: "Add timeout handling to tool confirmation flow"
short_code: "MIMIR-T-0113"
created_at: 2025-11-25T01:48:54.215449+00:00
updated_at: 2025-11-25T11:18:44.702028+00:00
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

# Add timeout handling to tool confirmation flow

## Parent Initiative

[[MIMIR-I-0010]]

## Objective

Add timeout handling to the LLM tool confirmation flow, preventing indefinite hangs when the user doesn't respond to tool confirmation prompts.

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [x] Configurable timeout for tool confirmation (default: 60 seconds)
- [x] Timeout triggers graceful cancellation of pending tool call
- [x] User notified when timeout occurs (via `tool-confirmation-timeout` event)
- [x] LLM conversation can continue after timeout (returns false to reject)
- [ ] Timeout setting exposed in application settings UI (frontend task)
- [x] No memory leaks from abandoned confirmation channels

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

### 2025-11-25: Backend Implementation Complete

Implemented timeout handling in the tool confirmation flow:

**Changes Made:**
- Added `tool_confirmation_timeout_secs` field to `ProviderSettings` (default: 60 seconds)
- Updated `LlmService.request_confirmation()` to use `tokio::time::timeout`
- On timeout: cleans up receiver, emits `tool-confirmation-timeout` event, returns false
- LLM conversation continues gracefully after timeout (tool action rejected)

**Files Modified:**
- `crates/mimir-dm/src/services/provider_settings.rs` - Added timeout setting
- `crates/mimir-dm/src/services/llm/llm_service.rs` - Implemented timeout logic

**Remaining Work:**
- Frontend UI to expose timeout setting in application settings (separate frontend task)