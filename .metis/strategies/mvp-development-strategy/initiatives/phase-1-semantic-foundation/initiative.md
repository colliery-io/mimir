---
id: phase-1-semantic-foundation
level: initiative
title: "Phase 1: Semantic Foundation"
created_at: 2025-07-30T01:45:12.724757+00:00
updated_at: 2025-07-30T01:45:12.724757+00:00
parent: mvp-development-strategy
blocked_by: []
archived: false

tags:
  - "#initiative"
  - "#phase/discovery"


exit_criteria_met: false
estimated_complexity: M
---

# Phase 1: Semantic Foundation Initiative

## Context

With structured data (Phase 0) and deterministic workflows (Phase 0.5) complete, we can now add intelligence to Mimir. This phase transforms our static database into a semantic knowledge base that understands context, intent, and relationships.

By implementing FTS5 full-text search, vector embeddings, and local LLM integration via Ollama, we enable natural language queries, intelligent content retrieval, and context-aware assistance. This is where Mimir evolves from a rules database into an intelligent DM companion.

## Goals & Non-Goals

**Goals:**
- Implement FTS5 for all text content with D&D-aware tokenization
- Add vector embeddings via sqlite-vss for semantic search
- Integrate Ollama for local LLM queries
- Build hybrid search combining exact and semantic matches
- Create RAG pipeline for rule explanations
- Enable natural language queries for all content
- Maintain sub-second response times

**Non-Goals:**
- Cloud-based LLMs (privacy/latency)
- Training custom models
- Image generation
- Voice interfaces
- Real-time learning from user input
- Multi-language support

## Detailed Design

### Search Architecture

```rust
// Unified search interface
pub struct SearchEngine {
    fts5: FTS5Index,
    vector_store: VectorStore,
    llm_client: OllamaClient,
    ranker: HybridRanker,
}

// Search query types
pub enum SearchQuery {
    Exact(String),           // FTS5 only
    Semantic(String),        // Vector only
    Hybrid(String),          // Both + reranking
    Natural(String),         // LLM-enhanced
}
```

### FTS5 Implementation

**Index Structure:**
```sql
-- Main FTS5 table for all searchable content
CREATE VIRTUAL TABLE search_index USING fts5(
    entity_id,
    entity_type,
    title,
    content,
    keywords,
    source,
    -- Custom tokenizer for D&D terms
    tokenize = 'porter unicode61 remove_diacritics 1'
);

-- Separate indices for different content types
CREATE VIRTUAL TABLE spells_fts USING fts5(...);
CREATE VIRTUAL TABLE rules_fts USING fts5(...);
CREATE VIRTUAL TABLE creatures_fts USING fts5(...);
```

**D&D-Aware Tokenization:**
- Handle special terms: "1d20", "DC 15", "+2 sword"
- Preserve spell/item names with special characters
- Synonym mapping: "healing" → includes "cure", "restore"
- Abbreviation expansion: "AC" → "armor class"

### Vector Search Setup

**Embedding Generation:**
```rust
impl EmbeddingGenerator {
    // Using Ollama's nomic-embed-text model
    async fn generate_embeddings(&self, text: &str) -> Vec<f32> {
        // Chunk text appropriately
        let chunks = self.chunk_text(text, 512);
        
        // Generate embeddings via Ollama
        let embeddings = self.ollama
            .embeddings("nomic-embed-text", chunks)
            .await?;
            
        // Store in sqlite-vss
        self.store_vectors(embeddings).await
    }
}
```

**Vector Storage:**
```sql
-- Vector storage using sqlite-vss
CREATE VIRTUAL TABLE vss_spells USING vss0(
    embedding(384),  -- nomic-embed-text dimension
);

CREATE TABLE spell_embeddings (
    id TEXT PRIMARY KEY,
    spell_id TEXT NOT NULL,
    chunk_index INTEGER,
    chunk_text TEXT,
    embedding_id INTEGER
);
```

### LLM Integration

**Ollama Client:**
```rust
pub struct OllamaClient {
    base_url: String,
    model: String,  // llama3, mistral, etc
    context_size: usize,
}

impl OllamaClient {
    // Streaming responses for better UX
    async fn query_stream(
        &self,
        prompt: &str,
        context: Vec<Document>,
    ) -> impl Stream<Item = String> {
        // Format context + prompt
        let full_prompt = self.build_rag_prompt(prompt, context);
        
        // Stream response
        self.ollama.generate_stream(full_prompt).await
    }
}
```

