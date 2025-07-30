# Mimir Import Bundle Format Specification

## Overview

Mimir starts with an empty database and allows users to import rulesets on demand. Each ruleset is distributed as a standardized bundle containing all necessary data in a clean, validated format. This document defines the structure and requirements for these import bundles.

## Bundle Structure

Each bundle is distributed as a `.tar.gz` archive containing:

```
<ruleset-id>.tar.gz             # e.g., "dnd5e-2014.tar.gz"
└── <ruleset-id>/               # Root directory in archive
    ├── manifest.json           # Bundle metadata and import orchestration
    ├── version.json            # Bundle version info
    ├── sources.json            # All books/supplements in this ruleset
    ├── races.json              # All races and subraces (denormalized)
    ├── classes.json            # All classes and subclasses (denormalized)
    ├── backgrounds.json        # Character backgrounds
    ├── feats.json              # Character feats
    ├── spells.json             # All spells with class lists
    ├── items.json              # All items including variants (denormalized)
    ├── creatures.json          # All creatures with embedded actions
    ├── books.json              # Book metadata index
    └── books/                  # Book content directory (FREE FORM - structure not prescribed)
        ├── phb/                # Example: Player's Handbook content
        │   ├── content.json    # Could contain full structured content
        │   └── assets/         # Could contain images, maps, etc.
        ├── mm/                 # Example: Monster Manual content
        ├── cos/                # Example: Curse of Strahd content
        └── .../                # Each book can have its own structure

```

## File Specifications

### manifest.json

The manifest contains meta data on the rule set. 

```json
{
  "format_version": "1.0.0",
  "bundle_id": "dnd5e-2014",
  "bundle_name": "D&D 5th Edition (2014)",
  "bundle_version": "2024.01.30",
  "rule_system": {
    "id": "dnd5e-2014",
    "name": "Dungeons & Dragons 5th Edition",
    "version": "2014",
    "publisher": "Wizards of the Coast"
  },
  "validation": {
    "total_entities": 15234,
    ...
    }
  },
  "metadata": { // this is meant to be free form information and not necessarily prescribed/required
    "generated_date": "2024-01-30T10:00:00Z",
    "generated_by": "mimir-bundler v1.0.0",
    "source_version": "5etools-2024.01.28"
  }
}
```

### sources.json

Complete catalog of all books and supplements:

```json
{
  "sources": [
    {
      "id": "PHB",
      "full_name": "Player's Handbook",
      "abbreviation": "PHB",
      "authors": ["Wizards RPG Team"],
      "published_date": "2014-08-19",
      "version": "1.0",
      "is_official": true,
      "is_srd": false,
      "book_type": "core",
      "page_count": 316,
      "isbn": "978-0-7869-6560-1",
      "url": "https://dnd.wizards.com/products/rpg_playershandbook",
      "contents": {
        "races": true,
        "classes": true,
        "spells": true,
        "items": true,
        "rules": true
      }
    },
    {
      "id": "MM",
      "full_name": "Monster Manual",
      "abbreviation": "MM",
      "authors": ["Wizards RPG Team"],
      "published_date": "2014-09-30",
      "version": "1.0",
      "is_official": true,
      "is_srd": false,
      "book_type": "core",
      "page_count": 350,
      "isbn": "978-0-7869-6561-8",
      "contents": {
        "creatures": true
      }
    }
  ]
}
```

### Entity Files

Each entity file contains an array of objects with consistent structure. All entities are fully denormalized - child entities contain all parent data.

#### races.json

