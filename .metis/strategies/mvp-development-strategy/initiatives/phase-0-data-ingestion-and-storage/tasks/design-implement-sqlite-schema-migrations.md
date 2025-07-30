---
id: design-implement-sqlite-schema-migrations
level: task
title: "Design and Implement SQLite Schema and Migrations"
created_at: 2025-07-30T22:00:00+00:00
updated_at: 2025-07-30T22:00:00+00:00
parent: phase-0-data-ingestion-and-storage
blocked_by: ["design-sqlite-schema"]
archived: false

tags:
  - "#task"
  - "#phase/todo"
  - "#database"
  - "#schema"
  - "#migrations"

exit_criteria_met: false
---

# Design and Implement SQLite Schema and Migrations

## Description

Create the foundational database schema implementation in Rust, including all entity tables, migration system, and supporting infrastructure for the Mimir data layer.

## Acceptance Criteria

- [ ] Create Rust schema module with table definitions
- [ ] Implement all core entity tables (races, classes, items, spells, creatures, backgrounds, feats)
- [ ] Create supporting tables (rule_systems, sources, content_references, import_log)
- [ ] Build migration system with version tracking
- [ ] Implement Rust structs matching database schemas
- [ ] Add schema validation and integrity checks
- [ ] Create database connection pool management
- [ ] Add transaction support for atomic operations
- [ ] Implement basic CRUD operations for each entity type
- [ ] Add comprehensive unit tests
- [ ] Create integration tests with test data

## Technical Plan

### 1. Module Structure
```
mimir-dm-db/src/
├── lib.rs              # Public API exports
├── schema.rs           # Diesel generated schema
├── models/             # Rust structs matching tables (one module per model)
│   ├── mod.rs
│   ├── rule_systems.rs
│   ├── sources.rs
│   ├── races.rs
│   ├── classes.rs
│   ├── items.rs
│   ├── spells.rs
│   ├── creatures.rs
│   ├── backgrounds.rs
│   └── feats.rs
├── json_types/         # Custom types for JSON fields
│   ├── mod.rs
│   ├── speed.rs
│   ├── ability_scores.rs
│   └── ...
├── dal/                # Data Access Layer - async repository pattern
│   ├── mod.rs
│   ├── traits.rs       # Common repository traits
│   ├── rule_systems.rs
│   ├── sources.rs
│   ├── races.rs
│   ├── classes.rs
│   ├── items.rs
│   ├── spells.rs
│   ├── creatures.rs
│   ├── backgrounds.rs
│   └── feats.rs
├── connection.rs       # Database connection management
└── error.rs           # Database-specific errors

migrations/             # Diesel migration files
├── 00000000000000_diesel_initial_setup/
│   ├── up.sql
│   └── down.sql
├── 2025-01-30-000001_create_rule_systems/
│   ├── up.sql
│   └── down.sql
├── 2025-01-30-000002_create_sources/
│   ├── up.sql
│   └── down.sql
└── ...
```

### 2. Implementation Strategy - Table by Table Approach

Each table will be implemented end-to-end before moving to the next:

#### For each table:
1. Create migration files (up.sql/down.sql)
2. Define model struct in its own module
3. Implement CRUD operations
4. Write unit tests
5. Write integration tests
6. Verify with migration runner

#### Implementation Order:
1. **Core Infrastructure**
   - Basic setup (lib.rs, error.rs, connection.rs)
   - Migration runner function
   - Base traits for CRUD operations

2. **Foundation Tables** (implement each fully before next)
   - rule_systems table
   - sources table

3. **Entity Tables** (implement each fully before next)
   - races table
   - classes table
   - items table
   - spells table
   - creatures table
   - backgrounds table
   - feats table

4. **Support Tables**
   - content_references table
   - import_log table
   - schema_version table

### 3. Key Design Decisions

#### Migration Structure
- Each migration in its own folder with up.sql and down.sql
- Migrations numbered sequentially (001, 002, etc.)
- Migration runner function manages applying/rolling back
- Track applied migrations in schema_version table

#### Model Organization
- Each model in its own module file
- Models module with mod.rs to export all models
- Clear separation between database representation and business logic

#### Development Flow
- Complete one table at a time from migration to tests
- No "big bang" implementation - incremental progress
- Each table implementation is independently testable

#### JSON Field Handling
- Use rusqlite's JSON support for complex fields
- Create custom serde serializers for specific formats
- Validate JSON on insert/update

#### ID Generation
- IDs come from import data (already slugified)
- No auto-generation in database
- Enforce uniqueness constraints

#### Connection Management
- Use connection pool for concurrent access
- Separate read/write connections
- WAL mode for better concurrency
- Prepared statement caching

### 4. Updated Implementation Timeline

1. **Basic setup** (30 min)
   - Create lib.rs with module exports
   - Set up error types
   - Create connection module
   - Create migration runner

2. **Per Table Implementation** (45 min each for simple tables, 1hr for complex)
   - Simple tables: rule_systems, sources, backgrounds, feats
   - Complex tables: races, classes, items, spells, creatures
   - Support tables: 30 min each

3. **Integration and Polish** (1 hour)
   - Full database integration tests
   - Performance benchmarks
   - Documentation

## Technical Notes

### Dependencies and Technology Choices:

**ORM Decision**: 
- Use Diesel 2.1 exclusively for all database operations
- Leverage Diesel's migration system
- Use Diesel's type-safe query builder
- This provides consistency and type safety throughout

**Key Dependencies**:
- diesel: ORM with SQLite support and migrations
- diesel_migrations: For embedding migrations in the binary
- serde/serde_json: JSON serialization for complex fields
- chrono: Timestamp handling (with Diesel support)
- tokio: Async runtime (use spawn_blocking for Diesel operations)

### Implementation Considerations:

1. **Connection Management**:
   - Single connection for SQLite (no pool needed)
   - Use `tokio::task::spawn_blocking` for all database operations
   - Enable WAL mode for better read concurrency
   - Consider read-only connection for queries if needed

2. **Foreign Key Dependencies**:
   - Implement tables in dependency order
   - Foundation tables (rule_systems, sources) must be first
   - Enable foreign key constraints in SQLite via Diesel

3. **JSON Field Handling**:
   - Create custom Diesel types for JSON fields
   - Use serde for automatic serialization/deserialization
   - Store as TEXT with json_valid() constraints
   - Create strongly-typed structs for all JSON data

4. **Generated Columns**:
   - Handle `is_magic` and `classes_display` as computed columns
   - These are read-only from Rust perspective
   - Diesel will ignore these in inserts/updates

5. **Async Integration**:
   - Wrap all Diesel operations in `spawn_blocking`
   - Create async-friendly repository traits
   - Handle blocking operations properly to avoid blocking Tokio runtime

### Error Handling:
- Use Diesel's error types with custom wrappers
- Distinguish between constraint violations and other errors
- Map Diesel errors to domain-specific errors
- Proper error context with thiserror

### Testing Strategy:
- Unit tests for each model
- Integration tests with test database
- Migration tests (up and down)
- Foreign key constraint tests
- JSON serialization tests
- Async wrapper tests

## Dependencies

- Depends on: design-sqlite-schema (completed)
- Blocks: implement-data-processor-trait