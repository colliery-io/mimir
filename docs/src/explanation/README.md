# Explanation

*Understanding-oriented documentation about Mimir's design and concepts*

The explanation section provides **understanding-oriented** documentation that helps you comprehend the reasoning behind Mimir's design, the problems it solves, and the trade-offs that were made during development.

## Available Explanations

### [Architecture Overview](./architecture.md)
High-level system design and component relationships:
- System architecture and data flow
- Technology choices and rationale
- Component interaction patterns
- Scalability and performance considerations

### [Search Strategy](./search-strategy.md)
Deep dive into Mimir's hybrid search implementation:
- FTS5 vs vector search trade-offs
- Query analysis and routing
- Result ranking and merging
- D&D-specific search optimizations

### [Agent Framework](./agent-framework.md)
Understanding the agent-based architecture:
- Agent communication patterns
- Message passing and state management
- Agent lifecycle and coordination
- Extensibility and plugin architecture

### [Performance Considerations](./performance.md)
Performance characteristics and optimization strategies:
- Memory usage patterns
- Query performance analysis
- Caching strategies
- Resource management

## Architecture Decision Records (ADRs)

ADRs document important architectural decisions and their context:

### [ADR-001: SQLite with Vector Extensions](./adr/001-sqlite-vec.md)
Why we chose SQLite + sqlite-vec over dedicated vector databases.

### [ADR-002: Agent Communication Patterns](./adr/002-agent-pattern.md)
The rationale behind our lightweight actor pattern implementation.

### [ADR-003: TUI Framework Selection](./adr/003-tui-framework.md)
Why we selected Ratatui for the terminal user interface.

## Purpose of Explanations

Explanation documentation serves to:
- **Provide context** for design decisions
- **Explain trade-offs** between different approaches
- **Share domain knowledge** about D&D campaign management
- **Document constraints** and assumptions
- **Educate** about underlying concepts and technologies

## When to Read Explanations

Read explanation docs when you:
- Want to understand the "why" behind design decisions
- Are considering contributing to the project
- Need to make architectural decisions in related projects  
- Are curious about the problem domain and solution approach
- Want to understand performance characteristics

## Contributing Explanations

Good explanation documentation:
- Provides historical context for decisions
- Explains both what was chosen and what was rejected
- Discusses trade-offs and their implications
- Uses concrete examples to illustrate abstract concepts
- Updates when assumptions or constraints change

See our [Contributing Guide](../CONTRIBUTING.md) for how to improve or add to the explanation documentation.