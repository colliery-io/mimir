---
id: create-playerservice-with-crud
level: task
title: "Create PlayerService with CRUD operations"
short_code: "MIMIR-T-0046"
created_at: 2025-11-10T18:56:58.889537+00:00
updated_at: 2025-11-10T18:56:58.889537+00:00
parent: MIMIR-I-0004
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/todo"


exit_criteria_met: false
strategy_id: NULL
initiative_id: MIMIR-I-0004
---

# Create PlayerService with CRUD operations

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0004]]

## Objective **[REQUIRED]**

Create a PlayerService in the backend core layer with full CRUD operations for managing players and their campaign associations.

## Acceptance Criteria **[REQUIRED]**

- [ ] PlayerService struct created in `crates/mimir-dm-core/src/services/player/mod.rs`
- [ ] create_player() method for adding new players to database
- [ ] get_player_by_id() method for retrieving player details
- [ ] update_player() method for editing player information
- [ ] delete_player() method for removing players
- [ ] add_player_to_campaign() method for associating players with campaigns
- [ ] remove_player_from_campaign() method for removing campaign associations
- [ ] list_players_for_campaign() method for getting all players in a campaign
- [ ] Unit tests for all CRUD operations with mock database connections

## Implementation Notes **[CONDITIONAL: Technical Task]**

### Technical Approach
- Create `crates/mimir-dm-core/src/services/player/mod.rs`
- Follow existing service patterns from CampaignService, ModuleService
- Accept diesel connection pool reference in service methods
- Return Result<T, MimirError> for error handling
- Use transactions for multi-table operations (player + campaign_players join)

### Dependencies
- MIMIR-T-0063 (database migrations)
- MIMIR-T-0043 (Player and CampaignPlayer models)
- Diesel ORM connection pool

### Risk Considerations
- Deleting a player should handle orphaned characters appropriately
- Campaign association checks must prevent duplicate entries
- Query performance for campaigns with many players
- Transaction rollback handling for failed multi-step operations

## Status Updates **[REQUIRED]**

*To be added during implementation*