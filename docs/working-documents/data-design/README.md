# Mimir Data Design Documentation

## Overview
This directory contains the comprehensive data design for the Mimir project, which transforms D&D 5e content from 5etools JSON format into an efficient SQLite database with separated search indices.

## Design Principles

### 1. **Separation of Concerns**
- Core entity data in normalized tables
- Full-text search (FTS5) in separate tables with foreign keys
- Vector embeddings in dedicated tables for semantic search
- Configurable chunking strategies for different content types

### 2. **Performance First**
- Sub-second query performance through proper indexing
- Specialized search tables for different entity types
- Efficient foreign key relationships
- Covering indices for common query patterns

### 3. **Flexibility**
- JSONB fields for complex, variable structures
- Configurable text chunking for embeddings
- Support for multiple embedding models
- Extensible schema for new content types

### 4. **Data Integrity**
- Referential integrity through foreign keys
- Validation rules in transformation layer
- Comprehensive error handling
- Import logging and tracking

## Documentation Structure

### [5etools Data Inventory](./5etools-data-inventory.md)
Complete inventory of 5etools JSON files organized by category:
- Core game mechanics (races, items, spells, etc.)
- Adventure and book content
- Class and subclass data
- Generated and indexed content
- Common data patterns and formatting

### [SQLite Schema Design](./sqlite-schema-design.sql)
Complete database schema with:
- Core entity tables (races, items, spells, creatures, etc.)
- Separated FTS5 search tables
- Vector embedding storage
- Relationship tables
- Metadata and configuration
- Optimized views for common queries

### [Data Dictionary](./data-dictionary.md)
Detailed field definitions including:
- Field types and constraints
- Business rules and validations
- JSONB structure specifications
- Enumerated value lists
- Relationship cardinalities

### [Transformation Rules](./transformation-rules.md)
Python-based transformation logic:
- ID generation strategies
- Cross-reference extraction
- Text processing for search
- Entity-specific transformation rules
- Import order for referential integrity
- Error handling strategies

### [Entity Relationship Diagram](./entity-relationship-diagram.md)
Visual and textual representation of:
- Entity relationships
- Cardinality rules
- Search table connections
- Referential integrity rules
- Index strategies

### [Chunking Strategy](./chunking-strategy.md)
Content chunking for vector embeddings:
- Entity-specific chunking methods
- Size and overlap configuration
- Context preservation techniques
- Quality validation metrics
- Special case handling

## Implementation Roadmap

### Phase 1: Core Infrastructure
1. Create SQLite database with base schema
2. Implement source and metadata tables
3. Set up import logging framework
4. Create base transformation utilities

### Phase 2: Entity Import
1. Import sources and reference data
2. Transform and load core entities (races, items, etc.)
3. Process class and spell data
4. Import creature data with actions

### Phase 3: Search Indices
1. Populate FTS5 tables
2. Create specialized search indices
3. Implement cross-reference extraction
4. Build reference lookup tables

### Phase 4: Vector Embeddings
1. Implement chunking strategies
2. Generate embeddings for all content
3. Store in embedding tables
4. Create similarity search functions

### Phase 5: Optimization
1. Analyze query patterns
2. Create covering indices
3. Optimize chunking strategies
4. Performance testing and tuning

## Key Design Decisions

### Why Separate Search Tables?
- **Performance**: Specialized indices for different search types
- **Flexibility**: Can re-index without touching core data
- **Maintenance**: Easy to update search strategies
- **Scalability**: Can add new search methods independently

### Why JSONB for Complex Fields?
- **Schema Evolution**: 5etools data structure changes over time
- **Query Flexibility**: SQLite's JSON functions enable complex queries
- **Storage Efficiency**: Better than multiple nullable columns
- **Compatibility**: Preserves original data structure

### Why Configurable Chunking?
- **Content Variety**: Different content types need different strategies
- **Model Evolution**: Can adapt to new embedding models
- **Quality Control**: Ensures semantic coherence in chunks
- **Performance**: Optimal chunk sizes for vector search

## Usage Examples

### Query Examples
```sql
-- Find all spells available to wizards at 3rd level
SELECT s.* FROM spells s
JOIN spell_classes sc ON s.id = sc.spell_id
JOIN classes c ON sc.class_id = c.id
WHERE c.name = 'Wizard' AND s.level = 3;

-- Full-text search for fire damage
SELECT * FROM spell_search 
WHERE spell_search MATCH 'fire damage'
ORDER BY rank;

-- Find creatures by CR range with resistances
SELECT * FROM creatures
WHERE CAST(challenge_rating AS REAL) BETWEEN 5 AND 10
AND json_array_length(damage_resistances) > 0;
```

### Embedding Search
```sql
-- Find similar content using embeddings
WITH target_embedding AS (
    SELECT embedding FROM content_embeddings
    WHERE entity_id = 'fireball_phb'
    LIMIT 1
)
SELECT 
    ce.entity_id,
    ce.entity_type,
    ce.chunk_text,
    vector_similarity(ce.embedding, te.embedding) as similarity
FROM content_embeddings ce, target_embedding te
WHERE ce.entity_type = 'spell'
ORDER BY similarity DESC
LIMIT 10;
```

## Maintenance Notes

### Adding New Entity Types
1. Create core entity table
2. Add FTS5 search table if needed
3. Define transformation rules
4. Configure chunking strategy
5. Update import order

### Updating Search Indices
1. Clear relevant FTS5 table
2. Re-run text extraction
3. Populate with new content
4. Verify search quality

### Schema Migrations
1. Use version tracking table
2. Write migration scripts
3. Test on sample data
4. Apply with transaction safety
5. Update documentation

## Performance Targets

- **Entity Lookup**: < 10ms
- **FTS Search**: < 50ms for common terms
- **Complex Queries**: < 100ms
- **Embedding Search**: < 200ms for top-10
- **Bulk Import**: < 5 minutes for full dataset

## Next Steps

1. Review and approve schema design
2. Set up development database
3. Implement transformation utilities
4. Begin phased data import
5. Create search interfaces
6. Integrate with Mimir application