# Book Archive Structure and File Formats

This document describes the structure and format of D&D 5etools book archives as used by Mimir.

## Overview

Mimir imports D&D content from 5etools JSON data organized into book archives. Each book represents a source publication (PHB, DMG, MM, etc.) and contains various content types like spells, items, monsters, classes, and more.

## Archive Directory Structure

Book archives are stored in the application data directory:
```
~/Library/Application Support/com.mimir.mimir-test/data/books/
├── PHB/                    # Player's Handbook
├── DMG/                    # Dungeon Master's Guide  
├── MM/                     # Monster Manual
├── test-book/              # Custom test book
└── test-book-two/          # Another test book
```

## Individual Book Structure

Each book directory follows this standard structure:

```
BookName/
├── metadata.json           # Book metadata and configuration
├── book/
│   └── book-{source}.json  # Book content and structure
├── spells/
│   ├── spells-{source}.json        # Spell definitions
│   └── fluff-spells-{source}.json  # Spell flavor text
├── items/
│   └── {source}.json       # Magic items and equipment
├── bestiary/
│   ├── bestiary-{source}.json      # Monster stat blocks
│   └── fluff-bestiary-{source}.json # Monster flavor text
├── class/
│   └── {source}.json       # Class and subclass definitions
├── races/
│   └── {source}.json       # Race and subrace definitions
├── backgrounds/
│   └── {source}.json       # Background definitions
├── feats/
│   └── {source}.json       # Feat definitions
├── conditions/
│   ├── {source}.json       # Condition definitions
│   └── fluff-{source}.json # Condition descriptions
├── actions/
│   └── {source}.json       # Action definitions
├── deities/
│   └── {source}.json       # Deity information
├── languages/
│   ├── {source}.json       # Language definitions
│   └── fluff-{source}.json # Language descriptions
├── objects/
│   └── {source}.json       # Object definitions
├── vehicles/
│   ├── {source}.json       # Vehicle stat blocks
│   └── fluff-{source}.json # Vehicle descriptions
├── rewards/
│   └── {source}.json       # Supernatural gifts/rewards
├── variantrules/
│   └── {source}.json       # Optional and variant rules
├── tables/
│   └── {source}.json       # Random tables
├── traps/
│   └── {source}.json       # Trap and hazard definitions
├── diseases/
│   └── {source}.json       # Disease definitions
└── img/
    ├── covers/
    │   └── {SOURCE}.webp    # Book cover image
    └── book/
        └── {SOURCE}/        # Internal book images
```

## File Format Specifications

### metadata.json

Contains book-level information:

```json
{
  "name": "Player's Handbook",
  "abbreviation": "PHB", 
  "source": "PHB",
  "version": "1.0",
  "url": "https://example.com",
  "published": "2014-08-19",
  "group": "core",
  "cover": {
    "file": "PHB.webp",
    "width": 400,
    "height": 600
  }
}
```

### Content Files Structure

All content files follow the same basic JSON structure:

```json
{
  "contentType": [
    {
      "name": "Content Name",
      "source": "PHB",
      "page": 123,
      // ... content-specific fields
    }
  ]
}
```

Where `contentType` is one of:
- `spell` - Spell definitions
- `item` - Magic items and equipment  
- `monster` - Creature stat blocks
- `class` - Class definitions
- `subclass` - Subclass definitions
- `race` - Race and subrace definitions
- `background` - Character backgrounds
- `feat` - Feat definitions
- `condition` - Condition definitions
- `action` - Action definitions
- `deity` - Deity information
- `language` - Language definitions
- `object` - Object definitions
- `vehicle` - Vehicle definitions
- `reward` - Supernatural gifts
- `variantrule` - Optional rules
- `table` - Random tables
- `trap` - Trap definitions
- `disease` - Disease definitions

## Common Field Patterns

### Core Fields
Most content entries include these standard fields:

- `name` (string) - Display name
- `source` (string) - Source book abbreviation
- `page` (number) - Page number in source book
- `entries` (array) - Main descriptive text
- `srd` (boolean) - Available in System Reference Document
- `basicRules` (boolean) - Available in basic rules

### Reference Fields
- `otherSources` (array) - Additional source references
- `reprintedAs` (array) - Where this content was reprinted

### Fluff Files
Some content types have separate "fluff" files containing:
- `images` - Associated artwork
- `entries` - Flavor text and lore
- `entriesTemplate` - Template for generating descriptions

## Content-Specific Formats

### Spells (spells/*.json)

Key fields beyond common ones:
- `level` (number) - Spell level (0-9)
- `school` (string) - School of magic (single letter code)
- `time` (array) - Casting time objects
- `range` (object) - Range specification
- `components` (object) - V/S/M components
- `duration` (array) - Duration objects
- `meta` (object) - Ritual/concentration flags
- `classes` (object) - Available classes
- `entriesHigherLevel` (array) - At higher levels text
- `damageInflict` (array) - Damage types dealt
- `conditionInflict` (array) - Conditions applied
- `savingThrow` (array) - Required saving throws
- `spellAttack` (array) - Attack types
- `areaTags` (array) - Area effect types
- `miscTags` (array) - Miscellaneous tags
- `scalingLevelDice` (object) - Cantrip scaling

