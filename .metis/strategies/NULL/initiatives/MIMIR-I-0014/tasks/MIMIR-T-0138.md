---
id: implement-session-prep-and-npc
level: task
title: "Implement session prep and NPC card templates"
short_code: "MIMIR-T-0138"
created_at: 2025-12-15T02:16:37.282602+00:00
updated_at: 2025-12-15T02:16:37.282602+00:00
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

# Implement session prep and NPC card templates

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0014]]

## Objective

Create Typst templates for session preparation sheets (DM reference for running a session) and NPC cards (quick reference during roleplay). Also includes generic handout template for player-facing documents.

## Acceptance Criteria

- [ ] `templates/session/prep.typ` - Session prep sheet (letter/A4)
- [ ] `templates/session/npc-card.typ` - NPC card (3" x 5" index card)
- [ ] `templates/session/handout.typ` - Generic player handout template
- [ ] `SessionCollector` in Rust for fetching session/module data
- [ ] Session prep includes: session notes, NPCs, locations, planned encounters, key items
- [ ] NPC card includes: name, role, appearance, personality, key info, secrets (DM only)
- [ ] Handout template supports title, body text, optional image placeholder
- [ ] UI integration in session view
- [ ] Print NPCs from session or standalone

## Implementation Notes

### Session Prep Sheet Layout

```
┌─────────────────────────────────────────┐
│ SESSION: The Goblin Caves              │
│ Module: Lost Mine of Phandelver        │
├─────────────────────────────────────────┤
│ SUMMARY                                 │
│ Brief overview of session goals...      │
├──────────────────┬──────────────────────┤
│ KEY NPCs         │ LOCATIONS            │
│ - Sildar         │ - Cragmaw Hideout    │
│ - Klarg          │ - Goblin Trail       │
├──────────────────┴──────────────────────┤
│ ENCOUNTERS                              │
│ 1. Goblin Ambush (4 goblins, CR 1/4)   │
│ 2. Cave Entrance (2 goblins)           │
├─────────────────────────────────────────┤
│ NOTES                                   │
│ - Remember Sildar knows about...        │
└─────────────────────────────────────────┘
```

### NPC Card Layout (3" x 5")

```
┌─────────────────────────────────────┐
│ SILDAR HALLWINTER                   │
│ Human Knight, Ally                  │
├─────────────────────────────────────┤
│ Appearance: Middle-aged, graying    │
│ beard, worn armor                   │
├─────────────────────────────────────┤
│ Personality: Honorable, direct      │
│ Goal: Find Iarno Albrek             │
├─────────────────────────────────────┤
│ Secret: Member of Lords' Alliance   │
└─────────────────────────────────────┘
```

### Dependencies

- MIMIR-T-0133 (crate setup)
- MIMIR-T-0134 (shared styles/components)

## Status Updates

*To be added during implementation*