Contains both base races and subraces in a single array. Subraces have all base race data populated:
```json
{
  "races": [
    {
      "id": "human",
      "name": "Human",
      "source": "PHB",
      "page": 29,
      "race_type": "race",
      "parent_race_id": null,
      "size": "M",
      "speed": {"walk": 30},
      "ability_scores": {"all": 1},
      "age": {"mature": 18, "max": 100},
      "alignment_tendency": "Humans tend toward no particular alignment.",
      "language_proficiencies": [
        {"any": 1, "from": ["common", "dwarvish", "elvish", "giant", "gnomish", "goblin", "halfling", "orc"]}
      ],
      "trait_tags": [],
      "entries": [
        {
          "type": "entries",
          "name": "Age",
          "entries": ["Humans reach adulthood in their late teens and live less than a century."]
        },
        {
          "type": "entries", 
          "name": "Size",
          "entries": ["Humans vary widely in height and build, from barely 5 feet to well over 6 feet tall."]
        }
      ]
    },
    {
      "id": "high-elf",
      "name": "High Elf",
      "source": "PHB",
      "page": 23,
      "race_type": "subrace",
      "parent_race_id": "elf",
      "size": "M",
      "speed": {"walk": 30},
      "ability_scores": {"dex": 2, "int": 1},  // Combined from base elf + subrace
      "age": {"mature": 100, "max": 750},      // Copied from base elf
      "alignment_tendency": "Elves love freedom, variety, and self-expression.",
      "language_proficiencies": [
        {"common": true, "elvish": true}        // Copied from base elf
      ],
      "trait_tags": ["Darkvision", "Keen Senses", "Fey Ancestry", "Trance"],
      "entries": [
        // All base elf entries included
        {
          "type": "entries",
          "name": "Darkvision", 
          "entries": ["Accustomed to twilit forests and the night sky, you have superior vision in dark and dim conditions."]
        },
        {
          "type": "entries",
          "name": "Keen Senses",
          "entries": ["You have proficiency in the {@skill Perception} skill."]
        },
        // Plus high elf specific entries
        {
          "type": "entries",
          "name": "Elf Weapon Training",
          "entries": ["You have proficiency with the longsword, shortsword, shortbow, and longbow."]
        },
        {
          "type": "entries",
          "name": "Cantrip",
          "entries": ["You know one cantrip of your choice from the wizard spell list."]
        }
      ]
    }
  ]
}
```

#### classes.json

Contains both base classes and subclasses in a single array. Subclasses have all base class data populated:

```json
{
  "classes": [
    {
      "id": "fighter",
      "name": "Fighter",
      "source": "PHB",
      "page": 70,
      "class_type": "class",
      "parent_class_id": null,
      "hit_die": 10,
      "primary_abilities": ["str", "dex"],
      "saving_throws": ["str", "con"],
      "skill_proficiency_count": 2,
      "skill_proficiency_choices": {
        "from": ["acrobatics", "animal handling", "athletics", "history", "insight", "intimidation", "perception", "survival"],
        "count": 2
      },
      "starting_proficiencies": {
        "armor": ["light", "medium", "heavy", "shields"],
        "weapons": ["simple", "martial"],
        "tools": []
      },
      "starting_equipment": {
        "default": [
          "(a) {@item chain mail|phb} or (b) {@item leather armor|phb}, {@item longbow|phb}, and 20 {@item arrows|phb}",
          "(a) a {@item martial weapon|phb} and a {@item shield|phb} or (b) two {@item martial weapons|phb}"
        ],
        "goldAlternative": "5d4 × 10 gp"
      },
      "spell_ability": null,
      "caster_progression": null,
      "subclass_title": "Martial Archetype",
      "subclass_level": 3,
      "features": [
        {
          "level": 1,
          "name": "Fighting Style",
          "entries": ["You adopt a particular style of fighting as your specialty."]
        },
        {
          "level": 1, 
          "name": "Second Wind",
          "entries": ["You have a limited well of stamina that you can draw on to protect yourself from harm."]
        }
      ],
      "entries": [
        "As a fighter, you gain the following class features."
      ]
    },
    {
      "id": "fighter-champion",
      "name": "Champion",
      "source": "PHB", 
      "page": 72,
      "class_type": "subclass",
      "parent_class_id": "fighter",
      "hit_die": 10,                              // Copied from Fighter
      "primary_abilities": ["str", "dex"],        // Copied from Fighter
      "saving_throws": ["str", "con"],            // Copied from Fighter
      "skill_proficiency_count": 2,               // Copied from Fighter
      "skill_proficiency_choices": {              // Copied from Fighter
        "from": ["acrobatics", "animal handling", "athletics", "history", "insight", "intimidation", "perception", "survival"],
        "count": 2
      },
      "starting_proficiencies": {                 // Copied from Fighter
        "armor": ["light", "medium", "heavy", "shields"],
        "weapons": ["simple", "martial"],
        "tools": []
      },
      "starting_equipment": {                     // Copied from Fighter
        "default": [
          "(a) {@item chain mail|phb} or (b) {@item leather armor|phb}, {@item longbow|phb}, and 20 {@item arrows|phb}",
          "(a) a {@item martial weapon|phb} and a {@item shield|phb} or (b) two {@item martial weapons|phb}"
        ],
        "goldAlternative": "5d4 × 10 gp"
      },
      "spell_ability": null,
      "caster_progression": null,
      "subclass_title": "Martial Archetype",      
      "subclass_level": 3,
      "features": [
        // All Fighter features included
        {
          "level": 1,
          "name": "Fighting Style",
          "entries": ["You adopt a particular style of fighting as your specialty."]
        },
        {
          "level": 1, 
          "name": "Second Wind",
          "entries": ["You have a limited well of stamina that you can draw on to protect yourself from harm."]
        },
        // Plus Champion-specific features
        {
          "level": 3,
          "name": "Improved Critical",
          "entries": ["Your weapon attacks score a critical hit on a roll of 19 or 20."]
        }
      ],
      "entries": [
        "The archetypal Champion focuses on the development of raw physical power honed to deadly perfection."
      ]
    }
  ]
}
```

