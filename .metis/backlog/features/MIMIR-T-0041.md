---
id: groq-provider
level: task
title: "Groq Provider"
short_code: "MIMIR-T-0041"
created_at: 2025-10-30T10:32:00.622395+00:00
updated_at: 2025-10-30T18:57:03.655080+00:00
parent: 
blocked_by: []
archived: false

tags:
  - "#task"
  - "#feature"
  - "#feature"
  - "#phase/completed"


exit_criteria_met: false
strategy_id: NULL
initiative_id: NULL
---

# Groq Provider

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **\[CONDITIONAL: Assigned Task\]**

\[\[Parent Initiative\]\]

## Objective **\[REQUIRED\]**

Implement a Groq API provider for mimir-dm-llm to enable ultra-fast LLM inference using Groq's cloud service. This provides an alternative to Ollama that doesn't require local installation, offering users a simple commercial API option with extremely low latency.

## Backlog Item Details **\[CONDITIONAL: Backlog Item\]**

{Delete this section when task is assigned to an initiative}

### Type

- \[ \] Bug - Production issue that needs fixing
- \[x\] Feature - New functionality or enhancement
- \[ \] Tech Debt - Code improvement or refactoring
- \[ \] Chore - Maintenance or setup work

### Priority

- \[ \] P0 - Critical (blocks users/revenue)
- \[ \] P1 - High (important for user experience)
- \[x\] P2 - Medium (nice to have)
- \[ \] P3 - Low (when time permits)

### Business Justification **\[CONDITIONAL: Feature\]**

