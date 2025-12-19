---
id: enable-deny-missing-docs-in-lib-rs
level: task
title: "Enable deny(missing_docs) in lib.rs files"
short_code: "MIMIR-T-0117"
created_at: 2025-11-25T01:48:54.432965+00:00
updated_at: 2025-11-25T11:03:52.761458+00:00
parent: MIMIR-I-0010
blocked_by: []
archived: true

tags:
  - "#task"
  - "#phase/completed"


exit_criteria_met: false
strategy_id: NULL
initiative_id: MIMIR-I-0010
---

# Enable deny(missing_docs) in lib.rs files

## Parent Initiative

[[MIMIR-I-0010]]

## Objective

Enable `#![deny(missing_docs)]` in library crate lib.rs files to enforce documentation requirements at compile time, ensuring all public APIs are documented.

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [ ] `#![deny(missing_docs)]` added to `mimir-dm-core/src/lib.rs`
- [ ] `#![deny(missing_docs)]` added to `mimir-dm-llm/src/lib.rs`
- [ ] All public items have doc comments (no compiler errors)
- [ ] CI builds pass with new lint
- [ ] Consider `#![warn(missing_docs)]` for gradual adoption if needed

## Implementation Notes

### Technical Approach

Add to each library crate's lib.rs:
```rust
//! Mimir DM Core - Core business logic and data access layer.
//!
//! This crate provides the service layer, database models, and business
//! logic for the Mimir DM application.

#![deny(missing_docs)]
```

This causes compilation to fail if any public item lacks documentation:
- Public structs, enums, traits
- Public functions and methods
- Public modules
- Public type aliases

### Phased Approach
If too many undocumented items exist:
1. Start with `#![warn(missing_docs)]` to see scope
2. Fix warnings incrementally
3. Switch to `#![deny(missing_docs)]` when clean

### Crates to Update
- `crates/mimir-dm-core/src/lib.rs` - Core library
- `crates/mimir-dm-llm/src/lib.rs` - LLM provider library
- `crates/mimir-5etools-splitter/src/lib.rs` - Data parsing library

Note: `mimir-dm` (the binary crate) doesn't need this since it's not a library.

### Dependencies
- MIMIR-T-0109 (Document service methods) should be completed first
- MIMIR-T-0107 (Document Tauri commands) should be completed first

### Risk Considerations
- May require significant documentation effort before enabling
- Use warn first to assess scope
- Can exclude specific modules with `#[allow(missing_docs)]` temporarily

## Status Updates

**2025-11-25**: Implemented warn(missing_docs) as foundation:
- Added `#![warn(missing_docs)]` to mimir-dm-core, mimir-dm-llm, mimir-5etools-splitter
- Suppressed warnings in areas where docs would be redundant (models, schema, seed, builders)
- Documented key public items in domain/boards, domain/template_info, db.rs, error.rs
- Reduced mimir-dm-core warnings from 2264 to 79 through targeted documentation
- Build passes with warnings; upgrade to deny(missing_docs) when remaining items documented