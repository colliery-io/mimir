---
id: foundation-hardening
level: initiative
title: "Foundation Hardening"
short_code: "MIMIR-I-0008"
created_at: 2025-11-24T20:11:49.278726+00:00
updated_at: 2025-11-25T01:42:22.623800+00:00
parent: 
blocked_by: []
archived: true

tags:
  - "#initiative"
  - "#phase/completed"


exit_criteria_met: false
estimated_complexity: XL
strategy_id: NULL
initiative_id: foundation-hardening
---

# Foundation Hardening Initiative

## Context

A comprehensive code review identified critical stability and reliability issues in the Mimir codebase:

- **Panic risks**: `expect()` calls in `main.rs` can crash the application on database initialization failures
- **Untested critical paths**: 20 catalog services have zero integration tests
- **Inconsistent error handling**: 19 services return `Result<T, String>` instead of typed `DbError`
- **Logging inconsistency**: Mixed use of `println!`/`eprintln!` vs `tracing` macros
- **Missing test infrastructure**: No coverage reporting in CI

This initiative addresses all P0 (critical) issues before any new feature work begins.

## Goals & Non-Goals

**Goals:**
- Eliminate all panic points in production code paths
- Achieve integration test coverage for all 20 catalog services
- Standardize error types across all services to use `DbError`
- Establish test coverage reporting with minimum threshold
- Complete missing catalog data seeding (cults, psionics, tables)
- Standardize logging to use `tracing` consistently

**Non-Goals:**
- Refactoring service patterns (stateful vs stateless) - deferred to Phase 2
- Documentation improvements - deferred to Phase 2
- Architecture changes (state consolidation, service splitting) - deferred to Phase 2
- E2E testing - deferred to Phase 2

## Detailed Design

### 1. Panic Point Remediation
Replace `expect()` calls in `main.rs` with graceful error handling that logs errors and exits cleanly rather than panicking.

### 2. Error Type Standardization
Migrate all catalog services from `Result<T, String>` to `Result<T, DbError>`. Services grouped by domain:
- **Group A**: spell, class, race, background, feat (core character creation)
- **Group B**: monster, action, condition (combat/encounters)
- **Group C**: item, vehicle, object, trap, reward (equipment/loot)
- **Group D**: deity, cult, language, psionic, variant_rule, optional_feature (lore/rules)

### 3. Integration Test Coverage
Add integration tests for each catalog service following existing patterns in `tests/integrations/`. Each service needs tests for:
- Search with various filter combinations
- Edge cases (empty results, invalid filters)
- Error scenarios

### 4. Test Infrastructure
- Integrate `cargo-tarpaulin` for coverage reporting
- Add coverage step to CI workflow
- Set initial threshold at 60%

### 5. Catalog Data Completeness
- Seed missing catalog data for cults, psionics, tables
- Create backend commands for tables catalog

### 6. Logging Standardization
Replace all `println!`/`eprintln!` calls in services with appropriate `tracing` macros.

## Exit Criteria

- [x] No `expect()` or `unwrap()` calls in `main.rs` initialization path
- [x] All 20 catalog services return `Result<T, DbError>`
- [x] All 20 catalog services have integration tests
- [x] CI reports test coverage on every PR
- [x] Coverage threshold of 60% enforced
- [x] All catalog data seeded (including cults, psionics, tables)
- [x] Backend commands exist for tables catalog
- [x] No `println!`/`eprintln!` in service layer code