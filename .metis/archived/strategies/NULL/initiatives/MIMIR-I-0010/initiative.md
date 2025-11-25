---
id: maintainability-and-documentation
level: initiative
title: "Maintainability and Documentation"
short_code: "MIMIR-I-0010"
created_at: 2025-11-24T20:11:49.344461+00:00
updated_at: 2025-11-25T11:33:06.663806+00:00
parent: 
blocked_by: []
archived: true

tags:
  - "#initiative"
  - "#phase/completed"


exit_criteria_met: false
estimated_complexity: XL
strategy_id: NULL
initiative_id: maintainability-and-documentation
---

# Maintainability and Documentation Initiative

## Context

Following completion of Foundation Hardening (MIMIR-I-0008), this initiative addresses the remaining technical debt and documentation gaps identified in the code review:

- **Code duplication**: 19 catalog services duplicate identical search/filter patterns
- **Inconsistent service patterns**: Mix of stateful (`<'a>`) and stateless services with no clear rationale
- **Large services**: CharacterService at 1,588 lines handles too many responsibilities
- **Heavy state management**: 7+ separate Arc-wrapped states in Tauri instead of consolidated AppState
- **Documentation gaps**: Only ~15% of public APIs have doc comments
- **Missing test infrastructure**: No mocking for LLM tests, tool confirmation has no timeout handling

This initiative focuses on long-term maintainability improvements after stability is assured.

## Goals & Non-Goals

**Goals:**
- Eliminate code duplication through generic catalog search abstraction
- Establish consistent service patterns across all services
- Break large services into focused, single-responsibility components
- Consolidate Tauri state management into single AppState
- Document all public APIs (Tauri commands, service methods)
- Improve test infrastructure with mocking and timeout handling
- Reorganize command handlers by responsibility

**Non-Goals:**
- Adding new features
- Changing external APIs or behavior
- Database schema changes
- Frontend refactoring (beyond TypeScript interfaces for API docs)

## Detailed Design

### 1. Generic Catalog Search Trait
Create `CatalogSearchService<T>` trait that encapsulates the common search pattern:
- Filter application
- Pagination
- Result transformation
- Error handling

Implement for all 19 catalog services, reducing duplication significantly.

### 2. Service Pattern Normalization
Audit all services and standardize on one pattern:
- **Recommendation**: Stateful pattern with `&'a mut DbConnection` for services that need transactions
- **Alternative**: Stateless with connection passed to each method for simple services
- Document the decision in an ADR

### 3. CharacterService Decomposition
Split into focused services:
- `CharacterCreationService` - Initial character creation, builder pattern
- `CharacterVersioningService` - Snapshots, version history, rollback
- `CharacterProgressionService` - Level up, ASI, multiclassing
- `CharacterEquipmentService` - Inventory, equipment, encumbrance

### 4. Tauri State Consolidation
Replace multiple `app.manage()` calls with single `AppState` struct:
```rust
pub struct AppState {
    pub db: Arc<DatabaseService>,
    pub context: Arc<Mutex<ContextService>>,
    pub llm: Arc<Mutex<Option<LlmService>>>,
    pub paths: Arc<AppPaths>,
    // ... other state
}
```

### 5. Documentation
- Add `#![deny(missing_docs)]` to enforce documentation on new code
- Document all 50+ Tauri commands with params, returns, errors
- Document all public service methods
- Add module-level documentation to complex services

### 6. Test Infrastructure Improvements
- Add `wiremock` or `mockall` for HTTP mocking in LLM tests
- Add timeout handling to tool confirmation flow
- Create shared test data builders

### 7. Commands Reorganization
Restructure `commands/` directory:
```
commands/
  query/        # Read-only catalog queries
  campaign/     # Campaign, module, session management
  character/    # Character and player operations
  llm/          # Chat and LLM operations
  system/       # Settings, context, window management
```

## Exit Criteria

- [x] Generic CatalogSearchTrait implemented with catalog_commands! macro for Tauri handlers
- [x] All services follow consistent pattern (documented in ADR-005)
- [x] CharacterService split into focused services (creation, progression, spells, inventory)
- [x] Single AppState struct replaces multiple managed states
- [x] All Tauri commands have doc comments with params/returns/errors
- [x] All public service methods have doc comments
- [x] LLM providers unified on OpenAI-compatible endpoints (MIMIR-T-0119)
- [x] Tool confirmation flow has configurable timeout
- [x] Commands directory reorganized by responsibility (6 subdirectories)
- [x] `#![warn(missing_docs)]` enabled in lib.rs files (foundation for future enforcement)