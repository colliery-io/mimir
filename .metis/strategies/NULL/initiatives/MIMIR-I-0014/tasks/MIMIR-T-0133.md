---
id: set-up-mimir-dm-print-crate-with
level: task
title: "Set up mimir-dm-print crate with Typst integration"
short_code: "MIMIR-T-0133"
created_at: 2025-12-15T02:16:30.101099+00:00
updated_at: 2025-12-15T14:53:17.729679+00:00
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

# Set up mimir-dm-print crate with Typst integration

## Parent Initiative

[[MIMIR-I-0014]]

## Objective

Create the foundational `mimir-dm-print` crate with Typst compiler integration. This establishes the core infrastructure for all print/PDF generation features.

## Acceptance Criteria

## Acceptance Criteria

- [ ] `crates/mimir-dm-print/` crate created and added to workspace
- [ ] `typst` and `typst-pdf` crates added as dependencies
- [ ] Custom `TypstWorld` implementation for Mimir (file resolution, fonts)
- [ ] `PrintService` struct with `render_to_pdf()` method
- [ ] Basic Tauri commands: `generate_pdf`, `get_available_templates`
- [ ] Template directory structure created (`templates/_shared/`, etc.)
- [ ] Simple "hello world" template compiles to PDF successfully
- [ ] Unit tests for PrintService

## Implementation Notes

### Crate Structure

```
crates/mimir-dm-print/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── service.rs       # PrintService
│   ├── world.rs         # MimirTypstWorld implementation
│   ├── error.rs         # Print-specific errors
│   └── commands.rs      # Tauri commands
└── templates/
    ├── _shared/
    │   └── .gitkeep
    └── test/
        └── hello.typ    # Test template
```

### Key Dependencies

```toml
[dependencies]
typst = "0.11"
typst-pdf = "0.11"
comemo = "0.4"  # Required for Typst caching
```

### TypstWorld Implementation

The `World` trait is how Typst resolves files, fonts, and other resources. We need a custom implementation that:
- Resolves template files from our templates directory
- Provides system fonts
- Handles data injection (JSON → Typst variables)

### Tauri Integration

Commands to expose:
- `generate_pdf(template: String, data: Value) -> Vec<u8>`
- `list_templates() -> Vec<TemplateInfo>`
- `save_pdf(path: String, bytes: Vec<u8>) -> Result<()>`

## Status Updates

*To be added during implementation*