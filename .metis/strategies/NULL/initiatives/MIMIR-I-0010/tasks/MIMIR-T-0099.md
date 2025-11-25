---
id: create-generic-catalogsearchtrait
level: task
title: "Create generic CatalogSearchTrait"
short_code: "MIMIR-T-0099"
created_at: 2025-11-25T01:48:45.341858+00:00
updated_at: 2025-11-25T02:16:50.700040+00:00
parent: MIMIR-I-0010
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/active"


exit_criteria_met: false
strategy_id: NULL
initiative_id: MIMIR-I-0010
---

# Create generic CatalogSearchTrait

## Parent Initiative

[[MIMIR-I-0010]]

## Objective

Create a generic `CatalogSearchTrait` that unifies the common patterns across all 20+ catalog services, reducing code duplication and establishing a consistent interface for catalog operations.

## Acceptance Criteria

## Acceptance Criteria

- [x] Define `CatalogSearchTrait` with common methods: `search`, `get_by_name_and_source`, `get_sources`
- [x] Define associated types for filters, summaries, and full entities
- [x] Implement the trait for at least 3 representative services (SpellServiceStateful, MonsterService, ItemService, TableService)
- [ ] Create generic Tauri command handlers that work with trait objects or generics (deferred - significant architectural change)
- [x] Document the trait with examples
- [x] All existing tests continue to pass (351 tests pass)

## Implementation Notes

### Technical Approach

The 20+ catalog services follow nearly identical patterns:
- `search_*(filters) -> Vec<*Summary>`
- `get_*_by_id(id) -> Option<Full*>`
- `get_*_by_name_and_source(name, source) -> Option<Full*>`
- `get_*_sources() -> Vec<String>`
- `import_*_from_book(conn, path, source) -> Result<usize>`

Define a trait:
```rust
pub trait CatalogService {
    type Filters: Default;
    type Summary;
    type Full;
    
    fn search(&mut self, filters: Self::Filters) -> Result<Vec<Self::Summary>>;
    fn get_by_id(&mut self, id: i32) -> Result<Option<Self::Full>>;
    fn get_sources(&mut self) -> Result<Vec<String>>;
}
```

### Files to Modify
- `crates/mimir-dm-core/src/services/mod.rs` - Define trait
- `crates/mimir-dm-core/src/services/spell_service.rs` - Implement trait
- `crates/mimir-dm-core/src/services/monster_service.rs` - Implement trait
- `crates/mimir-dm-core/src/services/item_service.rs` - Implement trait

### Dependencies
- MIMIR-T-0101 (ADR for service pattern) should be completed first to establish the pattern

### Risk Considerations
- Trait objects may have performance implications if used extensively
- Need to balance abstraction with Rust's type system constraints

## Status Updates

**2025-11-24**: Core implementation complete
- Created `CatalogService` trait in `catalog_trait.rs` with full documentation
- Implemented for 4 services: SpellServiceStateful, MonsterService, ItemService, TableService
- Added `SpellServiceStateful` wrapper to handle SpellService's static method pattern
- Added missing `get_monster_sources()` method to MonsterService
- Added `Default` derive to ItemFilters and MonsterFilters
- All 351 tests pass
- Commit: 9b73935

Generic Tauri command handlers deferred as a larger architectural change that would benefit from the ADR in MIMIR-T-0101 first.