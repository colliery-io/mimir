---
id: codebase-technical-debt-cleanup
level: initiative
title: "Codebase Technical Debt Cleanup and Refactoring"
short_code: "MIMIR-I-0001"
created_at: 2025-10-24T11:49:04.099120+00:00
updated_at: 2025-10-24T12:07:37.897759+00:00
parent: MIMIR-V-0001
blocked_by: []
archived: false

tags:
  - "#initiative"
  - "#phase/active"


exit_criteria_met: false
estimated_complexity: XL
strategy_id: NULL
initiative_id: codebase-technical-debt-cleanup
---

# Codebase Technical Debt Cleanup and Refactoring Initiative

## Context

Following a deep technical assessment of the Mimir codebase (215 Rust files, 3,964 TypeScript/Vue files), significant technical debt has been identified stemming from an incomplete migration from in-memory file-based catalogs to database-backed services. This has resulted in:

- ~9,000 lines of dead code including backup files and replaced implementations
- Duplicate catalog implementations (in-memory vs database-backed)
- Monolithic files exceeding 1,000-2,800 lines
- Business logic misplaced in UI command handlers instead of core services
- Inconsistent architectural patterns across the codebase

The codebase is functional but shows signs of rapid development without cleanup, leading to confusion for developers and increased maintenance burden.

## Goals & Non-Goals

**Goals:**
- Remove all dead code, backup files, and replaced implementations (~9,000 lines)
- Complete the catalog migration to database-backed services consistently
- Decompose monolithic files (>1,000 lines) into maintainable modules (<500 lines)
- Move business logic from command handlers to proper service layer
- Establish consistent architectural patterns across frontend and backend
- Improve codebase navigability and developer onboarding

**Non-Goals:**
- Changing functionality or user-facing features
- Performance optimization (unless directly related to refactoring)
- Adding new features
- Rewriting working code that follows good patterns





## Architecture

### Current State Issues

**1. Incomplete Catalog Migration**
- Mixed implementation: some catalogs use database, others use in-memory state
- Still in-memory: ItemCatalog, MonsterCatalog, ClassCatalog, DeityCatalog, TableCatalog, SpellCatalog, ActionCatalog, ConditionCatalog
- Already migrated: Background, Reward, Language, Feat, Race, Object, Trap, Cult, Psionic, Variant Rule, Optional Feature, Vehicle

**2. Dual Database Connection Patterns**
- UI Layer (db_connection.rs): r2d2 connection pooling with global DB_POOL
- Core Layer (connection.rs): Direct connections with async wrappers
- Inconsistent state handling between layers

**3. Misplaced Business Logic**
- Catalog services implemented in command handlers (mimir-dm/commands) instead of core services
- Example: ClassCatalog, DeityCatalog contain file I/O and data loading logic
- Should be: Command handlers as thin facades calling core services

### Target Architecture

**Catalog Services:**
```
Frontend -> Tauri Command (thin facade) -> Core Service -> DAL -> Database
```

**Module Structure:**
- Large monolithic services split by entity type
- catalog_service.rs (2,868 lines) -> catalog/ module with entity-specific files
- llm_service.rs (1,288 lines) -> llm/ module with concern-specific files
- books.rs (1,111 lines) -> books/ module split by responsibility

## Detailed Design

### Phase 1: Dead Code Removal

**Files to Delete:**
- catalog_background_old.rs (241 lines)
- catalog_reward_old.rs (250 lines)  
- catalog_language_old.rs (255 lines)
- modules.rs.bak (318 lines)
- catalog_monster_db.rs:5:12 (corrupted)
- catalog_feat.rs (replaced by catalog_feat_db.rs)
- catalog_race.rs (replaced by catalog_race_db.rs)
- catalog_object.rs (replaced by catalog_object_db.rs)
- catalog_trap.rs (replaced by catalog_trap_db.rs)
- catalog_cult.rs (replaced by catalog_cult_db.rs)
- catalog_psionic.rs (replaced by catalog_psionic_db.rs)
- catalog_variant_rule.rs (replaced by catalog_variant_rule_db.rs)
- catalog_optionalfeature.rs (replaced by catalog_optional_feature_db.rs)

**Code Cleanup:**
- Remove commented imports in mod.rs (lines 16-37)
- Address 8 TODO/FIXME comments

### Phase 2: Catalog Migration Completion

**Create DB-backed versions for:**
- ItemCatalog (from catalog.rs)
- MonsterCatalog (from catalog.rs)
- ClassCatalog (from catalog_class.rs)
- DeityCatalog (from catalog_deity.rs)
- SpellCatalog (from catalog_spell.rs)
- TableCatalog (from catalog_table.rs)
- ActionCatalog (from catalog_action.rs)
- ConditionCatalog (from catalog_condition.rs)

**Pattern to follow:**
```rust
// In catalog_*_db.rs
#[tauri::command]
pub async fn search_entity(
    query: Option<String>,
    filters: EntityFilters,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<Vec<Entity>, String> {
    let mut conn = db_service.get_connection()?;
    EntityService::search(&mut conn, filters)
        .map_err(|e| format!("Search failed: {}", e))
}
```

**Update main.rs:**
- Remove in-memory catalog Mutex state (lines 112-142)
- Remove catalog .manage() calls

