---
id: code-maintainability-refactoring
level: initiative
title: "Code Maintainability Refactoring"
short_code: "MIMIR-I-0020"
created_at: 2025-12-17T13:34:02.297933+00:00
updated_at: 2025-12-17T13:34:02.297933+00:00
parent: MIMIR-V-0001
blocked_by: []
archived: false

tags:
  - "#initiative"
  - "#phase/discovery"


exit_criteria_met: false
estimated_complexity: L
strategy_id: NULL
initiative_id: code-maintainability-refactoring
---

# Code Maintainability Refactoring Initiative

## Context

A comprehensive code review identified several maintainability issues in the codebase:
- **Large files** exceeding 1,000 lines that are difficult to navigate and maintain
- **Inconsistent error handling** with three different patterns (DbError, anyhow, String)
- **Code duplication** in catalog service search implementations
- **Complex nested types** without type aliases

## Goals & Non-Goals

**Goals:**
- Split large files (>1,000 lines) into focused, single-responsibility modules
- Unify error handling strategy across all crates
- Reduce code duplication through macros or shared abstractions
- Improve code navigation and discoverability

**Non-Goals:**
- Changing public API surface (unless necessary for error unification)
- Performance optimization (separate initiative)
- Adding new features

## Detailed Design

### Phase 1: Split Large Files

| File | Lines | Split Into |
|------|-------|------------|
| `mimir-dm-print/src/service.rs` | 1,456 | `pdf_builder.rs`, `typst_renderer.rs`, `data_aggregator.rs` |
| `character/character.rs` | 1,337 | `character_abilities.rs`, `character_spells.rs`, `character_inventory.rs` |
| `character_write_tools.rs` | 1,272 | `ability_tools.rs`, `spell_tools.rs`, `inventory_tools.rs` |
| `models/catalog/data.rs` | 809 | `ability_scores.rs`, `spell_slots.rs`, `character_version.rs` |

### Phase 2: Unify Error Handling

Current state:
```rust
// Pattern 1: Custom DbError
pub type Result<T> = std::result::Result<T, DbError>;

// Pattern 2: Anyhow  
anyhow::Result<T>

// Pattern 3: String errors (Tauri commands)
Result<T, String>
```

Target state:
- Use `thiserror` types in core library (`MimirError` enum)
- Convert to `String` only at the Tauri command boundary
- Remove `anyhow` from core crate, keep only in binary crates

### Phase 3: Reduce Catalog Service Duplication

Create macro for repetitive search implementations:
```rust
catalog_search_impl!(SpellService, CatalogSpell, catalog_spells);
catalog_search_impl!(FeatService, CatalogFeat, catalog_feats);
```

## Alternatives Considered

1. **Leave as-is**: Rejected - files are too large to navigate effectively
2. **Full rewrite**: Rejected - too risky, incremental refactoring is safer
3. **Extract to separate crates**: Considered for error handling, but adds complexity

## Implementation Plan

1. Split `service.rs` (print crate) - highest impact, most complex file
2. Split `character.rs` and `character_write_tools.rs` - related modules
3. Unify error types with new `MimirError` enum
4. Create catalog service macro for search implementations
5. Update all imports and tests