---
id: 001-service-layer-pattern
level: adr
title: "Service Layer Pattern Standardization"
number: 1
short_code: "MIMIR-A-0005"
created_at: 2025-11-25T02:17:15.090411+00:00
updated_at: 2025-11-25T02:18:08.011749+00:00
decision_date: 
decision_maker: 
parent: 
archived: false

tags:
  - "#adr"
  - "#phase/decided"


exit_criteria_met: false
strategy_id: NULL
initiative_id: NULL
---

# ADR-5: Service Layer Pattern Standardization

## Context

The mimir codebase has 25+ catalog services that have evolved organically, resulting in two distinct implementation patterns:

**Pattern A - Stateful Services** (MonsterService, ItemService, TableService, etc.):
```rust
pub struct MonsterService<'a> {
    pub conn: &'a mut SqliteConnection,
}

impl<'a> MonsterService<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self { ... }
    pub fn search_monsters(&mut self, filters: MonsterFilters) -> Result<Vec<MonsterSummary>> { ... }
}
```

**Pattern B - Stateless Services** (SpellService, ClassService):
```rust
pub struct SpellService;

impl SpellService {
    pub fn search_spells(conn: &mut SqliteConnection, filters: SpellFilters) -> Result<Vec<SpellSummary>> { ... }
}
```

This inconsistency causes:
1. Difficulty implementing generic traits (e.g., `CatalogService`) across all services
2. Confusion for developers about which pattern to use for new services
3. Inconsistent API ergonomics between different entity types
4. Need for wrapper types (e.g., `SpellServiceStateful`) to bridge the patterns

## Decision

We will standardize on **Pattern A (Stateful Services)** for all catalog services, with the following structure:

```rust
pub struct EntityService<'a> {
    pub conn: &'a mut SqliteConnection,
}

impl<'a> EntityService<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }
    
    // Instance methods for queries
    pub fn search(&mut self, filters: EntityFilters) -> Result<Vec<EntitySummary>> { ... }
    pub fn get_by_name_and_source(&mut self, name: &str, source: &str) -> Result<Option<Entity>> { ... }
    pub fn get_sources(&mut self) -> Result<Vec<String>> { ... }
}

// Static methods ONLY for operations that don't fit the instance pattern
impl EntityService<'_> {
    pub fn import_from_book(conn: &mut SqliteConnection, path: &Path, source: &str) -> Result<usize> { ... }
    pub fn remove_by_source(conn: &mut SqliteConnection, source: &str) -> Result<usize> { ... }
}
```

**Key points:**
- All query methods are instance methods on `&mut self`
- Import/removal operations remain static (they're batch operations typically called during setup)
- All services implement the `CatalogService` trait for generic operations

## Alternatives Analysis

| Option | Pros | Cons | Risk Level | Implementation Cost |
|--------|------|------|------------|-------------------|
| **A: Stateful (chosen)** | Clean API, trait-friendly, follows Rust idioms | Requires wrapper for legacy code, lifetime annotations | Low | Medium (gradual migration) |
| **B: Stateless** | No lifetimes, simpler signatures | Can't implement traits easily, verbose call sites | Low | High (rewrite trait approach) |
| **C: Hybrid (both patterns)** | Backward compatible, flexible | Inconsistent, confusing, duplicate code | Medium | Low |

## Rationale

The stateful pattern was chosen because:

1. **Trait Implementation**: Rust traits require `&self` or `&mut self` receivers. Stateful services naturally implement `CatalogService` trait without wrappers.

2. **API Ergonomics**: Cleaner call sites:
   ```rust
   // Stateful (cleaner)
   let mut service = MonsterService::new(conn);
   let monsters = service.search(filters)?;
   let sources = service.get_sources()?;
   
   // Stateless (verbose)
   let monsters = SpellService::search_spells(conn, filters)?;
   let sources = SpellService::get_spell_sources(conn)?;
   ```

3. **Rust Idioms**: The stateful pattern follows common Rust patterns for database access (similar to Diesel's approach).

4. **Existing Majority**: Most services (18 of 25) already use the stateful pattern, minimizing migration.

5. **Tauri Integration**: Stateful services work better with Tauri's state management for connection pooling.

## Consequences

### Positive
- Unified API across all 25+ catalog services
- All services can implement `CatalogService` trait without wrappers
- Generic Tauri command handlers become possible
- Clearer code patterns for new service development
- Better IDE support (method completion on service instances)

### Negative
- SpellService and ClassService require migration
- Temporary `SpellServiceStateful` wrapper needed during transition
- Slight increase in boilerplate (constructor, lifetime annotations)

### Neutral
- Import methods remain static (appropriate for batch operations)
- No performance impact (connection reference is zero-cost)

## Migration Guide

### For Existing Stateless Services

1. Add connection field and constructor:
```rust
pub struct SpellService<'a> {
    pub conn: &'a mut SqliteConnection,
}

impl<'a> SpellService<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }
}
```

2. Convert static methods to instance methods:
```rust
// Before
pub fn search_spells(conn: &mut SqliteConnection, filters: SpellFilters) -> Result<...>

// After  
pub fn search_spells(&mut self, filters: SpellFilters) -> Result<...> {
    // Replace `conn` with `self.conn`
}
```

3. Keep import/removal as static methods (these are batch operations).

4. Implement `CatalogService` trait.

### For Tauri Commands

```rust
#[tauri::command]
pub fn search_spells(db: State<DbConnection>, filters: SpellFilters) -> Result<Vec<SpellSummary>> {
    let mut conn = db.get()?;
    let mut service = SpellService::new(&mut conn);
    service.search_spells(filters)
}
```