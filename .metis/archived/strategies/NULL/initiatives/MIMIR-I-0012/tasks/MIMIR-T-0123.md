---
id: llm-model-evaluation-suite
level: task
title: "LLM Model Evaluation Suite"
short_code: "MIMIR-T-0123"
created_at: 2025-11-25T13:13:58.772343+00:00
updated_at: 2025-11-25T19:16:50.764349+00:00
parent: MIMIR-I-0012
blocked_by: []
archived: true

tags:
  - "#task"
  - "#phase/completed"


exit_criteria_met: false
strategy_id: NULL
initiative_id: MIMIR-I-0012
---

# LLM Model Evaluation Suite

## Parent Initiative

[[MIMIR-I-0012]]

## Objective

Create an evaluation suite for comparing different LLM models (Ollama local models, Groq cloud models) for our D&D assistant use case. The suite runs standardized tasks against each model and produces metrics to help choose the best model for different scenarios (quality vs speed vs cost).

## Key Questions to Answer

1. Which model handles tool calling most reliably?
2. Which model produces the best D&D content (descriptions, encounters, NPCs)?
3. Which smaller models are "good enough" for specific tasks?
4. Are there tasks where a fast/small model works just as well as a large one?

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [x] Evaluation runner can test any configured provider/model
- [x] Standard task set covering core use cases (see below)
- [x] Metrics captured: task success, tool accuracy, response quality (1-5)
- [x] Results output as JSON for comparison analysis
- [x] CLI command: `cargo run --bin llm-eval`
- [x] Comparison report generation (markdown table)

## Evaluation Tasks

### Category 1: Tool Calling (Objective - can measure correctness)
| Task | Input | Expected Tool | Validation |
|------|-------|---------------|------------|
| Spell lookup | "What does Fireball do?" | `search_spells` | Correct spell returned |
| Monster stats | "Stats for a Goblin" | `get_monster_details` | Correct monster returned |
| Character info | "Show me Thorin's inventory" | `list_players` + character tools | Correct data |
| Player list | "Who are my players?" | `list_players` | Returns seeded players |

### Category 2: Content Generation (Subjective - rate 1-5)
| Task | Input | Evaluation Criteria |
|------|-------|---------------------|
| NPC creation | "Create a mysterious merchant NPC" | Creativity, D&D appropriateness, detail |
| Encounter design | "Design a forest ambush encounter for level 3 party" | Balance, tactics, narrative |
| Location description | "Describe a haunted tavern" | Atmosphere, usable details, hooks |
| Session recap | "Summarize what happened in our last session" | Accuracy, narrative flow |

### Category 3: Reasoning & Planning
| Task | Input | Evaluation Criteria |
|------|-------|---------------------|
| Combat advice | "Should my rogue sneak attack or disengage?" | Tactical soundness |
| Build advice | "What feats work well for a Paladin?" | Rules accuracy, synergy |
| Rule clarification | "How does concentration work?" | Accuracy, clarity |

### Category 4: Edge Cases
- Simple greeting (should NOT call tools): "Hello"
- Ambiguous request: "Help me with my character"
- Impossible request: "Roll a d20 for me"

## Model Configuration

Models are fully configurable via CLI flags or config file - not hardcoded. The tool uses the same provider infrastructure as the main app.

```bash
# Test against any Ollama model
cargo run --bin llm-eval -- --provider ollama --model qwen2.5:7b

# Test against Groq
cargo run --bin llm-eval -- --provider groq --model llama-3.3-70b-versatile

# Use config file for multiple models
cargo run --bin llm-eval -- --config eval-config.json
```

```json
// eval-config.json
{
  "models": [
    { "provider": "ollama", "model": "qwen2.5:7b" },
    { "provider": "groq", "model": "llama-3.3-70b-versatile" }
  ]
}
```

## Implementation Notes

### Architecture

Standalone crate (like `mimir-5etools-splitter`):

```
crates/mimir-llm-eval/
├── Cargo.toml
├── src/
│   ├── main.rs           # CLI entry point (clap)
│   ├── runner.rs         # Evaluation runner
│   ├── tasks.rs          # Task definitions
│   └── report.rs         # Report generation
├── tasks/
│   ├── tool_calling.json # Category 1 tasks
│   ├── generation.json   # Category 2 tasks
│   └── reasoning.json    # Category 3 tasks
└── results/              # Output directory (gitignored)
```

Dependencies:
- `mimir-dm-llm` - Provider infrastructure
- `mimir-dm-core` - Database for tool validation
- `clap` - CLI parsing
- `serde` - Task/config serialization

### Key Data Structures
```rust
struct EvalTask {
    id: String,
    category: Category,
    prompt: String,
    expected_tools: Option<Vec<String>>,  // For tool calling tasks
    evaluation_criteria: Vec<String>,      // For subjective tasks
}

struct EvalResult {
    task_id: String,
    model: String,
    provider: String,
    response: String,
    tools_called: Vec<ToolCall>,
    // Computed metrics
    tool_accuracy: Option<f32>,    // For objective tasks
    quality_score: Option<u8>,     // For subjective (1-5, manual)
}
```

### CLI Usage
```bash
# Run all tasks against a specific model
cargo run --bin llm-eval -- --model llama3.2:3b

# Run specific category
cargo run --bin llm-eval -- --model qwen2.5:7b --category tool_calling

# Compare multiple models
cargo run --bin llm-eval -- --compare llama3.2:3b,qwen2.5:7b,groq:llama-3.3-70b

# Generate report from previous results
cargo run --bin llm-eval -- --report results/
```

### Output Example
```markdown
# LLM Evaluation Report - 2025-11-25

## Tool Calling Accuracy
| Model | Spell Lookup | Monster Stats | Character Info | Overall |
|-------|--------------|---------------|----------------|---------|
| llama3.2:3b | 80% | 60% | 70% | 70% |
| qwen2.5:7b | 95% | 90% | 85% | 90% |
| llama-3.3-70b | 100% | 95% | 95% | 97% |

## Content Quality (1-5 rating)
| Model | NPC Creation | Encounter Design | Descriptions | Overall |
|-------|--------------|------------------|--------------|---------|
| llama3.2:3b | 2.5 | 2.0 | 3.0 | 2.5 |
| qwen2.5:7b | 3.5 | 3.0 | 4.0 | 3.5 |
| llama-3.3-70b | 4.5 | 4.5 | 5.0 | 4.7 |

## Recommendations
- **Tool calling**: qwen2.5:7b is reliable and fast
- **Creative content**: llama-3.3-70b for best quality
- **Budget option**: llama3.2:3b adequate for simple lookups
```

## Dependencies
- Existing `mimir-dm-llm` provider infrastructure
- Dev seed data (for tool calling validation)

## Status Updates

*To be added during implementation*