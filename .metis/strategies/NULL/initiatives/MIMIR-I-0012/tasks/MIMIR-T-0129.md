---
id: create-tauri-command-integration
level: task
title: "Create Tauri command integration test harness"
short_code: "MIMIR-T-0129"
created_at: 2025-11-25T17:03:23.410022+00:00
updated_at: 2025-11-25T18:16:49.558153+00:00
parent: MIMIR-I-0012
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/completed"


exit_criteria_met: false
strategy_id: NULL
initiative_id: MIMIR-I-0012
---

# Create Tauri command integration test harness

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0012]]

## Objective

Create a test harness that validates Tauri commands work correctly with AppState, catching state management mismatches at test time rather than runtime.

## Problem

Tauri commands using `State<'_, T>` compile successfully even when `T` is not registered with `.manage()`. This causes runtime errors like "state not managed for field X" that are only discovered during manual testing.

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [ ] `create_test_app_state()` helper function in test utils
- [ ] Tests for chat commands (list, load, save, create, delete)
- [ ] Tests for log commands (list, read, tail)
- [ ] Tests for settings commands (get, save provider settings)
- [ ] Tests for book commands (get content, serve image, lookup reference)
- [ ] All tests pass in CI

## Test Cases **[CONDITIONAL: Testing Task]**

{Delete unless this is a testing task}

### Test Case 1: {Test Case Name}
- **Test ID**: TC-001
- **Preconditions**: {What must be true before testing}
- **Steps**: 
  1. {Step 1}
  2. {Step 2}
  3. {Step 3}
- **Expected Results**: {What should happen}
- **Actual Results**: {To be filled during execution}
- **Status**: {Pass/Fail/Blocked}

### Test Case 2: {Test Case Name}
- **Test ID**: TC-002
- **Preconditions**: {What must be true before testing}
- **Steps**: 
  1. {Step 1}
  2. {Step 2}
- **Expected Results**: {What should happen}
- **Actual Results**: {To be filled during execution}
- **Status**: {Pass/Fail/Blocked}

## Documentation Sections **[CONDITIONAL: Documentation Task]**

{Delete unless this is a documentation task}

### User Guide Content
- **Feature Description**: {What this feature does and why it's useful}
- **Prerequisites**: {What users need before using this feature}
- **Step-by-Step Instructions**:
  1. {Step 1 with screenshots/examples}
  2. {Step 2 with screenshots/examples}
  3. {Step 3 with screenshots/examples}

### Troubleshooting Guide
- **Common Issue 1**: {Problem description and solution}
- **Common Issue 2**: {Problem description and solution}
- **Error Messages**: {List of error messages and what they mean}

### API Documentation **[CONDITIONAL: API Documentation]**
- **Endpoint**: {API endpoint description}
- **Parameters**: {Required and optional parameters}
- **Example Request**: {Code example}
- **Example Response**: {Expected response format}

## Implementation Notes **[CONDITIONAL: Technical Task]**

{Keep for technical tasks, delete for non-technical. Technical details, approach, or important considerations}

### Technical Approach
{How this will be implemented}

### Dependencies
{Other tasks or systems this depends on}

### Risk Considerations
{Technical risks and mitigation strategies}

## Status Updates **[REQUIRED]**

*To be added during implementation*