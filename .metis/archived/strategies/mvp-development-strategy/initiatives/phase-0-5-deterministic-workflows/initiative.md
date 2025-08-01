---
id: phase-0-5-campaign-creation-wizard
level: initiative
title: "Phase 0.5: Campaign Creation Wizard"
created_at: 2025-07-30T00:52:28.680079+00:00
updated_at: 2025-07-30T00:52:28.680079+00:00
parent: mvp-development-strategy
blocked_by: []
archived: true

tags:
  - "#initiative"
  - "#phase/discovery"


exit_criteria_met: false
estimated_complexity: M
---

# Phase 0.5: Campaign Creation Wizard Initiative

## Context

With our data foundation in place from Phase 0, we can now build our first deterministic workflow. This phase focuses on implementing the core data model and programmatic API for campaign lifecycle management based on the proven campaign design framework.

Campaign management is the perfect starting workflow because:
- It validates our data model with real-world usage
- Provides immediate value to DMs throughout the campaign lifecycle
- Establishes patterns for all future workflows
- Implements battle-tested methodologies from experienced DMs
- Creates a foundation that supports the entire campaign journey

The key insights from our campaign design framework are:
1. **Just-in-time preparation** - Build only what's needed for immediate play
2. **Session-length driven design** - 1hr vs 2hr vs 3hr vs 4hr sessions fundamentally change structure
3. **The Big Three** - Big Bad, Stakes, and Personal Connection drive campaigns
4. **Module-based organization** - Sessions group into story arcs (modules)
5. **Pragmatic data design** - Denormalize fixed data, use filesystem for content

## Goals & Non-Goals

**Goals:**
- Design and implement core data model for campaign lifecycle management
- Create database schema for campaigns, modules, sessions, and documents
- Build programmatic API for campaign operations and workflows
- Implement session-length driven structure (1hr, 2hr, 3hr, 4hr+)
- Model "Big Three" framework: Big Bad, Stakes, and Personal Connection
- Create Six Truths as denormalized campaign fields
- Build module system for organizing story arcs
- Establish filesystem storage for document content
- Implement pragmatic denormalized design for fixed campaign data
- Create query interfaces for common DM workflows

**Non-Goals:**
- UI/TUI implementation (future phase)
- Push notifications or external integrations
- Character sheet generation (future phase)
- NPC stat block generation (future phase)
- Combat automation
- Real-time collaborative features
- Online/multiplayer networking
- AI content generation (future phase)
- Complex workflow automation (keep it simple)

## Detailed Design

### Core Domain Models

