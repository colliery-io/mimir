---
id: visual-display-system
level: initiative
title: "Visual Display System"
short_code: "MIMIR-I-0015"
created_at: 2025-12-06T16:02:36.548959+00:00
updated_at: 2025-12-06T16:02:36.548959+00:00
parent: 
blocked_by: []
archived: false

tags:
  - "#initiative"
  - "#phase/discovery"


exit_criteria_met: false
estimated_complexity: XL
strategy_id: NULL
initiative_id: visual-display-system
---

# Visual Display System Initiative

## Context

While Mimir handles campaign data and the Physical Print initiative addresses table materials, many groups benefit from visual displays during play. A second monitor or TV showing battle maps, exploration scenes, or dungeon layouts adds immersion beyond pure "theater of the mind."

This initiative introduces a visual display system - a separate window that can be dragged to an external monitor to show players maps, scenes, and tactical grids. Unlike the print system which renders existing data, this requires **new content types** (maps, scenes) and **interactive features** (tokens, fog of war).

**Priority**: This is second priority after Physical Print (MIMIR-I-0014) due to larger scope.

## Goals & Non-Goals

**Goals:**
- Display window that can be dragged to external monitor (extended desktop)
- Battle maps with grid overlay for tactical combat
- Exploration/dungeon maps for navigation and discovery
- World/region maps for travel and context
- Interactive features: tokens, fog of war, pan/zoom
- Integration with campaign data (show encounter monsters, player character tokens)

**Non-Goals:**
- Full VTT feature parity (Roll20, Foundry) - this is a display tool, not a game system
- Remote/networked play - this is for in-person table use
- Built-in map creation tools - maps imported from external sources initially
- Dice rolling or mechanical automation on the display
- Video/animation support in v1

## New Content Types Required

This initiative introduces new data models not currently in Mimir:

### Maps
- **Battle Map**: Grid-based tactical map with terrain, obstacles
- **Dungeon Map**: Multi-room exploration map with connectable areas
- **World Map**: Region/continent scale map for travel

### Map Assets
- **Tokens**: Character and monster representations (images or icons)
- **Markers**: Points of interest, notes, labels
- **Fog of War**: Hidden/revealed area tracking

## Use Cases

### Use Case 1: Combat Encounter Display
- **Actor**: DM
- **Scenario**: DM starts encounter → Opens display window on TV → Loads battle map → Places monster tokens → Players see tactical layout → DM reveals fog as combat progresses
- **Expected Outcome**: Players have clear view of battlefield on shared display

### Use Case 2: Dungeon Exploration
- **Actor**: DM
- **Scenario**: Party enters dungeon → DM loads dungeon map → Reveals rooms as party explores → Moves party token through corridors → Players track their progress visually
- **Expected Outcome**: Immersive exploration without full theater-of-mind burden

### Use Case 3: World Travel
- **Actor**: DM
- **Scenario**: Party plans travel → DM shows world map → Highlights route → Points out landmarks and dangers → Players understand geography
- **Expected Outcome**: Clear sense of world scale and travel context

## Architecture

### Overview

```
┌─────────────────────────────────────────────────────────────┐
│  Main Mimir Window (DM Screen)                              │
│  ┌─────────────────────┐  ┌─────────────────────┐          │
│  │  Campaign/Session   │  │  Display Control    │          │
│  │  Management         │  │  Panel              │          │
│  │                     │  │  - Map selection    │          │
│  │                     │  │  - Token controls   │          │
│  │                     │  │  - Fog of war       │          │
│  │                     │  │  - Reveal/hide      │          │
│  └─────────────────────┘  └─────────────────────┘          │
└─────────────────────────────────────────────────────────────┘
                              │
                              │ IPC (state sync)
                              ▼
┌─────────────────────────────────────────────────────────────┐
│  Player Display Window (TV/External Monitor)                │
│  ┌─────────────────────────────────────────────────────┐   │
│  │                                                      │   │
│  │              Battle Map / Scene View                 │   │
│  │                                                      │   │
│  │    [Token] [Token]                                   │   │
│  │              ████████ (fog)                          │   │
│  │    [Token]   ████████                                │   │
│  │                                                      │   │
│  └─────────────────────────────────────────────────────┘   │
│  (No controls - display only)                              │
└─────────────────────────────────────────────────────────────┘
```

### Tech Stack

- **Canvas Rendering**: HTML5 Canvas or WebGL for map/token rendering
- **Multi-Window**: Tauri's multi-window support for display window
- **State Sync**: IPC between main window and display window
- **Image Handling**: Support for common map image formats (PNG, JPG, WebP)
- **Grid System**: Configurable grid overlay (square, hex)

## Detailed Design

### Data Models

```rust
// Map types
pub struct Map {
    pub id: i32,
    pub campaign_id: i32,
    pub name: String,
    pub map_type: MapType,  // Battle, Dungeon, World
    pub image_path: String,
    pub width_px: u32,
    pub height_px: u32,
    pub grid_size: Option<u32>,  // pixels per grid square
    pub grid_type: GridType,     // Square, Hex, None
}

pub struct Token {
    pub id: i32,
    pub map_id: i32,
    pub name: String,
    pub image_path: Option<String>,
    pub icon: Option<String>,    // Fallback icon if no image
    pub size: TokenSize,         // Tiny, Small, Medium, Large, Huge, Gargantuan
    pub x: f32,                  // Grid position
    pub y: f32,
    pub visible_to_players: bool,
}

pub struct FogOfWar {
    pub map_id: i32,
    pub revealed_areas: Vec<RevealedArea>,  // Polygons or grid cells
}
```

### Display Window

The player display window is a separate Tauri window:
- No menu bar or controls (clean display)
- Receives state updates via IPC from main window
- Renders current map, visible tokens, revealed fog
- Supports pan/zoom (DM controlled)

### DM Control Panel

In the main window, a control panel allows:
- Select active map
- Place/move tokens (drag & drop)
- Toggle token visibility
- Reveal/hide fog areas (paint or polygon)
- Pan/zoom control for display
- Quick scene transitions

## Implementation Plan

### Phase 1: Foundation & Multi-Window
- Database models for maps (no tokens yet)
- Map import (image + grid configuration)
- Basic display window (shows static map image)
- IPC setup between main and display windows

### Phase 2: Grid & Navigation
- Grid overlay rendering (square, hex)
- Pan/zoom controls
- Map scaling to fit display
- Basic DM control panel

### Phase 3: Tokens
- Token data model
- Token placement and movement
- Token visibility toggle
- Character/monster token integration (pull from campaign data)

### Phase 4: Fog of War
- Fog overlay rendering
- Reveal tools (rectangle, polygon, brush)
- Fog state persistence
- Hide/reveal animations

### Phase 5: Polish & Integration
- Scene/map quick-switch
- Encounter integration (auto-place monsters from encounter)
- Token status indicators (conditions, HP)
- Performance optimization for large maps

### Future Considerations (Not in v1)
- Map creation/editing tools
- Animated effects (spell templates)
- Sound/ambiance triggers
- Dynamic lighting