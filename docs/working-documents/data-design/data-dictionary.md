# Mimir Data Dictionary

## Overview
This document defines the data elements, their types, constraints, and relationships for the Mimir D&D 5e database schema.

## Core Entities

### Sources
Represents sourcebooks and content origins.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| id | TEXT | PK, NOT NULL | Source code (e.g., 'PHB', 'MM') |
| full_name | TEXT | NOT NULL | Full source name |
| abbreviation | TEXT | | Short form abbreviation |
| published_date | DATE | | Publication date |
| version | TEXT | | Version identifier |
| is_official | BOOLEAN | DEFAULT TRUE | Official WotC content flag |
| metadata | JSONB | | Additional metadata |

### Races
Player character races and subraces.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| id | TEXT | PK | Generated from name+source |
| name | TEXT | NOT NULL | Race name |
| source_id | TEXT | FK(sources), NOT NULL | Source reference |
| page | INTEGER | | Page number in source |
| size | TEXT | | Size category (S/M/L) |
| speed | JSONB | | Movement speeds object |
| ability_scores | JSONB | | Ability score improvements |
| age | JSONB | | Age ranges |
| alignment_tendency | TEXT | | Typical alignment |
| language_proficiencies | JSONB | | Known/learnable languages |
| trait_tags | JSONB | | Searchable trait tags |
| entries | JSONB | NOT NULL | Full description/features |
| is_subrace | BOOLEAN | DEFAULT FALSE | Subrace indicator |
| parent_race_id | TEXT | FK(races) | Parent race reference |

### Items
All game items including equipment, magic items, etc.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| id | TEXT | PK | Generated identifier |
| name | TEXT | NOT NULL | Item name |
| source_id | TEXT | FK(sources), NOT NULL | Source reference |
| page | INTEGER | | Page number |
| type | TEXT | | Item type code |
| rarity | TEXT | | Rarity (common/uncommon/etc) |
| requires_attunement | BOOLEAN | DEFAULT FALSE | Attunement requirement |
| attunement_prereq | JSONB | | Attunement prerequisites |
| value_cp | INTEGER | | Value in copper pieces |
| weight_lb | REAL | | Weight in pounds |
| armor_class | INTEGER | | AC provided (armor/shields) |
| damage | JSONB | | Weapon damage info |
| properties | JSONB | | Item properties array |
| entries | JSONB | NOT NULL | Full description |
| base_item_id | TEXT | FK(items) | Base item reference |
| is_magic | BOOLEAN | DEFAULT FALSE | Magic item flag |

### Backgrounds
Character backgrounds.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| id | TEXT | PK | Generated identifier |
| name | TEXT | NOT NULL | Background name |
| source_id | TEXT | FK(sources), NOT NULL | Source reference |
| skill_proficiencies | JSONB | | Granted skills |
| language_proficiencies | JSONB | | Granted languages |
| tool_proficiencies | JSONB | | Granted tool proficiencies |
| starting_equipment | JSONB | | Starting gear |
| feature_name | TEXT | | Background feature name |
| feature_text | TEXT | | Feature description (extracted) |
| entries | JSONB | NOT NULL | Full content |

### Feats
Character feats and optional features.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| id | TEXT | PK | Generated identifier |
| name | TEXT | NOT NULL | Feat name |
| source_id | TEXT | FK(sources), NOT NULL | Source reference |
| prerequisites | JSONB | | Feat prerequisites |
| ability_increases | JSONB | | Ability score increases |
| entries | JSONB | NOT NULL | Full description |

### Classes
Character classes.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| id | TEXT | PK | Generated identifier |
| name | TEXT | NOT NULL | Class name |
| source_id | TEXT | FK(sources), NOT NULL | Source reference |
| hit_die | INTEGER | | Hit die size (d6/d8/d10/d12) |
| primary_abilities | JSONB | | Primary ability scores |
| saving_throws | JSONB | | Saving throw proficiencies |
| skill_proficiency_count | INTEGER | | Number of skills |
| skill_proficiency_choices | JSONB | | Available skill choices |
| starting_proficiencies | JSONB | | Starting proficiencies |
| starting_equipment | JSONB | | Starting equipment options |
| class_features | JSONB | | Feature progression |
| spell_ability | TEXT | | Spellcasting ability |
| caster_progression | TEXT | | Casting type |
| entries | JSONB | NOT NULL | Full description |