```rust
// Campaign Management - Core Data Model with denormalized fields
pub struct Campaign {
    id: u32,
    name: String,
    pitch_file_path: Option<String>,   // Points to markdown file
    
    // Session Configuration
    typical_session_length_minutes: u32,  // 60, 120, 180, or 240+
    target_prep_time_minutes: Option<u32>,
    
    // The Big Three (denormalized)
    big_bad: Option<String>,           // Primary antagonist/threat
    stakes: Option<String>,            // What happens if heroes fail
    personal_connection: Option<String>, // Why it matters to PCs
    
    // Six Truths (denormalized - always 6, campaign-specific)
    truth_1: Option<String>,
    truth_2: Option<String>,
    truth_3: Option<String>,
    truth_4: Option<String>,
    truth_5: Option<String>,
    truth_6: Option<String>,
    
    // Campaign Metadata
    genre: Option<String>,
    tone: Option<String>,
    expected_sessions: String,      // 'short', 'medium', 'long', 'ongoing'
    campaign_status: String,        // 'planning', 'active', 'hiatus', 'completed'
    
    // Combat/Exploration/Social pillars (1-5 rating)
    combat_rating: Option<u8>,
    exploration_rating: Option<u8>,
    social_rating: Option<u8>,
    
    // World info (denormalized)
    world_name: Option<String>,
    world_concept: Option<String>,
    starting_region_name: Option<String>,
    starting_settlement: Option<String>,
    
    // Key dates
    session_zero_date: Option<DateTime<Utc>>,
    first_session_date: Option<DateTime<Utc>>,
    last_session_date: Option<DateTime<Utc>>,
    
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

// Modules represent story arcs that unfold over multiple sessions
pub struct Module {
    id: u32,
    campaign_id: u32,
    module_number: u32,
    name: String,
    
    // Module overview (denormalized for quick access)
    hook_summary: Option<String>,
    expected_sessions: Option<u32>,
    level_range: Option<String>,    // e.g., "1-3"
    
    // Status tracking
    status: String,                 // 'planning', 'ready', 'active', 'completed'
    started_session_id: Option<u32>,
    completed_session_id: Option<u32>,
    
    // File references
    overview_file_path: Option<String>,
    
    created_at: DateTime<Utc>,
}

// Sessions - the core unit of play
pub struct Session {
    id: u32,
    campaign_id: u32,
    session_number: u32,
    session_date: Option<DateTime<Utc>>,
    
    // Prep tracking
    prep_started_at: Option<DateTime<Utc>>,
    prep_completed_at: Option<DateTime<Utc>>,
    prep_file_path: Option<String>,
    
    // Session execution
    actual_duration_minutes: Option<u32>,
    players_present: Option<String>,    // Comma-separated list
    recap_file_path: Option<String>,
    
    // Session design tracking
    designed_ending: Option<String>,
    actual_ending_type: Option<String>, // 'cliffhanger', 'natural', 'energy', 'time'
    ending_satisfaction: Option<u8>,    // 1-5 rating
    
    // Quick summary data
    strong_start_summary: Option<String>,
    major_events_summary: Option<String>,
    cliffhanger_summary: Option<String>,
    npcs_introduced: Option<String>,    // Comma-separated list
    
    created_at: DateTime<Utc>,
}

// Players are real people
pub struct Player {
    id: u32,
    name: String,
    email: Option<String>,
    chat_provider: Option<String>, // Where to contact them
    chat_handle: Option<String>, // On which identifier
    preferred_pronouns: Option<String>,
}

// Characters belong to players and campaigns
pub struct Character {
    id: u32,
    campaign_id: u32,
    player_id: u32,
    
    // Basic info
    character_name: String,
    race: Option<String>,
    class: Option<String>,
    level: u8,
    
    // Integration tracking
    backstory_file_path: Option<String>,
    personal_goal: Option<String>,
    integration_status: Option<String>, // 'pending', 'partial', 'complete'
    spotlight_count: u32,               // Track spotlight balance
    
    // Status
    active: bool,
    death_date: Option<DateTime<Utc>>,
    retirement_reason: Option<String>,
    
    created_at: DateTime<Utc>,
}

// Character connections (many-to-many)
pub struct CharacterConnection {
    character_id_1: u32,
    character_id_2: u32,
    connection_type: String,        // 'family', 'rival', 'debt', etc.
    description: Option<String>,
}

// NPCs - track all introduced NPCs
pub struct NPC {
    id: u32,
    campaign_id: u32,
    name: String,
    role: Option<String>,           // 'villain', 'ally', 'merchant', etc.
    
    // Quick reference data
    one_line_description: Option<String>,
    location_name: Option<String>,  // Simple string, not normalized
    faction_name: Option<String>,   // Simple string, not normalized
    
    // Recurring status
    is_recurring: bool,
    promoted_to_recurring_date: Option<DateTime<Utc>>,
    
    // Appearance tracking
    first_appearance_session_id: Option<u32>,
    last_appearance_session_id: Option<u32>,
    appearance_count: u32,
    
    // Status
    alive: bool,
    disposition_to_party: Option<String>, // 'hostile', 'neutral', 'friendly', 'allied'
    stat_block_file_path: Option<String>,
    notes_file_path: Option<String>,
    
    created_at: DateTime<Utc>,
}

// Locations - only major ones worth tracking
pub struct Location {
    id: u32,
    campaign_id: u32,
    name: String,
    location_type: Option<String>,  // 'settlement', 'dungeon', 'landmark'
    region_name: Option<String>,    // Simple string reference
    
    // Quick access info
    one_line_description: Option<String>,
    danger_level: Option<String>,
    
    // Development status
    development_status: Option<String>, // 'concept', 'outlined', 'mapped', 'detailed'
    map_file_path: Option<String>,
    description_file_path: Option<String>,
    
    // Discovery tracking
    discovered_session_id: Option<u32>,
    visited_count: u32,
    
    created_at: DateTime<Utc>,
}

// Document Management - tracks files on filesystem
pub struct Document {
    id: u32,
    campaign_id: u32,
    
    // Document identification
    doc_type: String,               // 'session_prep', 'recap', 'npc_sheet', etc.
    doc_name: String,
    file_path: String,              // Relative to campaign root
    
    // Relationship tracking (nullable - use what applies)
    session_id: Option<u32>,
    module_id: Option<u32>,
    character_id: Option<u32>,
    npc_id: Option<u32>,
    location_id: Option<u32>,
    
    // Status tracking
    status: Option<String>,         // 'draft', 'active', 'archived'
    last_modified: Option<DateTime<Utc>>,
    word_count: Option<u32>,
    
    // Metadata
    tags: Option<String>,           // Comma-separated for simplicity
    
    created_at: DateTime<Utc>,
}

// Simple document templates
pub struct DocumentTemplate {
    id: u32,
    template_name: String,
    doc_type: String,
    template_file_path: String,
    is_default: bool,
}

// Track what needs to be done
pub struct CampaignTask {
    id: u32,
    campaign_id: u32,
    task_description: String,
    due_date: Option<DateTime<Utc>>,
    priority: Option<String>,       // 'urgent', 'important', 'backlog'
    completed: bool,
    completed_date: Option<DateTime<Utc>>,
    
    // What it's related to
    related_session_id: Option<u32>,
    related_module_id: Option<u32>,
    
    created_at: DateTime<Utc>,
}

// Simple plot thread tracking
pub struct PlotThread {
    id: u32,
    campaign_id: u32,
    thread_name: String,
    description: Option<String>,
    
    // Status tracking
    status: Option<String>,         // 'background', 'active', 'resolved', 'abandoned'
    introduced_session_id: Option<u32>,
    resolved_session_id: Option<u32>,
    
    // Urgency
    is_ticking_clock: bool,
    deadline_session_number: Option<u32>,
}


```