- **User Value**:

  - Eliminates need to install and maintain Ollama for users who prefer cloud services
  - Provides extremely fast inference (Groq's LPU architecture delivers \~10x faster tokens/sec than typical cloud providers)
  - Simple API key setup - no local GPU requirements
  - Access to same quality models (Llama, Mixtral, Gemma) without local infrastructure

- **Business Value**:

  - Reduces barrier to entry for new users who don't want to set up local LLM infrastructure
  - Positions mimir as flexible - supports both local-first (Ollama) and cloud-first (Groq) workflows
  - Demonstrates provider abstraction architecture works for commercial APIs
  - Opens door for additional commercial providers (OpenAI, Anthropic, etc.)

- **Effort Estimate**: S (Small)

  - Groq uses OpenAI-compatible API, so implementation follows established pattern
  - Primary work is HTTP client setup and response mapping
  - Estimated 2-4 hours implementation + testing

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria **\[REQUIRED\]**

- \[ \] GroqProvider struct implements LlmProvider trait completely
- \[ \] Provider supports chat endpoint with message history
- \[ \] Provider supports completion endpoint for single-turn completions
- \[ \] Provider handles tool calls correctly (function calling)
- \[ \] API key must be provided via config map (upstream application handles sourcing)
- \[ \] Base URL can be customized via config (defaults to https://api.groq.com/openai/v1)
- \[ \] Rate limiting works correctly using existing RateLimitState infrastructure
- \[ \] HTTP errors are properly mapped to LlmError variants
- \[ \] Groq API error responses are parsed and surfaced with clear messages
- \[ \] Provider returns proper Usage statistics (token counts)
- \[ \] Provider is exported from providers/mod.rs
- \[ \] Unit tests verify configuration handling (API key from config, base URL)
- \[ \] Unit tests verify error case when API key missing from config
- \[ \] Code compiles without warnings
- \[ \] Documentation includes configuration examples and supported models list

## Implementation Notes **\[CONDITIONAL: Technical Task\]**

### Technical Approach

**Architecture Pattern**:Follow the existing OllamaProvider pattern as reference:

1. Create `groq.rs` in `crates/mimir-dm-llm/src/providers/`
2. Implement `GroqProvider` struct with `LlmProvider` trait
3. Export from `providers/mod.rs`

**Core Components:**

1. **GroqProvider struct:**

   - `config: ModelConfig` - Provider configuration
   - `rate_limit_state: RateLimitState` - Rate limiting state
   - `client: reqwest::Client` - HTTP client for API calls
   - `base_url: String` - API base URL (default: https://api.groq.com/openai/v1)
   - `api_key: String` - API authentication key

2. **Request/Response Types:**

   - `GroqChatRequest` - Chat completions request (OpenAI-compatible)
   - `GroqChatResponse` - Chat completions response
   - `GroqError` - Error response structure
   - Reuse existing `Message`, `Tool`, `ToolCall` types from traits module

3. **LlmProvider Implementation:**

   - `chat()` - POST to `/chat/completions` endpoint
   - `complete()` - Convert to chat format, use same endpoint
   - `embed()` - Return `LlmError::NotSupported` (Groq doesn't support embeddings)
   - Leverage existing `check_rate_limit()` and `supports_endpoint()` trait methods

**API Key Management:**

- API key must be present in `config.config["api_key"]`
- Return `LlmError::ConfigError` if missing
- Upstream application responsible for sourcing API key (env vars, secure storage, user input, etc.)

**Error Handling:**

- Parse Groq API error responses (JSON format with `error.message` and `error.type`)
- Map HTTP status codes to appropriate `LlmError` variants
- Provide clear error messages for common issues (invalid API key, rate limits, etc.)

**Configuration Example:**

```yaml
name: "groq-llama3"
model: "llama-3.3-70b-versatile"
provider: "groq"
supported_endpoints: ["chat", "completion"]
config:
  api_key: "gsk_..."  # Required - sourced by upstream application
  base_url: "https://api.groq.com/openai/v1"  # Optional, this is default
limit:
  renewal_period: "minutes"
  calls: 30
```

### Dependencies

**External Dependencies:**

- `reqwest` - Already in Cargo.toml for OllamaProvider, no new dependency needed
- `serde` / `serde_json` - Already available
- `async-trait` - Already available
- `tokio` - Already available

**Internal Dependencies:**

- `crates/mimir-dm-llm/src/traits/provider.rs` - LlmProvider trait
- `crates/mimir-dm-llm/src/config.rs` - ModelConfig, EndpointType
- Existing types: Message, Tool, ToolCall, Usage, Timing, etc.

**No Breaking Changes:**

- Pure addition, no modifications to existing providers
- No changes to trait definitions required

### Risk Considerations

**Low Risk:**

- OpenAI-compatible API means straightforward implementation
- Following established OllamaProvider pattern reduces unknowns
- Pure addition with no changes to existing code

**API Compatibility:**

- Risk: Groq API might deviate from OpenAI spec in subtle ways
- Mitigation: Comprehensive error handling and logging for debugging
- Mitigation: Unit tests cover expected request/response formats

**Rate Limiting:**

- Risk: Groq rate limits are strict for free tier
- Mitigation: Leverage existing RateLimitState infrastructure
- Mitigation: Clear error messages when rate limited

**API Key Security:**

- Risk: API keys in config files could be committed to version control
- Mitigation: Document environment variable approach as preferred
- Mitigation: Future work could add secure credential storage

**Testing Limitations:**

- Risk: Integration tests require real API key and make real API calls
- Mitigation: Unit tests cover configuration and error handling
- Mitigation: Manual testing with real API for verification
- Note: Consider mocking HTTP client for comprehensive testing in future

### Implementation Steps

 1. Create `src/providers/groq.rs` with module documentation
 2. Define request/response structs (GroqChatRequest, GroqChatResponse, etc.)
 3. Implement GroqProvider::new() with API key resolution logic
 4. Implement private make_request() helper for HTTP calls
 5. Implement LlmProvider::chat() method
 6. Implement LlmProvider::complete() method (converts to chat format)
 7. Implement LlmProvider::embed() method (returns NotSupported)
 8. Add unit tests for configuration handling
 9. Export from providers/mod.rs
10. Test compilation with `cargo check -p mimir-dm-llm`
11. Manual testing with real API key

### Supported Models (as of Groq API documentation)

**Text Models:**

- llama-3.3-70b-versatile (recommended default)
- llama-3.1-8b-instant
- llama-3.2-90b-vision-preview
- mixtral-8x7b-32768
- gemma2-9b-it

**Audio Models (not implementing for this task):**

- whisper-large-v3
- whisper-large-v3-turbo

Note: Groq's model availability changes frequently. Users should check https://console.groq.com/docs/models for current list.

## Status Updates **\[REQUIRED\]**

*To be added during implementation*