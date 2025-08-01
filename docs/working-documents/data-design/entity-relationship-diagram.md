# Entity Relationship Diagram (Denormalized)

## Overview
This document describes the relationships between entities in the Mimir database schema, optimized for read-heavy performance with denormalized structures.

## ERD Notation
- `PK` = Primary Key
- `FK` = Foreign Key
- `1` = One
- `*` = Many
- `0..1` = Zero or One
- `1..*` = One or Many
- `JSON` = JSON field containing denormalized data

## Core Entity Relationships

```
┌─────────────────┐
│  RULE_SYSTEMS   │
│─────────────────│
│ PK: id          │  (e.g., 'dnd5e-2014', 'dnd5e-2024', 'pf2e')
│ name            │
│ publisher       │
│ version         │
└─────────────────┘
       │ 1
       │
       │ *
┌─────────────────┐
│    SOURCES      │
│─────────────────│
│ PK: id          │  (e.g., 'PHB', 'MM', 'XGE')
│ FK: rule_system │
│ full_name       │
│ is_official     │
│ is_srd          │
└─────────────────┘
       │ 1
       │
       ├──────────────────────────────────────┬────────────────────────┬─────────────────┐
       │ *                                    │ *                      │ *               │ *
┌─────────────────┐     ┌─────────────────┐  ┌─────────────────┐     ┌─────────────────┐
│     RACES       │     │     ITEMS       │  │  BACKGROUNDS    │     │     FEATS       │
│─────────────────│     │─────────────────│  │─────────────────│     │─────────────────│
│ PK: id          │     │ PK: id          │  │ PK: id          │     │ PK: id          │
│ FK: rule_system │     │ FK: rule_system │  │ FK: rule_system │     │ FK: rule_system │
│ FK: source      │     │ FK: source      │  │ FK: source      │     │ FK: source      │
│ race_type       │     │ base_item_id◄───┘  │ name            │     │ name            │
│ parent_race_id◄─┘     │ [All fields     │  │ skills (JSON)   │     │ feat_type       │
│ [All fields     │     │  populated]     │  │ languages(JSON) │     │ prereqs (JSON)  │
│  populated]     │     │ properties(JSON)│  └─────────────────┘     └─────────────────┘
└─────────────────┘     │ magic_props(JSON)
                        └─────────────────┘

┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│    CLASSES      │     │    SPELLS       │     │   CREATURES     │
│─────────────────│     │─────────────────│     │─────────────────│
│ PK: id          │     │ PK: id          │     │ PK: id          │
│ FK: rule_system │     │ FK: rule_system │     │ FK: rule_system │
│ FK: source      │     │ FK: source      │     │ FK: source      │
│ class_type      │     │ name            │     │ name            │
│ parent_class_id◄┘     │ level           │     │ type            │
│ [All fields     │     │ school          │     │ cr              │
│  populated]     │     │ classes (JSON)  │     │ actions (JSON)  │
│ features (JSON) │     │ [No junction    │     │ traits (JSON)   │
└─────────────────┘     │  table needed]  │     │ reactions(JSON) │
                        └─────────────────┘     │ [No separate    │
                                                │  actions table] │
                                                └─────────────────┘
```

## Key Design Decisions

### Denormalization Benefits
1. **No Junction Tables**: 
   - Spells store classes as JSON array (no spell_classes table)
   - Creatures store actions/traits/reactions as JSON (no creature_actions table)
   - Items store properties as JSON (no item_properties table)
2. **Full Data Population**: Subclasses/subraces/item variants have all base fields populated
3. **Single Query Access**: No joins needed for common queries
4. **Simplified Queries**: Direct JSON queries instead of complex joins

### Multi-Ruleset Support
1. **Rule Systems Table**: Supports D&D 5e 2014, D&D 5e 2024, Pathfinder, etc.
2. **Source Hierarchy**: Sources belong to rule systems
3. **Entity Isolation**: Each entity linked to specific rule system and source
4. **Version Comparison**: Can query same content across rule systems

### Self-Referential Relationships (Maintained)
1. **Races**: `parent_race_id` for race/subrace hierarchy
2. **Classes**: `parent_class_id` for class/subclass hierarchy  
3. **Items**: `base_item_id` for item/variant relationships

## Referential Integrity

### Foreign Key Constraints
- All entities reference valid rule_system_id and source_id
- Parent references (parent_race_id, parent_class_id, base_item_id) must exist

### Cascade Rules
- Deleting a rule system or source is restricted if entities reference it
- Parent deletions (race/class/item) set child references to NULL

## Index Strategy

### Primary Indices
```sql
-- Every table has indices on:
CREATE INDEX idx_[table]_rule_system ON [table](rule_system_id);
CREATE INDEX idx_[table]_source ON [table](source_id);
CREATE INDEX idx_[table]_name ON [table](name);
```

### Relationship Indices
```sql
-- Self-referential relationships
CREATE INDEX idx_races_parent ON races(parent_race_id) WHERE parent_race_id IS NOT NULL;
CREATE INDEX idx_classes_parent ON classes(parent_class_id) WHERE parent_class_id IS NOT NULL;
CREATE INDEX idx_items_base ON items(base_item_id) WHERE base_item_id IS NOT NULL;
```

### JSON Query Indices
```sql
-- For querying JSON arrays efficiently
CREATE INDEX idx_spells_classes_json ON spells(classes);
-- SQLite will use these for json_each() queries
```