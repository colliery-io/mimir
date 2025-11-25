---
id: seed-missing-catalog-data
level: task
title: "Seed missing catalog data"
short_code: "MIMIR-T-0095"
created_at: 2025-11-24T20:29:01.031713+00:00
updated_at: 2025-11-25T01:29:03.590904+00:00
parent: MIMIR-I-0008
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/completed"


exit_criteria_met: false
strategy_id: NULL
initiative_id: MIMIR-I-0008
---

# Seed missing catalog data

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0008]]

## Objective

Seed missing catalog data for cults, psionics, and tables to ensure complete D&D 5e reference data is available.

*Note: This task was migrated from archived initiative MIMIR-I-0006*

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [ ] Cult data is seeded into the database
- [ ] Psionics data is seeded into the database
- [ ] Tables data is seeded into the database
- [ ] Seeding integrates with existing 5etools splitter pipeline
- [ ] Data can be queried through respective services
- [ ] Seeding is idempotent (can be run multiple times safely)

## Implementation Notes

### Data Sources
The `mimir-5etools-splitter` crate handles processing 5etools JSON data. Need to:
1. Identify source JSON files for cults, psionics, tables
2. Add parsing logic for these entity types
3. Create corresponding seeder functions

### Files to Modify/Create
- `crates/mimir-5etools-splitter/src/` - Add collectors for new types
- `crates/mimir-dm-core/src/seed/` - Add seeder functions
- `crates/mimir-dm-core/src/models/catalog/` - Ensure model types exist

### Entity Types to Add

**Cults:**
- Cult organizations from various sourcebooks
- Properties: name, source, description, goals, etc.

**Psionics:**
- Psionic powers and talents
- Properties: name, type, order, description, etc.

**Tables:**
- Random tables from DMG and other sources
- Properties: name, source, rows, dice, etc.

### Seeding Pattern
Follow existing pattern in `crates/mimir-dm-core/src/seed/`:
```rust
pub fn seed_cults(conn: &mut SqliteConnection) -> Result<(), DbError> {
    // Load JSON data
    // Transform to model structs
    // Batch insert into database
}
```

### Dependencies
- Database schema may need migrations for new tables
- Check if models/schema already exist for these types

### Risk Considerations
- Medium risk: New data types need schema changes
- Ensure backward compatibility with existing databases
- Test seeding on fresh and existing databases

## Status Updates **[REQUIRED]**

*To be added during implementation*