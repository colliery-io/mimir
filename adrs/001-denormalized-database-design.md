---
id: 001-denormalized-database-design
level: adr
title: "Denormalized Database Design"
number: 1
created_at: 2025-07-30T15:10:12.829590+00:00
updated_at: 2025-07-30T15:10:12.829590+00:00
decision_date: 2025-01-30
decision_maker: Engineering Team 
parent: 
archived: false

tags:
  - "#adr"
  - "#phase/decided"


exit_criteria_met: false
---

# ADR-1: Denormalized Database Design

## Context

Mimir is designed as a read-heavy D&D campaign assistant where data is imported once from 5etools JSON files and then queried constantly during gameplay. The traditional normalized database design would require multiple joins for common queries like "show me all details for a Fighter subclass" or "list all properties of a +1 Longsword". 

During schema design, we discovered that many entities have hierarchical relationships (races/subraces, classes/subclasses, items/variants) where the child entities need most or all of the parent's data plus their own modifications.

## Decision

We will use a fully denormalized database design where:

1. **Single Table Per Entity Type**: Classes and subclasses share one table, races and subraces share one table, items and variants share one table
2. **Full Data Population**: Child entities (subclasses, subraces, item variants) have ALL fields populated, including data copied from their parent
3. **JSON Arrays Instead of Junction Tables**: Store relationships like spell classes and creature actions as JSON arrays within the main record
4. **Self-Referential Foreign Keys**: Maintain parent_id relationships for filtering and hierarchy navigation

## Rationale

The denormalized approach was chosen because:

1. **Read Performance**: No joins needed for common queries - all data is available in a single row
2. **Query Simplicity**: Developers can write simple SELECT statements without complex JOIN logic
3. **Write-Once Pattern**: D&D rules data is imported once and rarely updated, making denormalization overhead negligible
4. **Natural Fit**: The inheritance pattern (subclass extends class) maps well to copying parent data

Alternatives considered:
- **Normalized Design**: Would require 5-6 joins for a complete subclass view
- **Materialized Views**: Add complexity and SQLite has limited support
- **Application-Level Joins**: Would push complexity to the application layer

## Consequences

### Positive
- **Blazing Fast Queries**: Single-table queries with no joins provide sub-millisecond response times
- **Simple Codebase**: Queries are straightforward SELECTs with WHERE clauses
- **Better Cache Utilization**: Related data is physically co-located in storage
- **Easier Debugging**: Query results are complete records, not assembled from multiple tables

### Negative
- **Storage Overhead**: Duplicated data increases database size (estimated 2-3x)
- **Import Complexity**: Import process must handle parent-child data merging
- **Update Anomalies**: If base data changes, all variants must be updated (mitigated by write-once pattern)

### Neutral
- **Schema Evolution**: Adding new fields requires updating all variant records
- **JSON Querying**: SQLite's JSON functions are needed for array fields (well-supported since 3.9.0)