### Subclasses
Class archetypes/subclasses.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| id | TEXT | PK | Generated identifier |
| name | TEXT | NOT NULL | Subclass name |
| short_name | TEXT | | Abbreviated name |
| class_id | TEXT | FK(classes), NOT NULL | Parent class |
| source_id | TEXT | FK(sources), NOT NULL | Source reference |
| subclass_features | JSONB | | Feature progression |
| spell_list | JSONB | | Additional spells |
| entries | JSONB | NOT NULL | Full description |

### Spells
Magic spells.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| id | TEXT | PK | Generated identifier |
| name | TEXT | NOT NULL | Spell name |
| source_id | TEXT | FK(sources), NOT NULL | Source reference |
| level | INTEGER | | Spell level (0-9) |
| school | TEXT | | Magic school code |
| casting_time | JSONB | | Time to cast |
| range | JSONB | | Spell range |
| components | JSONB | | V/S/M components |
| duration | JSONB | | Spell duration |
| is_ritual | BOOLEAN | DEFAULT FALSE | Ritual casting flag |
| is_concentration | BOOLEAN | DEFAULT FALSE | Concentration flag |
| saving_throw | JSONB | | Required saves |
| damage_type | JSONB | | Damage types dealt |
| entries | JSONB | NOT NULL | Spell description |
| upcast_info | JSONB | | Higher level effects |
| classes | JSONB | | Classes with access |

### Creatures
Monsters, NPCs, and creatures.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| id | TEXT | PK | Generated identifier |
| name | TEXT | NOT NULL | Creature name |
| source_id | TEXT | FK(sources), NOT NULL | Source reference |
| size | TEXT | | Size category |
| type | TEXT | | Creature type |
| type_tags | JSONB | | Type subtags |
| alignment | JSONB | | Alignment array |
| armor_class | JSONB | | AC and source |
| hit_points | JSONB | | HP average and formula |
| speed | JSONB | | Movement speeds |
| ability_scores | JSONB | | STR/DEX/CON/INT/WIS/CHA |
| saving_throws | JSONB | | Save bonuses |
| skills | JSONB | | Skill bonuses |
| damage_resistances | JSONB | | Damage resistances |
| damage_immunities | JSONB | | Damage immunities |
| condition_immunities | JSONB | | Condition immunities |
| senses | JSONB | | Special senses |
| languages | JSONB | | Known languages |
| challenge_rating | TEXT | | CR value |
| proficiency_bonus | INTEGER | | Proficiency bonus |
| traits | JSONB | | Special traits |
| actions | JSONB | | Available actions |
| reactions | JSONB | | Available reactions |
| legendary_actions | JSONB | | Legendary actions |
| lair_actions | JSONB | | Lair actions |
| regional_effects | JSONB | | Regional effects |
| entries | JSONB | NOT NULL | Full description |
| environment | JSONB | | Natural environments |
| is_npc | BOOLEAN | DEFAULT FALSE | NPC flag |

## Search Tables

### content_search (FTS5)
General full-text search across all content.

| Field | Type | Description |
|-------|------|-------------|
| entity_id | TEXT | Reference to entity |
| entity_type | TEXT | Type of entity |
| name | TEXT | Entity name |
| content | TEXT | Searchable content |
| tags | TEXT | Space-separated tags |

### spell_search (FTS5)
Specialized spell search.

| Field | Type | Description |
|-------|------|-------------|
| spell_id | TEXT | Spell reference (unindexed) |
| name | TEXT | Spell name |
| school | TEXT | School of magic |
| level | TEXT | Spell level |
| description | TEXT | Spell description |
| components | TEXT | Component requirements |
| classes | TEXT | Available to classes |

### creature_search (FTS5)
Specialized creature search.

| Field | Type | Description |
|-------|------|-------------|
| creature_id | TEXT | Creature reference (unindexed) |
| name | TEXT | Creature name |
| type | TEXT | Creature type |
| size | TEXT | Size category |
| traits | TEXT | Trait descriptions |
| actions | TEXT | Action descriptions |
| environment | TEXT | Environments |

