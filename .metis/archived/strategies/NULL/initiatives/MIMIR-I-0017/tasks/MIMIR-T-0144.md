---
id: add-tauri-commands-and-frontend
level: task
title: "Add Tauri commands and frontend integration for document export"
short_code: "MIMIR-T-0144"
created_at: 2025-12-16T03:23:40.933652+00:00
updated_at: 2025-12-16T03:55:00.422027+00:00
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

# Add Tauri commands and frontend integration for document export

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative

[[MIMIR-I-0017]] - Campaign Document Export

## Objective

Wire up the complete export functionality from UI to backend: add Tauri commands, extend frontend PrintService, and add export buttons to DocumentEditor and CampaignBoardView.

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

### Backend (Tauri Commands)
- [ ] Add `export_campaign_document(document_id) -> PrintResult` command
- [ ] Add `export_campaign_documents(campaign_id) -> PrintResult` command
- [ ] Register both commands in main.rs

### Frontend (PrintService.ts)
- [ ] Add `exportCampaignDocument(documentId): Promise<PrintResult>` method
- [ ] Add `exportCampaignDocuments(campaignId): Promise<PrintResult>` method

### UI Integration
- [ ] Add "Export PDF" button to DocumentEditor.vue toolbar
- [ ] Add "Export All as PDF" button to CampaignBoardView.vue
- [ ] Show loading state during export
- [ ] Trigger save dialog on completion

## Implementation Notes

### Files to Modify
- `crates/mimir-dm/src/commands/print/mod.rs` - Add Tauri commands
- `crates/mimir-dm/src/main.rs` - Register commands
- `crates/mimir-dm/frontend/src/services/PrintService.ts` - Add methods
- `crates/mimir-dm/frontend/src/features/campaigns/components/DocumentEditor.vue` - Add button
- `crates/mimir-dm/frontend/src/features/campaigns/views/CampaignBoardView.vue` - Add button

### Tauri Command Signatures
```rust
#[tauri::command]
pub async fn export_campaign_document(
    state: State<'_, AppState>,
    document_id: i32,
) -> Result<ApiResponse<PrintResult>, String>;

#[tauri::command]
pub async fn export_campaign_documents(
    state: State<'_, AppState>,
    campaign_id: i32,
) -> Result<ApiResponse<PrintResult>, String>;
```

### Dependencies
- MIMIR-T-0143 (PrintService methods)

## Status Updates

*To be added during implementation*