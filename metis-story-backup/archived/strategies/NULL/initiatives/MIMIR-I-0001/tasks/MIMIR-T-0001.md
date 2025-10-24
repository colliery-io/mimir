---
id: character-database-schema
level: task
title: "Character Database Schema"
short_code: "MIMIR-T-0001"
created_at: 2025-10-16T13:14:33.667671+00:00
updated_at: 2025-10-16T13:14:33.667671+00:00
parent: MIMIR-I-0001
blocked_by: []
archived: true

tags:
  - "#task"
  - "#phase/todo"


exit_criteria_met: false
strategy_id: NULL
initiative_id: MIMIR-I-0001
---

# Character Database Schema

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0001]]

## Objective

Create the database schema for persistent character data management, including player characters, their classes, inventory, and spells. This forms the foundation for all character-related features.

## Acceptance Criteria

## Acceptance Criteria

- [ ] Migration created for player_characters table
- [ ] Migration created for player_character_classes table  
- [ ] Migration created for player_character_inventory table
- [ ] Migration created for player_character_spells table
- [ ] Migration created for player_character_selections table
- [ ] Character sheet view created for easy querying
- [ ] All migrations reversible (up/down)
- [ ] Foreign key constraints properly defined
- [ ] All migrations run successfully
- [ ] Rollback works cleanly
- [ ] Foreign key constraints enforced
- [ ] JSON validation works
- [ ] Character sheet view returns correct data

## Test Cases **[CONDITIONAL: Testing Task]**

{Delete unless this is a testing task}

### Test Case 1: {Test Case Name}
- **Test ID**: TC-001
- **Preconditions**: {What must be true before testing}
- **Steps**: 
  1. {Step 1}
  2. {Step 2}
  3. {Step 3}
- **Expected Results**: {What should happen}
- **Actual Results**: {To be filled during execution}
- **Status**: {Pass/Fail/Blocked}

### Test Case 2: {Test Case Name}
- **Test ID**: TC-002
- **Preconditions**: {What must be true before testing}
- **Steps**: 
  1. {Step 1}
  2. {Step 2}
- **Expected Results**: {What should happen}
- **Actual Results**: {To be filled during execution}
- **Status**: {Pass/Fail/Blocked}

## Documentation Sections **[CONDITIONAL: Documentation Task]**

{Delete unless this is a documentation task}

### User Guide Content
- **Feature Description**: {What this feature does and why it's useful}
- **Prerequisites**: {What users need before using this feature}
- **Step-by-Step Instructions**:
  1. {Step 1 with screenshots/examples}
  2. {Step 2 with screenshots/examples}
  3. {Step 3 with screenshots/examples}

### Troubleshooting Guide
- **Common Issue 1**: {Problem description and solution}
- **Common Issue 2**: {Problem description and solution}
- **Error Messages**: {List of error messages and what they mean}

### API Documentation **[CONDITIONAL: API Documentation]**
- **Endpoint**: {API endpoint description}
- **Parameters**: {Required and optional parameters}
- **Example Request**: {Code example}
- **Example Response**: {Expected response format}

## Implementation Notes

### Database Schema Implementation

#### Core Tables
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

-- Other tables: player_character_classes, player_character_inventory, 
-- player_character_spells, player_character_selections
```

#### Character State JSON Structure
```json
{
    "ability_scores": {"str": 10, "dex": 10, "con": 10, "int": 10, "wis": 10, "cha": 10},
    "proficiencies": {
        "armor": ["light", "medium"],
        "weapons": ["simple"],
        "tools": ["thieves_tools"],
        "skills": ["stealth", "acrobatics"],
        "saves": ["dex", "int"],
        "languages": ["common", "elvish"]
    },
    "features": {"racial": [], "class": [], "background": [], "feats": []},
    "spell_casting": {
        "spell_ability": "int",
        "spell_save_dc": 15,
        "spell_attack_bonus": 7,
        "spell_slots": {"1": 4, "2": 3}
    }
}
```

### Dependencies
- Core rules must be imported first (for foreign keys)
- Existing campaign tables

### Technical Approach
1. Create migration files (`0016_create_player_characters/up.sql`, `down.sql`)
2. Add performance indexes (campaign_id, player_name, level)
3. Create character sheet view with table joins
4. Add updated_at triggers

## Status Updates **[REQUIRED]**

*To be added during implementation*