---
id: create-charactercreator-vue-multi
level: task
title: "Create CharacterCreator.vue multi-step wizard"
short_code: "MIMIR-T-0055"
created_at: 2025-11-10T18:57:03.055901+00:00
updated_at: 2025-11-10T18:57:03.055901+00:00
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

# Create CharacterCreator.vue multi-step wizard

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0004]]

## Objective **[REQUIRED]**

Create a multi-step wizard component for character creation guiding users through race selection, class selection, ability scores, background, equipment, and final review.

## Acceptance Criteria **[REQUIRED]**

- [ ] CharacterCreator.vue component created in `src/components/character/CharacterCreator.vue`
- [ ] Multi-step wizard with progress indicator (Step 1 of 6, etc.)
- [ ] Step 1: Basic Info (character name, player selection)
- [ ] Step 2: Race selection with racial trait display
- [ ] Step 3: Class selection with class feature preview
- [ ] Step 4: Ability score assignment (point buy, standard array, or manual)
- [ ] Step 5: Background, skills, proficiencies selection
- [ ] Step 6: Starting equipment and final review
- [ ] Next/Previous navigation with validation before advancing
- [ ] Character creation calls CharacterStore.createCharacter() on completion
- [ ] Cancel button with unsaved changes warning

## Implementation Notes **[CONDITIONAL: Technical Task]**

### Technical Approach
- Create `src/components/character/CharacterCreator.vue` with stepper UI
- Maintain wizard state in local component reactive refs
- Each step is a separate component or conditional template section
- Load race/class data from SRD reference files or embedded data
- Validate each step before allowing navigation to next
- Build complete CharacterData object progressively through steps
- On finish, call CharacterStore.createCharacter() with complete data

### Dependencies
- MIMIR-T-0053 (CharacterStore)
- D&D 5e SRD data for races, classes, backgrounds, equipment
- Stepper UI component (Vuetify v-stepper or custom)

### Risk Considerations
- Wizard state management complexity with 6+ steps
- D&D 5e race/class data must be complete and accurate
- Point buy calculator needs validation (27 points, 8-15 range)
- Starting equipment varies significantly by class
- User may want to save partial progress (future enhancement)
- Long wizard may intimidate new users

## Status Updates **[REQUIRED]**

*To be added during implementation*