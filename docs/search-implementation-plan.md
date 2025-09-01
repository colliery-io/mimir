# Search Capability Implementation Plan for Mimir

## Overview
This document outlines the implementation plan for adding comprehensive search capabilities to Mimir, including both full-text search (FTS) and vector embedding-based semantic search for catalog items and campaign documents.

## Phase 1: Database Schema & Infrastructure

### 1.1 Search Tables Creation

Create the following tables to support search functionality:

#### `search_index` table (FTS)
- `id` - Primary key
- `content_type` - Type of content (catalog_item/campaign_doc)
- `content_id` - Reference to the original content
- `title` - Document/item title
- `content` - Full text content for FTS
- `metadata` - JSON metadata for filtering
- `updated_at` - Last update timestamp

#### `embeddings` table (Vector Search)
- `id` - Primary key
- `content_type` - Type of content
- `content_id` - Reference to original content
- `chunk_id` - Identifier for text chunks
- `embedding` - BLOB for sqlite-vec vector storage
- `chunk_text` - The actual text chunk
- `metadata` - JSON metadata

#### `search_queue` table (Indexing Status)
- `id` - Primary key
- `content_type` - Type of content
- `content_id` - Reference to content
- `file_path` - Path to source file
- `mtime` - File modification time
- `status` - Indexing status (pending/processing/completed/failed)
- `last_indexed` - Timestamp of last successful indexing

### 1.2 Migration Files
- Create migration `0016_create_search_tables` with FTS5 virtual table and vector extension setup
- Initialize sqlite-vec extension in the connection pool
- Add indexes for performance optimization

## Phase 2: Search Service Architecture

### 2.1 Core Search Module
Location: `crates/mimir-dm-core/src/services/search/`

#### Components:
- **`mod.rs`** - Main search service interface
  - Unified search API
  - Service initialization
  - Configuration management

- **`indexer.rs`** - Background indexing service
  - File watcher using `notify` crate for campaign documents
  - Batch processing for catalog items
  - Chunking strategy (1000 tokens per chunk with 200 token overlap)
  - Incremental indexing based on mtime

- **`embeddings.rs`** - Vector embedding generation
  - **Native Rust embedding using `fastembed` crate**
  - Local model execution (no external API dependencies)
  - Support for multiple models (AllMiniLML6V2, BGE-small, etc.)
  - Caching layer to avoid re-embedding unchanged content
  - Batch embedding for efficiency
  - Zero-copy integration with sqlite-vec using `zerocopy`

- **`query.rs`** - Search query execution
  - Hybrid search combining FTS and vector similarity
  - Ranking algorithm with configurable weights
  - Result aggregation and deduplication
  - Relevance scoring

### 2.2 Background Worker
- Implement tokio-based background task system
- File system watcher for campaign directories
- Periodic scan for modified files (using mtime)
- Queue-based processing to avoid overloading
- Graceful shutdown handling

## Phase 3: Catalog Integration

### 3.1 Catalog Indexing
- Hook into existing catalog loading:
  - `SpellCatalog`
  - `ItemCatalog`
  - `MonsterCatalog`
- Index on catalog initialization
- Store structured metadata:
  - Source book
  - Type/category
  - Level/CR
  - Rarity
  - Schools/domains

### 3.2 Enhanced Catalog Search
- Extend existing search methods with FTS capabilities
- Add semantic search for finding similar items/spells/monsters
- Cross-reference search (e.g., "spells that summon creatures")
- Fuzzy matching for typos

## Phase 4: Campaign Document Integration

### 4.1 Document Indexing
- Watch campaign directories for `.md` file changes
- Parse frontmatter and content separately
- Index document hierarchy:
  - Campaign level
  - Module level
  - Session level
  - Handouts
- Track document relationships

### 4.2 Document Search Features
- Cross-document search within campaigns
- Timeline-aware search (by session dates)
- Tag and metadata filtering
- Search within specific document types
- Full-text highlighting in results

## Phase 5: API & Frontend

### 5.1 Tauri Commands

