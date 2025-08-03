---
id: campaign-board-view-implementation
level: initiative
title: "Campaign Board View Implementation"
created_at: 2025-08-02T19:44:31.247004+00:00
updated_at: 2025-08-02T19:44:31.247004+00:00
parent: mvp-development-strategy
blocked_by: []
archived: false

tags:
  - "#initiative"
  - "#phase/discovery"


exit_criteria_met: false
estimated_complexity: M
---

# Campaign Board View Implementation Initiative

## Context

When a campaign is selected in Mimir, users need a dedicated workspace for managing campaign content. Currently, there's no integrated view that combines campaign progress tracking, document management, and content authoring. Users need a "board" view that provides:

1. Visual campaign progress tracking (Gantt-style visualization)
2. Document checklist showing what needs to be created or is available
3. Integrated content editor for authoring campaign materials

This is the first of three main boards planned for the application (Campaign Board, Module Board, Session Board). The Campaign Board focuses on high-level campaign management and documentation.

## Goals & Non-Goals

**Goals:**
- Create a dedicated Campaign Board view that activates when a campaign is selected
- Implement a simple Gantt-style visualization for campaign progress tracking
- Build a document checklist sidebar showing template completion status
- Integrate a rich text editor for authoring campaign content
- Enable auto-save functionality for seamless content creation
- Maintain consistency with existing theme system and UI patterns

**Non-Goals:**
- Complex project management features (this is not a full PM tool)
- Real-time collaboration (single-user focus for MVP)
- Module and Session boards (separate initiatives)
- External file format support (focus on native format)
- Version control for documents (future enhancement)

## Detailed Design

### Architecture Overview

The Campaign Board will be implemented as a new route and view within the existing Vue 3 + Tauri architecture:

```
/campaigns/:id/board -> CampaignBoardView.vue
```

### UX Flow and Campaign Stages

The Campaign Board adapts its interface based on the campaign's current stage:

#### 1. Concept Stage (Initial)
- **Primary UI**: Split view - Spark cards on left, Document editor on right
- **Documents to Create**: 
  - Campaign Sparks (index cards for ideas)
  - **Campaign Pitch** (1 page)
  - **Big Three Document** (1 page)
  - **First Adventure Outline** (1-2 pages)
- **Progress Indicator**: "Concept → Session Zero → Integration → Active"
- **Transition Trigger**: "Players Interested" button (after pitch is shared) → moves to Session Zero

#### 2. Session Zero Stage
- **Primary UI**: Document checklist sidebar prominent, editor for active document
- **Documents to Create**:
  - **Starting Scenario** (1-2 pages)
  - **World Primer** (2-3 pages)
  - **Character Guidelines** (1 page)
  - **Table Expectations** (1 page)
  - **Character Integration Forms**
  - **Session Zero Packet** (auto-compiled from above)
- **Progress Indicator**: Shows document completion percentage
- **Transition Trigger**: "Session Zero Complete" button → moves to Integration

#### 3. Integration Stage
- **Primary UI**: Split between document editor and Module Board preview
- **Documents to Create**:
  - **Campaign Bible** (themes, world, rules)
  - **Character Integration Notes** (from Session Zero forms)
  - **Major NPCs Document**
  - **World Events Timeline**
- **New Feature**: MODULE BOARD becomes accessible for first time
- **Progress Indicator**: Shows both document completion and "Ready to Launch"
- **Transition Trigger**: First session runs → moves to Active

#### 4. Active Stage
- **Primary UI**: Campaign overview dashboard
- **Features**: 
  - All documents editable
  - Module Board fully active
  - Session Board accessible
  - Campaign timeline visible
  - Quick stats and progress

