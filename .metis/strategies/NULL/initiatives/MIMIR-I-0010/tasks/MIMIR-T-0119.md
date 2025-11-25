---
id: unify-on-openai-compatible
level: task
title: "Unify on OpenAI-compatible endpoints for all LLM providers"
short_code: "MIMIR-T-0119"
created_at: 2025-11-25T01:57:08.447417+00:00
updated_at: 2025-11-25T01:57:08.447417+00:00
parent: MIMIR-I-0010
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/todo"


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

- [ ] Create unified OpenAI-compatible client that works with any provider
- [ ] Refactor Ollama provider to use OpenAI-compatible endpoint (/v1/chat/completions)
- [ ] Refactor Groq provider to use same interface
- [ ] Update LLM trait to align with OpenAI request/response formats
- [ ] Migrate existing tests to use OpenAI-compatible endpoint format
- [ ] Remove Ollama-specific endpoint code (/api/generate, /api/chat)
- [ ] Document provider configuration (base URL, API key, model name)
- [ ] All existing tests pass with refactored providers

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