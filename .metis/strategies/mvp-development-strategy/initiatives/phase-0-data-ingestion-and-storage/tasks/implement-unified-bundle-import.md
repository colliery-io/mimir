---
id: implement-unified-bundle-import
level: task
title: "Implement Unified Bundle Import Pipeline"
created_at: 2025-07-30T18:00:00+00:00
updated_at: 2025-07-30T18:00:00+00:00
parent: phase-0-data-ingestion-and-storage
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/todo"

exit_criteria_met: false
estimated_complexity: L
---

# Implement Unified Bundle Import Pipeline

## Description

Create a single, unified import pipeline that can import standardized D&D 5e bundle archives (.tar.gz) into the Mimir database. This replaces the previously planned multiple processor architecture with a simpler, single-pipeline approach that leverages Python preprocessing.

## Acceptance Criteria

- [ ] Create `mimir-dm-import` crate for import functionality
- [ ] Add `import` subcommand to main `mimir` CLI
- [ ] Implement bundle validation (manifest.json, file integrity)
- [ ] Build atomic transaction support (all-or-nothing imports)
- [ ] Add progress reporting with user feedback
- [ ] Support both core and full D&D 5e bundle formats
- [ ] Implement error handling and recovery
- [ ] Add import status logging and history

## Technical Architecture

### Bundle Import Flow
1. **Validation Phase**: Verify bundle structure and manifest
2. **Loading Phase**: Extract and parse all entity files 
3. **Transformation Phase**: Convert to database format
4. **Import Phase**: Atomic database transaction
5. **Verification Phase**: Confirm successful import

### Key Components
- `BundleImporter` struct for orchestrating imports
- `BundleValidator` for pre-import validation
- `EntityTransformer` for format conversion
- CLI integration with progress bars
- Atomic transaction management

## Dependencies

- Depends on: completed SQLite schema (mimir-dm-db)
- Depends on: bundle format specification
- Depends on: Python bundle generation scripts

## Implementation Notes

This unified approach eliminates the need for:
- Multiple DataProcessor trait implementations
- Separate spell/creature/item processors
- Complex processor registration system

Instead, it focuses on a single clean workflow from bundle → database that leverages the existing repository patterns in `mimir-dm-db`.

## Exit Criteria

- [ ] Can import dnd5e-2014-core bundle successfully
- [ ] Can import dnd5e-2014-full bundle successfully
- [ ] CLI provides clear progress feedback
- [ ] Failed imports leave database unchanged
- [ ] Import history is properly logged
- [ ] Performance is acceptable (< 30 seconds for full bundle)