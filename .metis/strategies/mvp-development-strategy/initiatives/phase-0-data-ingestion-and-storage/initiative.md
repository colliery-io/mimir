---
id: phase-0-data-ingestion-and-storage
level: initiative
title: "Phase 0: Data Ingestion and Storage Foundation"
created_at: 2025-07-30T00:50:39.662886+00:00
updated_at: 2025-07-30T02:03:52.076089+00:00
parent: mvp-development-strategy
blocked_by: []
archived: false

tags:
  - "#initiative"
  - "#phase/decompose"


exit_criteria_met: false
estimated_complexity: M
---

# Phase 0: Data Ingestion and Storage Foundation Initiative

## Context

Mimir needs comprehensive D&D 5e data to function as an intelligent assistant. The 5etools project provides the most complete collection of structured D&D data in JSON format, but it requires significant processing to be useful for our application. This phase establishes the data foundation that all future features will build upon.

We must transform complex, nested JSON structures into a clean, queryable SQLite database while preserving all the richness needed for game mechanics, maintaining source attribution, and ensuring reasonable (to be defined) query performance.

## Goals & Non-Goals

**Goals:**
- Extract and transform 13 core 5etools JSON file types into normalized SQLite tables
- Create reusable import pipeline that can handle data updates
- Maintain source attribution (book, page) for all content
- Design extensible schema supporting future phases
- Document all data transformations and design decisions

**Non-Goals:**
- Full-text search indexing (Phase 1)
- Vector embeddings or semantic search (Phase 1)
- Data modification or homebrew support
- Real-time data sync with 5etools
- UI for data browsing
- Complex derived calculations

## Detailed Design

### Data Sources
Core 5etools files to process:

> Note - some of these files MIGHT actually be concatenations of multiple 5etools files. We will also be importing ALL core DnD 5e rule books. (i.e PHB	DMG MM	TCoE	VGtM	XGtE MToF FToD MotM BAM	 GttG	BoM) The pipeline will likely be required to do some "munging" to get to the following "list".

- `races.json` - Player character races
- `classes.json` - Character classes and subclasses  
- `spells.json` - All spells with full descriptions
- `items.json` - Equipment, magic items, treasures
- `deities.json`
- `bestiary/bestiary-*.json` - Monsters 
- `backgrounds.json` - Character backgrounds
- `feats.json` - Optional character feats
- `conditions.json` - Status conditions
- `skills.json` - Skill descriptions
- `actions.json` - Combat actions
- `languages.json` - Language list
- `books.json` - Source book metadata
- `adventures.json` - Published adventure metadata

### Schema Design

> This is a first pass of schema designs, they should be expected to be updated based on need.

**Core Tables:**
```sql
-- Unified entity table for cross-referencing
entities (
  id TEXT PRIMARY KEY,  -- Generated consistent ID
  type TEXT NOT NULL,   -- spell, item, creature, etc
  name TEXT NOT NULL,
  source_book TEXT,
  source_page INTEGER
)

-- Type-specific tables with denormalized data
spells (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  level INTEGER,
  school TEXT,
  casting_time TEXT,
  range TEXT,
  components TEXT,
  duration TEXT,
  description TEXT,      -- Cleaned markdown
  higher_levels TEXT,
  classes JSON,          -- Array of class names
  source_book TEXT,
  source_page INTEGER
)

creatures (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  size TEXT,
  type TEXT,
  alignment TEXT,
  cr REAL,
  xp INTEGER,
  hp_formula TEXT,
  ac_value INTEGER,
  ac_description TEXT,
  speeds JSON,
  abilities JSON,        -- STR, DEX, etc
  traits JSON,
  actions JSON,
  legendary_actions JSON,
  source_book TEXT,
  source_page INTEGER
)
```

### Import Pipeline Architecture

> We will have a pipeline per "rule set", we're starting with 2014, but will have 2024 as well - potentially other systems entirely later. But all will have the same "target" schema for obvious reasons. 

