# 5etools Data Inventory and Structure Analysis

## Overview
The 5etools data consists of JSON files organized into several categories. The data represents D&D 5th Edition game content including core rules, adventures, monsters, spells, and more.

## File Organization

### 1. Core Game Mechanics (`/data/`)
These files contain the fundamental game content:

#### Character Creation
- `races.json` - Player character races
- `backgrounds.json` - Character backgrounds  
- `feats.json` - Character feats
- `optionalfeatures.json` - Optional class features
- `charcreationoptions.json` - Other character options

#### Items & Equipment
- `items.json` - All items (weapons, armor, magic items, etc.)
- `items-base.json` - Base item definitions
- `magicvariants.json` - Magic item variants
- `vehicles.json` - Vehicles and mounts

#### Game Rules
- `variantrules.json` - Optional/variant rules
- `conditionsdiseases.json` - Conditions and diseases
- `trapshazards.json` - Traps and environmental hazards
- `actions.json` - Combat actions
- `skills.json` - Skill definitions
- `senses.json` - Creature senses
- `languages.json` - Language definitions

#### Rewards & Treasures
- `rewards.json` - Non-item rewards
- `cultsboons.json` - Supernatural gifts/boons
- `loot.json` - Random loot tables
- `decks.json` - Card decks (e.g., Deck of Many Things)

#### Other Core Files
- `tables.json` - Various game tables
- `objects.json` - Environmental objects
- `deities.json` - Deity information
- `psionics.json` - Psionic powers
- `recipes.json` - Crafting recipes
- `monsterfeatures.json` - Reusable monster features
- `life.json` - Creature types and forms
- `names.json` - Name generation tables

### 2. Class Data (`/data/class/`)
Each class has two files:
- `class-[name].json` - Class mechanics and features
- `fluff-class-[name].json` - Descriptive text and lore

Classes include: artificer, barbarian, bard, cleric, druid, fighter, monk, mystic, paladin, ranger, rogue, sidekick, sorcerer, warlock, wizard

### 3. Spell Data (`/data/spells/`)
- `spells-[source].json` - Spells organized by sourcebook
- `fluff-spells-[source].json` - Spell descriptions
- `sources.json` - Source mapping
- `index.json` - Spell index

### 4. Creature/Monster Data (`/data/bestiary/`)
- `bestiary-[source].json` - Monsters by sourcebook
- `fluff-bestiary-[source].json` - Monster lore
- `legendarygroups.json` - Legendary creature groups
- `template.json` - Creature templates

### 5. Adventures (`/data/adventure/`)
- `adventure-[code].json` - Adventure content by book code

### 6. Books/Sources (`/data/book/`)
- `book-[code].json` - Book metadata and content

### 7. Generated Data (`/data/generated/`)
Pre-processed data for performance:
- `gendata-tables.json` - Compiled tables
- `gendata-spell-source-lookup.json` - Spell source mapping
- `gendata-subclass-lookup.json` - Subclass mapping
- `bookref-*.json` - Quick reference data

### 8. Fluff Files
Files prefixed with `fluff-` contain descriptive text, lore, and images separated from mechanical data for:
- Items, races, backgrounds, feats, creatures, spells, classes, etc.

### 9. Foundry Integration Files
Files prefixed with `foundry-` contain data formatted for Foundry VTT integration.

### 10. Index Files
- Main content indexes in each directory
- Homebrew index at `/homebrew/index.json`
- Search indexes at `/search/`

## Common Data Patterns

### 1. Source References
Every entry includes source information:
```json
{
  "source": "PHB",      // Primary source book code
  "page": 123,          // Page number
  "otherSources": [...] // Additional sources
}
```

### 2. Cross-References
5etools uses special tags for cross-references:
- `{@creature NAME|SOURCE}` - Link to creature
- `{@spell NAME|SOURCE}` - Link to spell
- `{@item NAME|SOURCE}` - Link to item
- `{@skill NAME}` - Link to skill
- `{@condition NAME}` - Link to condition
- `{@damage DICE}` - Damage roll
- `{@dice DICE}` - Generic dice roll
- `{@dc NUMBER}` - Difficulty class
- `{@hit NUMBER}` - Attack bonus
- `{@atk TYPE}` - Attack type (mw=melee weapon, etc.)

### 3. Entry Structure
Content uses nested entry objects:
```json
"entries": [
  {
    "type": "entries",
    "name": "Feature Name",
    "entries": ["Description text..."]
  }
]
```

### 4. Ability Scores
Standardized ability score format:
```json
"ability": [
  {
    "dex": 2,
    "wis": 1
  }
]
```

### 5. Prerequisites
Prerequisites for feats, features, etc.:
```json
"prerequisite": [
  {
    "level": 4,
    "race": ["elf", "half-elf"],
    "ability": [{"str": 13}]
  }
]
```

### 6. Size, Type, Alignment
Creatures use arrays for flexible values:
```json
"size": ["M"],
"type": {
  "type": "humanoid",
  "tags": ["elf"]
},
"alignment": ["C", "G"]  // Chaotic Good
```

### 7. Speed
Movement speeds in structured format:
```json
"speed": {
  "walk": 30,
  "fly": 50,
  "swim": 30
}
```

### 8. Tags
Various tag types for categorization:
- `traitTags` - Common traits
- `languageTags` - Language codes  
- `damageTags` - Damage types
- `miscTags` - Miscellaneous tags
- `conditionInflict` - Conditions inflicted
- `savingThrow` - Required saves

## Data Volume Estimates

Based on file listings:
- **Core mechanics**: ~50 files
- **Classes**: 15 classes × 2 files = 30 files
- **Spells**: ~20 source files
- **Creatures**: ~100 source files (50 mechanics + 50 fluff)
- **Adventures**: ~80 files
- **Books**: ~40 files

Total: Approximately 300-400 JSON files containing tens of thousands of game entities.