### UI Wireframe

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│ [🎭] [▼ My First Campaign]  [➕]                                          [⚙️]  │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  ┌─────────────┐  ┌─────────────────┐  ┌──────────────┐  ┌────────────┐      │
│  │   CONCEPT    │  │  SESSION ZERO   │  │ INTEGRATION  │  │   ACTIVE   │      │
│  │      ●       │→ │                 │→ │              │→ │            │      │
│  └─────────────┘  └─────────────────┘  └──────────────┘  └────────────┘      │
│                                                                                 │
│  Campaign: My First Campaign                                 [Next Stage ▶]     │
│                                                                                 │
├────────────────────────┬────────────────────────────────────────────────────────┤
│                        │                                                        │
│  📋 Documents          │  Campaign Pitch                                        │
│  ─────────────         │  ─────────────────────────────────────────────────    │
│                        │                                                        │
│  Concept Stage (1/4)   │  # Campaign Pitch Template                            │
│  ▓▓▓▓░░░░░░░░░░░░     │                                                        │
│                        │  ## Campaign Name                                      │
│  ✏️ Campaign Pitch     │  [Your Campaign Name]                                 │
│  ⬜ Big Three          │                                                        │
│  ⬜ First Adventure    │  ## The Hook                                          │
│  ⬜ Campaign Sparks    │  What draws the players into your world?              │
│                        │                                                        │
│  ─────────────         │  ## The Stakes                                        │
│  Session Zero (0/6)    │  What happens if the heroes fail?                     │
│  ░░░░░░░░░░░░░░░░     │                                                        │
│                        │  ## The Setting                                       │
│  🔒 Starting Scenario  │  Where and when does this take place?                 │
│  🔒 World Primer       │                                                        │
│  🔒 Character Guide    │  ---                                                  │
│  🔒 Table Expectations │  [Auto-save enabled]                                  │
│  🔒 Integration Forms  │                                                        │
│  🔒 Session Zero Pack  │                                                        │
│                        │                                                        │
│  ─────────────         │                                                        │
│  Integration (0/4)     │                                                        │
│  ░░░░░░░░░░░░░░░░     │                                                        │
│                        │                                                        │
│  🔒 Campaign Bible     │                                                        │
│  🔒 Character Notes    │                                                        │
│  🔒 Major NPCs         │                                                        │
│  🔒 World Timeline     │                                                        │
│                        │                                                        │
└────────────────────────┴────────────────────────────────────────────────────────┘
```

### Component Structure

```
CampaignBoardView.vue
├── CampaignStageProgress.vue (Kanban-style stage indicators)
├── DocumentChecklistSidebar.vue (All documents, locked by stage)
├── DocumentEditor.vue (Tiptap editor for active document)
└── StageTransitionButton.vue (Next Stage control)
```

### State Management

**CampaignBoardState:**
```typescript
interface CampaignBoardState {
  activeCampaign: Campaign | null
  campaignStage: 'concept' | 'session_zero' | 'integration' | 'active'
  documentChecklist: DocumentChecklistItem[]
  activeDocument: DocumentInstance | null
  stageProgress: {
    concept: { documentsCompleted: number, totalDocuments: 4 }
    sessionZero: { documentsCompleted: number, totalDocuments: 6 }
    integration: { documentsCompleted: number, totalDocuments: 4 }
  }
}
```

### Technology Choices

1. **Stage Progress**: Simple CSS flexbox with active state styling
2. **Document Checklist**: List with progress bars and lock icons
3. **Text Editor**: Tiptap with markdown support
4. **Auto-save**: Debounced saves every 2 seconds
5. **Stage Transitions**: Simple validation and state update

### Database Schema

```sql
-- Add to campaigns table
ALTER TABLE campaigns ADD COLUMN stage TEXT DEFAULT 'concept';