### Phase 3: Monolithic File Decomposition

**catalog_service.rs (2,868 lines) -> catalog/ module:**
```
services/catalog/
├── mod.rs (re-exports)
├── spell.rs
├── monster.rs
├── item.rs
├── class.rs
├── feat.rs
├── race.rs
├── background.rs
├── action.rs
├── condition.rs
└── ... (one per entity type)
```

**llm_service.rs (1,288 lines) -> llm/ module:**
```
services/llm/
├── mod.rs
├── service.rs (core orchestration)
├── ollama_client.rs
├── tool_executor.rs
├── token_counter.rs
└── chat_macros.rs
```

**books.rs (1,111 lines) -> books/ module:**
```
commands/books/
├── mod.rs
├── upload.rs (archive handling)
├── import.rs (DB import)
└── validate.rs (validation)
```

**Frontend:**
- useCatalog.ts -> composables/catalog/ by entity type
- chat.ts -> stores/chat/ by concern
- Large Vue components -> extract reusable child components

### Phase 4: Architecture Standardization

**Database Connections:**
- Standardize on r2d2 pooled connections
- Update core services to use pooled connections
- Remove direct connection pattern

**Business Logic Migration:**
- Move catalog logic from commands to core services
- Command handlers become thin facades
- Core services handle all business logic

**State Management:**
- Replace global OnceLock/Mutex with Tauri state
- Use dependency injection via State extractors

**Error Handling:**
- Standardize on descriptive error format: `format!("Operation failed: {}", e)`
- Include context in all error messages

### Phase 5: Quality & Documentation

**Testing:**
- Move module_service_tests.rs to proper location
- Ensure consistent test structure

**Dependencies:**
- Remove unused workspace dependencies
- Standardize workspace vs local declarations

**Documentation:**
- Document architectural decisions
- Create contribution guidelines
- Add module-level documentation



## Testing Strategy

### Validation Approach
- After each phase, run full test suite to ensure no regressions
- Verify all Tauri commands still function correctly
- Manual smoke testing of catalog search/retrieval functionality
- Ensure build succeeds after each deletion/refactor

### Risk Mitigation
- Use feature branches for each phase
- Create commits after each file deletion/refactor for easy rollback
- Test in development environment before merging
- Keep old code in git history for reference if needed

## Alternatives Considered

**Alternative 1: Complete Rewrite**
Rejected because working functionality exists and business value would be lost during rewrite. Incremental refactoring allows continuous delivery while improving code quality.

**Alternative 2: Leave As-Is**
Rejected because technical debt is actively hindering development velocity and onboarding. The codebase will become increasingly difficult to maintain without intervention.

**Alternative 3: Big Bang Refactor**
Rejected in favor of phased approach to manage risk and allow continuous integration. Five phases allow incremental progress with validation at each step, reducing risk of breaking changes.

## Implementation Plan

### Phase 1: Low-Hanging Fruit (1-2 days)
**Objective:** Remove dead code and clarify codebase

**Tasks:**
- Delete 3 `_old` catalog files
- Delete 8 replaced catalog implementations  
- Remove corrupted file
- Clean up commented imports in mod.rs (lines 16-37)
- Address 8 TODO/FIXME comments

**Impact:** Remove ~9,000 lines of dead code, reduce developer confusion

### Phase 2: Complete Catalog Migration (1 week)
**Objective:** Finish database migration for all catalogs

**Tasks:**
- Create `_db` versions for: item, monster, class, deity, spell, table, action, condition catalogs
- Update main.rs to remove in-memory catalog state (lines 112-142)
- Delete old catalog implementation files
- Verify all catalog commands use database services

**Impact:** Consistent data access, reduced memory footprint, ~3,000 lines removed

### Phase 3: Decompose Monolithic Files (2 weeks)
**Objective:** Break up large files into maintainable modules

**Rust:**
- Split catalog_service.rs (2,868 lines) into entity-specific modules
- Split llm_service.rs (1,288 lines) into focused modules
- Split books.rs (1,111 lines) by concern

**Frontend:**
- Split useCatalog.ts (1,872 lines) by entity type
- Split chat store (859 lines) by concern
- Extract components from large Vue files

**Impact:** Easier navigation, better organization, clearer boundaries

### Phase 4: Architectural Cleanup (2-3 weeks)
**Objective:** Fix architectural inconsistencies

**Tasks:**
- Standardize database connection pattern
- Move business logic from command handlers to core services
- Refactor global state to use Tauri state management
- Decompose large Vue components
- Investigate and cleanup duplicate formatters

**Impact:** Clearer architecture, better separation of concerns, easier testing

### Phase 5: Polish & Documentation (1 week)
**Objective:** Clean up remaining issues

**Tasks:**
- Standardize error handling patterns
- Fix test organization
- Dependency cleanup in Cargo.toml
- Document architectural decisions
- Create contribution guidelines

**Impact:** Consistent code quality, easier onboarding

### Total Timeline: 5-7 weeks

### Success Metrics
- Code reduction: ~12,000 lines removed (~10% of Rust codebase)
- All files <500 lines except generated code
- Zero in-memory catalogs remaining
- Consistent architectural patterns across codebase
- Improved build times and test execution
- Zero functionality regressions