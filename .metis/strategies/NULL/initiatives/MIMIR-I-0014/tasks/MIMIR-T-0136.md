---
id: implement-spell-card-and-spell
level: task
title: "Implement spell card and spell list templates"
short_code: "MIMIR-T-0136"
created_at: 2025-12-15T02:16:30.403164+00:00
updated_at: 2025-12-15T02:16:30.403164+00:00
parent: MIMIR-I-0014
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/todo"


exit_criteria_met: false
strategy_id: NULL
initiative_id: MIMIR-I-0014
---

# Implement spell card and spell list templates

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0014]]

## Objective

Create Typst templates for spell cards (poker-size for card stock printing) and spell lists (table format), plus multi-up layout for printing multiple cards per page.

## Acceptance Criteria

- [ ] `templates/spells/card.typ` - Individual spell card (2.5" x 3.5")
- [ ] `templates/spells/list.typ` - Spell list table format (letter/A4)
- [ ] `templates/spells/cards-multiup.typ` - 9 cards per letter page layout
- [ ] `SpellCollector` in Rust for fetching spell data
- [ ] Spell card includes: name, level, school, casting time, range, components, duration, description
- [ ] School icon on each card
- [ ] Spell list sorted by level with key details
- [ ] Print character's known/prepared spells as card deck
- [ ] Print from spell catalog (individual or batch)
- [ ] UI integration in character spell view and catalog

## Implementation Notes

### Spell Card Layout (2.5" x 3.5")

```
┌─────────────────────┐
│ [School] Spell Name │
│ Level X School      │
├─────────────────────┤
│ Cast: 1 action      │
│ Range: 120 ft       │
│ Comp: V, S, M (...)│
│ Dur: Instantaneous  │
├─────────────────────┤
│                     │
│ Description text    │
│ flows here...       │
│                     │
└─────────────────────┘
```

### Multi-up Layout

For letter paper (8.5" x 11"):
- 3 columns x 3 rows = 9 cards per page
- Cut lines between cards
- Margins for printer bleed

### Data Sources

1. **Character spells**: Fetch from character's known/prepared spells
2. **Catalog spells**: Fetch from spell catalog by ID or search

### Dependencies

- MIMIR-T-0133 (crate setup)
- MIMIR-T-0134 (shared styles/components)

## Status Updates

*To be added during implementation*