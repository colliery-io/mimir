---
id: physical-print-output-system
level: initiative
title: "Physical Print Output System"
short_code: "MIMIR-I-0014"
created_at: 2025-12-06T16:02:36.478217+00:00
updated_at: 2025-12-15T03:01:59.194149+00:00
parent: 
blocked_by: []
archived: false

tags:
  - "#initiative"
  - "#phase/active"


exit_criteria_met: false
estimated_complexity: L
strategy_id: NULL
initiative_id: physical-print-output-system
---

# Physical Print Output System Initiative

## Context

Mimir now has solid campaign management with characters, sessions, and catalog data. The next step is enabling DMs and players to get this information into physical form for table play. Physical materials remain essential for many groups - character sheets for quick reference, spell cards for casters, monster stat blocks for DMs, and session prep notes.

Currently, data lives only in the app. This initiative adds a professional print output system using Typst (a modern Rust-native document compiler) with a template architecture that produces clean, printer-friendly PDFs.

## Goals & Non-Goals

**Goals:**
- Generate printable PDFs from existing Mimir data (characters, spells, monsters, sessions)
- Professional layouts with tables, iconography, and clean typography
- B&W printer friendly - no textures or heavy backgrounds
- Template + stylesheet architecture for consistent theming
- Multiple output formats: standard PDF, direct print, card stock layouts (spell cards, etc.)
- Support both DM and player use cases

**Non-Goals:**
- Fancy textures, parchment backgrounds, or color-dependent designs
- User-editable templates in v1 (architecture supports it, but locked down initially)
- New content types - this renders existing Mimir data only
- Web/online sharing of generated documents

## Output Types

### Character Materials
- **Character Sheet** (letter/A4) - Full character with abilities, proficiencies, equipment, spells
- **Character Summary** (half-page) - Quick reference with key stats only

### Spell Materials
- **Spell Cards** (2.5" x 3.5" poker size) - Individual spell with all details, multi-up printing
- **Spell List** (letter/A4) - All prepared/known spells in table format

### Monster/Encounter Materials
- **Monster Stat Block** (variable) - Single monster with full stats
- **Encounter Sheet** (letter/A4) - Multiple monsters for an encounter with initiative tracker
- **Monster Cards** (3" x 5" index) - Quick reference stat blocks

### Session/Campaign Materials
- **Session Prep Sheet** (letter/A4) - Session notes, NPCs, locations, encounters
- **NPC Cards** (3" x 5") - NPC details for quick reference
- **Handouts** (variable) - Player-facing information sheets

## Use Cases

### Use Case 1: Print Character Sheet
- **Actor**: Player or DM
- **Scenario**: Select character → Choose "Print" → Select "Character Sheet" template → Preview → Print/Save PDF
- **Expected Outcome**: Professional character sheet ready for table use

### Use Case 2: Print Spell Deck
- **Actor**: Player (spellcaster)
- **Scenario**: Select character → Choose "Print Spell Cards" → System generates cards for known/prepared spells → Print on card stock
- **Expected Outcome**: Deck of spell cards for quick reference during play

### Use Case 3: DM Encounter Prep
- **Actor**: DM
- **Scenario**: Select session/encounter → Choose "Print Encounter Sheet" → System compiles monster stats, terrain notes → Print
- **Expected Outcome**: Single sheet with all encounter information

## Architecture

### Overview

```
┌─────────────────────────────────────────────────────────────┐
│                        Mimir App                            │
├─────────────────────────────────────────────────────────────┤
│  UI Layer                                                   │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │ Print Menu  │  │  Preview    │  │  Settings   │         │
│  │ (per view)  │  │  Window     │  │  (defaults) │         │
│  └──────┬──────┘  └──────┬──────┘  └─────────────┘         │
├─────────┴────────────────┴──────────────────────────────────┤
│  Print Service (Rust)                                       │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │ Data        │  │ Template    │  │ Typst       │         │
│  │ Collectors  │→ │ Engine      │→ │ Compiler    │→ PDF    │
│  └─────────────┘  └─────────────┘  └─────────────┘         │
├─────────────────────────────────────────────────────────────┤
│  Templates (Typst files)                                    │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │ character/  │  │ spells/     │  │ monsters/   │         │
│  │ - sheet.typ │  │ - card.typ  │  │ - stat.typ  │         │
│  │ - summary   │  │ - list.typ  │  │ - encounter │         │
│  └─────────────┘  └─────────────┘  └─────────────┘         │
│  ┌─────────────┐                                           │
│  │ _shared/    │  (icons, styles, common components)       │
│  └─────────────┘                                           │
└─────────────────────────────────────────────────────────────┘
```

### Tech Stack

- **Typst**: Rust-native document compiler (embeds via `typst` crate)
- **Templates**: `.typ` files with Typst markup
- **Styles**: Shared Typst style definitions for consistent look
- **Icons**: SVG icons embedded in templates (class icons, school icons, etc.)

## Detailed Design

### Typst Integration