-- Document instances (using existing template system)
CREATE TABLE document_instances (
  id INTEGER PRIMARY KEY,
  campaign_id INTEGER NOT NULL,
  template_id TEXT NOT NULL,
  title TEXT NOT NULL,
  content TEXT,
  frontmatter TEXT, -- JSON
  status TEXT NOT NULL DEFAULT 'not_started',
  stage TEXT NOT NULL, -- 'concept', 'session_zero', 'integration', etc
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (campaign_id) REFERENCES campaigns(id),
  FOREIGN KEY (template_id) REFERENCES template_documents(document_id)
);
```

## Alternatives Considered

### Text Editor Alternatives

1. **Monaco Editor (VS Code editor)**
   - **Pros**: Powerful code editing, syntax highlighting, multi-cursor
   - **Cons**: Heavyweight for narrative content, complex integration
   - **Rejected because**: Overkill for D&D campaign content authoring

2. **Quill.js**
   - **Pros**: Simple API, good for basic rich text
   - **Cons**: Less extensible, older architecture
   - **Rejected because**: Limited customization for D&D-specific features

3. **Milkdown**
   - **Pros**: Markdown-first, plugin architecture
   - **Cons**: Less mature, smaller community
   - **Rejected because**: Tiptap has better Vue 3 integration

### Gantt Chart Alternatives

1. **Custom D3.js Implementation**
   - **Pros**: Full control, highly customizable
   - **Cons**: Time-intensive, requires D3 expertise
   - **Rejected because**: Vue-Ganttastic provides sufficient features

2. **Frappe Gantt**
   - **Pros**: Feature-rich, good documentation
   - **Cons**: Not Vue-native, jQuery dependency
   - **Rejected because**: Prefer Vue-native solution

### Document Management Alternatives

1. **File System Based**
   - **Pros**: Direct file access, version control friendly
   - **Cons**: Complex syncing, platform differences
   - **Rejected because**: Database provides better querying and relationships

2. **Single Document View**
   - **Pros**: Simpler implementation
   - **Cons**: Poor overview of campaign documentation status
   - **Rejected because**: Users need to see all document statuses at once

## Implementation Plan

### Phase 1: Foundation (Week 1)
- Create Campaign Board route (`/campaigns/:id/board`)
- Implement `CampaignBoardView.vue` with basic layout
- Create `useCampaignBoardStore` and `useDocumentStore`
- Add database migrations for document_instances and campaign_milestones
- Create Tauri commands for document CRUD operations
- Update campaign selector to navigate to board view

### Phase 2: Document Management (Week 2)
- Build `DocumentChecklistSidebar.vue` component
- Implement document status tracking logic
- Create document instance creation from templates
- Add progress calculation for each document type
- Integrate with existing template system

### Phase 3: Progress Visualization (Week 2-3)
- Install and configure Vue-Ganttastic
- Create `CampaignProgressGantt.vue` component
- Define campaign milestone structure
- Implement milestone calculation based on campaign/module status
- Add milestone management backend

### Phase 4: Text Editor Integration (Week 3-4)
- Install and configure Tiptap editor
- Create `DocumentEditor.vue` with toolbar
- Implement auto-save with debouncing
- Add markdown/WYSIWYG mode toggle
- Support frontmatter editing
- Create custom extensions for D&D content (dice notation, stat blocks)

### Phase 5: Polish & Integration (Week 4-5)
- Implement responsive layout with collapsible sidebar
- Add keyboard shortcuts (Ctrl+S, navigation)
- Create loading states and error handling
- Add document export functionality
- Optimize performance and lazy loading
- Theme integration for all new components

## Testing Strategy

### Unit Testing
- Test document CRUD operations in isolation
- Validate auto-save debouncing logic
- Test milestone calculation algorithms
- Verify template variable substitution

### Integration Testing
- Test navigation flow from campaign selection to board view
- Verify document creation from templates
- Test editor content persistence
- Validate theme switching in all new components

### User Acceptance Testing
- Create a sample campaign and author all document types
- Test workflow from empty campaign to fully documented
- Verify auto-save doesn't lose data
- Test responsive behavior on different screen sizes
- Validate keyboard shortcuts and accessibility

### Performance Testing
- Measure editor performance with large documents (10k+ words)
- Test sidebar responsiveness with many documents
- Verify lazy loading reduces initial load time
- Monitor memory usage during extended editing sessions

### Exit Criteria
- [ ] Campaign board loads when campaign is selected
- [ ] All template document types can be created and edited
- [ ] Auto-save works reliably without data loss
- [ ] Progress visualization accurately reflects campaign state
- [ ] Editor handles D&D content (tables, stat blocks, etc.)
- [ ] UI is responsive and follows existing design patterns
- [ ] All new components support theme switching