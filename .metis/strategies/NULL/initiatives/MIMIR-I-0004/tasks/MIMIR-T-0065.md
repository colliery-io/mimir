---
id: create-database-migrations-for
level: task
title: "Create database migrations for players and characters tables"
short_code: "MIMIR-T-0065"
created_at: 2025-11-10T18:56:57.987210+00:00
updated_at: 2025-11-11T16:57:11.901232+00:00
parent: MIMIR-I-0004
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/completed"


exit_criteria_met: false
strategy_id: NULL
initiative_id: MIMIR-I-0004
---

# Create database migrations for players and characters tables

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0004]]

## Objective **[REQUIRED]**

Create database migrations for the players, campaign_players, characters, and character_versions tables to support character management in Mimir campaigns.

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria **[REQUIRED]**

- [ ] Migration file created for `players` table with id, name, email, notes, created_at
- [ ] Migration file created for `campaign_players` join table with campaign_id, player_id, joined_at, active
- [ ] Migration file created for `characters` table with id, campaign_id, player_id, character_name, current_level, current_version, directory_path, created_at, last_updated_at
- [ ] Migration file created for `character_versions` table with id, character_id, version_number, file_path, character_data (TEXT for YAML/JSON), snapshot_reason, level, created_at
- [ ] All foreign key constraints properly defined
- [ ] All indexes created for query performance (campaign_id, player_id, character_id)
- [ ] Down migrations created for all tables
- [ ] Migration tested with `diesel migration run` and `diesel migration revert`

## Implementation Notes **[CONDITIONAL: Technical Task]**

### Technical Approach
- Create numbered migration files in `crates/mimir-dm-core/migrations/`
- Use Diesel's migration format (up.sql and down.sql)
- Follow existing Mimir migration patterns from campaigns/modules/sessions tables
- Ensure character_data column is TEXT type for storing YAML/JSON blobs

### Dependencies
- Diesel CLI installed
- Existing database schema knowledge

### Risk Considerations
- Ensure TEXT column size is sufficient for large character data (spells, inventory, etc.)
- Foreign key cascades need careful consideration (what happens when campaign/player deleted?)
- Index performance on character lookups

## Status Updates **[REQUIRED]**

### 2025-11-11
**Status**: Completed

Created migration 031_create_players_and_characters with:
- players table: id, name, email, notes, created_at
- campaign_players join table: id, campaign_id, player_id, joined_at, active with UNIQUE constraint
- characters table: id, campaign_id, player_id, character_name, current_level, current_version, directory_path, created_at, last_updated_at
- character_versions table: id, character_id, version_number, file_path, character_data (TEXT), snapshot_reason, level, created_at with UNIQUE constraint on (character_id, version_number)

All foreign key constraints defined with ON DELETE CASCADE.
Indexes created for campaign_id, player_id, character_id, character_name, and level columns.
Down migration drops tables in reverse order.
Updated schema.rs with table definitions and joinable declarations.
All tests pass (50 tests).