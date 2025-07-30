---
id: design-sqlite-schema
level: task
title: "Design SQLite Database Schema"
created_at: 2025-07-30T02:36:00+00:00
updated_at: 2025-07-30T02:36:00+00:00
parent: phase-0-data-ingestion-and-storage
blocked_by: ["analyze-5etools-json-schema"]
archived: false

tags:
  - "#task"
  - "#phase/completed"

exit_criteria_met: true
---

# Design SQLite Database Schema

## Description

Design the SQLite database schema that will store all D&D 5e data, balancing normalization with query performance.

## Acceptance Criteria

- [x] Design core entity tables (spells, items, creatures, etc.)
- [x] Create unified entities table for cross-referencing
- [x] Define appropriate data types for each field
- [x] Design JSON columns for complex nested data
- [x] Create index strategy for common queries
- [x] Document foreign key relationships
- [x] Write migration scripts

## Technical Notes

Key design decisions:
- Prefer denormalization for read performance
- Use JSON columns for arrays and complex nested structures
- Consistent ID generation strategy
- Source attribution in every table
- Consider future phases (FTS5, vector search)

## Dependencies

- Depends on: analyze-5etools-json-schema

## Completion Summary

### What We Did
- Designed denormalized SQLite schema optimized for read-heavy performance
- Created multi-ruleset architecture to support D&D 5e 2014, 2024, and future systems
- Chose JSON storage strategy for complex nested data structures
- Defined all core entity tables with appropriate indexes

### Outcomes
- Complete SQLite schema in `sqlite-schema-core-entities.sql`
- Entity relationship diagram showing denormalized design
- Comprehensive data dictionary with field definitions
- Transformation rules for JSON to SQL conversion

### Key Decisions
- Full denormalization (classes/subclasses, races/subraces, items/variants in single tables)
- JSON arrays for variable-length relationships (no junction tables)
- TEXT columns with json_valid() constraints for JSON storage
- Compound IDs including rule system to prevent conflicts

### Location of Assets
All schema documentation in `/docs/data-design/`:
- `sqlite-schema-core-entities.sql` - Complete schema definition
- `entity-relationship-diagram.md` - Visual representation
- `data-dictionary.md` - Field specifications
- `transformation-rules.md` - Import logic