### Database Schema Considerations

The data model will be implemented using Diesel ORM with the following key relationships:

1. **Campaign** is the root aggregate with denormalized Big Three and Six Truths
2. **Module** represents story arcs that play out over multiple sessions
3. **Session** is the core unit of play
4. **Player** can have multiple characters across campaigns
5. **Character** belongs to one player and one campaign
6. **Document** tracks metadata with file paths to filesystem storage
7. **Location** and **NPC** use simple strings for regions/factions (not normalized)
8. **PlotThread** tracks ongoing story elements

### Filesystem Storage Strategy

Documents are stored on the filesystem with database tracking metadata:

```
campaigns/
└── {campaign_name}/
    ├── campaign_bible.md
    ├── pitch.md
    ├── session_zero/
    │   ├── agenda.md
    │   ├── safety_tools.md
    │   └── character_creation_guide.md
    ├── world/
    │   ├── overview.md
    │   ├── pantheon.md
    │   └── timeline.md
    ├── regions/
    │   └── {region_name}/
    │       ├── overview.md
    │       ├── map.png
    │       └── settlements/
    ├── modules/
    │   └── {module_number}_{module_name}/
    │       ├── overview.md
    │       ├── npcs/
    │       └── locations/
    ├── sessions/
    │   └── session_{number}/
    │       ├── prep.md
    │       ├── recap.md
    │       └── handouts/
    ├── characters/
    │   └── {character_name}/
    │       ├── backstory.md
    │       ├── notes.md
    │       └── art/
    ├── npcs/
    │   ├── recurring/
    │   │   └── {npc_name}/
    │   │       ├── notes.md
    │   │       ├── stat_block.md
    │   │       └── art/
    │   └── {npc_name}.md  # One-off NPCs get single files
    └── resources/
        ├── maps/
        ├── handouts/
        └── references/
```

### Design Principles

1. **Denormalized Fixed Data**: The Big Three, Six Truths, and campaign settings stored directly in campaigns table
2. **ENUMs in Code, Strings in DB**: Type safety in application, flexibility in database
3. **File-Based Documents**: Content lives in markdown files, database tracks location and metadata
4. **Pragmatic Relationships**: Only normalize when truly needed for queries
5. **Status Tracking Focus**: Database excels at tracking document state and relationships

### Core Business Logic & Workflows

#### Campaign Creation Workflow

**Phase 1: Campaign Inception (Create Foundation)**
1. **Create Campaign** → Generates folder structure
2. **Define Core Concept** → Create campaign pitch with:
   - Big Three (Big Bad, Stakes, Personal Connection)
   - Campaign theme and tone
   - Expected duration (short/medium/long/mega)
   - Combat/Exploration/Social pillar ratings
   - Six Truths about the world