```rust
// Embed Typst compiler in Rust
use typst::eval::Tracer;
use typst_pdf::pdf;

pub struct PrintService {
    world: MimirTypstWorld,  // Custom World implementation for Mimir
}

impl PrintService {
    pub fn render_to_pdf(&self, template: &str, data: serde_json::Value) -> Result<Vec<u8>> {
        // 1. Load template
        // 2. Inject data as Typst variables
        // 3. Compile to PDF
        // 4. Return bytes
    }
}
```

### Template Structure

```typst
// templates/character/sheet.typ
#import "../_shared/styles.typ": *
#import "../_shared/icons.typ": class-icon

#let character-sheet(data) = {
  set page(paper: "us-letter", margin: 0.5in)
  set text(font: "Inter", size: 10pt)
  
  // Header
  grid(columns: (1fr, auto),
    [= #data.name],
    class-icon(data.class)
  )
  
  [_Level #data.level #data.race #data.class_]
  
  // Ability scores block
  abilities-block(data.abilities)
  
  // ... rest of layout
}
```

### Data Flow

1. **User triggers print** from UI (character view, spell list, etc.)
2. **Tauri command** calls PrintService with entity ID and template choice
3. **Data Collector** fetches entity data from database, structures for template
4. **Template Engine** loads `.typ` file, injects data
5. **Typst Compiler** produces PDF bytes
6. **Output**: Save to file, open in preview, or send to printer

### File Organization

```
crates/mimir-dm-print/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── service.rs          # PrintService
│   ├── collectors/         # Data collectors per entity type
│   │   ├── character.rs
│   │   ├── spell.rs
│   │   └── monster.rs
│   └── world.rs            # Typst World implementation
├── templates/
│   ├── _shared/
│   │   ├── styles.typ      # Common styles
│   │   ├── icons.typ       # Icon definitions
│   │   └── components.typ  # Reusable components (stat blocks, tables)
│   ├── character/
│   │   ├── sheet.typ
│   │   └── summary.typ
│   ├── spells/
│   │   ├── card.typ
│   │   └── list.typ
│   └── monsters/
│       ├── statblock.typ
│       └── encounter.typ
└── assets/
    └── icons/              # SVG icons
```

## UI/UX Design

### Print Integration Points

Print functionality integrates into existing views:

- **Character Sheet View**: "Print" dropdown → Character Sheet, Summary, Spell Cards
- **Catalog Views**: "Print" button on spell/monster detail → Card or Stat Block
- **Session View**: "Print Prep Sheet" button
- **Encounter Builder** (future): "Print Encounter" button

### Preview Window

Before printing, users see a preview:
- Rendered PDF display
- Page navigation for multi-page documents
- "Save PDF" / "Print" buttons
- Template selector (if multiple options)

### Settings

Print settings in app preferences:
- Default paper size (Letter/A4)
- Default save location
- Card stock layout preferences (cards per page)

## Alternatives Considered

### HTML + CSS → PDF (via Puppeteer/wkhtmltopdf)
- **Pros**: Familiar web tech, flexible styling
- **Cons**: Heavy dependency (bundled Chromium), rendering inconsistencies, slow
- **Rejected**: Adds 100MB+ to app size, overkill for document generation

### Native Rust PDF (printpdf/genpdf)
- **Pros**: No dependencies, fast
- **Cons**: Limited layout capabilities, custom DSL needed, hard to iterate on designs
- **Rejected**: Too low-level for complex layouts like character sheets

### LaTeX
- **Pros**: Powerful typesetting, well-established
- **Cons**: Heavy toolchain, slow compilation, complex syntax, poor Rust integration
- **Rejected**: Complexity doesn't justify benefits for this use case

### Typst (Selected)
- **Pros**: Rust-native, fast, modern syntax, purpose-built for documents, great typography
- **Cons**: Newer ecosystem, learning curve
- **Selected**: Best fit for Rust stack, clean template system, high-quality output

## Implementation Plan

### Phase 1: Foundation
- Add `typst` crate dependency
- Create `mimir-dm-print` crate structure
- Implement basic `TypstWorld` for file resolution
- Create PrintService with `render_to_pdf` method
- Build shared styles (`_shared/styles.typ`)
- Add Tauri commands for PDF generation

### Phase 2: Character Templates
- Character sheet template (full)
- Character summary template (half-page)
- Data collector for character entities
- UI integration in character view

### Phase 3: Spell Templates
- Spell card template (poker size)
- Spell list template (table format)
- Multi-up card layout (9 per letter page)
- Data collector for spells
- UI integration in character spell view and catalog

### Phase 4: Monster Templates
- Monster stat block template
- Encounter sheet template (multiple monsters)
- Monster card template (index card size)
- Data collector for monsters
- UI integration in catalog and encounter views

### Phase 5: Session/Campaign Templates
- Session prep sheet template
- NPC card template
- Handout template (generic)
- UI integration in session view

### Phase 6: Polish
- Preview window implementation
- Print settings UI
- Icon set completion
- Template refinement based on actual use