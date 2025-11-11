---
id: create-characterservice-with
level: task
title: "Create CharacterService with character creation logic"
short_code: "MIMIR-T-0047"
created_at: 2025-11-10T18:56:59.282300+00:00
updated_at: 2025-11-10T18:56:59.282300+00:00
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

# Create CharacterService with character creation logic

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0004]]

## Objective **[REQUIRED]**

Create a CharacterService that handles character creation, versioning, file system operations, and integration with the database for full character lifecycle management.

## Acceptance Criteria **[REQUIRED]**

- [ ] CharacterService struct created in `crates/mimir-dm-core/src/services/character/mod.rs`
- [ ] create_character() method that creates directory, saves YAML, generates markdown, creates DB record
- [ ] get_character() method that loads from DB and reads YAML file
- [ ] update_character() method that creates new version, saves YAML, updates DB
- [ ] delete_character() method that removes files and DB records
- [ ] list_characters_for_campaign() method for retrieving all characters in a campaign
- [ ] get_character_versions() method for retrieving version history
- [ ] Character directories follow pattern: `campaigns/{campaign_id}/characters/{character_name}/`
- [ ] Unit tests for character CRUD operations with temporary file system

## Implementation Notes **[CONDITIONAL: Technical Task]**

### Technical Approach
- Create `crates/mimir-dm-core/src/services/character/mod.rs`
- Use std::fs for file operations with proper error handling
- Serialize CharacterData to YAML and save to file system
- Generate markdown character sheet on every save using MarkdownRenderer
- Store character metadata in database, full data in YAML files
- Version control: increment version number, create snapshot with reason

### Dependencies
- MIMIR-T-0042 (database migrations)
- MIMIR-T-0043 (Character and CharacterVersion models)
- MIMIR-T-0044 (CharacterData structs)
- MIMIR-T-0045 (MarkdownRenderer)

### Risk Considerations
- File system operations must handle concurrent access safely
- YAML parse errors need clear error messages
- Character name collisions within same campaign
- Version history could grow large over time
- Ensure atomic operations (DB + file system succeed together or fail together)

## Status Updates **[REQUIRED]**

*To be added during implementation*