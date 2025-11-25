---
id: unify-on-openai-compatible
level: task
title: "Unify on OpenAI-compatible endpoints for all LLM providers"
short_code: "MIMIR-T-0119"
created_at: 2025-11-25T01:57:08.447417+00:00
updated_at: 2025-11-25T11:18:04.324339+00:00
parent: MIMIR-I-0010
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/completed"


exit_criteria_met: false
strategy_id: NULL
initiative_id: MIMIR-I-0010
---

# Unify on OpenAI-compatible endpoints for all LLM providers

## Parent Initiative

[[MIMIR-I-0010]]

## Objective

Refactor the LLM provider abstraction to use OpenAI-compatible API endpoints as the standard interface, replacing the Ollama-specific endpoints currently in use. This enables consistent testing against any OpenAI-compatible provider (Ollama, Groq, OpenAI, local vLLM, etc.) and simplifies the provider implementation.

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [x] Create unified OpenAI-compatible client that works with any provider
- [x] Refactor Ollama provider to use OpenAI-compatible endpoint (/v1/chat/completions)
- [x] Refactor Groq provider to use same interface
- [x] Update LLM trait to align with OpenAI request/response formats
- [x] Migrate existing tests to use OpenAI-compatible endpoint format
- [x] Remove Ollama-specific endpoint code (/api/generate, /api/chat)
- [x] Document provider configuration (base URL, API key, model name)
- [x] All existing tests pass with refactored providers

## Implementation Notes

### Technical Approach

1. **OpenAI-compatible endpoint standard**: Most LLM providers now support OpenAI-compatible endpoints:
   - Ollama: `http://localhost:11434/v1/chat/completions`
   - Groq: `https://api.groq.com/openai/v1/chat/completions`
   - OpenAI: `https://api.openai.com/v1/chat/completions`
   - vLLM: `http://localhost:8000/v1/chat/completions`

2. **Provider configuration**: Each provider differs only by:
   - Base URL
   - API key (optional for local providers like Ollama)
   - Default model name

3. **Shared request/response types**: Use OpenAI's chat completion types as the standard

### Files to Modify
- `crates/mimir-dm-llm/src/providers/ollama.rs` - Replace with OpenAI-compatible client
- `crates/mimir-dm-llm/src/providers/groq.rs` - Align to shared interface
- `crates/mimir-dm-llm/src/lib.rs` - Update trait definitions
- `crates/mimir-dm-llm/tests/` - Update test helpers

### Dependencies
None - this is a refactoring task

### Risk Considerations
- Ollama's OpenAI-compatible endpoint may have slight behavioral differences
- Need to verify streaming support works consistently across providers

## Status Updates

*To be added during implementation*