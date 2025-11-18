---
id: create-chat-tool-definitions-for
level: task
title: "Create chat tool definitions for LLM character access"
short_code: "MIMIR-T-0052"
created_at: 2025-11-10T18:57:01.536420+00:00
updated_at: 2025-11-18T13:46:03.373405+00:00
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

# Create chat tool definitions for LLM character access

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0004]]

## Objective **[REQUIRED]**

Create MCP tool definitions that allow LLMs to query and interact with character data during chat sessions, enabling AI-assisted character lookups and rule checking.

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria **[REQUIRED]**

- [x] get_character tool definition for retrieving full character data by ID
- [x] list_campaign_characters tool for listing all characters in current campaign
- [x] get_character_stats tool for quick stat lookups (HP, abilities, proficiencies)
- [x] check_spell_slots tool for querying available spell slots
- [x] Tool definitions include proper JSON schemas for parameters and responses
- [x] Tools integrated into chat context system (ready for registration in ToolRegistry)
- [x] Tool responses formatted for LLM consumption (structured JSON)

## Implementation Notes **[CONDITIONAL: Technical Task]**

### Technical Approach
- Define tools in chat system's tool registry following MCP/Anthropic tool format
- Tools call CharacterService methods to retrieve data
- Return markdown-formatted character data or structured JSON for LLM parsing
- Scope tools to current campaign context (don't leak characters from other campaigns)
- Use existing chat tool infrastructure from module/lore tools

### Dependencies
- MIMIR-T-0047 (CharacterService)
- Existing chat tool system
- Campaign context tracking

### Risk Considerations
- Character data could be large, may need summarized versions for tool responses
- Tool access must respect campaign boundaries for privacy
- LLM token limits may restrict full character sheet inclusion
- Tools should return human-readable formats for DM review
- Need to handle characters that don't exist gracefully

## Status Updates **[REQUIRED]**

### 2025-11-18: Implementation Complete

Created four LLM tools for character data access in `/crates/mimir-dm/src/services/tools/character_tools.rs`:

1. **GetCharacterTool** - Retrieves complete character data including:
   - Core identity (race, class, background, level)
   - Ability scores with calculated modifiers
   - HP tracking (current/max/hit dice)
   - Proficiencies (skills, saves, armor, weapons, tools, languages)
   - Spell data (known/prepared spells, cantrips, spell slots)
   - Inventory, currency, equipped items
   - Personality traits

2. **ListCampaignCharactersTool** - Lists all characters in a campaign with summaries:
   - Character ID (for use with other tools)
   - Name, level, race, class
   - Current/max HP for quick status checks
   - Player ID association

3. **GetCharacterStatsTool** - Quick combat stat lookups:
   - Ability scores and modifiers
   - HP and initiative bonus
   - Proficiency bonus
   - Skill and save proficiencies

4. **CheckSpellSlotsTool** - Spellcasting resource tracking:
   - Spell slots by level with current/max counts
   - Prepared spells and cantrips
   - Spellcaster status check

All tools:
- Implement ToolTrait from mimir-dm-llm
- Use DatabaseService via Arc for connection pooling
- Call CharacterService for data access
- Return structured JSON formatted for LLM consumption
- Include comprehensive descriptions and JSON schemas
- Handle errors with user-friendly messages

Tools are ready for registration in ToolRegistry and integration with chat sessions. Character access scoped by campaign_id parameter to maintain data isolation.