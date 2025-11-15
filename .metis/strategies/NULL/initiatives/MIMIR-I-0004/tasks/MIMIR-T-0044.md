---
id: create-characterdata-struct-and
level: task
title: "Create CharacterData struct and related types for YAML serialization"
short_code: "MIMIR-T-0044"
created_at: 2025-11-10T18:56:58.355732+00:00
updated_at: 2025-11-15T16:53:46.148108+00:00
parent: MIMIR-I-0004
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/completed"


exit_criteria_met: false
strategy_id: NULL
initiative_id: MIMIR-I-0004
---

# Create CharacterData struct and related types for YAML serialization

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0004]]

## Objective **[REQUIRED]**

Create comprehensive Rust data structures for D&D 5e character representation with full YAML serialization support, including abilities, skills, proficiencies, spells, and inventory.

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria **[REQUIRED]**

- [ ] CharacterData struct created in `crates/mimir-dm-core/src/models/character/data.rs` with Serialize, Deserialize, Clone, Debug
- [ ] AbilityScores struct with STR, DEX, CON, INT, WIS, CHA fields and modifier calculation methods
- [ ] Skills struct with all 18 D&D 5e skills, proficiency tracking, and bonus calculation
- [ ] Proficiencies struct including armor, weapons, tools, languages, saving throws
- [ ] SpellData struct with spell slots, prepared spells, known spells, and spell save DC calculation
- [ ] Inventory struct with items, currency, equipment slots, and encumbrance calculation
- [ ] All structs serialize/deserialize to/from YAML format correctly
- [ ] Unit tests for ability score modifiers, skill calculations, and YAML round-trip serialization

## Implementation Notes **[CONDITIONAL: Technical Task]**

### Technical Approach
- Create `crates/mimir-dm-core/src/models/character/data.rs` module
- Use serde_yaml for YAML serialization/deserialization
- Implement helper methods for D&D 5e calculations (modifiers, proficiency bonus, etc.)
- Follow standard D&D 5e character sheet structure for field organization
- Use Option<T> for nullable character fields (not all characters have spells, etc.)

### Dependencies
- MIMIR-T-0043 (Character models must exist)
- serde and serde_yaml crates
- D&D 5e SRD reference for accurate stat calculations

### Risk Considerations
- YAML structure must be flexible enough to handle multiclassing
- Spell slot calculation varies by class and multiclass combinations
- Equipment slots and attunement limits need careful modeling
- Ensure backward compatibility if character data structure evolves

## Status Updates **[REQUIRED]**

### 2025-11-11
**Status**: Completed

Created comprehensive character data structures for YAML serialization in src/models/character/data.rs:

**Core Structures:**
- AbilityScores: STR, DEX, CON, INT, WIS, CHA with modifier calculation methods (correct floor division for D&D rules)
- Proficiencies: Skills, saves, armor, weapons, tools, languages tracking
- SpellSlots: Max/current tracking with expend/recover methods
- SpellData: Known spells, prepared spells, cantrips, spell slots by level
- InventoryItem: Name, quantity, weight, value, notes
- EquippedItems: Armor, shield, main_hand, off_hand slots
- Personality: Traits, ideals, bonds, flaws
- CharacterData: Complete character state with all D&D 5e fields

**Helper Methods:**
- Ability modifier calculation (handles negative scores correctly)
- Proficiency bonus by level (2-6 based on character level)
- Spell slot management (expend, recover, recover_all)
- Skill/save proficiency checks

**Tests:**
- Ability modifier calculation (including edge cases)
- Proficiency bonus by level
- Spell slot management
- YAML round-trip serialization

All structs use serde with proper defaults and Option<T> for nullable fields.
Updated character/mod.rs to export all data types.
All tests pass (55 tests including 5 new).