3. **Initial World Building** → Just enough for start:
   - World concept (1-2 pages)
   - Starting region only
   - Basic pantheon/magic system notes
4. **Plan Session Zero** → Create session zero package:
   - Agenda document
   - Character creation guidelines
   - Safety tools & expectations
   - Integration requirements

**Phase 2: Session Zero & Integration**
1. **Run Session Zero** → Must achieve exit criteria:
   - All players have created characters
   - Inter-character relationships established
   - Characters tied to starting location
   - Group patron/unifying element defined
   - Personal plot hooks for each character
   - Schedule and expectations agreed
2. **Post Session Zero Development** → Based on characters:
   - Starting area map (3-5 adventure sites)
   - Core NPC roster (10-15 from backstories)
   - Initial adventure hooks
   - First module outline

**Phase 3: Launch Campaign**
1. **Create First Module** → Overview, NPCs, locations
2. **Prep Session 1** → Using 8-step process
3. **Run First Session** → Campaign is now active

#### Session Preparation Workflow (The 8 Steps)
1. **Review Characters** - Load current status, goals
2. **Create Strong Start** - Generate/update session prep doc
3. **Outline Scenes** - Add to session prep (3-5 based on session length)
4. **Define Secrets/Clues** - 10 discoverable pieces of information
5. **Develop Locations** - Create location docs as needed
6. **Prepare NPCs** - Create/update NPC files
7. **Choose Monsters** - Link to combat encounters
8. **Select Rewards** - Note in session prep

#### Document Generation Triggers
- **Campaign Creation** → campaign_bible.md, pitch.md templates
- **Session Zero Scheduled** → session_zero/ folder with templates
- **Module Start** → module overview template
- **Session Prep** → session_{n}/prep.md from template
- **NPC Introduction** → Simple record, then file after 2nd appearance
- **Location Discovery** → Location record, details added as developed

#### Module Lifecycle
1. **Planning** → Create module, define hook and expected sessions
2. **Ready** → All prep for first session complete
3. **Active** → Currently being played
4. **Completed** → Final session concluded, outcomes recorded

#### NPC Promotion Logic
- First appearance → Database record only
- Second appearance → Suggest promotion to recurring
- Promotion accepted → Create npc folder with templates
- Role = 'villain'/'patron' → Always suggest promotion

### Key Queries to Support

The system should efficiently support these common DM workflows:

1. **What do I need to prep for next session?**
   - Next session date and number
   - Active module status
   - Pending campaign tasks
   - Active plot threads with deadlines

2. **Which character needs spotlight time?**
   - Characters by spotlight count
   - Session attendance tracking
   - Character integration status

3. **What NPCs are in this location?**
   - NPCs by location name
   - Recurring vs one-off NPCs
   - Disposition to party

4. **Which NPCs should be promoted to recurring?**
   - NPCs with 2+ appearances
   - Important roles (villain, patron)
   - Not yet marked as recurring

5. **What plot threads are active?**
   - Thread status and urgency
   - Sessions since introduction
   - Ticking clock deadlines

## Alternatives Considered

**Generic RPG System**
- Pro: Support multiple game systems
- Con: Loses D&D-specific optimizations
- Rejected: Focus on doing one thing excellently

**Web-Based Forms**
- Pro: Easier UI development, rich interactions
- Con: Violates TUI/local-first principles
- Rejected: Terminal interface is core requirement

**AI-First Approach**
- Pro: More flexible NPC generation
- Con: Unreliable without deterministic base
- Rejected: A solid base will allow for a more deterministic behavior of the agent anyways.



## Implementation Plan

### Phase 1: Database Schema Design (Week 1)

- **Core Tables**
  - Design campaigns table with denormalized Big Three and Six Truths
  - Create modules table for story arc organization
  - Design sessions table with prep tracking and summaries
  - Create players and characters tables with relationships

- **Supporting Tables**
  - Design NPCs table with appearance tracking
  - Create locations table with simple string regions
  - Design documents table for filesystem metadata
  - Create plot_threads and campaign_tasks tables

- **Junction Tables**
  - Design character_connections for PC relationships
  - Create session_attendance for tracking
  - Design proper indexes for performance

