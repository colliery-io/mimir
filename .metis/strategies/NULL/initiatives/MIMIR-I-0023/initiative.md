---
id: react-pattern-for-multi-step
level: initiative
title: "ReAct Pattern for Multi-Step Reasoning"
short_code: "MIMIR-I-0023"
created_at: 2025-12-17T17:37:56.809771+00:00
updated_at: 2025-12-17T17:43:06.507224+00:00
parent: MIMIR-V-0001
blocked_by: []
archived: false

tags:
  - "#initiative"
  - "#phase/design"


exit_criteria_met: false
estimated_complexity: L
strategy_id: NULL
initiative_id: react-pattern-for-multi-step
---

# ReAct Pattern for Multi-Step Reasoning Initiative

## Context

The current agent uses a reactive loop that jumps straight to tool calling without explicit reasoning. This leads to:
- Fragmented multi-step task execution
- No visible reasoning chain for debugging
- Poor error recovery (repeats failing approaches)
- Hard 20-iteration limit can cut off complex workflows

The ReAct (Reasoning + Acting) pattern forces the agent to think before acting, producing structured reasoning traces.

## Goals & Non-Goals

**Goals:**
- Implement ReAct pattern: Thought → Action → Observation → Thought...
- Make reasoning visible in conversation
- Improve multi-step task decomposition
- Enable better error recovery through explicit reasoning

**Non-Goals:**
- Changing LLM provider interface
- Adding external planning systems

## Detailed Design

### ReAct Pattern Overview

```
User: "Create an NPC shopkeeper with a sword and add to the tavern module"

Agent:
Thought: I need to break this into steps: 1) Create NPC, 2) Add sword to inventory, 3) Find tavern module, 4) Add NPC to module
Action: create_character({name: "Shopkeeper", is_npc: true, ...})
Observation: Character created with ID 42

Thought: Now I need to add a sword to their inventory
Action: add_inventory_item({character_id: 42, item: "Longsword"})
Observation: Longsword added to inventory

Thought: Now I need to find the tavern module
Action: list_modules({campaign_id: 1})
Observation: Found module "The Rusty Tavern" with ID 5

Thought: Finally, add the NPC to that module. Task complete.
Action: (none - final response)
```

### System Prompt Changes

Add ReAct instruction block:
```
You are an AI assistant using the ReAct pattern. For each step:

1. **Thought**: Analyze what you need to do next. Consider:
   - What is the overall goal?
   - What information do you have/need?
   - What's the next logical step?
   - Did the previous action succeed or fail?

2. **Action**: Call exactly one tool, or respond if done.

3. **Observation**: You'll receive the tool result.

Always show your reasoning in Thought blocks before acting.
Format: <thought>Your reasoning here</thought>
```

### Code Changes

**File: `chat_processor.rs`**

1. Update system prompt generation to include ReAct instructions
2. Parse `<thought>` blocks from LLM response (separate from `<thinking>`)
3. Store thoughts in conversation for visibility
4. Add thought extraction to message processing

**File: `tools/mod.rs`**

1. Update `generate_system_rules()` to include ReAct format
2. Add examples of proper ReAct usage

### Message Flow (New)

```
[System: ReAct instructions + tools]
[User: "Create NPC with sword"]
[Assistant: <thought>I need to...</thought> + tool_call]
[Tool: result]
[Assistant: <thought>That worked, now...</thought> + tool_call]
[Tool: result]
[Assistant: <thought>Done!</thought> + final response]
```

### Key Differences from Current

| Current | ReAct |
|---------|-------|
| Hidden `<thinking>` stripped | Visible `<thought>` preserved |
| Jump to tool calls | Explicit reasoning before action |
| No step tracking | Clear step-by-step progression |
| Implicit error handling | Explicit "that failed, try X" |

## Implementation Plan

1. ✅ Update system prompt with ReAct instructions
2. ✅ Add thought block parsing (separate from thinking)
3. ✅ Preserve thoughts in conversation history
4. [ ] Update frontend to display thought blocks (styling)
5. [ ] Add integration tests for multi-step tasks

## Implementation Status

**Completed (2025-12-17):**

1. **System Prompt** (`defaultSystemPrompt.ts`):
   - Added "Reasoning Pattern (ReAct)" section
   - Documented THOUGHT → ACTION → OBSERVATION → REPEAT cycle
   - Added `<thought>` block usage examples
   - Specified when to use/skip explicit reasoning

2. **Tool Guidance** (`tools/mod.rs`):
   - Added "ReAct Pattern for Multi-Step Tasks" section to `generate_tool_awareness_guidance()`
   - Added concrete example showing thought blocks interleaved with tool calls

3. **Chat Processor** (`chat_processor.rs`):
   - Updated `strip_thinking_blocks()` to preserve `<thought>` blocks
   - Added documentation clarifying `<thinking>` (internal, stripped) vs `<thought>` (ReAct, preserved)

**Remaining:**
- Frontend styling for thought blocks (optional enhancement)
- Integration tests to validate ReAct behavior