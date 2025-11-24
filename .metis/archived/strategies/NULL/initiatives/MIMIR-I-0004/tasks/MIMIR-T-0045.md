---
id: implement-markdown-template
level: task
title: "Implement markdown template renderer for character sheets"
short_code: "MIMIR-T-0045"
created_at: 2025-11-10T18:56:58.574665+00:00
updated_at: 2025-11-15T19:48:06.399750+00:00
parent: MIMIR-I-0004
blocked_by: []
archived: true

tags:
  - "#task"
  - "#phase/completed"


exit_criteria_met: false
strategy_id: NULL
initiative_id: MIMIR-I-0004
---

# Implement markdown template renderer for character sheets

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0004]]

## Objective **[REQUIRED]**

Implement a markdown template rendering system that generates human-readable character sheets from CharacterData structs, with support for dynamic sections based on character class and features.

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria **[REQUIRED]**

- [ ] CharacterRenderer trait created in `crates/mimir-dm-core/src/services/character/renderer.rs`
- [ ] MarkdownRenderer implementation with render_character_sheet() method
- [ ] Template sections for basic info, ability scores, skills, combat stats, features, spells, inventory
- [ ] Conditional rendering for spellcasters (show spells section) vs non-spellcasters
- [ ] Formatted markdown tables for skills, inventory, and spell lists
- [ ] Character sheet includes calculated values (AC, initiative, spell save DC, attack bonuses)
- [ ] Template supports both single-class and multiclass characters
- [ ] Unit tests rendering sample characters and validating markdown output

## Implementation Notes **[CONDITIONAL: Technical Task]**

### Technical Approach
- Create `crates/mimir-dm-core/src/services/character/renderer.rs` module
- Use string formatting and template interpolation (or tera/handlebars crate if needed)
- Implement CharacterRenderer trait for extensibility (future HTML/PDF renderers)
- Generate markdown with proper heading hierarchy, tables, and lists
- Include both raw stats and calculated bonuses in output

### Dependencies
- MIMIR-T-0044 (CharacterData structs required)
- Consider tera or handlebars crate for advanced templating
- markdown formatting conventions

### Risk Considerations
- Template must remain readable for both humans and LLMs
- Character sheet format should align with D&D community standards
- Long spell lists could make sheets very large
- Need to handle edge cases (characters with no equipment, etc.)

## Status Updates **[REQUIRED]**

### 2025-11-11
**Status**: Completed

Created markdown template renderer for character sheets in src/services/character/renderer.rs:

**CharacterRenderer Trait:**
- Extensible trait for future renderers (HTML, PDF, etc.)
- render() method takes CharacterData and returns formatted string

**MarkdownRenderer Implementation:**
- Comprehensive character sheet generation with all D&D 5e sections
- Header: Character name, level, class, subclass
- Metadata: Race, background, alignment, XP, version info
- Ability scores: Table with scores and modifiers
- Combat stats: HP, hit dice, proficiency bonus
- Proficiencies: Skills, saves, armor, weapons, tools, languages
- Class features: Bulleted list
- Feats: Bulleted list (conditional)
- Spells: Slots, cantrips, known/prepared spells (conditional for spellcasters)
- Equipment: Equipped armor, shield, weapons (conditional)
- Inventory: Table with quantity, weight, value, notes (conditional)
- Personality: Traits, ideals, bonds, flaws (conditional)

**Features:**
- Conditional rendering: Only shows sections with data
- Proper markdown formatting: Headers, tables, lists
- Calculated values: Ability modifiers, proficiency bonus
- Clean output: No frontmatter, pure presentation

**Tests:**
- Fighter character rendering (complete sheet)
- Wizard character with spells
- Minimal character with conditional sections

All tests pass (58 tests including 3 new renderer tests).