---
id: design-standard-import-bundle-format
level: task
title: "Design Standard Import Bundle Format"
created_at: 2025-07-30T15:30:00+00:00
updated_at: 2025-07-30T15:30:00+00:00
parent: phase-0-data-ingestion-and-storage
blocked_by: ["design-sqlite-database-schema"]
archived: false

tags:
  - "#task"
  - "#phase/completed"

exit_criteria_met: true
---

# Design Standard Import Bundle Format

## Description

Design a standardized JSON bundle format for distributing D&D ruleset data that can be imported into Mimir. The application will start with zero data, and users will select which rulesets to import. This format should be optimized for clean imports while maintaining data fidelity.

## Acceptance Criteria

- [x] Define bundle directory structure and naming conventions
- [x] Create manifest.json schema for import orchestration
- [x] Design source mapping format (books/supplements metadata)
- [x] Define entity file organization (split strategy for large datasets)
- [x] Document parent-child dependency ordering
- [x] Include all book/adventure metadata for completeness
- [x] Create validation schema for bundle integrity
- [x] Design versioning strategy for bundle updates
- [x] Define compression/packaging format for distribution

## Technical Notes

Key considerations:
- Load entire bundle into memory for processing
- Maintain 5etools data structure where sensible
- Clear separation between rulesets (2014, 2024, etc.)
- Include all source books even if not immediately used
- Optimize file sizes for reasonable download times
- Support incremental updates in the future

Example structure:
```
dnd5e-2014/
  manifest.json
  sources.json (includes all books)
  races-core.json
  races-subraces.json
  classes-core.json
  classes-subclasses.json
  spells.json
  items-base.json
  items-magic.json
  creatures/
    by-cr/
    by-source/
```

## Dependencies

- Depends on: design-sqlite-database-schema
- Blocks: implement-base-dataprocessor-trait

## Completion Summary

### What We Did
- Designed a simplified bundle format with one JSON file per database table
- Defined denormalized structure where child entities contain all parent data
- Created clean ID generation without redundant source/ruleset info
- Specified tar.gz distribution format for efficient downloads
- Included free-form books directory for flexible content storage

### Outcomes
- Complete specification in `/docs/data-design/import-bundle-format.md`
- Clear examples for all entity types (races, classes, items, spells, creatures, etc.)
- Identified key parsing challenges and transformation rules
- Validation requirements and entity count tracking

### Scope Note
This initial implementation is designed to support character creation workflows (races, classes, backgrounds, feats, spells, items). The format will be expanded as we add more content types to the database (e.g., detailed adventure content, encounter builders, etc.).

### Location of Assets
- `/docs/data-design/import-bundle-format.md` - Complete specification with examples