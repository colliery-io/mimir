---
id: quality-assurance-and-developer
level: initiative
title: "Quality Assurance and Developer Experience"
short_code: "MIMIR-I-0012"
created_at: 2025-11-25T13:13:48.174643+00:00
updated_at: 2025-12-06T15:27:29.489800+00:00
parent: 
blocked_by: []
archived: true

tags:
  - "#initiative"
  - "#phase/completed"


exit_criteria_met: false
estimated_complexity: L
strategy_id: NULL
initiative_id: quality-assurance-and-developer
---

# Quality Assurance and Developer Experience Initiative

*This template includes sections for various types of initiatives. Delete sections that don't apply to your specific use case.*

## Context

The Mimir codebase has grown significantly with multiple layers:
- **Backend**: Rust Tauri commands, services, DAL, database
- **Frontend**: Vue 3 components, Pinia stores, TypeScript
- **Integration**: Tauri state management, LLM tool calling

Recent refactoring (consolidating state into AppState) broke multiple features at runtime despite passing type-checking. This highlighted gaps in our testing strategy - we have good unit/integration tests for services but lack:
1. Tauri command integration tests (state injection validation)
2. End-to-end GUI tests (user flow validation)
3. LLM tool calling tests (prompt/behavior validation)

## Goals & Non-Goals

**Goals:**
- Catch state management regressions before runtime
- Validate critical user flows automatically
- Enable safe refactoring with confidence
- Test LLM tool calling behavior with reproducible scenarios
- Establish testing patterns for future development

**Non-Goals:**
- 100% code coverage (focus on critical paths)
- Testing every edge case (prioritize high-value tests)
- Replacing manual exploratory testing entirely

## Testing Pyramid

```
         /\
        /  \  E2E (Playwright)
       /----\  - Critical user flows
      /      \  - Visual regression
     /--------\
    /          \ Integration Tests
   /   Tauri    \  - Command + AppState
  /   Commands   \  - State injection validation
 /----------------\
/                  \ Unit Tests (existing)
/    Services/DAL   \  - Business logic
/____________________\  - Database operations
```

## Detailed Design

### Layer 1: Tauri Command Integration Tests

**Problem Solved**: State management mismatches (like the recent AppState issue)

**Approach**:
- Create test harness that initializes real AppState
- Call Tauri commands directly (bypassing IPC)
- Verify commands don't panic and return expected types

**Location**: `crates/mimir-dm/tests/commands/`

**Example Test**:
```rust
#[tokio::test]
async fn test_list_log_files_with_app_state() {
    let app_state = create_test_app_state().await;
    let state = tauri::State::new(app_state);
    
    let result = list_log_files(state).await;
    assert!(result.is_ok());
}
```

### Layer 2: Playwright E2E Tests

**Problem Solved**: UI regressions, broken user flows

**Approach**:
- Test critical paths: chat, settings, campaign management
- Run against dev build with test database
- Screenshot comparison for visual regression

**Location**: `crates/mimir-dm/tests/e2e/`

**Critical Flows**:
1. App launch and campaign selection
2. Chat session creation and message send
3. Settings save and provider switch
4. Log viewer access

### Layer 3: LLM Tool Calling Tests

**Problem Solved**: Prompt changes breaking tool behavior

**Approach**:
- Mock LLM responses with recorded fixtures
- Test tool selection for common scenarios
- Verify tool arguments are correctly parsed

**Location**: `crates/mimir-dm/tests/llm/`

## Test Data Strategy

### Dev Seed Data
- Test campaign with modules and sessions
- Test players with characters at various levels
- Test chat sessions with message history

### Fixtures
- LLM response recordings for tool calling tests
- Expected state snapshots for regression tests

## Alternatives Considered

1. **Only Unit Tests**: Rejected - doesn't catch runtime state issues
2. **Manual Testing Only**: Rejected - not scalable, error-prone
3. **Full Cypress Suite**: Rejected - Playwright better for Tauri apps
4. **Snapshot Testing Only**: Rejected - too brittle for UI changes

## Implementation Plan

### Phase 1: Tauri Command Test Harness
- Create `create_test_app_state()` helper
- Write tests for critical command groups (chat, logs, settings)
- Add to CI pipeline

### Phase 2: Playwright Setup
- Install and configure Playwright for Tauri
- Write smoke tests for app launch
- Add critical flow tests

### Phase 3: LLM Test Harness
- Create HTTP mock server for LLM responses
- Record response fixtures for common scenarios
- Write tool selection verification tests

### Phase 4: CI Integration
- Add all test layers to GitHub Actions
- Configure test database seeding
- Add coverage reporting