#### items.json

Contains all items including base items and their variants. Variants have all base item data populated:

```json
{
  "items": [
    {
      "id": "longsword",
      "name": "Longsword",
      "source": "PHB",
      "page": 149,
      "base_item_id": null,
      "type": "M",              // Melee weapon
      "weight_lb": 3,
      "value_cp": 1500,         // 15 gp
      "armor_class": null,
      "damage": {"dice": "1d8", "type": "slashing"},
      "properties": ["versatile"],
      "versatile_damage": {"dice": "1d10", "type": "slashing"},
      "rarity": null,           // Mundane item
      "requires_attunement": false,
      "magic_bonus": null,
      "additional_properties": null,
      "entries": [
        "Proficiency with a longsword allows you to add your proficiency bonus to the attack roll for any attack you make with it."
      ]
    },
    {
      "id": "longsword-+1",
      "name": "Longsword, +1",
      "source": "DMG",
      "page": 213,
      "base_item_id": "longsword",
      "type": "M",              // Copied from base
      "weight_lb": 3,           // Copied from base
      "value_cp": null,         // Magic items often don't have fixed prices
      "armor_class": null,
      "damage": {"dice": "1d8", "type": "slashing"},  // Copied from base
      "properties": ["versatile"],                     // Copied from base
      "versatile_damage": {"dice": "1d10", "type": "slashing"}, // Copied from base
      "rarity": "uncommon",
      "requires_attunement": false,
      "magic_bonus": 1,
      "additional_properties": {
        "bonusWeapon": "+1"
      },
      "entries": [
        "You have a +1 bonus to attack and damage rolls made with this magic weapon."
      ]
    }
  ]
}
```

#### spells.json

Contains all spells with their class lists stored as JSON arrays:

```json
{
  "spells": [
    {
      "id": "fireball",
      "name": "Fireball",
      "source": "PHB",
      "page": 241,
      "level": 3,
      "school": "V",            // Evocation
      "casting_time": [{"number": 1, "unit": "action"}],
      "range": {"type": "point", "distance": {"type": "feet", "amount": 150}},
      "components": {"v": true, "s": true, "m": "a tiny ball of bat guano and sulfur"},
      "duration": [{"type": "instant"}],
      "is_ritual": false,
      "is_concentration": false,
      "saving_throw": ["dexterity"],
      "damage_type": ["fire"],
      "classes": ["sorcerer", "wizard", "light-cleric", "fiend-warlock"],  // Denormalized list
      "entries": [
        "A bright streak flashes from your pointing finger to a point you choose within range and then blossoms with a low roar into an explosion of flame.",
        "Each creature in a 20-foot-radius sphere centered on that point must make a Dexterity saving throw. A target takes 8d6 fire damage on a failed save, or half as much damage on a successful one."
      ],
      "upcast_info": [
        {
          "type": "entries",
          "name": "At Higher Levels",
          "entries": ["When you cast this spell using a spell slot of 4th level or higher, the damage increases by 1d6 for each slot level above 3rd."]
        }
      ]
    }
  ]
}
```

