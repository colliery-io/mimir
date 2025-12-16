---
id: extend-printservice-with-campaign
level: task
title: "Extend PrintService with campaign export methods"
short_code: "MIMIR-T-0143"
created_at: 2025-12-16T03:23:40.787694+00:00
updated_at: 2025-12-16T03:55:00.284649+00:00
parent: MIMIR-I-0017
blocked_by: []
archived: true

tags:
  - "#task"
  - "#phase/completed"


exit_criteria_met: false
strategy_id: NULL
initiative_id: MIMIR-I-0017
---

# Extend PrintService with campaign export methods

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative

[[MIMIR-I-0017]] - Campaign Document Export

## Objective

Extend the Rust PrintService in mimir-dm-print to add methods for rendering campaign documents to PDF, both individually and as a combined document.

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [ ] Add `render_campaign_document(file_path, campaign_name) -> Result<Vec<u8>>` method:
  - Reads markdown file from disk
  - Uses markdown module to parse and convert
  - Renders using `campaign/document.typ` template
  - Returns PDF bytes
- [ ] Add `render_campaign_combined(documents, campaign_name) -> Result<Vec<u8>>` method:
  - Accepts array of document info structs
  - Reads and parses all markdown files
  - Renders using `campaign/combined.typ` template
  - Returns single combined PDF
- [ ] Export markdown module from lib.rs
- [ ] Add unit tests for both methods

## Implementation Notes

### Files to Modify
- `crates/mimir-dm-print/src/service.rs` - Add new methods
- `crates/mimir-dm-print/src/lib.rs` - Export markdown module

### Method Signatures
```rust
pub struct CampaignDocumentInfo {
    pub file_path: String,
    pub title: String,
    pub document_type: String,
    pub level: String,  // campaign, module, session, handout
}

impl PrintService {
    pub fn render_campaign_document(
        &self,
        file_path: &str,
        campaign_name: &str,
    ) -> Result<Vec<u8>, PrintError>;

    pub fn render_campaign_combined(
        &self,
        documents: Vec<CampaignDocumentInfo>,
        campaign_name: &str,
    ) -> Result<Vec<u8>, PrintError>;
}
```

### Dependencies
- MIMIR-T-0141 (markdown module)
- MIMIR-T-0142 (Typst templates)

## Status Updates

*To be added during implementation*