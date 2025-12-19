---
id: implement-monster-stat-block-and
level: task
title: "Implement monster stat block and encounter templates"
short_code: "MIMIR-T-0137"
created_at: 2025-12-15T02:16:37.209348+00:00
updated_at: 2025-12-15T15:53:27.489122+00:00
parent: MIMIR-I-0014
blocked_by: []
archived: true

tags:
  - "#task"
  - "#phase/completed"


exit_criteria_met: false
strategy_id: NULL
initiative_id: MIMIR-I-0014
---

# Implement monster stat block and encounter templates

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0014]]

## Objective

Create Typst templates for monster stat blocks (standard D&D format) and encounter sheets that compile multiple monsters onto a single page for DM reference during combat.

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [ ] `templates/monsters/statblock.typ` - Single monster stat block
- [ ] `templates/monsters/encounter.typ` - Multi-monster encounter sheet
- [ ] `templates/monsters/card.typ` - Index card size (3" x 5") quick reference
- [ ] `MonsterCollector` in Rust for fetching monster data from catalog
- [ ] Stat block follows standard D&D 5e format (AC, HP, Speed, abilities, actions, etc.)
- [ ] Encounter sheet includes initiative tracker section
- [ ] Monster card fits key combat stats on index card
- [ ] Print from monster catalog
- [ ] UI integration in catalog monster view

## Implementation Notes

### Stat Block Layout (Standard D&D Format)

```
┌─────────────────────────────────────┐
│ GOBLIN                              │
│ Small humanoid (goblinoid), NE      │
├─────────────────────────────────────┤
│ AC 15 (leather armor, shield)       │
│ HP 7 (2d6)                          │
│ Speed 30 ft.                        │
├─────────────────────────────────────┤
│ STR  DEX  CON  INT  WIS  CHA       │
│  8   14   10   10    8    8        │
│ (-1) (+2) (+0) (+0) (-1) (-1)      │
├─────────────────────────────────────┤
│ Skills Stealth +6                   │
│ Senses darkvision 60 ft., PP 9     │
│ Languages Common, Goblin            │
│ CR 1/4 (50 XP)                      │
├─────────────────────────────────────┤
│ Nimble Escape. The goblin can...   │
├─────────────────────────────────────┤
│ ACTIONS                             │
│ Scimitar. Melee Weapon Attack...   │
│ Shortbow. Ranged Weapon Attack...  │
└─────────────────────────────────────┘
```

### Encounter Sheet Layout

- Header: Encounter name, location, difficulty
- Initiative tracker grid (empty boxes for rolling)
- Multiple stat blocks arranged to fit page
- Notes section for DM

### Dependencies

- MIMIR-T-0133 (crate setup)
- MIMIR-T-0134 (shared styles/components)

## Status Updates

*To be added during implementation*