```rust
// Unified bundle import approach
struct BundleImporter {
    db: SqliteConnection,
    validator: BundleValidator,
}

impl BundleImporter {
    fn import_bundle(&self, bundle_path: &Path) -> Result<()> {
        // 1. Validate bundle structure and manifest
        // 2. Extract and parse all entity files
        // 3. Transform to database format
        // 4. Atomic database import
        // 5. Verify and log import
    }
}
```

### Data Transformations
- Strip 5etools tags: `{@spell fireball}` → `fireball`, but keep it in descriptions we'll use it for rendering later.
- Convert dice notation: Keep original for display
- Flatten nested structures where sensible
- Preserve arrays as JSON for complex data
- Generate deterministic IDs: `spell_fireball_phb`

### Performance Optimizations
- Batch inserts with transactions
- Create indices after bulk load
- Use prepared statements
- Parallel JSON parsing


## Alternatives Considered

**PostgreSQL with JSONB**
- Pro: Better JSON querying, more powerful
- Con: Requires server setup, violates local-first principle
- Rejected: Complexity outweighs benefits for single-user app

**Document Database (MongoDB)**
- Pro: Natural fit for nested JSON data
- Con: Poor aggregation performance, large runtime
- Rejected: SQLite sufficient with JSON columns where needed

**Keep Raw JSON Files**
- Pro: No transformation needed, simple
- Con: Terrible query performance, no relationships
- Rejected: Cannot meet sub-100ms query requirement

**Graph Database (Neo4j)**
- Pro: Natural for D&D relationships (class→subclass→features)
- Con: Massive overhead, complex deployment
- Rejected: Overkill for our use case

**Full Normalization**
- Pro: Theoretical purity, no redundancy
- Con: Complex joins hurt performance, harder to maintain
- Rejected: Denormalization better for read-heavy workload

## Implementation Plan

### Week 1-2: Schema Design & Prototyping
- Analyze all 5etools JSON structures
- Design SQLite schema with appropriate types
- Create database migration framework
- Build prototype with 2-3 file types

### Week 3-4: Unified Import Pipeline
- Implement unified BundleImporter struct
- Create bundle validation and extraction logic
- Build atomic transaction management
- Add progress reporting with user feedback
- Implement error handling and rollback

### Week 5-6: CLI Integration & Polish
- Integrate with main mimir CLI as import subcommand
- Add import logging and history tracking
- Handle edge cases in bundle processing
- Build comprehensive validation suite
- Performance optimization and testing
- Performance optimization pass

### Week 7: Testing & Documentation
- Comprehensive integration tests
- Performance benchmarking
- Document all transformations
- Create data dictionary
- Write usage examples

### Week 8: Polish & Delivery
- CLI tool for import
- Automated validation reports
- Performance tuning
- Final documentation
- Prepare for Phase 0.5 integration

## Testing Strategy

### Unit Tests
- Test unified bundle import workflow
- Validate bundle structure and manifest parsing
- Ensure atomic transaction behavior
- Test error handling and rollback scenarios
- Validate ID generation consistency
- Verify error cases handled gracefully

### Integration Tests
- Full pipeline execution with sample data
- Cross-reference integrity checks
- Source attribution accuracy
- Performance benchmarks per file type
- Database size validation

### Validation Suite
```rust
// Automated checks after import
- Total entity count matches source
- No duplicate IDs across types
- All required fields populated
- Source references valid
- Foreign key relationships intact
```

### Query Performance Tests
- Spell lookup by name: < 10ms
- Creature filtering by CR: < 50ms
- Class feature retrieval: < 20ms
- Item search by type: < 30ms
- Complex joins: < 100ms

### Data Quality Metrics
- Zero data loss from source files
- 100% of entities have valid IDs
- All markdown cleaned properly
- Source attribution 100% complete
- No orphaned references