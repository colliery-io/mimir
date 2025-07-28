# Architecture Overview

Mimir is designed as a modular, local-first D&D campaign assistant built using Rust. The architecture emphasizes performance, data privacy, and extensibility through a multi-crate workspace structure.

## Design Principles

### Local-First Architecture
- **Offline operation**: Core functionality works without internet connection
- **Data privacy**: All campaign data stays on your machine
- **Performance**: SQLite-based storage with optimized indexing
- **Portability**: Single binary deployment with embedded database

### Modular Design
- **Separation of concerns**: Each crate has a focused responsibility
- **Dependency management**: Clear dependency graph prevents circular references
- **Testing isolation**: Each module can be tested independently
- **Plugin architecture**: Future extensibility through well-defined interfaces

## Crate Structure

The workspace consists of six specialized crates:

### `mimir-dm` (Main Binary)
The primary application crate that orchestrates all components:

- **Entry point**: Command-line interface and application initialization
- **Configuration**: Global application settings and user preferences
- **Orchestration**: Coordinates between UI, database, and AI components
- **Distribution**: Single binary for end-user installation

Dependencies: All other crates in the workspace

### `mimir-dm-core` (Foundation Types)
Shared data structures and traits used across all crates:

```rust
// Example core types
pub struct Campaign {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub settings: serde_json::Value,
}

pub trait StorageProvider {
    async fn create_campaign(&self, campaign: &Campaign) -> Result<()>;
    async fn find_campaign(&self, id: &str) -> Result<Option<Campaign>>;
}
```

Key responsibilities:
- **Data models**: Campaign, NPC, Plot, Session, Rule structs
- **Trait definitions**: Storage, LLM, and Agent interfaces
- **Error handling**: Centralized error types and result patterns
- **Serialization**: JSON schema definitions and validation

### `mimir-dm-db` (Database Layer)
Database operations and storage management:

```rust
// Example storage implementation
pub struct SqliteStorage {
    conn: Arc<Mutex<rusqlite::Connection>>,
}

impl StorageProvider for SqliteStorage {
    async fn create_campaign(&self, campaign: &Campaign) -> Result<()> {
        // Implementation using prepared statements
    }
}
```

Key features:
- **SQLite integration**: Using Diesel ORM for type-safe queries
- **Vector search**: sqlite-vec integration for semantic similarity
- **Full-text search**: FTS5 indexes for text-based queries
- **Migration management**: Schema versioning and upgrades
- **Connection pooling**: Efficient database access patterns

### `mimir-dm-ai` (AI Integration)
Large Language Model integration and embedding generation:

```rust
// Example AI client
pub struct OllamaClient {
    base_url: String,
    client: reqwest::Client,
}

impl LLMProvider for OllamaClient {
    async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>> {
        // nomic-embed-text integration
    }
    
    async fn chat_completion(&self, messages: &[Message]) -> Result<String> {
        // llama3/mistral model interaction
    }
}
```

Key capabilities:
- **Ollama integration**: Local LLM server communication
- **Embedding generation**: Text-to-vector conversion for semantic search
- **Model management**: Support for multiple models (llama3, mistral)
- **Streaming responses**: Real-time AI-generated content
- **Caching**: Intelligent caching of embeddings and responses

### `mimir-dm-agents` (Agent Framework)
Specialized AI agents for D&D-specific tasks:

```rust
// Example agent implementation
pub struct NPCAgent {
    storage: Arc<dyn StorageProvider>,
    llm: Arc<dyn LLMProvider>,
}

impl NPCAgent {
    pub async fn generate_personality(&self, npc: &NPC) -> Result<String> {
        // AI-powered personality generation
    }
    
    pub async fn suggest_dialogue(&self, context: &DialogueContext) -> Result<Vec<String>> {
        // Context-aware dialogue suggestions
    }
}
```

Agent types:
- **NPC Agent**: Character creation and management
- **Plot Agent**: Story arc tracking and suggestions
- **Session Agent**: Game session orchestration
- **Rule Agent**: D&D rule lookup and interpretation

### `mimir-dm-tui` (Terminal Interface)
Text-based user interface using Ratatui:

```rust
// Example TUI component
pub struct CampaignView {
    campaigns: Vec<Campaign>,
    selected: usize,
}

impl Widget for CampaignView {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Render campaign list with selection
    }
}
```

UI components:
- **Campaign management**: Create, edit, and switch campaigns
- **NPC browser**: Search and manage non-player characters
- **Plot tracker**: Visualize story arcs and connections
- **Session logger**: Record game sessions and notes
- **Search interface**: Unified search across all content types

## Data Flow Architecture

### Request Flow
1. **User input** → TUI layer captures user interactions
2. **Command parsing** → Main binary interprets user commands
3. **Business logic** → Core types and agent processing
4. **Data access** → Database layer with optimized queries
5. **AI integration** → LLM calls for enhanced functionality
6. **Response rendering** → TUI displays results to user

### Search Architecture
Mimir implements hybrid search combining multiple strategies:

1. **Keyword search**: FTS5 indexes for exact term matching
2. **Semantic search**: Vector embeddings for conceptual similarity
3. **Filtered search**: Category and metadata filtering
4. **Ranked results**: Relevance scoring across search types

### Event System
- **Database triggers**: Automatic FTS index maintenance
- **AI background processing**: Embedding generation queue
- **State synchronization**: UI updates on data changes
- **Error propagation**: Structured error handling throughout the stack

## Technology Stack

### Core Technologies
- **Rust**: Systems programming language for performance and safety
- **SQLite**: Embedded database with advanced search capabilities
- **Diesel**: Type-safe ORM for database operations
- **Tokio**: Async runtime for concurrent operations
- **Ratatui**: Terminal UI framework

### AI/ML Integration
- **Ollama**: Local LLM server for privacy-preserving AI
- **sqlite-vec**: Vector similarity search extension
- **nomic-embed-text**: 384-dimensional embedding model
- **FTS5**: Full-text search with advanced tokenization

### Development Tools
- **Cargo**: Rust package management and build system
- **mdBook**: Documentation generation with Diataxis structure
- **Diesel CLI**: Database migration management
- **Make**: Task automation and build orchestration

## Performance Characteristics

### Database Performance
- **Query optimization**: Strategic indexing for common patterns
- **Vector search**: O(log n) similarity queries via sqlite-vec
- **Connection management**: Shared connection pool across components
- **Transaction batching**: Bulk operations for data imports

### Memory Management
- **Zero-copy deserialization**: Efficient data structure handling
- **Streaming processing**: Large dataset handling without memory pressure
- **Resource pooling**: Reuse of expensive objects (DB connections, HTTP clients)
- **Garbage collection**: Rust's ownership model eliminates GC overhead

### Concurrent Operations
- **Async architecture**: Non-blocking I/O for all external operations
- **Background processing**: AI operations don't block UI
- **Database transactions**: ACID compliance with concurrent access
- **Error isolation**: Component failures don't cascade

## Security Considerations

### Data Privacy
- **Local storage**: No cloud dependencies for core functionality
- **Encryption at rest**: SQLite database encryption options
- **Network isolation**: AI processing happens locally via Ollama
- **Audit trails**: Database-level change tracking

### Input Validation
- **Type safety**: Rust's type system prevents many common errors
- **SQL injection prevention**: Parameterized queries via Diesel
- **JSON schema validation**: Structured data validation
- **User input sanitization**: XSS and injection attack prevention

This architecture provides a solid foundation for building a comprehensive D&D campaign assistant while maintaining high performance, data privacy, and extensibility.
