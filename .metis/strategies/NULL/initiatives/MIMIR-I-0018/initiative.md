---
id: module-play-mode-dm-screen
level: initiative
title: "Module Play Mode (DM Screen)"
short_code: "MIMIR-I-0018"
created_at: 2025-12-16T16:12:33.980954+00:00
updated_at: 2025-12-16T16:23:55.800754+00:00
parent: MIMIR-V-0001
blocked_by: []
archived: false

tags:
  - "#initiative"
  - "#phase/active"


exit_criteria_met: false
estimated_complexity: L
strategy_id: NULL
initiative_id: module-play-mode-dm-screen
---

# Module Play Mode (DM Screen) Initiative

## Context

Currently there's a blurred line between campaign **generation** (prep work) and campaign **execution** (running the game). At the module level, all the information is correct but the flow feels off - the UI is optimized for creating/organizing content, not for surfacing/using it during play.

DMs need a dedicated "play mode" that acts as a digital DM screen - purpose-built for running the game, with quick access to NPCs, monsters, locations, and session notes without the noise of prep-oriented UI.

## Goals & Non-Goals

**Goals:**
- Create a dedicated Play Mode view for modules (`/campaigns/:id/modules/:id/play`)
- Surface module content in a play-optimized layout (NPC cards, stat blocks, locations)
- Enable live session note-taking without leaving the play view
- Provide quick reference to module overview and key plot points
- Clear visual distinction between "prep mode" and "play mode"

**Non-Goals:**
- Full VTT functionality (maps, tokens, fog of war)
- Player-facing shared screen (this is DM-only)
- Real-time multiplayer/sync
- Dice rolling (for now)

## Use Cases

### UC-1: Enter Play Mode
- **Actor**: DM
- **Scenario**: DM is about to run a session, navigates to module, clicks "Play Mode" button
- **Expected Outcome**: View transitions to play-optimized layout with quick access panels

### UC-2: Reference NPC During Play
- **Actor**: DM
- **Scenario**: Player asks about an NPC, DM clicks NPC in sidebar, card expands with details
- **Expected Outcome**: NPC card shows personality, voice, secrets, goals - all at a glance

### UC-3: Run Combat Encounter
- **Actor**: DM
- **Scenario**: Combat starts, DM needs monster stat blocks
- **Expected Outcome**: Monster stats accessible in sidebar, can be expanded/collapsed quickly

### UC-4: Take Session Notes
- **Actor**: DM
- **Scenario**: Something significant happens, DM needs to record it
- **Expected Outcome**: Session notes panel allows live editing without leaving play mode

### UC-5: Check Plot Points
- **Actor**: DM
- **Scenario**: DM needs to remember what secrets to reveal or hooks to plant
- **Expected Outcome**: Module overview/secrets section is collapsible but accessible

## UI/UX Design

### Layout Concept

```
+------------------------------------------------------------------+
|  [< Back to Module]              MODULE NAME           [End Session] |
+------------------------------------------------------------------+
|                    |                                              |
|  QUICK ACCESS      |              MAIN AREA                       |
|  SIDEBAR           |                                              |
|                    |   +------------------------------------+     |
|  [NPCs]            |   |                                    |     |
|   - Gundren        |   |        SESSION NOTES               |     |
|   - Sildar         |   |        (live editing)              |     |
|   - Klarg          |   |                                    |     |
|                    |   +------------------------------------+     |
|  [Monsters]        |                                              |
|   - Goblin (4)     |   +------------------------------------+     |
|   - Bugbear        |   |                                    |     |
|                    |   |     EXPANDED CARD AREA             |     |
|  [Locations]       |   |     (NPC/Monster when clicked)     |     |
|   - Cave Entrance  |   |                                    |     |
|   - Wolf Den       |   +------------------------------------+     |
|                    |                                              |
|  [Plot Points]     |                                              |
|   - Secret 1       |                                              |
|   - Hook 2         |                                              |
|                    |                                              |
+------------------------------------------------------------------+
```

### Key UI Elements

1. **Quick Access Sidebar** (left, collapsible)
   - Grouped by type: NPCs, Monsters, Locations, Plot Points
   - Click to expand card in main area
   - Visual indicators for "used this session"

2. **Session Notes Panel** (main area, always visible)
   - Live markdown editing
   - Auto-saves
   - Timestamped entries option

3. **Card Expansion Area** (main area, below notes)
   - Shows full NPC card / stat block when selected
   - Can pin multiple cards
   - Print button for quick PDF

4. **Module Overview** (collapsible header or modal)
   - Key plot points, secrets, hooks
   - "What the players know" vs "What they don't"

### Visual Distinction
- Different color scheme/accent for play mode (darker? warmer?)
- Clear "PLAY MODE" indicator
- Minimal chrome - focus on content

## Alternatives Considered

1. **Toggle within existing module view**
   - Rejected: Too easy to accidentally switch modes, unclear mental model
   
2. **Floating overlay/panel**
   - Rejected: Competes with existing UI, feels bolted-on rather than purposeful

3. **Separate "Session" entity with its own view**
   - Considered but deferred: Adds data model complexity, play mode can work without it initially

## Implementation Plan

### Phase 1: Foundation
- Create route `/campaigns/:id/modules/:id/play`
- Basic layout with sidebar + main area
- Navigation between prep and play modes

### Phase 2: Quick Access Sidebar + Monster Tagging
- Add `module_monsters` table (module_id, monster_name, source, quantity, encounter_tag)
- UI to add monsters from catalog to module with optional encounter grouping
- Load module NPCs, monsters, locations into sidebar
- Group monsters by encounter tag in sidebar
- Expandable/collapsible groups
- Click to select

### Phase 3: Card Display
- Render NPC cards and monster stat blocks in main area
- Reuse existing print templates/components where possible
- Pin/unpin functionality

### Phase 4: Session Notes Integration
- Embed session notes editor in play mode
- Auto-save functionality
- Link to current session record

### Phase 5: Polish
- Visual theming for play mode
- Keyboard shortcuts
- "Used this session" tracking
- Module overview quick reference