#### creatures.json

Contains all creatures with actions, traits, and reactions embedded as JSON arrays:

```json
{
  "creatures": [
    {
      "id": "goblin",
      "name": "Goblin",
      "source": "MM",
      "page": 166,
      "size": "S",
      "type": "humanoid",
      "type_tags": ["goblinoid"],
      "alignment": ["N", "E"],
      "armor_class": [{"ac": 15, "from": ["leather armor", "shield"]}],
      "hit_points": {"average": 7, "formula": "2d6"},
      "speed": {"walk": 30},
      "ability_scores": {"str": 8, "dex": 14, "con": 10, "int": 10, "wis": 8, "cha": 8},
      "saving_throws": null,
      "skills": {"stealth": "+6"},
      "damage_resistances": null,
      "damage_immunities": null,
      "condition_immunities": null,
      "senses": ["darkvision 60 ft.", "passive Perception 9"],
      "languages": ["Common", "Goblin"],
      "challenge_rating": "1/4",
      "proficiency_bonus": 2,
      "traits": [
        {
          "name": "Nimble Escape",
          "entries": [
            "The goblin can take the Disengage or Hide action as a bonus action on each of its turns."
          ]
        }
      ],
      "actions": [
        {
          "name": "Scimitar",
          "entries": [
            "{@atk mw} {@hit 4} to hit, reach 5 ft., one target. {@h}5 ({@damage 1d6 + 2}) slashing damage."
          ]
        },
        {
          "name": "Shortbow",
          "entries": [
            "{@atk rw} {@hit 4} to hit, range 80/320 ft., one target. {@h}5 ({@damage 1d6 + 2}) piercing damage."
          ]
        }
      ],
      "reactions": null,
      "legendary_actions": null,
      "lair_actions": null,
      "regional_effects": null,
      "entries": null,
      "environment": ["forest", "hill", "underdark"],
      "is_npc": false
    }
  ]
}
```

#### backgrounds.json

Contains all character backgrounds:

```json
{
  "backgrounds": [
    {
      "id": "acolyte",
      "name": "Acolyte",
      "source": "PHB",
      "page": 127,
      "skill_proficiencies": ["insight", "religion"],
      "language_proficiencies": [
        {"any": 2}
      ],
      "tool_proficiencies": null,
      "starting_equipment": {
        "default": [
          "{@item holy symbol|phb}",
          "{@item prayer book|phb}",
          "5 sticks of {@item incense}",
          "{@item vestments|phb}",
          "{@item common clothes|phb}",
          "{@item pouch|phb} containing 15 gp"
        ]
      },
      "feature_name": "Shelter of the Faithful",
      "feature_text": "As an acolyte, you command the respect of those who share your faith...",
      "entries": [
        {
          "type": "entries",
          "name": "Feature: Shelter of the Faithful",
          "entries": [
            "As an acolyte, you command the respect of those who share your faith, and you can perform the religious ceremonies of your deity. You and your adventuring companions can expect to receive free healing and care at a temple, shrine, or other established presence of your faith..."
          ]
        }
      ]
    }
  ]
}
```

#### feats.json

Contains all character feats:

