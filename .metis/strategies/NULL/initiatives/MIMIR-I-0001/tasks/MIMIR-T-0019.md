---
id: extract-reusable-components-from
level: task
title: "Extract reusable components from large Vue files"
short_code: "MIMIR-T-0019"
created_at: 2025-10-24T11:53:50.191283+00:00
updated_at: 2025-10-24T11:53:50.191283+00:00
parent: MIMIR-I-0001
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/todo"


exit_criteria_met: false
strategy_id: NULL
initiative_id: MIMIR-I-0001
---

# Extract reusable components from large Vue files

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0001]]

## Objective **[REQUIRED]**

Extract reusable child components from large Vue files (>500 lines) to improve maintainability and code organization. This continues the pattern from T-0017 (useCatalog split) and T-0018 (chat store split) by decomposing monolithic files into focused, reusable components while maintaining zero functional changes.

### Technical Debt Impact

- **Current Problems**: 
  - 5 Vue components exceed 500 lines (up to 753 lines)
  - Monolithic components mix multiple concerns in single files
  - Difficult to locate and modify specific UI sections
  - Limited component reusability across the application
  - Violates single responsibility principle
  
- **Benefits of Fixing**: 
  - All component files under 500 lines (target <300 for new components)
  - Improved code discoverability and navigation
  - Enhanced component reusability
  - Easier testing of isolated UI sections
  - Follows established patterns from previous refactoring tasks
  
- **Risk Assessment**: 
  - Low risk - components are well-tested through UI usage
  - No functional changes, pure refactoring
  - Vue's reactivity system handles component composition well
  - TypeScript will catch any prop/emit interface issues

## Acceptance Criteria **[REQUIRED]**

- [ ] Identified and extracted reusable components from files >500 lines
- [ ] All parent components reduced to <500 lines (target <400 where possible)
- [ ] All extracted child components are <300 lines
- [ ] Clear component boundaries with well-defined props and emits
- [ ] No functional changes - application works identically
- [ ] Application builds successfully with no TypeScript errors
- [ ] All components properly typed with TypeScript interfaces



## Implementation Notes

### Target Files (>500 lines)

Priority files identified for component extraction:

1. **StageLandingView.vue** (753 lines)
   - Extract: StageHeader, StageTransitionCard, ModulesList, ModuleCreateModal
   - Target: <400 lines after extraction

2. **LogViewerWindow.vue** (715 lines)
   - Extract: LogFilters, LogEntry, LogToolbar
   - Target: <400 lines after extraction

3. **ModuleDocumentSidebar.vue** (685 lines)
   - Extract: DocumentTree, DocumentItem, DocumentActions
   - Target: <400 lines after extraction

4. **CampaignManagementModal.vue** (597 lines)
   - Extract: CampaignForm, CampaignList, CampaignActions
   - Target: <400 lines after extraction

5. **ToolConfirmation.vue** (566 lines)
   - Extract: FileEditPreview, FileWritePreview, ConfirmationActions
   - Target: <400 lines after extraction

### Technical Approach

Follow the same pattern as T-0017 and T-0018:

1. **Analyze component structure**
   - Identify logical sections (header, list, form, actions, etc.)
   - Find repeated patterns suitable for extraction
   - Map props and emits needed for each extracted component

2. **Create child components**
   - Place in same directory as parent (e.g., `components/LogViewer/`)
   - Define clear TypeScript interfaces for props
   - Keep components focused on single responsibility
   - Target <300 lines per child component

3. **Update parent component**
   - Import and use child components
   - Pass data via props, handle events via emits
   - Maintain exact same functionality
   - Verify reactivity still works correctly

4. **Component naming convention**
   - Parent: `ParentName.vue`
   - Children: `ParentNameSection.vue` (e.g., `LogViewerFilters.vue`)
   - Keep related components together in subdirectory

### Example Structure

Before:
```
components/
└── LogViewerWindow.vue (715 lines)
```

After:
```
components/LogViewer/
├── LogViewerWindow.vue (<400 lines)
├── LogViewerFilters.vue (<200 lines)
├── LogViewerEntry.vue (<150 lines)
└── LogViewerToolbar.vue (<100 lines)
```

### Dependencies
- Follows patterns from MIMIR-T-0017 (useCatalog split) and MIMIR-T-0018 (chat store split)
- Requires understanding of Vue 3 composition API and component communication
- Must preserve all reactive state and event handling

### Risk Considerations
- Need to maintain props/emits contracts carefully
- Must preserve all reactive dependencies
- Event bubbling may need adjustment
- Scoped styles may need refinement
- Must test all user interactions still work

## Status Updates **[REQUIRED]**

*To be added during implementation*