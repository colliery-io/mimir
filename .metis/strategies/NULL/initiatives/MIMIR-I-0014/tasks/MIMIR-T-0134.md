---
id: create-shared-typst-styles-and
level: task
title: "Create shared Typst styles and components"
short_code: "MIMIR-T-0134"
created_at: 2025-12-15T02:16:30.185038+00:00
updated_at: 2025-12-15T02:16:30.185038+00:00
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

# Create shared Typst styles and components

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0014]]

## Objective

Build the shared Typst styles, reusable components, and icon library that all templates will use. This ensures consistent typography, spacing, and visual elements across all print outputs.

## Acceptance Criteria

- [ ] `templates/_shared/styles.typ` with base typography and page settings
- [ ] `templates/_shared/components.typ` with reusable layout components
- [ ] `templates/_shared/icons.typ` with class and spell school icons
- [ ] Ability score block component (6-stat grid with modifiers)
- [ ] Stat block component (for monsters)
- [ ] Table styles (bordered, zebra-striped options)
- [ ] Card layout component (for spell cards, NPC cards)
- [ ] All components render correctly in B&W
- [ ] Documentation comments in Typst files

## Implementation Notes

### styles.typ

```typst
// Base document settings
#let mimir-doc(body) = {
  set page(paper: "us-letter", margin: 0.5in)
  set text(font: "Inter", size: 10pt)
  set heading(numbering: none)
  body
}

// Typography presets
#let title-text = text.with(size: 16pt, weight: "bold")
#let subtitle-text = text.with(size: 12pt, style: "italic")
#let label-text = text.with(size: 8pt, weight: "bold")
```

### components.typ

```typst
// Ability score block (2x3 grid)
#let ability-scores(str, dex, con, int, wis, cha) = { ... }

// D&D stat block (monster format)
#let stat-block(name, size, type, ac, hp, speed, abilities, ...) = { ... }

// Card frame for spell/NPC cards  
#let card(title, subtitle, body) = { ... }
```

### icons.typ

- Class icons: Fighter, Wizard, Rogue, Cleric, etc. (simple SVG or Typst drawings)
- Spell school icons: Evocation, Abjuration, etc.
- Damage type icons: Fire, Cold, Lightning, etc.

### Dependencies

- MIMIR-T-0133 (crate setup) must be complete

## Status Updates

*To be added during implementation*