### RAG Pipeline

**Retrieval Strategy:**
1. Query understanding via LLM
2. Parallel FTS5 + vector search
3. Result fusion and reranking
4. Context assembly
5. Response generation

**Context Management:**
```rust
pub struct RAGPipeline {
    max_context_tokens: usize,  // 4096 typical
    chunk_size: usize,          // 512 tokens
    overlap: usize,             // 64 tokens
}

impl RAGPipeline {
    fn assemble_context(&self, docs: Vec<Document>) -> String {
        // Smart truncation preserving whole sections
        // Priority to most relevant chunks
        // Include source citations
    }
}
```

### Query Examples

**Simple Lookup:**
"What is fireball?" → Direct FTS5 match → Return spell description

**Semantic Search:**
"Spells that deal area damage" → Vector search → Return similar spells

**Complex Query:**
"How does grappling work?" → RAG pipeline → Assembled rules explanation

**Natural Language:**
"Can a rogue sneak attack with a spell?" → LLM interpretation → Rule clarification

## Alternatives Considered

**Cloud LLMs (OpenAI/Anthropic)**
- Pro: More capable models, no local resources
- Con: Privacy concerns, latency, requires internet
- Rejected: Violates local-first principle

**Custom Model Training**
- Pro: Perfectly tailored to D&D domain
- Con: Massive effort, requires ML expertise
- Rejected: Ollama models sufficient for MVP

**ElasticSearch**
- Pro: Industry standard, powerful features
- Con: Heavy runtime, complex deployment
- Rejected: SQLite extensions sufficient

**PostgreSQL pgvector**
- Pro: Mature vector support
- Con: Server requirement, not local-first
- Rejected: sqlite-vss meets our needs

**Simple Keyword Search**
- Pro: Fast, predictable
- Con: Misses semantic understanding
- Rejected: Users expect intelligent search

## Implementation Plan

### Week 1-2: FTS5 Foundation
- Install and configure FTS5 extension
- Design index schema for all content types
- Implement D&D-aware tokenizer
- Build search query parser
- Create result ranking algorithm

### Week 3-4: Vector Search
- Integrate sqlite-vss extension
- Set up Ollama with embedding model
- Design chunking strategy for different content
- Generate embeddings for all data
- Implement similarity search

### Week 5-6: LLM Integration  
- Build Ollama client with streaming
- Test different local models (llama3, mistral)
- Implement prompt engineering
- Add context window management
- Handle model fallbacks

### Week 7-8: RAG Pipeline
- Design retrieval strategies
- Build query router (exact vs semantic)
- Implement result fusion
- Add source citation tracking
- Create response formatting

### Week 9-10: Search Optimization
- Performance tuning for all queries
- Implement caching layer
- Add query suggestion
- Build feedback loop
- Optimize index sizes

### Week 11-12: Integration & Testing
- Integrate with Phase 0.5 workflows
- Add search to all UI components
- Comprehensive testing
- Performance benchmarking
- User documentation

## Testing Strategy

### Search Quality Tests
- Precision: Relevant results in top 5
- Recall: All relevant content found
- D&D term handling: "1d20", "DC 15" work correctly
- Synonym recognition: "heal" finds "cure wounds"
- Typo tolerance: "fierball" → "fireball"

### Performance Benchmarks
- Simple keyword search: < 50ms
- Semantic search: < 200ms  
- RAG query with LLM: < 2s first token
- Index build time: < 5 minutes
- Memory usage: < 500MB

### LLM Validation
- Correct rule interpretations
- Accurate source citations
- No hallucinated content
- Consistent responses
- Appropriate confidence levels

### Integration Tests
- Search from all UI contexts works
- Results properly formatted
- Sources correctly linked
- Streaming responses smooth
- Fallbacks work correctly

### User Acceptance
- "Find spells like fireball" returns area damage spells
- "How does multiclassing work?" provides clear explanation
- "What can a 5th level wizard do?" comprehensive answer
- Natural language queries understood 90%+ of time
- Results more helpful than ctrl+f in PDF
