---
id: create-characterlist-vue-and
level: task
title: "Create CharacterList.vue and CharacterSheet.vue components"
short_code: "MIMIR-T-0056"
created_at: 2025-11-10T18:57:03.645117+00:00
updated_at: 2025-11-21T10:38:07.220706+00:00
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

# Create CharacterList.vue and CharacterSheet.vue components

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0004]]

## Objective **[REQUIRED]**

Create CharacterList and CharacterSheet Vue components for displaying campaign characters in a list view and showing detailed character information in a formatted sheet view.

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria **[REQUIRED]**

- [ ] CharacterList.vue component created in `src/components/character/CharacterList.vue`
- [ ] List displays all characters with name, player, class, level, race in card or table format
- [ ] Click character to navigate to CharacterSheet view

- [ ] CharacterSheet.vue component created in `src/components/character/CharacterSheet.vue`
- [ ] Sheet displays all character data in organized sections (stats, skills, combat, features, spells, inventory)
- [ ] Calculated values displayed (AC, initiative, skill bonuses, spell save DC)

- [ ] Level Up button that opens LevelUpDialog
- [ ] Delete character button with confirmation

## Implementation Notes **[CONDITIONAL: Technical Task]**

### Technical Approach
- Create `src/components/character/CharacterList.vue` with grid or table layout
- Create `src/components/character/CharacterSheet.vue` mimicking D&D character sheet layout
- Use CharacterStore to load character data
- Render markdown character sheet or custom HTML layout
- Organize sheet into collapsible sections for better UX
- Display both raw and calculated values (e.g., "14 (+2)" for abilities)
- Use Vue Router for navigation between list and detail views

### Dependencies
- MIMIR-T-0053 (CharacterStore)
- Vue Router for navigation
- UI components for layout and styling

### Risk Considerations
- Character sheet layout is complex, may need multiple iterations
- Long spell lists or inventory could make sheet very long (scrolling)
- Need print-friendly stylesheet for physical character sheets
- Calculated values must update reactively when character data changes
- Performance with many characters in list (consider virtual scrolling)

## Status Updates **[REQUIRED]**

*To be added during implementation*