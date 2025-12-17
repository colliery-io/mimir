---
id: context-window-management
level: initiative
title: "Context Window Management"
short_code: "MIMIR-I-0024"
created_at: 2025-12-17T17:37:56.890759+00:00
updated_at: 2025-12-17T17:37:56.890759+00:00
parent: MIMIR-V-0001
blocked_by: []
archived: false

tags:
  - "#initiative"
  - "#phase/discovery"


exit_criteria_met: false
estimated_complexity: M
strategy_id: NULL
initiative_id: context-window-management
---

# Context Window Management Initiative

## Context

The current agent appends every tool result to conversation history permanently. Long conversations can exceed token limits, causing failures or truncation. There's no mechanism to summarize or compress history.

## Goals & Non-Goals

**Goals:**
- Implement sliding window for tool results
- Add conversation summarization at thresholds
- Track important facts separately from raw history
- Prevent token limit overflows

**Non-Goals:**
- External vector database for long-term memory
- Semantic search over history



## Detailed Design

### Problem Analysis

Current flow:
1. Frontend sends **full conversation history** with each message
2. Backend adds tool call/result messages during processing (up to 20 iterations)
3. No mechanism to limit context growth

Risk: Long conversations exceed model context limits (128K-256K tokens), causing failures.

### Approach: Token-Based Sliding Window

Implement backend context management that:
1. Estimates conversation token count
2. Applies sliding window when approaching limit (80% threshold)
3. Preserves: system prompt, recent messages, tool results from current request

### Token Estimation

Simple heuristic: ~4 characters per token (conservative estimate)
```rust
fn estimate_tokens(content: &str) -> usize {
    content.len() / 4
}
```

### Sliding Window Rules

When estimated tokens exceed 80% of model context:
1. Keep system message (first message)
2. Keep messages from current request (tool calls in progress)
3. Drop oldest user/assistant pairs until under threshold
4. Add summary marker: "[Earlier conversation truncated for context limits]"

### Implementation Location

`chat_processor.rs`:
- Add `prune_messages_for_context()` function
- Call before each LLM request in the tool loop
- Track which messages are from current request vs history

### Configuration

- `MAX_CONTEXT_TOKENS`: Model-specific (default: 128000)
- `CONTEXT_THRESHOLD`: 0.8 (trigger pruning at 80%)
- `MIN_HISTORY_TURNS`: 3 (always keep last 3 user/assistant pairs)

## Alternatives Considered

1. **Frontend-side pruning**: Rejected - backend has better view of actual context growth during tool loops
2. **Summarization with LLM**: More complex, adds latency and cost; sliding window simpler for v1
3. **Vector database for history**: Overkill for current use case; could add later if needed
4. **Fixed message count limit**: Less flexible than token-based approach

## Implementation Plan **[REQUIRED]**

1. ✅ Explore current message handling
2. ✅ Design context window approach
3. ✅ Implement sliding window in chat_processor.rs
4. Test with long conversations

## Implementation Status

**Completed (2025-12-17):**

### Context Window Management (`chat_processor.rs`)

**Constants:**
- `DEFAULT_MAX_CONTEXT_TOKENS` = 128000
- `CONTEXT_THRESHOLD` = 0.8 (80% triggers pruning)
- `MIN_HISTORY_TURNS` = 3 (always keep last 3 user/assistant pairs)

**Functions Added:**
- `estimate_tokens(content: &str) -> usize` - Estimates tokens (~4 chars/token)
- `estimate_conversation_tokens(messages: &[Message]) -> usize` - Total conversation estimate
- `prune_messages_for_context(messages, max_tokens, protected_count) -> (Vec<Message>, bool)` - Sliding window pruning

**Integration:**
- Called before each LLM request in the tool execution loop
- Tracks messages added during current request to protect from pruning
- Adds truncation marker when pruning occurs
- Logs pruning events to chat logger

**Behavior:**
- When context exceeds 80% threshold, drops oldest messages
- Always preserves: system prompt, recent messages, current tool chain
- Adds context note: "[Context note: N earlier messages were truncated...]"