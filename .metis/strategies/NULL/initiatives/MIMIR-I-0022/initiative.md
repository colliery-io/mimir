---
id: expand-llm-tool-coverage
level: initiative
title: "Expand LLM Tool Coverage"
short_code: "MIMIR-I-0022"
created_at: 2025-12-17T17:37:56.730043+00:00
updated_at: 2025-12-17T17:37:56.730043+00:00
parent: MIMIR-V-0001
blocked_by: []
archived: false

tags:
  - "#initiative"
  - "#phase/discovery"


exit_criteria_met: false
estimated_complexity: M
strategy_id: NULL
initiative_id: expand-llm-tool-coverage
---

# Expand LLM Tool Coverage Initiative

## Context

The Mimir chatbot currently exposes only 15 tools to the LLM, but 50+ Tauri commands exist that could enable richer agent interactions. Users expect to manage NPCs, equipment, modules, and query D&D content through natural language, but these capabilities aren't accessible.

## Goals & Non-Goals

**Goals:**
- Expose equipment/inventory management tools (update equipped, remove items, currency)
- Expose module management tools (create, list, add monsters)
- Expose catalog query tools (search monsters, items, spells, classes, races)
- Maintain consistent tool interface patterns

**Non-Goals:**
- Changing the underlying Tauri command implementations
- Adding new database functionality (use existing services)

## Detailed Design

### Priority 1: Equipment/Inventory Tools
```
UpdateEquippedTool - Change armor, shield, main_hand, off_hand
RemoveInventoryItemTool - Remove items from inventory
UpdateCurrencyTool - Adjust gold/silver/copper/etc
```

### Priority 2: Module Management Tools
```
CreateModuleTool - Create adventure modules
ListModulesTool - List campaign modules
AddModuleMonsterTool - Add monsters to encounters
GetModuleDetailsTool - Get module information
```

### Priority 3: Catalog Query Tools
```
SearchMonstersTool - Search monster catalog
SearchItemsTool - Search item catalog
SearchSpellsTool - Search spell catalog
GetMonsterDetailsTool - Get full monster stat block
GetItemDetailsTool - Get item details
GetSpellDetailsTool - Get spell details
```

### Implementation Pattern
Each tool follows existing pattern in `commands.rs`:
1. Implement `Tool` trait from `mimir-dm-llm`
2. Define parameters_schema as JSON Schema
3. Add requires_confirmation for write operations
4. Register in `build_campaign_tool_registry()`

## Implementation Plan

1. ✅ Add equipment tools (3 tools)
2. ✅ Add module tools (4 tools)
3. ✅ Add catalog query tools (3 tools)
4. ✅ Register tools in chat_processor

## Implementation Status

**Completed (2025-12-17):**

### Equipment/Inventory Tools (`character_write_tools.rs`)
- `UpdateEquippedTool` - Change armor, shield, main_hand, off_hand
- `RemoveInventoryItemTool` - Remove items from inventory
- `UpdateCurrencyTool` - Adjust gold/silver/copper/electrum/platinum

### Catalog Query Tools (`catalog_tools.rs` - new file)
- `SearchMonstersTool` - Search by name, CR, size, type, alignment
- `SearchItemsTool` - Search by name, type, rarity, value
- `SearchSpellsTool` - Search by name, level, school

### Tool Registration
- All new tools registered in `build_campaign_tool_registry()` in `chat_processor.rs`

### Module Management Tools (`module_tools.rs` - new file)
- `CreateModuleTool` - Create module with optional type-specific templates
- `ListModulesTool` - List modules for a campaign with optional status filter
- `GetModuleTool` - Get module details, documents, and completion status
- `UpdateModuleStatusTool` - Transition module through workflow stages

**New Tool Count**: 15 → 25 tools exposed to LLM

**Remaining (Lower Priority)**:
- Additional catalog detail tools (GetMonster, GetItem, GetSpell for full stat blocks)