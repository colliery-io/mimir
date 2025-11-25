---
id: add-model-selection-ui-for-llm
level: task
title: "Add model selection UI for LLM providers"
short_code: "MIMIR-T-0121"
created_at: 2025-11-25T13:13:58.687018+00:00
updated_at: 2025-11-25T13:13:58.687018+00:00
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

# Add model selection UI for LLM providers

## Parent Initiative

[[MIMIR-I-0012]]

## Objective

Allow users to select and switch between different LLM models within a provider from the settings UI. Users running local models via Ollama may have multiple models available (e.g., qwen, llama, mistral) with different parameter sizes (7B, 20B, 70B, 120B), and need the ability to switch based on task complexity or hardware constraints.

## Acceptance Criteria

- [ ] Settings UI displays a dropdown/selector for available models
- [ ] For Ollama: Query available models via the `/api/tags` endpoint
- [ ] For Groq: Show list of supported models from configuration
- [ ] Selected model persists across application restarts
- [ ] Model change takes effect immediately for new chat sessions
- [ ] Display model metadata where available (parameter count, quantization, context window)
- [ ] Handle gracefully when selected model becomes unavailable

## Implementation Notes

### Technical Approach

**Backend (Rust):**
1. Add `get_available_models` Tauri command that queries the provider
2. For Ollama: Call `GET /api/tags` to list installed models
3. For Groq: Return static list of supported models
4. Store selected model in `provider_settings.json`
5. Update `LlmService` to use configured model instead of hardcoded default

**Frontend (Vue):**
1. Add model selector component to Settings view
2. Fetch available models on settings page load
3. Display model info (name, size, context window if available)
4. Save selection via existing provider settings mechanism

### Key Files to Modify
- `crates/mimir-dm/src/services/provider_settings.rs` - Add model field
- `crates/mimir-dm-llm/src/providers/ollama.rs` - Add list_models method
- `crates/mimir-dm-llm/src/providers/groq.rs` - Add list_models method  
- `crates/mimir-dm-llm/src/traits/provider.rs` - Add trait method
- `ui/src/views/Settings.vue` - Add model selector UI

### Dependencies
- Existing provider settings infrastructure
- OpenAI-compatible endpoint already unified (MIMIR-T-0119)

## Status Updates

*To be added during implementation*