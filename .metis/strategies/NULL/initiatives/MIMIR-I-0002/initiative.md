---
id: character-creation-integration
level: initiative
title: "Character Creation Integration"
short_code: "MIMIR-I-0002"
created_at: 2025-10-24T12:02:52.048605+00:00
updated_at: 2025-10-24T12:07:29.712746+00:00
parent: MIMIR-V-0001
blocked_by: []
archived: false

tags:
  - "#initiative"
  - "#phase/ready"


exit_criteria_met: false
estimated_complexity: M
strategy_id: NULL
initiative_id: character-creation-integration
---

# Character Creation Integration Initiative

## Executive Summary

This initiative implements character creation and between-session maintenance in Mimir. The system focuses on persistent character data management - stats, equipment, spell preparation, and level progression - not in-game tracking. Players will use printed character sheets during sessions to track HP, spell slots, and other consumable resources.

## Context

Previously part of the MVP Development Strategy, this initiative delivers the player character workflow that completes the campaign-to-player loop. With the data ingestion pipeline complete, we can now leverage the rules database to power intelligent character creation.

## Goals & Non-Goals

**Goals:**
- Complete PHB character creation workflow in under 10 minutes
- Between-session character management (level up, spell prep, inventory)
- Professional character sheet generation (printable)
- Database schema for persistent character data
- 5etools data conversion to character creation rules
- Rules engine for character building validation

**Non-Goals:**
- In-session tracking (HP, spell slots, conditions)
- Combat automation or turn tracking
- Character sheet digital editing during play
- Multiplayer character sharing
- AI-generated character backgrounds

## Architecture Overview

```
┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│  5etools Data   │────▶│ Rules Converter  │────▶│  Rules Database │
└─────────────────┘     └──────────────────┘     └────────┬────────┘
                                                           │
┌─────────────────┐     ┌──────────────────┐              │
│ Character Sheet │◀────│   Rules Engine   │◀─────────────┘
│    Generator    │     └──────────────────┘
└─────────────────┘              │
                                 ▼
                         ┌──────────────────┐
                         │ Character Data   │
                         └──────────────────┘
```

## Implementation Plan

### Phase 1: Database Schema (Weeks 1-2)
- Character tables (player_characters, player_character_classes, player_character_inventory)
- Character creation rules tables
- JSON state storage for complex character data
- Support for persistent data only (no consumable tracking)

### Phase 2: 5etools Conversion (Weeks 3-4)
- Convert races, classes, backgrounds to creation rules
- Extract proficiency grants, ability score increases
- Build spell slot progressions and feature tables
- Focus on permanent bonuses and maximum values

### Phase 3: Rules Engine (Weeks 5-6)
- Grant handler (proficiencies, features, ability scores)
- Calculate handler (HP max, AC, spell slots)
- Selection handler (ASI, feats, skills)
- Persistent value calculations only

### Phase 4: Character Creation Workflow (Weeks 7-8)
- Step-by-step creation process
- Ability scores → Race → Class → Background → Equipment → Spells
- All selections resolved before play
- Equipment management (what they own, not usage tracking)

### Phase 5: Character Maintenance (Weeks 9-10)
- Level up workflow with HP and feature advancement
- Spell preparation for next session
- Inventory management between sessions
- Character sheet generation (print-ready)

### Phase 6: Integration (Weeks 11-12)
- Campaign character management
- Party summary for DM
- Import/export for backup
- Final polish and testing

## Success Criteria

1. **Character Creation**: Complete PHB character in under 10 minutes, all choices resolved before play, professional sheet output
2. **Between-Session Management**: Level up in under 2 minutes, easy spell preparation, simple inventory management
3. **Data Integrity**: All persistent character data accurately stored and retrieved
4. **Sheet Quality**: Generated sheets are print-ready and match official format