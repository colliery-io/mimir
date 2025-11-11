---
id: create-chat-tool-definitions-for
level: task
title: "Create chat tool definitions for LLM character access"
short_code: "MIMIR-T-0052"
created_at: 2025-11-10T18:57:01.536420+00:00
updated_at: 2025-11-10T18:57:01.536420+00:00
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

# Create chat tool definitions for LLM character access

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0004]]

## Objective **[REQUIRED]**

Create MCP tool definitions that allow LLMs to query and interact with character data during chat sessions, enabling AI-assisted character lookups and rule checking.

## Acceptance Criteria **[REQUIRED]**

- [ ] get_character tool definition for retrieving full character data by name or ID
- [ ] list_campaign_characters tool for listing all characters in current campaign
- [ ] get_character_stats tool for quick stat lookups (AC, HP, abilities)
- [ ] check_spell_slots tool for querying available spell slots
- [ ] Tool definitions include proper JSON schemas for parameters and responses
- [ ] Tools integrated into chat context system for current campaign
- [ ] LLM can access character sheets during DM chat sessions
- [ ] Tool responses formatted for LLM consumption (clear, structured text)

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

*To be added during implementation*