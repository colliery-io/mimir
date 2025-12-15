---
id: implement-character-sheet-and
level: task
title: "Implement character sheet and summary templates"
short_code: "MIMIR-T-0135"
created_at: 2025-12-15T02:16:30.296150+00:00
updated_at: 2025-12-15T15:05:53.770051+00:00
parent: MIMIR-I-0014
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/active"


exit_criteria_met: false
strategy_id: NULL
initiative_id: MIMIR-I-0014
---

# Implement character sheet and summary templates

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0014]]

## Objective

Create Typst templates for character sheets (full and summary versions) plus the Rust data collector that fetches character data from the database and structures it for the templates.

## Acceptance Criteria

## Acceptance Criteria

- [ ] `templates/character/sheet.typ` - Full character sheet (letter/A4)
- [ ] `templates/character/summary.typ` - Half-page quick reference
- [ ] `CharacterCollector` in Rust that fetches and structures character data
- [ ] Character sheet includes: name, race, class, level, abilities, proficiencies, equipment, spells
- [ ] Summary includes: name, class/level, AC, HP, key abilities, attacks
- [ ] Tauri command to generate character PDF
- [ ] UI "Print" button added to character sheet view
- [ ] Generated PDFs render correctly and are printer-friendly

## Implementation Notes

### Character Sheet Layout

```
┌─────────────────────────────────────────┐
│  [Name]                    [Class Icon] │
│  Level X Race Class                     │
├─────────────────────────────────────────┤
│  ┌─────┐ ┌─────┐ ┌─────┐               │
│  │ STR │ │ DEX │ │ CON │  HP: XX/XX    │
│  │ +3  │ │ +1  │ │ +2  │  AC: XX       │
│  └─────┘ └─────┘ └─────┘               │
│  ┌─────┐ ┌─────┐ ┌─────┐               │
│  │ INT │ │ WIS │ │ CHA │               │
│  │ +0  │ │ +1  │ │ -1  │               │
│  └─────┘ └─────┘ └─────┘               │
├─────────────────────────────────────────┤
│  Proficiencies        │  Equipment      │
│  - Skills             │  - Armor        │
│  - Saves              │  - Weapons      │
│  - Tools              │  - Items        │
├─────────────────────────────────────────┤
│  Features & Traits    │  Spells         │
└─────────────────────────────────────────┘
```

### Data Collector

```rust
pub struct CharacterCollector;

impl CharacterCollector {
    pub fn collect(db: &DbPool, character_id: i32) -> Result<CharacterPrintData> {
        // Fetch character, abilities, equipment, spells
        // Structure into CharacterPrintData for template
    }
}
```

### Dependencies

- MIMIR-T-0133 (crate setup)
- MIMIR-T-0134 (shared styles/components)

## Status Updates

*To be added during implementation*