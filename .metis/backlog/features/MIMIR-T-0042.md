---
id: provider-selection-ui-in-settings
level: task
title: "Provider Selection UI in Settings"
short_code: "MIMIR-T-0042"
created_at: 2025-10-30T22:45:00+00:00
updated_at: 2025-10-30T18:56:39.680678+00:00
parent: 
blocked_by: []
archived: false

tags:
  - "#task"
  - "#feature"
  - "#phase/todo"


exit_criteria_met: false
strategy_id: NULL
initiative_id: NULL
---

# Provider Selection UI in Settings

## Objective **[REQUIRED]**

Add UI in the application settings to allow users to select their LLM provider (Ollama or Groq) and configure provider-specific settings. This enables users to choose between local inference (Ollama) and cloud-based inference (Groq) based on their preferences and infrastructure.

## Backlog Item Details **[CONDITIONAL: Backlog Item]**

### Type
- [ ] Bug - Production issue that needs fixing
- [x] Feature - New functionality or enhancement
- [ ] Tech Debt - Code improvement or refactoring
- [ ] Chore - Maintenance or setup work

### Priority
- [ ] P0 - Critical (blocks users/revenue)
- [x] P1 - High (important for user experience)
- [ ] P2 - Medium (nice to have)
- [ ] P3 - Low (when time permits)

### Business Justification **[CONDITIONAL: Feature]**
- **User Value**:
  - Allows users to choose between local-first (Ollama) and cloud-first (Groq) workflows
  - No need to restart application or edit config files to switch providers
  - Clear UI for provider-specific configuration (API keys, base URLs, etc.)
  - Users can leverage Groq's speed advantage or Ollama's privacy/offline capabilities based on their needs

- **Business Value**:
  - Demonstrates mimir's flexibility and multi-provider architecture
  - Reduces support burden by providing clear configuration UI instead of manual config editing
  - Opens door for more providers in the future (OpenAI, Anthropic, etc.)
  - Positions mimir as adaptable to different user workflows and constraints

- **Effort Estimate**: M (Medium)
  - Requires frontend settings UI components
  - Backend settings storage and validation
  - LlmService initialization logic updates
  - Provider switching without full app restart (or clear restart requirement)
  - Estimated 1-2 days implementation + testing

## Acceptance Criteria

## Acceptance Criteria **[REQUIRED]**

- [ ] Settings UI has a "Provider" section
- [ ] User can select between "Ollama" and "Groq" via dropdown/radio buttons
- [ ] When Ollama selected, UI shows base URL field (default: http://localhost:11434)
- [ ] When Groq selected, UI shows API key field (password/secure input)
- [ ] Groq does NOT show base URL override (uses default API endpoint only)
- [ ] Settings are persisted to application storage
- [ ] Model selection remains fixed to gpt-oss (not configurable in this task)
- [ ] Validation ensures required fields are filled (e.g., API key for Groq)
- [ ] Clear error messages if provider initialization fails (e.g., invalid API key, Ollama not running)
- [ ] Settings changes require application restart OR provider reinitialization (document which approach)
- [ ] Default provider is Ollama (existing behavior)
- [ ] UI indicates which provider is currently active/configured

## Implementation Notes **[CONDITIONAL: Technical Task]**

### Technical Approach

**Frontend (Tauri/React):**
1. Add Settings page/section if not already present
2. Create provider selection component:
   - Provider type selector (Ollama/Groq)
   - Conditional fields based on selected provider
   - Form validation
3. Save settings via Tauri command
4. Display current provider status

**Backend (mimir-dm):**
1. Extend settings/config storage to include:
   - `provider_type: String` ("ollama" | "groq")
   - `provider_config: HashMap<String, String>` for provider-specific settings
2. Update `LlmService` initialization:
   - Read provider type from settings
   - Build appropriate `ModelConfig` based on provider type
   - Use `OllamaProvider::new()` or `GroqProvider::new()` based on selection
3. Add Tauri command to save provider settings
4. Add Tauri command to get current provider config (for UI display)

**Configuration Structure:**
```rust
struct ProviderSettings {
    provider_type: String,  // "ollama" or "groq"
    ollama_config: Option<OllamaConfig>,
    groq_config: Option<GroqConfig>,
}

struct OllamaConfig {
    base_url: String,  // default: "http://localhost:11434"
}

struct GroqConfig {
    api_key: String,  // required
    base_url: Option<String>,  // optional override
}
```

**Model Name Mapping:**
Different providers use different model naming conventions for the same model:
- **Ollama**: `gpt-oss:20b` (current REQUIRED_MODEL constant)
- **Groq**: `openai/gpt-oss-20b` (Groq's registry format)

**Provider Initialization:**
```rust
// Model name constants
const OLLAMA_MODEL: &str = "gpt-oss:20b";
const GROQ_MODEL: &str = "openai/gpt-oss-20b";

fn create_provider_from_settings(settings: ProviderSettings) -> Result<Arc<dyn LlmProvider>> {
    let config = match settings.provider_type.as_str() {
        "ollama" => {
            let ollama_config = settings.ollama_config
                .ok_or_else(|| anyhow!("Missing Ollama config"))?;
            create_ollama_config(OLLAMA_MODEL, Some(&ollama_config.base_url))
        }
        "groq" => {
            let groq_config = settings.groq_config
                .ok_or_else(|| anyhow!("Missing Groq config"))?;
            create_groq_config(GROQ_MODEL, &groq_config.api_key)
        }
        _ => return Err(anyhow!("Unknown provider type: {}", settings.provider_type)),
    };

    ProviderFactory::create_provider(config).await
}
```

**Note:** This will require creating a `ProviderFactory` pattern in `mimir-dm-llm` or handling provider creation in `mimir-dm` directly.

### Dependencies

**Blockers:**
- MIMIR-T-0041 must be merged (Groq provider implementation)

**Related Work:**
- May need settings persistence mechanism if not already present
- May need to expose GroqProvider from mimir-dm-llm public API

### Risk Considerations

**Provider Switching:**
- **Risk**: Switching providers mid-session could break in-flight requests or chat history
- **Mitigation**: Either require app restart or implement clean provider swap (close old, init new)
- **Decision needed**: Restart vs hot-swap approach

**API Key Security:**
- **Risk**: Storing API keys in plain text on disk
- **Mitigation**: Use OS keychain/secure storage if available on target platforms
- **Short-term**: Store in encrypted application settings file

**Error Handling:**
- **Risk**: User configures Groq but doesn't have valid API key, or selects Ollama but it's not running
- **Mitigation**: Validate on save, provide clear error messages, allow fallback/retry

**Model Mismatch:**
- **Risk**: Groq might not have gpt-oss model available
- **Mitigation**: Document which model is used (may need to map to Groq equivalent)
- **Note**: Task description says "model fixed as gpt-oss" - may need to use different Groq model

### Open Questions

1. Should provider switching require app restart or can we hot-swap?
2. Where should provider settings be stored? (app config file, database, OS settings)
3. How do we handle API key storage securely?
4. Should we validate provider connection on save (test API call)?
5. What happens to existing chat history when provider changes?
6. Does Groq support a model equivalent to gpt-oss? Or do we use llama-3.3-70b-versatile?

## Status Updates **[REQUIRED]**

*To be added during implementation*