```json
{
  "feats": [
    {
      "id": "alert",
      "name": "Alert",
      "source": "PHB",
      "page": 165,
      "prerequisites": null,
      "ability_increases": null,
      "feat_type": null,  // For 2024 edition: "general", "origin", etc.
      "entries": [
        "Always on the lookout for danger, you gain the following benefits:",
        {
          "type": "list",
          "items": [
            "You gain a +5 bonus to initiative.",
            "You can't be surprised while you are conscious.",
            "Other creatures don't gain advantage on attack rolls against you as a result of being unseen by you."
          ]
        }
      ]
    },
    {
      "id": "great-weapon-master",
      "name": "Great Weapon Master",
      "source": "PHB",
      "page": 167,
      "prerequisites": null,
      "ability_increases": null,
      "feat_type": null,
      "entries": [
        "You've learned to put the weight of a weapon to your advantage, letting its momentum empower your strikes. You gain the following benefits:",
        {
          "type": "list",
          "items": [
            "On your turn, when you score a critical hit with a melee weapon or reduce a creature to 0 hit points with one, you can make one melee weapon attack as a bonus action.",
            "Before you make a melee attack with a heavy weapon that you are proficient with, you can choose to take a -5 penalty to the attack roll. If the attack hits, you add +10 to the attack's damage."
          ]
        }
      ]
    }
  ]
}
```

#### books.json

Index of all books with metadata and references to full content in books/ directory.

**Note**: The `books/` directory structure is intentionally FREE FORM and not prescribed by this specification. Each book can organize its content as appropriate - whether that's structured JSON, markdown files, assets, or any other format. The only requirement is that books.json provides the metadata index.

```json
{
  "books": [
    {
      "id": "cos",
      "name": "Curse of Strahd",
      "source": "CoS",
      "group": "adventure",
      "published": "2016-03-15",
      "level": {
        "start": 1,
        "end": 10
      },
      "storyline": "Ravenloft",
      "contents": [
        {
          "name": "Introduction",
          "headers": ["Running the Adventure", "Marks of Horror", "A Classic Retold"]
        },
        {
          "name": "Chapter 1: Into the Mists",
          "headers": ["Strahd Von Zarovich", "Fortunes of Ravenloft"]
        }
      ],
      "cover": {
        "type": "internal",
        "path": "covers/cos.webp"
      },
      "content_path": "books/cos/content.json",
      "has_assets": true
    },
    {
      "id": "xge",
      "name": "Xanathar's Guide to Everything",
      "source": "XGE",
      "group": "supplement",
      "published": "2017-11-21",
      "contents": [
        {
          "name": "Introduction",
          "headers": ["Using This Book", "The Core Rules"]
        },
        {
          "name": "Chapter 1: Character Options",
          "headers": ["Subclasses", "This Is Your Life"]
        }
      ],
      "cover": {
        "type": "internal",
        "path": "covers/xge.webp"
      }
    }
  ]
}
```

### Version Control

#### version.json
```json
{
  "bundle_version": "2024.01.30",
  "compatible_with": {
    "mimir_min": "1.0.0",
    "mimir_max": "2.0.0"
  },
  "previous_versions": [
    {
      "version": "2024.01.15",
      "changes": ["Added Tasha's Cauldron content", "Fixed spell component errors"]
    }
  ]
}
```

## Import Process

1. **Validation Phase**
   - Verify manifest.json exists and is valid
   - Check all expected entity files exist
   - Validate checksums if provided
   - Ensure rule_system doesn't already exist in database

2. **Loading Phase**
   - Load entire bundle into memory
   - Parse sources.json first (required for all entities)
   - Parse all entity files (can be done in parallel)
   - Build in-memory maps for self-referential relationships

3. **Transformation Phase**
   - Apply denormalization rules (copy parent data to children)
   - Generate compound IDs with rule_system suffix
   - Convert 5etools format to our schema format
   - Validate all foreign key references

4. **Import Phase**
   - Insert rule_system record
   - Insert all sources
   - Import entities in dependency order
   - Track progress for UI feedback

5. **Verification Phase**
   - Verify entity counts match manifest
   - Run integrity checks
   - Update import_log table

## Distribution Format

