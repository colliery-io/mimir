---
id: foundation-infrastructure
level: initiative
title: "Foundation Infrastructure"
created_at: 2025-07-28T16:26:41.917455+00:00
updated_at: 2025-07-28T16:26:41.917455+00:00
parent: initial-implementation
blocked_by: []
archived: false

tags:
  - "#initiative"
  - "#phase/discovery"


exit_criteria_met: false
estimated_complexity: M
---

# Foundation Infrastructure Initiative

## Context

This initiative covers Phase 1 of the implementation plan (Weeks 1-2), establishing the technical foundation that all subsequent development depends upon. Without solid database infrastructure, AI integration, and rule data indexing, no intelligent features can be implemented.

The foundation must be designed to support the full vision while being immediately functional for basic operations. This includes SQLite with vector search extensions, local Ollama integration for embeddings and text generation, and comprehensive indexing of D&D 5e rule data for intelligent lookup.

## Goals & Non-Goals

**Goals:**
- Set up SQLite database with sqlite-vss vector extensions and FTS5 full-text search
- Integrate Ollama for local AI processing with llama3/mistral models and nomic-embed-text embeddings
- Import and index 5e-bits rule data with dual indexing (FTS5 + vector embeddings)
- Create core database schema supporting NPCs, plots, sessions, and campaign data
- Establish hybrid search foundation combining exact matches with semantic similarity
- Validate performance baseline for <5 second query response times

**Non-Goals:**
- User interface development (handled in Phase 3)
- Agent framework implementation (handled in Phase 2)  
- Advanced AI features or consistency checking (handled in Phase 4)
- Campaign import/export functionality
- Multi-database support or data migration tools

## Detailed Design

**SQLite Setup with Vector Extensions:**
- Compile and integrate sqlite-vss extension for vector similarity search
- Configure FTS5 virtual tables with D&D-specific tokenizers ("porter unicode61 remove_diacritics 2")
- Set up separate tables for rules_fts, npcs_fts, sessions_fts with optimized schemas

**Ollama Integration Architecture:**
- HTTP client for Ollama API at localhost:11434
- Support for llama3/mistral text generation and nomic-embed-text embeddings
- Async streaming interface for real-time responses
- Connection pooling and error handling for reliability

**Database Schema Design:**
```sql
-- Core campaign tables
campaigns(id, name, created_at, settings)
npcs(id, campaign_id, name, description, personality, relationships)
plots(id, campaign_id, title, summary, status, connections)
sessions(id, campaign_id, number, date, summary, notes)

-- Vector and FTS5 tables
rules_vectors(id, content, embedding)
campaign_vectors(id, content_type, content_id, embedding)
rules_fts(title, content, category)
```

**Hybrid Search Implementation:**
- Query analysis to determine FTS5 vs vector search priority
- Weighted scoring: 0.5 * fts_score + 0.35 * vector_similarity + 0.15 * recency_weight
- Result merging and deduplication across search types

## Alternatives Considered

**Vector Database Alternatives:**
- PostgreSQL with pgvector: Rejected due to complexity and deployment overhead for local-first application
- Chroma/Weaviate: Rejected due to external service dependencies conflicting with local-first principle
- Pure FTS5: Rejected due to inability to handle semantic similarity queries

**AI Integration Alternatives:**
- OpenAI API: Rejected due to privacy concerns and internet dependency
- Hugging Face Transformers: Rejected due to Python dependency and memory requirements
- Local GGML models: Considered but Ollama provides better abstraction and model management

**Database Alternatives:**
- JSON files: Rejected due to poor search performance and lack of ACID properties
- Embedded databases (RocksDB, sled): Rejected due to lack of SQL query capabilities and vector search support

## Implementation Plan

**Week 1:**
- Day 1-2: Set up Rust project structure, dependencies (rusqlite, tokio, reqwest)
- Day 3-4: Compile and integrate sqlite-vss extension, create basic database connection
- Day 5-7: Implement Ollama HTTP client with async streaming support

**Week 2:**
- Day 1-3: Design and implement core database schema with migration system
- Day 4-5: Configure FTS5 virtual tables with D&D-specific tokenization
- Day 6-7: Import and process 5e-bits JSON data, create dual indices (FTS5 + vector embeddings)

**Deliverables:**
- Working SQLite database with vector search capabilities
- Functional Ollama integration with embedding generation
- Complete rule data indexed and searchable
- Performance baseline established (<5s query response)

## Testing Strategy

**Unit Tests:**
- Database connection and migration functionality
- Ollama client HTTP operations and error handling
- FTS5 tokenization and search accuracy
- Vector embedding generation and similarity calculations

**Integration Tests:**
- End-to-end rule data import and indexing process
- Hybrid search result accuracy and performance
- Database schema integrity and constraint validation

**Performance Tests:**
- Query response time benchmarks (<5s target)
- Memory usage under load (4GB RAM constraint)
- Concurrent query handling capabilities

**Validation Criteria:**
- All 5e-bits rule data successfully imported and searchable
- Vector embeddings generated for 100% of rule content
- FTS5 search returns accurate results for spell names, NPC names
- Hybrid search demonstrates improved relevance over pure keyword search