```rust
// Unified search across all content
#[tauri::command]
async fn search_unified(
    query: String,
    filters: SearchFilters,
    limit: Option<usize>
) -> Result<SearchResults, String>

// Catalog-specific search with filters
#[tauri::command]
async fn search_catalog(
    query: String,
    catalog_type: CatalogType,
    filters: CatalogFilters
) -> Result<CatalogSearchResults, String>

// Campaign document search
#[tauri::command]
async fn search_campaign(
    campaign_id: i32,
    query: String,
    document_types: Vec<String>
) -> Result<DocumentSearchResults, String>

// Manual index rebuild
#[tauri::command]
async fn rebuild_search_index(
    content_type: Option<String>
) -> Result<IndexStatus, String>

// Index health and statistics
#[tauri::command]
async fn get_search_status() -> Result<SearchIndexStatus, String>
```

### 5.2 Frontend Components
- **Universal search bar component**
  - Keyboard shortcuts (Cmd/Ctrl+K)
  - Auto-complete suggestions
  - Recent searches
  
- **Search results viewer**
  - Content type indicators (icons/badges)
  - Relevance scoring display
  - Quick preview on hover
  - Navigation to source
  - Pagination controls
  
- **Search filters panel**
  - Content type selection
  - Date range filters
  - Metadata filters
  - Sort options

## Phase 6: Configuration & Optimization

### 6.1 Configuration
Add search settings to application config:

```yaml
search:
  embedding:
    model: "AllMiniLML6V2"    # fastembed model (local)
    chunk_size: 1000           # Tokens per chunk
    chunk_overlap: 200         # Overlap between chunks
    cache_model: true          # Cache model in memory
  indexing:
    update_frequency: 60       # Seconds between index updates
    batch_size: 256           # Items per batch (fastembed default)
  weights:
    fts_weight: 0.6           # Weight for FTS results
    vector_weight: 0.4        # Weight for vector similarity
  cache:
    enabled: true
    ttl: 3600                 # Cache TTL in seconds
```

### 6.2 Performance Optimizations
- Implement incremental indexing
- Add search result caching with TTL
- Optimize vector similarity queries with approximate algorithms
- Implement pagination for large result sets
- Use connection pooling for database access
- Implement rate limiting for embedding API calls

## Implementation Order

1. **Database schema and migrations**
   - Create tables and indexes
   - Set up sqlite-vec

2. **Basic FTS implementation for catalog**
   - Index existing catalog data
   - Implement basic text search

3. **Background indexer service**
   - File watching system
   - Queue processing

4. **Vector embeddings with Ollama**
   - Embedding generation
   - Vector storage and retrieval

5. **Campaign document watching and indexing**
   - Directory monitoring
   - Document parsing and chunking

6. **Unified search API**
   - Combine FTS and vector search
   - Result ranking and filtering

7. **Frontend integration**
   - Search UI components
   - Result display

8. **Performance optimization**
   - Caching layer
   - Query optimization

## Key Dependencies to Add

```toml
[workspace.dependencies]
# Native Rust embeddings (no external API needed)
fastembed = "4.0"

# File watching
notify = "6.0"

# Optional: Advanced FTS (if SQLite FTS5 isn't sufficient)
tantivy = "0.22"

# Already present:
# sqlite-vec = "0.1.6"  # Vector search
# zerocopy = "0.7.33"   # For efficient vector conversion
```

## Embedding Models Available in fastembed

The following models can be used locally without any external API:

- **AllMiniLML6V2** (Default) - 384 dimensions, 23MB, good general purpose
- **BGE-small-en** - 384 dimensions, 33MB, high quality
- **BGE-base-en** - 768 dimensions, 134MB, higher quality but slower
- **BAAI/bge-large-en-v1.5** - 1024 dimensions, best quality
- **sentence-transformers/all-MiniLM-L12-v2** - 384 dimensions, lightweight

## Success Metrics

- Search latency < 100ms for catalog items
- Search latency < 200ms for campaign documents
- Indexing lag < 5 seconds for file changes
- Memory usage < 500MB for search index
- 95% relevance accuracy for top 5 results

## Future Enhancements

- Multi-language support
- Image search for uploaded assets
- Voice search integration
- Search analytics and learning
- Collaborative filtering for recommendations
- Export search results
- Saved searches and alerts