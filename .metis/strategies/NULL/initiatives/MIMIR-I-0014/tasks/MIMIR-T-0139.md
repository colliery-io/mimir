---
id: add-print-ui-integration-and
level: task
title: "Add print UI integration and preview window"
short_code: "MIMIR-T-0139"
created_at: 2025-12-15T02:16:37.372628+00:00
updated_at: 2025-12-15T15:59:08.870096+00:00
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

# Add print UI integration and preview window

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0014]]

## Objective

Build the unified print UI experience: preview window for viewing PDFs before printing, print settings in app preferences, and consistent "Print" buttons/menus across all relevant views.

## Acceptance Criteria

## Acceptance Criteria

- [ ] PDF preview component that displays generated PDFs
- [ ] Preview window with page navigation (prev/next for multi-page)
- [ ] "Save PDF" button to save to file system
- [ ] "Print" button to send to system print dialog
- [ ] Print settings in Settings view (paper size, default save location)
- [ ] Consistent print button/dropdown in character, catalog, and session views
- [ ] Template selector when multiple templates available for an entity
- [ ] Loading state while PDF generates
- [ ] Error handling with user-friendly messages

## Implementation Notes

### Preview Component

```vue
<template>
  <div class="pdf-preview">
    <div class="preview-toolbar">
      <button @click="prevPage" :disabled="currentPage <= 1">Prev</button>
      <span>Page {{ currentPage }} of {{ totalPages }}</span>
      <button @click="nextPage" :disabled="currentPage >= totalPages">Next</button>
    </div>
    <div class="preview-canvas">
      <!-- Render PDF page here -->
    </div>
    <div class="preview-actions">
      <button @click="savePdf">Save PDF</button>
      <button @click="print">Print</button>
    </div>
  </div>
</template>
```

### PDF Rendering Options

1. **pdf.js**: Mozilla's PDF renderer (heavy but full-featured)
2. **Native**: Use `<embed>` or `<object>` with PDF blob URL
3. **Image conversion**: Convert PDF pages to images server-side

Recommend starting with native embed for simplicity.

### Print Integration Points

| View | Print Options |
|------|---------------|
| Character Sheet | Sheet, Summary, Spell Cards |
| Spell Catalog | Single Card, Batch Cards |
| Monster Catalog | Stat Block, Card |
| Session | Prep Sheet, NPC Cards |

### Settings

```typescript
interface PrintSettings {
  paperSize: 'letter' | 'a4';
  defaultSaveLocation: string;
  cardsPerPage: 9 | 6 | 4;  // For multi-up layouts
}
```

### Dependencies

- All template tasks (T-0133 through T-0138) should be complete
- Tauri file dialog for save location

## Status Updates

*To be added during implementation*