---
id: create-player-character-and
level: task
title: "Create Player, Character, and CharacterVersion models with Diesel"
short_code: "MIMIR-T-0043"
created_at: 2025-11-10T18:56:58.153040+00:00
updated_at: 2025-11-15T16:28:42.002700+00:00
parent: MIMIR-I-0004
blocked_by: []
archived: true

tags:
  - "#task"
  - "#phase/completed"


exit_criteria_met: false
strategy_id: NULL
initiative_id: MIMIR-I-0004
---

# Create Player, Character, and CharacterVersion models with Diesel

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0004]]

## Objective **[REQUIRED]**

Create Rust models with Diesel ORM integration for Player, CampaignPlayer, Character, and CharacterVersion entities to enable database operations.

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria **[REQUIRED]**

- [ ] Player model created in `crates/mimir-dm-core/src/models/player/mod.rs` with Queryable, Insertable, Serialize, Deserialize
- [ ] CampaignPlayer model created for join table
- [ ] Character model created in `crates/mimir-dm-core/src/models/character/mod.rs` (metadata only)
- [ ] CharacterVersion model created for version tracking
- [ ] All models added to schema.rs with proper table mappings
- [ ] Models compile without errors
- [ ] Unit tests for model creation/serialization

## Implementation Notes **[CONDITIONAL: Technical Task]**

### Technical Approach
- Create `crates/mimir-dm-core/src/models/player/mod.rs`
- Create `crates/mimir-dm-core/src/models/character/mod.rs`
- Follow existing model patterns from Campaign, Module, Session
- Use Diesel derive macros: #[derive(Queryable, Insertable, Serialize, Deserialize)]
- Add to schema.rs after migrations run

### Dependencies
- MIMIR-T-0063 (migrations must be created first)
- Existing schema.rs structure

### Risk Considerations
- Ensure character_data field in CharacterVersion is properly typed as String for TEXT column
- Handle nullable fields correctly (Option<T>)

## Status Updates **[REQUIRED]**

### 2025-11-11
**Status**: Completed

Created all Diesel models for player and character management:

**Player models** (src/models/player/mod.rs):
- Player: id, name, email, notes, created_at
- NewPlayer: Insertable struct for creating players
- UpdatePlayer: AsChangeset struct for updating players
- CampaignPlayer: id, campaign_id, player_id, joined_at, active
- NewCampaignPlayer: Insertable struct for associating players with campaigns
- UpdateCampaignPlayer: AsChangeset struct for updating associations

**Character models** (src/models/character/mod.rs):
- Character: id, campaign_id, player_id, character_name, current_level, current_version, directory_path, created_at, last_updated_at
- NewCharacter: Insertable struct for creating characters
- UpdateCharacter: AsChangeset struct for updating character metadata
- CharacterVersion: id, character_id, version_number, file_path, character_data (String for YAML/JSON), snapshot_reason, level, created_at
- NewCharacterVersion: Insertable struct for creating character versions

All models use proper Diesel derive macros (Queryable, Selectable, Insertable, AsChangeset, Associations, Serialize, Deserialize).
Foreign key relationships defined using belongs_to attributes.
Added player and character modules to models/mod.rs.
All tests pass (50 tests).