### item_search (FTS5)
Specialized item search.

| Field | Type | Description |
|-------|------|-------------|
| item_id | TEXT | Item reference (unindexed) |
| name | TEXT | Item name |
| type | TEXT | Item type |
| rarity | TEXT | Rarity |
| description | TEXT | Item description |
| properties | TEXT | Item properties |

### content_embeddings
Vector embeddings for semantic search.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| id | INTEGER | PK, AUTOINCREMENT | Embedding ID |
| entity_id | TEXT | NOT NULL | Entity reference |
| entity_type | TEXT | NOT NULL | Entity type |
| chunk_index | INTEGER | NOT NULL | Chunk number |
| chunk_text | TEXT | NOT NULL | Text that was embedded |
| embedding | BLOB | NOT NULL | Vector data |
| model_version | TEXT | NOT NULL | Model used |

## Relationship Tables

### spell_classes
Links spells to classes that can cast them.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| spell_id | TEXT | PK, FK(spells) | Spell reference |
| class_id | TEXT | PK, FK(classes) | Class reference |

### item_properties
Stores item properties as key-value pairs.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| item_id | TEXT | PK, FK(items) | Item reference |
| property | TEXT | PK | Property name |
| value | TEXT | | Property value |

### creature_actions
Detailed creature actions.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| id | INTEGER | PK, AUTOINCREMENT | Action ID |
| creature_id | TEXT | FK(creatures), NOT NULL | Creature reference |
| action_type | TEXT | NOT NULL | Type of action |
| name | TEXT | NOT NULL | Action name |
| description | TEXT | | Action description |
| recharge | TEXT | | Recharge notation |
| attack_bonus | INTEGER | | Attack modifier |
| damage | JSONB | | Damage information |
| dc | INTEGER | | Save DC |
| sort_order | INTEGER | | Display order |

### content_references
Tracks cross-references between entities.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| id | INTEGER | PK, AUTOINCREMENT | Reference ID |
| source_entity_id | TEXT | NOT NULL | Source entity |
| source_entity_type | TEXT | NOT NULL | Source type |
| target_entity_id | TEXT | NOT NULL | Target entity |
| target_entity_type | TEXT | NOT NULL | Target type |
| reference_type | TEXT | | Type of reference |
| context | TEXT | | Reference context |

## Data Types

### Size Categories
- `T` - Tiny
- `S` - Small  
- `M` - Medium
- `L` - Large
- `H` - Huge
- `G` - Gargantuan

### School Codes
- `A` - Abjuration
- `C` - Conjuration
- `D` - Divination
- `E` - Enchantment
- `V` - Evocation
- `I` - Illusion
- `N` - Necromancy
- `T` - Transmutation

### Alignment Codes
Arrays of:
- `L` - Lawful
- `N` - Neutral
- `C` - Chaotic
- `G` - Good
- `E` - Evil
- `U` - Unaligned
- `A` - Any

### Rarity Values
- `common`
- `uncommon`
- `rare`
- `very rare`
- `legendary`
- `artifact`

### Caster Progression
- `full` - Full caster (Wizard, Cleric, etc.)
- `half` - Half caster (Paladin, Ranger)
- `third` - Third caster (Eldritch Knight, Arcane Trickster)
- `pact` - Pact Magic (Warlock)

## JSONB Field Structures

### Speed Object
```json
{
  "walk": 30,
  "fly": 60,
  "swim": 30,
  "climb": 20,
  "burrow": 15
}
```

### Ability Scores
```json
[
  {"str": 2},
  {"dex": 1},
  {"choose": {
    "from": ["int", "wis", "cha"],
    "count": 1,
    "amount": 1
  }}
]
```

### Damage Object
```json
{
  "dice": "2d6",
  "type": "slashing",
  "bonus": 3
}
```

### Range Object
```json
{
  "type": "point",
  "distance": {
    "type": "feet",
    "amount": 120
  }
}
```

### Duration Object
```json
{
  "type": "timed",
  "duration": {
    "type": "minute",
    "amount": 10
  },
  "concentration": true
}
```