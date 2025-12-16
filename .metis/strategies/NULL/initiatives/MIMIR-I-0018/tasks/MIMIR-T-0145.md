---
id: add-module-monsters-table-and
level: task
title: "Add module_monsters table and service"
short_code: "MIMIR-T-0145"
created_at: 2025-12-16T16:23:42.202635+00:00
updated_at: 2025-12-16T16:23:42.202635+00:00
parent: MIMIR-I-0018
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/todo"


exit_criteria_met: false
strategy_id: NULL
initiative_id: MIMIR-I-0018
---

# Add module_monsters table and service

## Parent Initiative

[[MIMIR-I-0018]] - Module Play Mode (DM Screen)

## Objective

Create the backend infrastructure to associate monsters from the catalog with specific modules, enabling the Play Mode to display relevant monsters for each module.

## Acceptance Criteria

- [ ] Migration creates `module_monsters` table with schema:
  - `id` (primary key)
  - `module_id` (foreign key to modules)
  - `monster_name` (String)
  - `monster_source` (String) 
  - `quantity` (i32, default 1)
  - `encounter_tag` (Option<String>)
  - `created_at`, `updated_at` timestamps
- [ ] `ModuleMonster` model with Diesel mappings
- [ ] `ModuleMonsterRepository` with CRUD operations
- [ ] `ModuleMonsterService` with:
  - `add_monster_to_module(module_id, monster_name, source, quantity, encounter_tag)`
  - `remove_monster_from_module(module_id, monster_id)`
  - `update_monster_entry(id, quantity, encounter_tag)`
  - `get_monsters_for_module(module_id)` - returns with full monster data from catalog
  - `get_monsters_grouped_by_encounter(module_id)` - groups by encounter_tag
- [ ] Tauri commands for frontend integration
- [ ] Unit tests for service methods

## Implementation Notes

### Schema

```sql
CREATE TABLE module_monsters (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    module_id INTEGER NOT NULL REFERENCES modules(id) ON DELETE CASCADE,
    monster_name TEXT NOT NULL,
    monster_source TEXT NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 1,
    encounter_tag TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX idx_module_monsters_module_id ON module_monsters(module_id);
```

### Files to Create/Modify

- `crates/mimir-dm-core/migrations/034_create_module_monsters/up.sql`
- `crates/mimir-dm-core/src/models/campaign/module_monsters.rs`
- `crates/mimir-dm-core/src/dal/campaign/module_monsters.rs`
- `crates/mimir-dm-core/src/services/module_monster_service.rs`
- `crates/mimir-dm/src/commands/modules.rs` (add Tauri commands)

### Dependencies

- Existing `MonsterService` for catalog lookups
- Existing module infrastructure

## Status Updates

*To be added during implementation*