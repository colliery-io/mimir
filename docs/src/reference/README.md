# Reference

*Information-oriented technical reference for Mimir*

The reference section provides **information-oriented** documentation covering technical details, API specifications, configuration options, and other factual information you need when working with Mimir.

## Available Reference Documentation

### [CLI Commands](./cli-commands.md)
Complete reference for all command-line interface options:
- Command syntax and options
- Usage examples
- Exit codes and error handling
- Environment variables

### [Configuration](./configuration.md)
Comprehensive configuration reference:
- Configuration file format and location
- All available options and their defaults
- Environment variable overrides
- Configuration validation

### [Database Schema](./database-schema.md)
Technical details of Mimir's database structure:
- Table schemas and relationships
- Index definitions and optimization
- Migration procedures
- Data types and constraints

### [API Documentation](./api/README.md)
Auto-generated API documentation for all Rust crates:
- [Core Types](./api/core.md) - Fundamental data structures and traits
- [Database Layer](./api/database.md) - Storage and query interfaces
- [AI Integration](./api/ai.md) - LLM and embedding APIs
- [Agent Framework](./api/agents.md) - Agent communication and lifecycle
- [TUI Components](./api/tui.md) - User interface components

## Reference Format

Reference documentation follows these principles:
- **Comprehensive** - Covers all available options and parameters
- **Accurate** - Automatically updated with code changes where possible
- **Searchable** - Well-organized with clear headings and cross-references
- **Concise** - Factual information without unnecessary explanation

## Using Reference Documentation

Reference docs are best used when you:
- Need to look up specific syntax or parameters
- Want to understand all available options
- Are integrating with Mimir programmatically
- Need authoritative information about behavior

## API Documentation Generation

The API documentation is generated automatically from the Rust source code using `cargo doc`. This ensures the documentation stays synchronized with the actual implementation.

To generate the latest API docs locally:

```bash
cargo doc --no-deps --open
```

## Accuracy and Updates

Reference documentation is maintained to stay current with the codebase:
- API docs are generated automatically from source code
- Configuration reference is validated against the actual config parser
- CLI reference is extracted from the argument parser definitions
- Database schema is generated from the migration files