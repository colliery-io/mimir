---
id: integrate-character-management
level: task
title: "Integrate character management into campaign view"
short_code: "MIMIR-T-0060"
created_at: 2025-11-10T18:57:06.000273+00:00
updated_at: 2025-11-10T18:57:06.000273+00:00
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

# Integrate character management into campaign view

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0004]]

## Objective **[REQUIRED]**

Integrate character management components into the campaign view, adding navigation, routing, and UI elements to access player and character features from the main campaign interface.

## Acceptance Criteria **[REQUIRED]**

- [ ] Add "Players" tab/section to campaign view navigation
- [ ] Add "Characters" tab/section to campaign view navigation
- [ ] Vue Router routes configured for /campaign/:id/players, /campaign/:id/characters, /campaign/:id/character/:charId
- [ ] PlayerManager component integrated into Players view
- [ ] CharacterList component integrated into Characters view
- [ ] CharacterSheet component accessible via route with character ID
- [ ] Navigation breadcrumbs showing current location (Campaign > Characters > Character Name)
- [ ] Quick access buttons in campaign header (Create Character, Manage Players)
- [ ] Character count badge on Characters tab showing active character count

## Implementation Notes **[CONDITIONAL: Technical Task]**

### Technical Approach
- Modify campaign view layout to include Players and Characters tabs
- Add new routes to router configuration (src/router/index.ts)
- Create view wrapper components if needed for consistent layout
- Pass campaign ID as route parameter to all character-related routes
- Update campaign navigation component to highlight active section
- Add floating action button or header button for quick character creation

### Dependencies
- MIMIR-T-0054 (PlayerManager component)
- MIMIR-T-0055 (CharacterCreator component)
- MIMIR-T-0056 (CharacterList and CharacterSheet components)
- Existing campaign view structure
- Vue Router

### Risk Considerations
- Need to load players/characters when navigating to respective tabs
- Campaign context must be maintained across character views
- Deep linking to specific character sheets must work correctly
- Back navigation should return to appropriate context
- Consider lazy loading character components for performance

## Status Updates **[REQUIRED]**

*To be added during implementation*