---
id: testing-harness-cleanup
level: task
title: "Testing Harness Cleanup"
short_code: "MIMIR-T-0039"
created_at: 2025-10-28T10:43:51.021930+00:00
updated_at: 2025-10-28T10:43:51.021930+00:00
parent: 
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/backlog"
  - "#tech-debt"


  - "#tech-debt"
exit_criteria_met: false
strategy_id: NULL
initiative_id: NULL
---

# Testing Harness Cleanup

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[Parent Initiative]]

## Objective **[REQUIRED]**

Refactor and consolidate the test execution patterns and organization across the Mimir workspace to reduce complexity, improve maintainability, and establish clear testing patterns.

## Backlog Item Details **[CONDITIONAL: Backlog Item]**

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

### Technical Debt Impact **[CONDITIONAL: Tech Debt]**
- **Current Problems**: 
  - Too many test commands in angreal (all, unit, backend, frontend, integration, e2e, coverage)
  - Test organization is inconsistent across crates
  - Unclear separation between unit tests, integration tests, and e2e tests
  - Test file structure has misplaced files (e.g., module_service_tests.rs in tests/ instead of src/)
  - Both integration_test.rs and integrations/ directory exist in mimir-dm-core
  - Redundant test execution patterns across different commands

- **Benefits of Fixing**: 
  - Clearer test execution model with fewer, more meaningful commands
  - Better organized test files following Rust conventions
  - Faster feedback loops with properly separated unit vs integration tests
  - Easier onboarding for new contributors
  - More maintainable test suite

- **Risk Assessment**: 
  - Developers may waste time figuring out which test command to use
  - Test suites may become harder to maintain as project grows
  - CI/CD pipelines may become unnecessarily complex
  - Test execution time may increase due to poor organization

## Acceptance Criteria **[REQUIRED]**

- [ ] Test commands reduced to 3-4 essential commands (e.g., test, test:watch, test:coverage)
- [ ] All unit tests live in src/ files with #[cfg(test)] modules
- [ ] Integration tests properly organized in tests/ directory following Rust conventions
- [ ] Clear documentation of when to use each test command
- [ ] CI pipeline updated to use the new test organization
- [ ] Test execution time documented for each test type
- [ ] All existing tests still pass after reorganization



## Implementation Notes **[CONDITIONAL: Technical Task]**

### Technical Approach

#### Current State Analysis
- **Angreal Commands**: 7 test commands (all, unit, backend, frontend, integration, e2e, coverage)
- **Test Organization Issues**:
  - `crates/mimir-dm-core/tests/module_service_tests.rs` - Should be in src/ as unit tests
  - `crates/mimir-dm-core/tests/integration_test.rs` AND `tests/integrations/` - Inconsistent structure
  - `crates/mimir-dm-llm/tests/` - Has integration tests that require external dependencies (Ollama)

#### Proposed Refactoring

**1. Consolidate Test Commands** (.angreal/task_test.py)
- Keep: `test` (runs all tests), `test:watch` (watch mode), `test:coverage` (coverage reporting)
- Remove: separate backend/frontend/unit/integration/e2e commands (use cargo/npm directly or flags)
- Simplify: Use cargo workspace test features and npm scripts for filtering

**2. Reorganize Test Files**
- Move unit tests to `src/` with `#[cfg(test)]` modules
- Keep integration tests in `tests/` directories
- Standardize integration test structure across all crates
- Remove duplicate/conflicting test files

**3. Update CI Pipeline**
- Align CI test execution with new command structure
- Document test execution time for each layer

### Dependencies
- None - this is a standalone refactoring

### Risk Considerations
- Breaking existing developer workflows - mitigation: document changes clearly
- CI pipeline disruption - mitigation: update CI in same PR
- Test failures during reorganization - mitigation: verify all tests pass before and after

## Status Updates **[REQUIRED]**

*To be added during implementation*