---
id: create-tool-calling-test-harness
level: task
title: "Create tool calling test harness for prompt refinement"
short_code: "MIMIR-T-0123"
created_at: 2025-11-25T13:13:58.772343+00:00
updated_at: 2025-11-25T13:13:58.772343+00:00
parent: MIMIR-I-0012
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/todo"


exit_criteria_met: false
strategy_id: NULL
initiative_id: MIMIR-I-0012
---

# Create tool calling test harness for prompt refinement

## Parent Initiative

[[MIMIR-I-0012]]

## Objective

Create a systematic test harness to evaluate LLM tool calling capabilities across different models and prompts. This enables iterative refinement of the agentic system prompt to improve tool selection accuracy, parameter extraction, and multi-step task completion. The harness should produce metrics that allow comparing models and prompt variations.

## Acceptance Criteria

- [ ] Test harness can run predefined test scenarios against any configured LLM
- [ ] Test scenarios cover: single tool calls, multi-tool sequences, parameter extraction accuracy
- [ ] Results capture: correct tool selected, parameters correct, task completed successfully
- [ ] Support running same scenarios across different models for comparison
- [ ] Output structured results (JSON/CSV) for analysis
- [ ] Include baseline test scenarios for core tools (character lookup, spell search, etc.)
- [ ] CLI command to run test suite: `cargo run --bin mimir-llm-test`
- [ ] Document prompt engineering findings in ADR

## Test Scenarios to Include

### Single Tool Calls
- "What spells can a level 5 wizard cast?" -> spell search tool
- "Show me the stats for a Goblin" -> monster lookup tool
- "What items does my character have?" -> character inventory tool

### Multi-Step Tasks
- "Add a longsword to my character's inventory" -> get character, add item, save
- "Level up my character and choose the ASI feat" -> multi-step character modification

### Edge Cases
- Ambiguous requests requiring clarification
- Requests for tools that don't exist
- Malformed or incomplete information

## Implementation Notes

### Technical Approach

1. Create `mimir-llm-test` binary crate or add to existing test infrastructure
2. Define `TestScenario` struct with input prompt, expected tool calls, validation criteria
3. Implement `TestRunner` that:
   - Sends scenario to LLM
   - Captures tool call response
   - Validates against expected behavior
   - Records metrics (latency, token usage, correctness)
4. Create JSON file format for defining test scenarios
5. Build comparison report generator

### Key Components
```rust
struct TestScenario {
    name: String,
    input_prompt: String,
    expected_tools: Vec<ExpectedToolCall>,
    validation: ValidationCriteria,
}

struct TestResult {
    scenario: String,
    model: String,
    passed: bool,
    tool_accuracy: f32,
    param_accuracy: f32,
    latency_ms: u64,
    token_usage: TokenUsage,
}
```

### Dependencies
- Requires working LLM provider configuration
- Uses existing tool definitions from `mimir-dm-llm`

## Status Updates

*To be added during implementation*