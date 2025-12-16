---
id: campaign-document-export
level: initiative
title: "Campaign Document Export"
short_code: "MIMIR-I-0017"
created_at: 2025-12-16T03:23:08.713096+00:00
updated_at: 2025-12-16T03:55:07.443424+00:00
parent: MIMIR-V-0001
blocked_by: []
archived: false

tags:
  - "#initiative"
  - "#phase/completed"


exit_criteria_met: false
estimated_complexity: M
strategy_id: NULL
initiative_id: campaign-document-export
---

# Campaign Document Export Initiative

*This template includes sections for various types of initiatives. Delete sections that don't apply to your specific use case.*

## Context **[REQUIRED]**

Campaign documents (session outlines, NPC trackers, campaign bibles, etc.) are stored as markdown files with YAML frontmatter. DMs need the ability to export these documents as styled PDFs for printing, sharing with players, or archiving. The existing print infrastructure (Typst templates, PrintService) provides a foundation to build on.

## Goals & Non-Goals **[REQUIRED]**

**Goals:**
- Export individual campaign documents as styled PDFs
- Export entire campaign as a single combined PDF with table of contents
- Maintain consistent styling with existing character sheet PDFs
- Support all 23 campaign document types

**Non-Goals:**
- Custom per-document-type templates (use generic template initially)
- HTML/web export
- Real-time preview during editing

## Use Cases

### Use Case 1: Export Single Document
- **Actor**: DM editing a campaign document
- **Scenario**: Click "Export PDF" button in document editor toolbar
- **Expected Outcome**: Save dialog appears, styled PDF is saved to chosen location

### Use Case 2: Export Full Campaign
- **Actor**: DM on campaign board view
- **Scenario**: Click "Export All as PDF" button, system generates combined PDF with TOC
- **Expected Outcome**: Single PDF with cover page, table of contents, and all documents with page breaks

## Detailed Design

### Data Flow
1. Frontend requests export via Tauri command
2. Backend reads markdown file(s) from campaign directory
3. Markdown parser extracts YAML frontmatter and converts content to Typst markup
4. PrintService compiles Typst template with document data
5. PDF bytes returned as base64 to frontend
6. Frontend triggers save dialog

### Key Components
- `mimir-dm-print/src/markdown.rs` - Markdown to Typst converter
- `mimir-dm-print/templates/campaign/document.typ` - Single document template
- `mimir-dm-print/templates/campaign/combined.typ` - Combined PDF with TOC

## UI/UX Design

### Document Editor
- Add "Export PDF" button to editor header toolbar (next to save status)
- Loading spinner during generation

### Campaign Board View
- Add "Export All as PDF" button in campaign actions area
- Progress indicator for large campaigns

## Alternatives Considered

1. **ZIP of individual PDFs** - Rejected; single PDF is easier to print and share
2. **HTML export** - Rejected; PDF better for physical printing use case
3. **Per-document-type templates** - Deferred; generic template covers initial needs

## Implementation Plan

| Task | Description |
|------|-------------|
| MIMIR-T-0141 | Create markdown to Typst converter module |
| MIMIR-T-0142 | Create Typst templates for campaign documents |
| MIMIR-T-0143 | Extend PrintService with campaign export methods |
| MIMIR-T-0144 | Add Tauri commands and frontend integration |