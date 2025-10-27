---
id: fix-test-organization-and-move
level: task
title: "Fix test organization and move module_service_tests.rs"
short_code: "MIMIR-T-0026"
created_at: 2025-10-24T11:54:16.153447+00:00
updated_at: 2025-10-24T11:54:16.153447+00:00
parent: MIMIR-I-0001
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/completed"


exit_criteria_met: true
strategy_id: NULL
initiative_id: MIMIR-I-0001
---

# Fix test organization and move module_service_tests.rs

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0001]]

## Objective **[REQUIRED]**

Move `module_service_tests.rs` from `src/services/` to the proper `tests/` directory and update its structure to follow Rust integration test conventions. This improves code organization and makes test discovery more standard.

### Type
- [ ] Bug - Production issue that needs fixing
- [ ] Feature - New functionality or enhancement
- [x] Tech Debt - Code improvement or refactoring
- [ ] Chore - Maintenance or setup work

### Priority
- [ ] P0 - Critical (blocks users/revenue)
- [ ] P1 - High (important for user experience)
- [x] P2 - Medium (nice to have)
- [ ] P3 - Low (when time permits)

### Technical Debt Impact

**Current Problems**:
- `module_service_tests.rs` is located in `src/services/` alongside implementation code
- File is wrapped in `#[cfg(test)] mod tests {}` but contains integration tests, not unit tests
- Integration tests should be in the `tests/` directory per Rust conventions
- Makes test organization confusing and non-standard
- Not referenced in `mod.rs` - orphaned file in source tree

**Benefits of Fixing**:
- Follows standard Rust project structure (integration tests in `tests/`)
- Clearer separation between source code and integration tests
- Each test file in `tests/` is compiled as a separate crate (better isolation)
- Easier for new developers to find and understand test organization
- Consistent with other integration tests already in `tests/` directory

**Risk Assessment**:
- Very low risk: Just moving a file and updating imports
- Tests continue to run exactly the same way
- No functional changes to test logic

## Acceptance Criteria **[REQUIRED]**

- [ ] `module_service_tests.rs` moved from `src/services/` to `tests/` directory
- [ ] Test file structure updated: remove `#[cfg(test)] mod tests {}` wrapper
- [ ] All test functions properly accessible as integration tests
- [ ] `cargo test` passes with all tests running successfully
- [ ] No references to old test file location remain in codebase

## Implementation Notes

### Technical Approach

**Current State**:
- File location: `crates/mimir-dm-core/src/services/module_service_tests.rs`
- File structure: Wrapped in `#[cfg(test)] mod tests {}`
- Contains integration-style tests (full DB setup, migrations, seeding)

**Target State**:
- File location: `crates/mimir-dm-core/tests/module_service_tests.rs`
- File structure: Top-level test functions (no `mod tests {}` wrapper)
- Each test function starts with `#[test]` attribute
- Imports from `mimir_dm_core::` as external crate

**Steps**:
1. Move file: `src/services/module_service_tests.rs` → `tests/module_service_tests.rs`
2. Remove `#[cfg(test)] mod tests {}` wrapper
3. Update imports from `crate::` to `mimir_dm_core::`
4. Verify tests with `cargo test`

### Dependencies

None - this is a self-contained refactoring task.

### Risk Considerations

**Very Low Risk**:
- No logic changes, only file organization
- Tests verify their own functionality
- Easy to revert if issues arise

## Status Updates **[REQUIRED]**

### Implementation Complete - 2025-10-27

Successfully reorganized module service tests to follow standard Rust integration test conventions.

**Changes Made**:
1. **Moved test file**: `src/services/module_service_tests.rs` → `tests/module_service_tests.rs`
2. **Restructured test file**:
   - Removed `#[cfg(test)] mod tests {}` wrapper
   - Changed all imports from `crate::` to `mimir_dm_core::`
   - Test functions now at top level (proper integration test structure)
3. **Cleaned up references**:
   - Removed `#[cfg(test)] #[path = "module_service_tests.rs"] mod tests;` from `module_service.rs`

**Testing Results**:
- All 13 tests pass: ✅
  - test_create_module
  - test_module_numbering
  - test_get_module
  - test_list_campaign_modules
  - test_transition_module_stage
  - test_invalid_transition
  - test_backward_transitions
  - test_update_module
  - test_initialize_module_documents
  - test_check_module_completion
  - test_increment_module_sessions
  - test_find_modules_needing_next
  - test_delete_module
- Test execution time: 0.66s

**Benefits**:
- Follows standard Rust project structure
- Integration tests now in proper `tests/` directory
- Each test file compiled as separate crate (better isolation)
- Clearer separation between source code and tests
- Consistent with other integration tests in the project

All acceptance criteria met:
- [x] `module_service_tests.rs` moved from `src/services/` to `tests/` directory
- [x] Test file structure updated: removed `#[cfg(test)] mod tests {}` wrapper
- [x] All test functions properly accessible as integration tests
- [x] `cargo test` passes with all tests running successfully
- [x] No references to old test file location remain in codebase