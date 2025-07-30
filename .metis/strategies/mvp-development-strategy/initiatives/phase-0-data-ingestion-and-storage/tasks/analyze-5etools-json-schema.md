---
id: analyze-5etools-json-schema
level: task
title: "Analyze 5etools JSON Schema Structure"
created_at: 2025-07-30T02:35:00+00:00
updated_at: 2025-07-30T02:35:00+00:00
parent: phase-0-data-ingestion-and-storage
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/completed"

exit_criteria_met: true
---

# Analyze 5etools JSON Schema Structure

## Description

Analyze all 5etools JSON file types to understand their structure, identify common patterns, and document transformation requirements for SQLite storage. Design a denormalized schema optimized for read-heavy performance with multi-ruleset support.

## Acceptance Criteria

- [x] Document schema for each of the 13 file types (actually ~300-400 files)
- [x] Identify common patterns across file types
- [x] Map nested structures to relational model
- [x] Document all 5etools-specific formatting tags
- [x] Create data dictionary with field descriptions
- [x] Identify required vs optional fields
- [x] Document cross-reference patterns

## Technical Notes

Focus on:
- races.json
- classes.json  
- spells.json
- items.json
- bestiary/bestiary-*.json
- backgrounds.json
- feats.json
- conditions.json
- skills.json
- actions.json
- languages.json
- books.json
- adventures.json

## Dependencies

None - this is the first task in Phase 0.

## Completion Summary

### What We Did
- Analyzed ~300-400 JSON files in the 5etools data set (far more than the initially estimated 13)
- Identified the hierarchical organization: core files, book-specific files, fluff files, and generated indices
- Examined representative samples from each major content category
- Documented common patterns including 5etools formatting tags (`{@spell}`, `{@creature}`, etc.)
- Designed a denormalized database schema optimized for read-heavy performance

### Outcomes
- **Complete Data Inventory**: Catalogued all JSON file types and their organization
- **Denormalized Schema Design**: Created SQLite schema with JSON storage for complex fields
- **Multi-Ruleset Support**: Added support for D&D 5e 2014, D&D 5e 2024, and future systems
- **Transformation Rules**: Defined how to convert JSON to relational format with denormalization
- **Documentation Suite**: Created comprehensive documentation for implementation

### Location of Assets
All documentation created in `/docs/data-design/`:
- `5etools-data-inventory.md` - Complete file inventory and patterns
- `sqlite-schema-core-entities.sql` - Final denormalized schema (search tables deferred)
- `entity-relationship-diagram.md` - ERD showing denormalized relationships
- `data-dictionary.md` - Field definitions and enumerations
- `transformation-rules.md` - JSON to SQL conversion rules

### Key Decisions Made
1. **Denormalization Strategy**: Chose full denormalization over normalized design for read performance
   - Classes/subclasses in single table with parent references
   - Races/subraces in single table with all data populated
   - Items/variants with base data copied to variants
   - Creature actions/traits stored as JSON arrays (no separate tables)
   - Spell classes stored as JSON array (no junction table)

2. **Multi-Ruleset Architecture**: Added rule_systems and sources hierarchy to support multiple game systems

3. **JSON Storage**: Use SQLite's JSON1 extension with TEXT columns and json_valid() constraints

4. **Deferred Search**: Moved FTS5 and vector embedding tables to Phase 1 (out of scope)

5. **ID Generation**: Include rule system in IDs to allow same content across different rule systems