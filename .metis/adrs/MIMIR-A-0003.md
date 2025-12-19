---
id: 003-json-storage-strategy
level: adr
title: "JSON Storage Strategy"
number: 3
short_code: "MIMIR-A-0003"
created_at: 2025-10-24T12:01:58.409597+00:00
updated_at: 2025-10-24T12:02:38.150845+00:00
decision_date: 
decision_maker: Engineering Team
parent: 
archived: false

tags:
  - "#adr"
  - "#phase/decided"


exit_criteria_met: false
strategy_id: NULL
initiative_id: NULL
---

# ADR-3: JSON Storage Strategy

## Context

D&D game content has complex, nested data structures:
- Spells have varying numbers of classes that can cast them
- Creatures have arrays of actions, traits, and legendary actions
- Items have lists of properties and magical effects
- Classes have feature progressions and spell slot tables

SQLite offers JSON support through its JSON1 extension (built-in since version 3.9.0), allowing storage and querying of JSON data. We need to decide how to handle these complex structures.

## Decision

We will use SQLite's JSON1 extension to store complex data:

1. **TEXT Columns with JSON**: Store JSON data in TEXT columns with CHECK(json_valid(column)) constraints
2. **Arrays for Variable-Length Lists**: 
   - Spell classes stored as JSON array (e.g., ["wizard", "sorcerer"])
   - Creature actions/traits stored as JSON arrays
   - Item properties stored as JSON array
3. **Objects for Complex Structures**:
   - Ability scores as objects (e.g., {"str": 15, "dex": 14})
   - Speed as objects (e.g., {"walk": 30, "fly": 60})
4. **JSON Functions for Queries**: Use json_each(), json_extract() for searching within JSON

## Rationale

JSON storage was chosen because:

1. **Preserves Structure**: Maintains the original 5etools data structure for easy import/export
2. **Flexible Schema**: Can handle varying numbers of elements without schema changes
3. **Native Support**: SQLite's JSON1 is mature and performant
4. **Query Capability**: Can search within JSON using SQL functions

Alternatives considered:
- **Normalized Tables**: Would require many junction tables (spell_classes, creature_actions, etc.)
- **Serialized Blobs**: Would lose query capability
- **XML Storage**: More verbose and less tool support
- **PostgreSQL JSONB**: Would require different database engine

## Consequences

### Positive
- **Rich Queries**: Can search for spells by class using json_each()
- **Maintains Fidelity**: Original data structure preserved
- **Easy Import**: Minimal transformation from source JSON files
- **Future Compatibility**: New fields can be added to JSON without schema changes

### Negative
- **Index Limitations**: Cannot create traditional indexes on JSON array elements
- **Query Complexity**: JSON queries are more verbose than simple column queries
- **Type Safety**: JSON data is not strongly typed at database level

### Neutral
- **Performance**: JSON queries are slower than column queries but still sub-second
- **Storage Size**: JSON adds some overhead compared to normalized tables
- **Tooling**: Some database tools have limited JSON visualization support