# Mimir - D&D Campaign Assistant Configuration

## Tech Stack

### Core Technologies
- **Language**: Rust (2021 edition)
- **Framework**: Tokio (async runtime)
- **UI**: Ratatui (Terminal UI)
- **Database**: SQLite with:
  - Diesel ORM (v2.1)
  - sqlite-vec (vector similarity search)
  - FTS5 (full-text search)
- **LLM Integration**: 
  - Ollama (local LLM provider)
  - Custom abstraction layer for multiple providers
  - Embedding support via nomic-embed-text
- **Build System**: Cargo workspace
- **Error Handling**: anyhow + thiserror
- **Serialization**: serde (JSON/YAML)
- **CLI**: Clap v4
- **Project Management**: Metis (.metis directory)
- **Task Automation**: Angreal (.angreal directory)

### Architecture Pattern
- Multi-crate workspace structure
- Actor-based agent framework
- Local-first design (no cloud dependencies)
- RAG (Retrieval-Augmented Generation) implementation
- Triple-index search strategy (FTS5 + Vector + Structured)

## Project Structure

```
/mimir
├── src/                    # Main binary entry point
├── crates/                 # Workspace crates
│   ├── mimir-dm-core/     # Core types and traits
│   ├── mimir-dm-db/       # Database layer with vector search
│   ├── mimir-dm-llm/      # LLM provider abstraction
│   ├── mimir-dm-agents/   # Agent framework implementation
│   └── mimir-dm-tui/      # Terminal UI components
├── data/                   # 5etools data for D&D rules
├── docs/                   # mdBook documentation
├── .metis/                # Project management
└── .angreal/              # Task automation
```

## Agent Assignments

### By Component

#### Core Infrastructure
- `/crates/mimir-dm-core` → @rust-service-architect
  - Core traits and types design
  - Domain model implementation
  - Shared abstractions

#### Database Layer
- `/crates/mimir-dm-db` → @database-specialist + @rust-service-architect
  - SQLite schema design
  - Diesel migrations
  - Vector search implementation
  - FTS5 configuration
  - Query optimization

#### LLM Integration
- `/crates/mimir-dm-llm` → @rust-service-architect
  - Provider abstraction design
  - Ollama integration
  - Rate limiting implementation
  - Streaming support

#### Agent Framework
- `/crates/mimir-dm-agents` → @rust-service-architect + @design-first-architect
  - Actor pattern implementation
  - Message passing system
  - Agent coordination

#### Terminal UI
- `/crates/mimir-dm-tui` → @rust-cli-builder
  - Ratatui components
  - Keyboard navigation
  - State management

#### Data Processing
- `/data` → @data-design-specialist + @database-specialist
  - 5etools data parsing
  - Schema transformation
  - Import pipelines

### By Task Type

#### New Features
- Complex multi-crate features → @technical-lead-orchestrator
- Agent design → @design-first-architect then @rust-service-architect
- Database features → @database-specialist
- UI features → @rust-cli-builder

#### Performance Optimization
- Query optimization → @database-specialist
- Rust performance → @rust-performance-engineer
- Memory optimization → @rust-performance-engineer

#### Architecture & Design
- System design → @design-first-architect
- Data modeling → @data-design-specialist
- API contracts → @design-first-architect

#### Code Quality
- Code review → @code-reviewer
- Test strategy → @test-strategy-agent
- Documentation → @documentation-curator

#### Project Management
- Work tracking → @agile-delivery-coach (uses Metis)
- Build automation → @integration-orchestrator (uses Angreal)
- Workflow automation → @integration-orchestrator

## Routing Rules

### Database Work
```
IF task involves:
  - Schema changes → @database-specialist
  - Vector search → @database-specialist + @rust-service-architect
  - Query performance → @database-specialist
  - Migrations → @database-specialist
```

### LLM/AI Work
```
IF task involves:
  - New LLM providers → @rust-service-architect
  - Embedding logic → @rust-service-architect
  - RAG implementation → @design-first-architect then @rust-service-architect
  - Prompt engineering → @rust-service-architect
```

### Agent Framework
```
IF task involves:
  - New agent types → @design-first-architect then @rust-service-architect
  - Message protocols → @design-first-architect
  - Actor coordination → @rust-service-architect
```

### UI/UX Work
```
IF task involves:
  - TUI components → @rust-cli-builder
  - Keyboard shortcuts → @rust-cli-builder
  - Screen layouts → @rust-cli-builder
  - User workflows → @design-first-architect then @rust-cli-builder
```

### Data Import/Processing
```
IF task involves:
  - 5etools parsing → @data-design-specialist
  - Data transformation → @data-design-specialist
  - Batch processing → @rust-service-architect
```

## Workflows

### Feature Development Flow
1. @agile-delivery-coach - Create Metis task/initiative
2. @design-first-architect - Design interfaces and contracts
3. @technical-lead-orchestrator - Break down into crate-specific work
4. Specialist agents - Implement in respective crates
5. @test-strategy-agent - Design test approach
6. @code-reviewer - Review implementation
7. @integration-orchestrator - Update CI/CD if needed

### Database Schema Change
1. @data-design-specialist - Design schema changes
2. @database-specialist - Create Diesel migrations
3. @rust-service-architect - Update Rust models
4. @test-strategy-agent - Design migration tests
5. @database-specialist - Optimize queries

### New Agent Implementation
1. @design-first-architect - Design agent interface
2. @rust-service-architect - Implement actor pattern
3. @test-strategy-agent - Design agent tests
4. @documentation-curator - Document agent behavior

### Performance Optimization
1. @rust-performance-engineer - Profile application
2. @database-specialist - Analyze query performance
3. @rust-performance-engineer - Optimize hot paths
4. @test-strategy-agent - Create performance benchmarks

## Tool Integration

### Metis Project Management
- Vision tracking: `.metis/vision.md`
- Strategy documents: `.metis/strategies/`
- Work breakdown: Use @agile-delivery-coach
- Phase transitions: Track implementation progress

### Angreal Task Automation
- Task definitions: `.angreal/`
- Common tasks:
  - Database migrations
  - Test execution
  - Build verification
  - Documentation generation

## Common Scenarios

### "Add a new D&D rule feature"
1. @agile-delivery-coach - Create Metis task
2. @data-design-specialist - Design data model
3. @database-specialist - Update schema
4. @rust-service-architect - Implement core logic
5. @rust-cli-builder - Add UI components
6. @test-strategy-agent - Design tests
7. @documentation-curator - Update docs

### "Improve search performance"
1. @database-specialist - Analyze current queries
2. @rust-performance-engineer - Profile application
3. @database-specialist - Optimize indices
4. @rust-performance-engineer - Optimize Rust code
5. @test-strategy-agent - Create benchmarks

### "Add new LLM provider"
1. @design-first-architect - Design provider interface
2. @rust-service-architect - Implement provider
3. @test-strategy-agent - Design integration tests
4. @documentation-curator - Document configuration

### "Debug agent communication issue"
1. @project-analyst - Trace message flow
2. @rust-service-architect - Fix implementation
3. @test-strategy-agent - Add regression tests

## Development Guidelines

### Code Organization
- Keep crate boundaries clean
- Use workspace dependencies
- Share types through mimir-dm-core
- Implement traits for abstraction

### Performance Priorities
- Sub-second response times
- Efficient memory usage
- Optimized database queries
- Minimal allocations in hot paths

### Testing Strategy
- Unit tests per crate
- Integration tests for workflows
- Performance benchmarks
- Mock LLM providers for tests

### Documentation
- API documentation in code
- Architecture decisions in ADRs
- User guides in /docs
- Examples in documentation