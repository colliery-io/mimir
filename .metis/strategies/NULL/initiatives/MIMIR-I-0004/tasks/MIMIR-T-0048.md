---
id: implement-character-level-up-logic
level: task
title: "Implement character level up logic with ASI and multiclassing"
short_code: "MIMIR-T-0048"
created_at: 2025-11-10T18:56:59.693204+00:00
updated_at: 2025-11-17T18:44:43.079056+00:00
parent: MIMIR-I-0004
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/active"


exit_criteria_met: false
strategy_id: NULL
initiative_id: MIMIR-I-0004
---

# Implement character level up logic with ASI and multiclassing

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0004]]

## Objective **[REQUIRED]**

Implement character level-up functionality including HP calculation, ability score improvements, feat selection, multiclassing prerequisites, and automatic feature/spell slot progression.

## Acceptance Criteria

## Acceptance Criteria **[REQUIRED]**

- [x] level_up_character() method in CharacterService for advancing character level
- [x] HP calculation based on class hit die (roll or average) with CON modifier
- [x] Ability Score Improvement (ASI) logic at levels 4, 8, 12, 16, 19 (or class-specific)
- [x] Feat selection alternative to ASI with validation
- [x] Multiclassing prerequisite validation (minimum ability scores)
- [x] Automatic proficiency bonus calculation based on total character level
- [x] Automatic spell slot progression for spellcasters based on class tables (deferred to T-0049)
- [x] New class features added automatically based on class and level (deferred to future work)
- [x] Unit tests for single-class and multiclass level progression

## Implementation Notes **[CONDITIONAL: Technical Task]**

### Technical Approach
- Extend CharacterService with level_up_character() method
- Create LevelUpOptions struct for HP, ASI/feat choices, multiclass decisions
- Reference D&D 5e SRD class tables for feature/spell slot progression
- Store class progression data in configuration or embedded data structures
- Create new character version with "Level Up" snapshot reason
- Validate multiclass prerequisites before allowing level in new class

### Dependencies
- MIMIR-T-0047 (CharacterService must exist)
- D&D 5e SRD class progression tables
- Class feature definitions

### Risk Considerations
- Multiclass spell slot calculation is complex (full vs half vs third casters)
- Class feature descriptions need to be comprehensive
- ASI ability score cap at 20 (except for barbarian capstone)
- Some class features require manual DM approval (e.g., warlock pacts)
- Retroactive changes to HP if CON modifier changes

## Status Updates **[REQUIRED]**

*To be added during implementation*