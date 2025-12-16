---
id: create-markdown-to-typst-converter
level: task
title: "Create markdown to Typst converter module"
short_code: "MIMIR-T-0141"
created_at: 2025-12-16T03:23:40.500983+00:00
updated_at: 2025-12-16T03:54:38.244244+00:00
parent: MIMIR-I-0017
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/completed"


exit_criteria_met: false
strategy_id: NULL
initiative_id: MIMIR-I-0017
---

# Create markdown to Typst converter module

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative

[[MIMIR-I-0017]] - Campaign Document Export

## Objective

Create a Rust module that parses campaign markdown documents (with YAML frontmatter) and converts the markdown content to Typst markup for PDF rendering.

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [ ] Parse YAML frontmatter from markdown files using `gray_matter` crate
- [ ] Convert markdown syntax to Typst markup:
  - Headers (`#` -> `=`)
  - Bold (`**text**` -> `*text*`)
  - Italic (`*text*` -> `_text_`)
  - Lists (preserve `-` syntax)
  - Links (`[text](url)` -> `#link("url")[text]`)
  - Code blocks (preserve with raw blocks)
  - Tables (convert to Typst table syntax)
- [ ] Return `ParsedDocument { frontmatter: serde_json::Value, typst_content: String }`
- [ ] Handle edge cases (empty files, no frontmatter, malformed markdown)

## Implementation Notes

### Files to Create
- `crates/mimir-dm-print/src/markdown.rs` - Main converter module

### Technical Approach
- Use `gray_matter` (already in deps) for frontmatter extraction
- Use `pulldown-cmark` crate for markdown parsing
- Walk the markdown AST and emit Typst markup

### Dependencies
- None (first task in sequence)

## Status Updates

*To be added during implementation*