### Phase 2: Diesel Implementation (Week 2)

- **Schema Creation**
  - Write Diesel migrations for all tables
  - Implement proper foreign key constraints
  - Create indexes for common query patterns
  - Test migration up/down procedures

- **Model Implementation**
  - Create Rust structs matching database schema
  - Implement Diesel traits for all models
  - Create NewXxx structs for insertions
  - Design update patterns for state changes

- **Filesystem Integration**
  - Design campaign folder structure creation
  - Implement path resolution for documents
  - Create file watcher for external edits
  - Build template file copying system

### Phase 3: Core Services & API (Week 3)

- **Campaign Service**
  - Create campaign with folder structure
  - Update campaign status and metadata
  - Query campaigns by various criteria
  - Handle campaign archival

- **Module & Session Services**
  - Create modules with proper numbering
  - Session creation with prep tracking
  - Update session summaries and outcomes
  - Query upcoming sessions needing prep

- **Content Services**  
  - NPC creation and appearance tracking
  - NPC promotion to recurring status
  - Location discovery tracking
  - Plot thread management

- **Document Service**
  - Create documents from templates
  - Track document relationships
  - Update document metadata
  - Handle file path resolution

### Phase 4: Testing & Polish (Week 4)

- **Data Integrity**
  - Test foreign key constraints
  - Verify denormalized data consistency
  - Test filesystem/database sync
  - Validate status transitions

- **Query Performance**
  - Benchmark common DM queries
  - Optimize with proper indexes
  - Test with realistic data volumes
  - Profile memory usage

- **API Testing**
  - Integration tests for workflows
  - Error handling verification
  - Transaction rollback testing
  - Concurrent access handling

## Testing Strategy

### Data Model Validation
- Campaign stores denormalized Big Three and Six Truths correctly
- Module numbering remains unique within campaigns
- Session numbering increments properly
- Character spotlight tracking works across sessions
- NPC appearance counting triggers promotion suggestions

### Business Logic Testing
- Campaign creation generates proper folder structure
- Module status transitions follow valid paths
- Session prep tracking calculates time correctly
- NPC promotion logic fires at 2+ appearances
- Plot thread urgency calculations work with ticking clocks

### Repository Layer Tests
- CRUD operations work for all entity types
- Denormalized fields update consistently
- String-based regions/factions query correctly
- File path resolution works cross-platform
- Attendance tracking handles missing players

### Service Layer Tests
- CampaignService creates filesystem structure
- ModuleService enforces sequential numbering
- SessionService calculates next session correctly
- NPCService suggests promotions appropriately
- DocumentService resolves paths reliably

### Integration Tests
- Campaign launch workflow (pitch → session zero → first session)
- Session cycle (prep → run → recap → next)
- Module transitions maintain continuity
- NPC lifecycle (introduce → track → promote)
- Document creation from templates

### Query Performance Tests
- "What do I prep next?" returns in <50ms
- "Who needs spotlight?" calculates correctly
- "NPCs by location" handles 100+ NPCs
- "Active plot threads" sorts by urgency
- Bulk operations complete in reasonable time

## Exit Criteria

This initiative will be considered complete when:

1. **Core Data Model Implemented**
   - [ ] Campaigns table with denormalized Big Three and Six Truths
   - [ ] Modules table for story arc organization
   - [ ] Sessions table with prep tracking fields
   - [ ] All supporting tables (NPCs, Locations, Documents, etc.)
   - [ ] Diesel models and migrations completed

2. **Filesystem Integration Working**
   - [ ] Campaign folder structure creation
   - [ ] Document path resolution
   - [ ] Template file system
   - [ ] File metadata tracking in database

3. **Core Services Functional**
   - [ ] CampaignService with CRUD operations
   - [ ] ModuleService managing story arcs
   - [ ] SessionService with prep workflow
   - [ ] NPCService with promotion logic
   - [ ] DocumentService with template support

4. **Key Queries Optimized**
   - [ ] "What to prep next" query
   - [ ] "Character spotlight" query
   - [ ] "NPCs by location" query
   - [ ] "Active plot threads" query
   - [ ] All queries under 100ms

5. **MVP Workflows Complete**
   - [ ] Campaign creation workflow
   - [ ] Session prep and recap cycle
   - [ ] Module progression tracking
   - [ ] NPC appearance tracking
   - [ ] Basic document management