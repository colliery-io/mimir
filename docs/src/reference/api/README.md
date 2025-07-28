# API Documentation

This section contains comprehensive API documentation for all Mimir crates, automatically generated from the Rust source code.

## Crates Overview

### [Core Types](./core.md)
**`mimir-dm-core`** - Fundamental data structures, traits, and shared types.

Key components:
- Campaign, NPC, Plot, Session, and Rule data structures
- Storage and LLM provider traits
- Agent framework interfaces
- Search and query types

### [Database Layer](./database.md)
**`mimir-dm-db`** - Database operations, migrations, and storage implementations.

Key components:
- SQLite storage implementation with sqlite-vec integration
- Database models and schema definitions
- Migration management
- FTS5 and vector search implementations

### [AI Integration](./ai.md)
**`mimir-dm-ai`** - LLM integration and embedding generation.

Key components:
- Ollama HTTP client
- Embedding generation and caching
- Model management and configuration
- Streaming response handling

### [Agent Framework](./agents.md)
**`mimir-dm-agents`** - Agent implementations and coordination.

Key components:
- NPC management agent
- Plot tracking agent
- Session orchestration agent
- Rule advisor agent

### [TUI Components](./tui.md)
**`mimir-dm-tui`** - Terminal user interface components and widgets.

Key components:
- Main application loop
- Campaign view components
- NPC and plot management interfaces
- Search and query interfaces

## Quick Access

- **[Complete API Documentation](../../target/doc/mimir_dm/index.html)** - Full generated API docs
- **[Core Types](../../target/doc/mimir_dm_core/index.html)** - `mimir-dm-core` API
- **[Database Layer](../../target/doc/mimir_dm_db/index.html)** - `mimir-dm-db` API  
- **[AI Integration](../../target/doc/mimir_dm_ai/index.html)** - `mimir-dm-ai` API
- **[Agent Framework](../../target/doc/mimir_dm_agents/index.html)** - `mimir-dm-agents` API
- **[TUI Components](../../target/doc/mimir_dm_tui/index.html)** - `mimir-dm-tui` API

## Usage

The API documentation is generated using `cargo doc` and includes:
- Type definitions and trait implementations
- Function signatures and parameters
- Usage examples where available
- Cross-references between related types

## Generating Documentation

To generate the latest API documentation:

```bash
# Generate docs for all crates
cargo doc --workspace --no-deps

# Generate and open docs in browser
cargo doc --workspace --no-deps --open

# Generate docs with private items (for contributors)
cargo doc --workspace --no-deps --document-private-items
```

## Integration with mdBook

The API documentation is integrated into this mdBook through automated scripts that:
1. Generate fresh API docs from source code
2. Extract relevant sections for each crate
3. Format for integration with the overall documentation structure
4. Ensure cross-references work correctly