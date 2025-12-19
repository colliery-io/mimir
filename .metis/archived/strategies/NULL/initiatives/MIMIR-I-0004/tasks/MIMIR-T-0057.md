---
id: create-levelupdialog-vue-component
level: task
title: "Create LevelUpDialog.vue component"
short_code: "MIMIR-T-0057"
created_at: 2025-11-10T18:57:04.213847+00:00
updated_at: 2025-11-22T03:48:50.763932+00:00
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

# Create LevelUpDialog.vue component

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0004]]

## Objective **[REQUIRED]**

Create a LevelUpDialog Vue component that guides users through the level-up process, including HP rolls, ASI/feat selection, class feature acquisition, and spell learning.

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria **[REQUIRED]**

- [ ] LevelUpDialog.vue component created in `src/components/character/LevelUpDialog.vue`
- [ ] Dialog triggered from CharacterSheet "Level Up" button
- [ ] Class selection step for multiclassing (if applicable) with prerequisite validation
- [ ] HP roll interface with option to roll or take average
- [ ] ASI selection at appropriate levels with +2/+1 or +1/+1 distribution
- [ ] Feat selection as alternative to ASI with feat list and descriptions
- [ ] New class features displayed automatically based on level
- [ ] Spell selection for spellcasters (new spells known or prepared)
- [ ] Review step showing all changes before applying
- [ ] Level up calls CharacterStore.levelUpCharacter() on completion

## Implementation Notes **[CONDITIONAL: Technical Task]**

### Technical Approach
- Create `src/components/character/LevelUpDialog.vue` as modal dialog
- Use stepper or sequential form sections for level-up steps
- Validate multiclass prerequisites before allowing class selection
- Implement dice roller UI for HP (with re-roll option if house rules allow)
- Load feat list from D&D 5e SRD data
- Show ASI option only at appropriate levels (4, 8, 12, 16, 19)
- Call CharacterStore.levelUpCharacter() with all selections

### Dependencies
- MIMIR-T-0053 (CharacterStore with levelUp action)
- D&D 5e SRD data for feats, class features, spell lists
- Dice roller component (or create simple one)

### Risk Considerations
- Multiclass spellcasting rules are complex (caster level vs character level)
- Some feats have prerequisites that need validation
- Fighter gets ASI at different levels than other classes
- Need to handle partial level-up (user cancels mid-process)
- Undo functionality may be desired but complex to implement

## Status Updates **[REQUIRED]**

*To be added during implementation*