### Archive Format
- **Format**: `.tar.gz` (gzip-compressed tar archive)
- **Compression Level**: 9 (maximum compression)
- **Archive Name**: `<ruleset-id>-v<version>.tar.gz`
- **Internal Structure**: Single root directory matching ruleset-id

### Example Archive Creation
```bash
# Create bundle archive
tar -czf dnd5e-2014-v2024.01.30.tar.gz dnd5e-2014/

# Verify archive contents
tar -tzf dnd5e-2014-v2024.01.30.tar.gz | head

# Expected output:
# dnd5e-2014/
# dnd5e-2014/manifest.json
# dnd5e-2014/sources.json
# dnd5e-2014/version.json
# dnd5e-2014/core/
# dnd5e-2014/core/races.json
# ...
```

### Distribution
> To be determined, assumption is anywhere it can be downloaded from.

### Bundle Catalog
```json
{
  "bundles": [
    {
      "id": "dnd5e-2014",
      "name": "D&D 5e (2014)",
      "size_mb": 45,
      "size_compressed_mb": 12,
      "version": "2024.01.30",
      "url": "https://cdn.mimir.app/bundles/dnd5e-2014-v2024.01.30.tar.gz",
      "checksum": "sha256:...",
      "changelog_url": "https://cdn.mimir.app/bundles/dnd5e-2014-changelog.html",
      "min_app_version": "1.0.0"
    }
  ]
}
```

## Design Rationale

1. **Memory Loading**: Entire bundle loads into memory because:
   - Simplifies dependency resolution
   - Enables transaction-based imports
   - Bundle sizes are reasonable (<100MB compressed)

2. **File Splitting**: Creatures split by CR because:
   - Largest dataset (thousands of entries)
   - Natural division for partial imports
   - Maintains reasonable file sizes

3. **Parent-Child Separation**: Races/subraces in separate files because:
   - Clear dependency order
   - Easier validation
   - Supports future partial imports

4. **Book Metadata Inclusion**: Complete book data included for:
   - Source attribution
   - Future content filtering
   - Legal compliance display

5. **JSON Format**: Chosen over alternatives because:
   - Human readable for debugging
   - Direct compatibility with 5etools
   - Built-in validation support
   - Compresses well

6. **tar.gz Archive Format**: Benefits include:
   - 70-80% size reduction (45MB → 12MB)
   - Single file download
   - Preserves directory structure
   - Universal support across platforms
   - Streaming extraction possible
   - Integrity via single checksum

## Parsing Challenges

### Key Data Transformations

1. **Parent-Child Denormalization**
   - Identify subraces by presence of `raceName` or `raceSource` fields
   - Identify subclasses within class files by `subclassShortName`
   - Identify item variants by `baseItem` field
   - Copy all parent data to child records

2. **ID Generation Rules**
   - Slugify names to create IDs
   - Handle special characters in magic items (+1, +2, etc.)
   - Ensure uniqueness within entity type
   - Handle name collisions with source suffix if needed

3. **Data Merging**
   - Combine mechanical data with fluff entries
   - Merge ability score improvements (base + subrace)
   - Combine class features across levels
   - Preserve 5etools formatting tags

4. **Array Flattening**
   - Extract spell classes from complex class/subclass structure
   - Flatten creature action arrays
   - Simplify proficiency choices

5. **Source Mapping**
   - Map 5etools source abbreviations to full source records
   - Track page numbers for all content
   - Maintain otherSources references

## Bundle Validation

### Required Files
- `manifest.json` - Bundle metadata
- `version.json` - Version information
- `sources.json` - All source books
- At least one entity file

### Validation Rules
1. All IDs must be unique within each entity type
2. All source references must exist in sources.json
3. Parent references must resolve to existing entities
4. Required fields must be present for each entity type
5. JSON must be valid and match expected structure

### Entity Counts
Track counts in manifest for validation:
```json
{
  "entity_counts": {
    "sources": 50,
    "races": 125,      // Includes subraces
    "classes": 145,    // Includes subclasses
    "backgrounds": 13,
    "feats": 85,
    "spells": 514,
    "items": 1455,     // Includes variants
    "creatures": 2031
  }
}