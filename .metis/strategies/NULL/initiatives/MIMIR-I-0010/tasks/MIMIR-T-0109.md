---
id: document-all-public-service-methods
level: task
title: "Document all public service methods"
short_code: "MIMIR-T-0109"
created_at: 2025-11-25T01:48:54.008811+00:00
updated_at: 2025-11-25T01:48:54.008811+00:00
parent: MIMIR-I-0010
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/todo"


exit_criteria_met: false
strategy_id: NULL
initiative_id: MIMIR-I-0010
---

# Document all public service methods

## Parent Initiative

[[MIMIR-I-0010]]

## Objective

Add comprehensive documentation comments to all public methods in the mimir-dm-core service layer, enabling better code understanding and preparing for `deny(missing_docs)`.

## Acceptance Criteria

- [ ] All public service methods have `///` doc comments
- [ ] Doc comments include: purpose, parameters, return type, error conditions
- [ ] Module-level documentation (`//!`) added to each service file
- [ ] `cargo doc --package mimir-dm-core` generates clean documentation
- [ ] Documentation follows Rust API guidelines

## Implementation Notes

### Technical Approach

Current state: Services have minimal documentation:
```rust
pub fn search_spells(&mut self, filters: SpellFilters) -> Result<Vec<SpellSummary>> {
```

Target state:
```rust
//! Spell catalog service for searching and retrieving spell data.
//!
//! This service provides access to the spell catalog stored in the database,
//! supporting full-text search and filtering by level, school, and source.

/// Search the spell catalog with optional filters.
///
/// Performs a database query against the catalog_spells table, applying
/// any provided filters. Results are sorted alphabetically by name.
///
/// # Arguments
/// * `filters` - Search criteria including name query, levels, schools, sources
///
/// # Returns
/// * `Ok(Vec<SpellSummary>)` - List of matching spell summaries
/// * `Err(DbError)` - If the database query fails
///
/// # Example
/// ```ignore
/// let mut service = SpellService::new(conn);
/// let filters = SpellFilters { name: Some("fire".into()), ..Default::default() };
/// let spells = service.search_spells(filters)?;
/// ```
pub fn search_spells(&mut self, filters: SpellFilters) -> Result<Vec<SpellSummary>> {
```

### Services to Document (27 files)
- Core: `campaign_service.rs`, `module_service.rs`, `session_service.rs`
- Characters: `character/` module (multiple files)
- Catalog: All 20+ catalog services (spell, monster, item, etc.)
- Other: `document_service.rs`, `template_service.rs`, `player_service.rs`

### Dependencies
- Should be done before MIMIR-T-0117 (deny(missing_docs))

### Risk Considerations
- Time-consuming but low risk
- Incremental approach recommended

## Status Updates

*To be added during implementation*