Example spell structure:
```json
{
  "spell": [
    {
      "name": "Fireball",
      "source": "PHB",
      "page": 241,
      "level": 3,
      "school": "V",
      "time": [{"number": 1, "unit": "action"}],
      "range": {
        "type": "point",
        "distance": {"type": "feet", "amount": 150}
      },
      "components": {
        "v": true,
        "s": true, 
        "m": "a tiny ball of bat guano and sulfur"
      },
      "duration": [{"type": "instant"}],
      "entries": [
        "A bright streak flashes from your pointing finger..."
      ],
      "entriesHigherLevel": [
        {
          "type": "entries",
          "name": "At Higher Levels", 
          "entries": ["When you cast this spell using..."]
        }
      ],
      "damageInflict": ["fire"],
      "savingThrow": ["dexterity"],
      "areaTags": ["S"],
      "classes": {
        "fromClassList": [
          {"name": "Sorcerer", "source": "PHB"},
          {"name": "Wizard", "source": "PHB"}
        ]
      }
    }
  ]
}
```

### Items (items/*.json)

Key fields for equipment and magic items:
- `type` (string) - Item type code
- `typeName` (string) - Human-readable type
- `rarity` (string) - Magic item rarity
- `reqAttune` (string/boolean) - Attunement requirements
- `value` (number) - Cost in copper pieces
- `weight` (number) - Weight in pounds
- `ac` (number) - Armor class for armor
- `strength` (number) - Strength requirement
- `stealth` (boolean) - Stealth disadvantage
- `dmg1`/`dmg2` (string) - Damage dice
- `dmgType` (string) - Damage type code
- `property` (array) - Weapon properties
- `range` (string) - Weapon range
- `bonusWeapon` (string) - Attack/damage bonus
- `bonusAc` (string) - AC bonus
- `charges` (number) - Item charges
- `curse` (boolean) - Cursed item flag

### Monsters (bestiary/*.json)

Key fields for creature stat blocks:
- `size` (array) - Creature size
- `type` (object) - Creature type and tags
- `alignment` (array) - Alignment
- `ac` (array) - Armor class
- `hp` (object) - Hit points
- `speed` (object) - Movement speeds
- `str`, `dex`, `con`, `int`, `wis`, `cha` (number) - Ability scores
- `save` (object) - Saving throw bonuses
- `skill` (object) - Skill bonuses
- `vulnerable`, `resist`, `immune` (array) - Damage modifiers
- `conditionImmune` (array) - Condition immunities
- `senses` (array) - Special senses
- `passive` (number) - Passive Perception
- `languages` (array) - Known languages
- `cr` (string/object) - Challenge Rating
- `trait` (array) - Special abilities
- `action` (array) - Actions
- `bonus` (array) - Bonus actions
- `reaction` (array) - Reactions
- `legendary` (array) - Legendary actions
- `legendaryGroup` (object) - Legendary action group
- `environment` (array) - Natural environments

### Classes (class/*.json)

Key fields for character classes:
- `hd` (object) - Hit die information
- `proficiency` (array) - Starting proficiencies
- `startingProficiencies` (object) - Detailed proficiency breakdown
- `startingEquipment` (object) - Starting equipment options
- `multiclassing` (object) - Multiclassing requirements
- `classFeatures` (array) - Level progression features
- `subclassTitle` (string) - Subclass category name
- `casterProgression` (string) - Spellcasting progression type
- `spellcastingAbility` (string) - Spellcasting ability score
- `cantripProgression` (array) - Cantrips known by level
- `spellsKnownProgression` (array) - Spells known progression
- `additionalSpells` (array) - Bonus spells by subclass
- `preparedSpells` (string) - Prepared spells formula

## Data Validation and Integrity

### Required Fields
All content entries must have:
- `name` - Unique within content type and source
- `source` - Must match book's source abbreviation

### Referential Integrity
- Cross-references use `{source}|{name}` format
- Image paths relative to book's img/ directory
- Spell lists reference by name and source

### Validation Rules
- Source codes must be valid book abbreviations
- Page numbers must be positive integers
- Dice expressions follow `XdY+Z` pattern
- Ability scores are integers 1-30
- Challenge ratings follow specific format

## Import Process

1. **Directory Scan** - Mimir scans the books directory for valid book folders
2. **Metadata Loading** - Reads metadata.json for book information  
3. **Content Discovery** - Identifies available content files
4. **JSON Parsing** - Parses and validates JSON structure
5. **Data Transformation** - Converts to internal format
6. **Index Building** - Creates search indices
7. **Cross-Reference Resolution** - Links related content

## File Naming Conventions

- Source codes are uppercase (PHB, DMG, MM, etc.)
- File names use lowercase source codes
- Fluff files prefixed with `fluff-`
- Image files use book source code for directory names
- Cover images named exactly as source code + `.webp`

## Extending the Format

To add new content types:
1. Create new subdirectory in book structure
2. Follow standard JSON array format
3. Include required core fields
4. Document type-specific fields
5. Update import process to handle new type
6. Add UI components for display

This structure allows for flexible content organization while maintaining consistency across different D&D source materials.