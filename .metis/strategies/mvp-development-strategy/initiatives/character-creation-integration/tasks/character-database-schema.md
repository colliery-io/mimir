---
id: character-database-schema
level: task
title: "Character Database Schema"
created_at: 2025-08-17T02:38:00.000000+00:00
updated_at: 2025-08-17T02:38:00.000000+00:00
parent: character-creation-integration
blocked_by: ["bundle-core-rules-in-binary"]
archived: false

tags:
  - "#task"
  - "#phase/todo"

exit_criteria_met: false
---

# Character Database Schema

## Overview
Create the database schema for persistent character data management, including player characters, their classes, inventory, and spells. This forms the foundation for all character-related features.

## Acceptance Criteria

- [ ] Migration created for player_characters table
- [ ] Migration created for player_character_classes table  
- [ ] Migration created for player_character_inventory table
- [ ] Migration created for player_character_spells table
- [ ] Migration created for player_character_selections table
- [ ] Character sheet view created for easy querying
- [ ] All migrations reversible (up/down)
- [ ] Foreign key constraints properly defined

## Database Tables

### Core Tables
```sql
-- Player Characters (persistent data only)
CREATE TABLE player_characters (
    id INTEGER PRIMARY KEY,
    campaign_id INTEGER REFERENCES campaigns(id),
    rule_system_id TEXT NOT NULL REFERENCES rule_systems(id),
    name TEXT NOT NULL,
    player_name TEXT,
    race_id TEXT REFERENCES races(id),
    background_id TEXT REFERENCES backgrounds(id),
    alignment TEXT,
    experience_points INTEGER DEFAULT 0,
    level INTEGER DEFAULT 1,
    hit_points_max INTEGER,
    armor_class INTEGER,
    initiative_bonus INTEGER,
    speed INTEGER DEFAULT 30,
    ability_scores TEXT CHECK(json_valid(ability_scores)),
    character_state TEXT CHECK(json_valid(character_state)),
    notes TEXT,
    backstory TEXT,
    appearance TEXT,
    personality_traits TEXT,
    ideals TEXT,
    bonds TEXT,
    flaws TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Multiclassing support
CREATE TABLE player_character_classes (
    player_character_id INTEGER REFERENCES player_characters(id),
    class_id TEXT REFERENCES classes(id),
    class_level INTEGER NOT NULL,
    subclass_id TEXT REFERENCES classes(id),
    hit_dice_count INTEGER NOT NULL,
    PRIMARY KEY (player_character_id, class_id)
);

-- Equipment tracking
CREATE TABLE player_character_inventory (
    player_character_id INTEGER REFERENCES player_characters(id),
    item_id TEXT REFERENCES items(id),
    quantity INTEGER DEFAULT 1,
    equipped BOOLEAN DEFAULT FALSE,
    attunement_required BOOLEAN DEFAULT FALSE,
    attuned BOOLEAN DEFAULT FALSE,
    custom_name TEXT,
    notes TEXT,
    PRIMARY KEY (player_character_id, item_id)
);

-- Spell management
CREATE TABLE player_character_spells (
    player_character_id INTEGER REFERENCES player_characters(id),
    spell_id TEXT REFERENCES spells(id),
    spell_source TEXT NOT NULL,
    source_id TEXT NOT NULL,
    always_prepared BOOLEAN DEFAULT FALSE,
    prepared BOOLEAN DEFAULT FALSE,
    PRIMARY KEY (player_character_id, spell_id, spell_source)
);

-- Track character creation choices
CREATE TABLE player_character_selections (
    id INTEGER PRIMARY KEY,
    player_character_id INTEGER REFERENCES player_characters(id),
    rule_id TEXT,
    selection_type TEXT NOT NULL,
    selection_value TEXT NOT NULL CHECK(json_valid(selection_value)),
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);
```

### Character State JSON Structure
```json
{
    "ability_scores": {
        "str": 10, "dex": 10, "con": 10,
        "int": 10, "wis": 10, "cha": 10
    },
    "proficiencies": {
        "armor": ["light", "medium"],
        "weapons": ["simple"],
        "tools": ["thieves_tools"],
        "skills": ["stealth", "acrobatics"],
        "saves": ["dex", "int"],
        "languages": ["common", "elvish"]
    },
    "features": {
        "racial": [],
        "class": [],
        "background": [],
        "feats": []
    },
    "spell_casting": {
        "spell_ability": "int",
        "spell_save_dc": 15,
        "spell_attack_bonus": 7,
        "spell_slots": {"1": 4, "2": 3}
    }
}
```

## Implementation Steps

1. **Create migration files**
   - `0016_create_player_characters/up.sql`
   - `0016_create_player_characters/down.sql`

2. **Add indexes for performance**
   - Index on campaign_id for party queries
   - Index on player_name for search
   - Index on level for filtering

3. **Create character sheet view**
   - Join relevant tables
   - Aggregate class information
   - Format for easy consumption

4. **Add triggers for updated_at**
   - Auto-update timestamp on changes

## Testing Requirements

- [ ] All migrations run successfully
- [ ] Rollback works cleanly
- [ ] Foreign key constraints enforced
- [ ] JSON validation works
- [ ] Character sheet view returns correct data

## Dependencies
- Core rules must be imported first (for foreign keys)
- Existing campaign tables

## Estimated Effort
2-3 days

## Notes
- Keep schema focused on persistent data only
- Combat tracking (HP, spell slots) handled in session
- Consider future expansion for homebrew content