---
id: create-5etools-to-bundle-parser
level: task
title: "Create 5etools to Bundle Parser for D&D 5e 2014"
created_at: 2025-07-30T16:30:00+00:00
updated_at: 2025-07-30T21:30:00+00:00
completed_at: 2025-07-30T21:30:00+00:00
parent: phase-0-data-ingestion-and-storage
blocked_by: ["design-standard-import-bundle-format"]
archived: false

tags:
  - "#task"
  - "#phase/completed"
  - "#parser"
  - "#data-transformation"

exit_criteria_met: true
---

# Create 5etools to Bundle Parser for D&D 5e 2014

## Description

Build a Python parser that transforms 5etools JSON data into our standardized import bundle format. This parser will handle the denormalization, ID generation, and data merging required to create clean import bundles for the D&D 5e 2014 ruleset.

## Acceptance Criteria

- [x] Create Python script at `/data/scripts/parse_5etools_2014.py`
- [x] Parse and transform races (including subraces with parent data)
- [x] Parse and transform classes (including subclasses with parent data)
- [x] Parse and transform items (including variants with base item data)
- [x] Parse and transform spells with flattened class lists
- [x] Parse and transform creatures with embedded actions/traits
- [x] Parse and transform backgrounds, feats, and conditions
- [x] Generate sources.json from all book references
- [x] Create manifest.json with entity counts and metadata
- [x] Handle 5etools formatting tags preservation
- [x] Implement ID generation rules (slugified names)
- [x] Merge mechanical data with fluff entries
- [x] Output tar.gz bundle in correct structure
- [x] Add validation to ensure all parent references resolve
- [x] Include comprehensive error handling and logging
- [x] **BONUS**: Create core-only parser (`parse_5etools_2014_core.py`)
- [x] **BONUS**: Implement absolute deduplication (0 duplicates)
- [x] **BONUS**: Add source priority system for canonical versions

## Technical Notes

Implementation approach:
- Use Python for ease of updates and JSON manipulation
- Location: `/data/scripts/parse_5etools_2014.py`
- Input: 5etools data directory
- Output: `dnd5e-2014-v{date}.tar.gz`

Key transformations from our data design:
1. **Denormalization Strategy**:
   - Races/Subraces: Copy all base race fields to subrace records
   - Classes/Subclasses: Copy all base class fields to subclass records  
   - Items/Variants: Copy all base item fields to variant records
   - No separate junction tables - embed arrays (spell classes, creature actions)

2. **ID Generation Rules**:
   - Simple slugified names: "Bag of Holding" → `bag-of-holding`
   - NO source/ruleset in ID (we have separate columns for those)
   - Subclasses: prefix with parent (e.g., `fighter-champion`)
   - Handle special chars in magic items: "+1 Longsword" → `longsword-+1`

3. **5etools Specific Parsing**:
   - Identify subraces by `raceName` or `raceSource` fields
   - Identify subclasses by `subclassShortName` within class files
   - Identify item variants by `baseItem` field
   - Merge ability scores (base race + subrace bonuses)

4. **Source File Locations** (from our inventory):
   - Core mechanics: `/data/*.json`
   - Classes: `/data/class/class-*.json` and `fluff-class-*.json`
   - Spells: `/data/spells/spells-*.json`
   - Creatures: `/data/bestiary/bestiary-*.json`
   - ~300-400 total JSON files to process

5. **Output Structure** (from import-bundle-format.md):
   ```
   dnd5e-2014/
   ├── manifest.json
   ├── version.json
   ├── sources.json
   ├── races.json        # Both races and subraces
   ├── classes.json      # Both classes and subclasses
   ├── items.json        # All items including variants
   ├── spells.json       # With classes as JSON array
   ├── creatures.json    # With embedded actions/traits
   ├── backgrounds.json
   ├── feats.json
   └── books/           # Free-form directory
   ```

Dependencies:
- Python 3.8+
- Standard library only (json, tarfile, gzip, pathlib)
- No external dependencies for portability

Example usage:
```bash
python data/scripts/parse_5etools_2014.py \
  --input /path/to/5etools-data \
  --output /path/to/output/dnd5e-2014-v2024.01.30.tar.gz
```

## Reference Documents

Key design documents to follow:
- `/docs/data-design/import-bundle-format.md` - Target output format
- `/docs/data-design/transformation-rules.md` - Denormalization logic
- `/docs/data-design/5etools-data-inventory.md` - Source file locations
- `/docs/data-design/sqlite-schema-core-entities.sql` - Final database schema

## Dependencies

- Depends on: design-standard-import-bundle-format
- Blocks: implement-unified-bundle-import (Rust importer)

## Completion Summary

**Completed**: 2025-07-30

### Delivered Artifacts

1. **Full Parser**: `/data/scripts/parse_5etools_2014.py`
   - Processes all 2014-era content (core + supplements)
   - Excludes 2022+ content (MPMM, etc.)
   - Output: `dnd5e-2014-v1.210.46.tar.gz` (1.6 MB)

2. **Core Parser**: `/data/scripts/parse_5etools_2014_core.py`
   - Processes only PHB, MM, DMG content
   - Strict source filtering
   - Output: `dnd5e-2014-core-v1.210.46.tar.gz` (354 KB)

3. **Validation Script**: `/data/scripts/validate_bundle.py`
   - Validates bundle integrity
   - Checks ID uniqueness, references, consistency

4. **Documentation**:
   - `/data/README.md` - High-level data architecture
   - `/data/scripts/README.md` - Detailed parser documentation
   - Comprehensive docstrings in parser files

### Key Achievements

- **Absolute Deduplication**: Reduced duplicates from 367 to 0
- **Source Priority System**: Core books take precedence
- **Base Item Fix**: Corrected "Item|SOURCE" parsing
- **Semantic Versioning**: Matches source data v1.210.46
- **Clean ID Generation**: Human-readable slugs

### Bundle Statistics

**Full Bundle (dnd5e-2014-v1.210.46)**:
- 164 races, 153 classes, 1,749 items
- 522 spells, 3,400 creatures
- 122 backgrounds, 114 feats

**Core Bundle (dnd5e-2014-core-v1.210.46)**:
- 38 races, 54 classes, 778 items
- 361 spells, 458 creatures
- 20 backgrounds, 42 feats

### Lessons Learned

1. 5etools data has many duplicates across sources
2. MPMM contains 2024-oriented rewrites that should be excluded
3. Base item references use "Name|SOURCE" format
4. Subraces don't always have explicit name fields
5. Absolute deduplication requires both proactive and reactive approaches
