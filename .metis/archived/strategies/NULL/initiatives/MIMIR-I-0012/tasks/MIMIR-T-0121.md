---
id: add-model-selection-ui-for-llm
level: task
title: "Add model selection UI for LLM providers"
short_code: "MIMIR-T-0121"
created_at: 2025-11-25T13:13:58.687018+00:00
updated_at: 2025-11-25T15:06:10.086372+00:00
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

# Add model selection UI for LLM providers

## Parent Initiative

[[MIMIR-I-0012]]

## Objective

Allow users to select and switch between different LLM models within a provider from the settings UI. Users running local models via Ollama may have multiple models available (e.g., qwen, llama, mistral) with different parameter sizes (7B, 20B, 70B, 120B), and need the ability to switch based on task complexity or hardware constraints.

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [x] Settings UI displays a dropdown/selector for available models
- [x] For Ollama: Query available models via the `/api/tags` endpoint
- [x] For Groq: Show list of supported models from configuration
- [x] Selected model persists across application restarts
- [x] Model change takes effect immediately for new chat sessions
- [x] Display model metadata where available (model name shown; extended metadata deferred)
- [x] Handle gracefully when selected model becomes unavailable (error message displayed)

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

### 2025-11-25: Backend Implementation Complete

Completed all backend changes for model selection:

**Changes Made:**
- Added `model` field to `OllamaConfig` and `GroqConfig` in `provider_settings.rs`
- Implemented `list_models()` for `GroqProvider` with static list of available models
- Updated `LlmService` to use configured model instead of hardcoded `OLLAMA_MODEL`/`GROQ_MODEL` constants
- Renamed constants to `DEFAULT_OLLAMA_MODEL`/`DEFAULT_GROQ_MODEL` for fallback behavior
- Updated `list_available_models` Tauri command to use provider's `list_models()` method
- Added `list_models` delegation to `Provider` enum

**Commit:** f8fceca - "Add model selection support for LLM providers"

### 2025-11-25: Frontend Implementation Complete

Completed all frontend changes for model selection:

**Changes Made:**
- Added model dropdown to Provider Configuration section in SettingsView.vue
- Display available models from current provider (dynamic for Ollama, static for Groq)
- Added refresh button to reload model list
- Added loading state and error handling for model fetching
- Updated TypeScript interfaces to include optional `model` field

**Commit:** d51906d - "Add model selection UI to Settings view"

**Task Complete** - All acceptance criteria met