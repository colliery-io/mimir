---
id: implement-spell-management-and
level: task
title: "Implement spell management and slot calculation"
short_code: "MIMIR-T-0049"
created_at: 2025-11-10T18:57:00.127074+00:00
updated_at: 2025-11-10T18:57:00.127074+00:00
parent: MIMIR-I-0004
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/todo"


exit_criteria_met: false
strategy_id: NULL
initiative_id: MIMIR-I-0004
---

# Implement spell management and slot calculation

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0004]]

## Objective **[REQUIRED]**

Implement comprehensive spell management including spell learning, preparation, casting, slot tracking, and automatic spell save DC/attack bonus calculation.

## Acceptance Criteria **[REQUIRED]**

- [ ] SpellManager module in CharacterService for spell operations
- [ ] add_spell_to_known() method with class spell list validation
- [ ] prepare_spells() method with preparation limit validation
- [ ] cast_spell() method that consumes appropriate spell slot
- [ ] rest() method (short/long) that restores spell slots appropriately
- [ ] calculate_spell_slots() method based on class level(s) and multiclassing
- [ ] calculate_spell_save_dc() method based on spellcasting ability
- [ ] calculate_spell_attack_bonus() method based on spellcasting ability
- [ ] Unit tests for spell slot consumption, preparation limits, and DC/attack calculations

## Implementation Notes **[CONDITIONAL: Technical Task]**

### Technical Approach
- Extend CharacterService with spell management methods
- Track current spell slots vs maximum spell slots in CharacterData
- Implement multiclass spellcasting table for slot calculation
- Store known spells and prepared spells separately
- Validate spell preparation based on class rules (Wizard prepares, Sorcerer knows)
- Calculate spell save DC: 8 + proficiency + spellcasting ability modifier
- Calculate spell attack: proficiency + spellcasting ability modifier

### Dependencies
- MIMIR-T-0044 (SpellData struct)
- MIMIR-T-0047 (CharacterService)
- D&D 5e spell lists by class
- Multiclass spellcasting table

### Risk Considerations
- Warlock pact magic slots work differently (refresh on short rest)
- Some classes have unique spellcasting (e.g., Ranger/Paladin start at level 2)
- Ritual casting doesn't consume slots
- Some features grant bonus spell slots (e.g., Arcane Recovery)
- Spell slot restoration timing varies by class feature

## Status Updates **[REQUIRED]**

*To be added during implementation*