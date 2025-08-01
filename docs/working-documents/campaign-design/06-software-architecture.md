# D&D Campaign Management System Architecture

## Design Philosophy
- **Denormalize** fixed-size, campaign-specific data (like the 6 truths)
- **Normalize** only when dealing with variable quantities or many-to-many relationships
- **Filesystem** for actual document content, database for metadata and relationships

## Database Schema Design

### Core Campaign Table
```sql
CREATE TABLE campaigns (
    id INTEGER PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    pitch_file_path VARCHAR(500), -- Points to markdown file
    
    -- Session Configuration
    typical_session_length_minutes INTEGER DEFAULT 240, -- 4 hours default
    target_prep_time_minutes INTEGER, -- Scales with session length
    
    -- The Big Three (denormalized)
    big_bad VARCHAR(500),
    stakes VARCHAR(500),
    personal_connection VARCHAR(500),
    
    -- Six Truths (denormalized - always 6, campaign-specific)
    truth_1 TEXT,
    truth_2 TEXT,
    truth_3 TEXT,
    truth_4 TEXT,
    truth_5 TEXT,
    truth_6 TEXT,
    
    -- Campaign Metadata
    genre VARCHAR(100),
    tone VARCHAR(100),
    expected_sessions VARCHAR(50), -- 'short', 'medium', 'long', 'ongoing'
    campaign_status VARCHAR(50), -- 'planning', 'active', 'hiatus', 'completed'
    
    -- Combat/Exploration/Social pillars (1-5 rating)
    combat_rating INTEGER CHECK (combat_rating BETWEEN 1 AND 5),
    exploration_rating INTEGER CHECK (exploration_rating BETWEEN 1 AND 5),
    social_rating INTEGER CHECK (social_rating BETWEEN 1 AND 5),
    
    -- Key dates
    session_zero_date DATE,
    first_session_date DATE,
    last_session_date DATE,
    
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Simple world info embedded in campaign
ALTER TABLE campaigns ADD COLUMN world_name VARCHAR(255);
ALTER TABLE campaigns ADD COLUMN world_concept TEXT;
ALTER TABLE campaigns ADD COLUMN starting_region_name VARCHAR(255);
ALTER TABLE campaigns ADD COLUMN starting_settlement VARCHAR(255);
```

### Session Management
```sql
-- Sessions are the core unit of play
CREATE TABLE sessions (
    id INTEGER PRIMARY KEY,
    campaign_id INTEGER REFERENCES campaigns(id),
    session_number INTEGER NOT NULL,
    session_date DATE,
    
    -- Prep tracking
    prep_started_at TIMESTAMP,
    prep_completed_at TIMESTAMP,
    prep_file_path VARCHAR(500), -- Points to markdown file
    
    -- Session execution
    actual_duration_minutes INTEGER,
    players_present TEXT, -- Simple comma-separated list
    recap_file_path VARCHAR(500),
    
    -- Session design tracking
    designed_ending TEXT, -- What ending we built toward
    actual_ending_type VARCHAR(50), -- 'cliffhanger', 'natural', 'energy', 'time'
    ending_satisfaction INTEGER CHECK (ending_satisfaction BETWEEN 1 AND 5),
    
    -- Quick summary data (for browsing/searching)
    strong_start_summary VARCHAR(500),
    major_events_summary TEXT,
    cliffhanger_summary VARCHAR(500),
    
    -- Quick NPC tracking before they get full records
    npcs_introduced TEXT, -- Comma-separated list of new NPCs this session
    
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(campaign_id, session_number)
);

-- Modules are collections of sessions
CREATE TABLE modules (
    id INTEGER PRIMARY KEY,
    campaign_id INTEGER REFERENCES campaigns(id),
    module_number INTEGER,
    name VARCHAR(255),
    
    -- Module overview (denormalized for quick access)
    hook_summary TEXT,
    expected_sessions INTEGER,
    level_range VARCHAR(50), -- e.g., "1-3"
    
    -- Status tracking
    status VARCHAR(50), -- 'planning', 'ready', 'active', 'completed'
    started_session_id INTEGER REFERENCES sessions(id),
    completed_session_id INTEGER REFERENCES sessions(id),
    
    -- File references
    overview_file_path VARCHAR(500),
    
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(campaign_id, module_number)
);
```

### Character & Player Tracking
```sql
-- Players are real people
CREATE TABLE players (
    id INTEGER PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255),
    discord_handle VARCHAR(255),
    preferred_pronouns VARCHAR(50),
    joined_date DATE
);

-- Characters belong to players and campaigns
CREATE TABLE characters (
    id INTEGER PRIMARY KEY,
    campaign_id INTEGER REFERENCES campaigns(id),
    player_id INTEGER REFERENCES players(id),
    
    -- Basic info
    character_name VARCHAR(255),
    race VARCHAR(100),
    class VARCHAR(100),
    level INTEGER DEFAULT 1,
    
    -- Integration tracking
    backstory_file_path VARCHAR(500),
    personal_goal TEXT,
    integration_status VARCHAR(50), -- 'pending', 'partial', 'complete'
    spotlight_count INTEGER DEFAULT 0, -- Track spotlight balance
    
    -- Status
    active BOOLEAN DEFAULT TRUE,
    death_date DATE,
    retirement_reason TEXT,
    
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Track character connections (many-to-many)
CREATE TABLE character_connections (
    character_id_1 INTEGER REFERENCES characters(id),
    character_id_2 INTEGER REFERENCES characters(id),
    connection_type VARCHAR(255), -- 'family', 'rival', 'debt', etc.
    description TEXT,
    PRIMARY KEY (character_id_1, character_id_2)
);
```

### NPCs and Locations
```sql
-- NPCs - track all introduced NPCs, but distinguish recurring ones
CREATE TABLE npcs (
    id INTEGER PRIMARY KEY,
    campaign_id INTEGER REFERENCES campaigns(id),
    name VARCHAR(255),
    role VARCHAR(255), -- 'villain', 'ally', 'merchant', etc.
    
    -- Quick reference data
    one_line_description VARCHAR(500),
    location_name VARCHAR(255), -- Where they're usually found
    faction_name VARCHAR(255), -- Simple string, not normalized
    
    -- Recurring status
    is_recurring BOOLEAN DEFAULT FALSE, -- Promoted after 2nd appearance or DM decision
    promoted_to_recurring_date DATE,
    
    -- Appearance tracking
    first_appearance_session_id INTEGER REFERENCES sessions(id),
    last_appearance_session_id INTEGER REFERENCES sessions(id),
    appearance_count INTEGER DEFAULT 1,
    
    -- Status
    alive BOOLEAN DEFAULT TRUE,
    disposition_to_party VARCHAR(100), -- 'hostile', 'neutral', 'friendly', 'allied'
    stat_block_file_path VARCHAR(500), -- Usually only for recurring NPCs
    notes_file_path VARCHAR(500),
    
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Locations - only major ones worth tracking
CREATE TABLE locations (
    id INTEGER PRIMARY KEY,
    campaign_id INTEGER REFERENCES campaigns(id),
    name VARCHAR(255),
    location_type VARCHAR(100), -- 'settlement', 'dungeon', 'landmark'
    region_name VARCHAR(255), -- Simple string reference
    
    -- Quick access info
    one_line_description VARCHAR(500),
    danger_level VARCHAR(50),
    
    -- Development status
    development_status VARCHAR(50), -- 'concept', 'outlined', 'mapped', 'detailed'
    map_file_path VARCHAR(500),
    description_file_path VARCHAR(500),
    
    -- Discovery tracking
    discovered_session_id INTEGER REFERENCES sessions(id),
    visited_count INTEGER DEFAULT 0,
    
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

### Document Management
```sql
-- Track all documents and their status
CREATE TABLE documents (
    id INTEGER PRIMARY KEY,
    campaign_id INTEGER REFERENCES campaigns(id),
    
    -- Document identification
    doc_type VARCHAR(100), -- 'session_prep', 'recap', 'npc_sheet', etc.
    doc_name VARCHAR(255),
    file_path VARCHAR(500) NOT NULL,
    
    -- Relationship tracking (nullable - use what applies)
    session_id INTEGER REFERENCES sessions(id),
    module_id INTEGER REFERENCES modules(id),
    character_id INTEGER REFERENCES characters(id),
    npc_id INTEGER REFERENCES npcs(id),
    location_id INTEGER REFERENCES locations(id),
    
    -- Status tracking
    status VARCHAR(50), -- 'draft', 'active', 'archived'
    last_modified TIMESTAMP,
    word_count INTEGER,
    
    -- Metadata
    tags TEXT, -- Comma-separated for simplicity
    
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Simple document templates
CREATE TABLE document_templates (
    id INTEGER PRIMARY KEY,
    template_name VARCHAR(255),
    doc_type VARCHAR(100),
    template_file_path VARCHAR(500),
    is_default BOOLEAN DEFAULT FALSE
);
```

### Tracking Tables
```sql
-- Track what needs to be done
CREATE TABLE campaign_tasks (
    id INTEGER PRIMARY KEY,
    campaign_id INTEGER REFERENCES campaigns(id),
    task_description TEXT,
    due_date DATE,
    priority VARCHAR(50), -- 'urgent', 'important', 'backlog'
    completed BOOLEAN DEFAULT FALSE,
    completed_date DATE,
    
    -- What it's related to
    related_session_id INTEGER REFERENCES sessions(id),
    related_module_id INTEGER REFERENCES modules(id),
    
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Simple plot thread tracking
CREATE TABLE plot_threads (
    id INTEGER PRIMARY KEY,
    campaign_id INTEGER REFERENCES campaigns(id),
    thread_name VARCHAR(255),
    description TEXT,
    
    -- Status tracking
    status VARCHAR(50), -- 'background', 'active', 'resolved', 'abandoned'
    introduced_session_id INTEGER REFERENCES sessions(id),
    resolved_session_id INTEGER REFERENCES sessions(id),
    
    -- Urgency
    is_ticking_clock BOOLEAN DEFAULT FALSE,
    deadline_session_number INTEGER
);

-- Session attendance
CREATE TABLE session_attendance (
    session_id INTEGER REFERENCES sessions(id),
    player_id INTEGER REFERENCES players(id),
    character_id INTEGER REFERENCES characters(id),
    attended BOOLEAN DEFAULT TRUE,
    absence_reason VARCHAR(255),
    PRIMARY KEY (session_id, player_id)
);
```

## Filesystem Structure

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

## Key Design Decisions

### Why This Approach?

1. **Denormalized Fixed Data**: Campaign truths, the big three, and pillar ratings are stored directly in the campaigns table since they're always accessed together and have fixed sizes.

2. **Simple Strings Over Foreign Keys**: Faction names, region names, etc. are stored as simple strings rather than normalized tables when:
   - They're primarily for reference/grouping
   - They don't need complex attributes
   - The list is relatively small and campaign-specific

3. **File-Based Documents**: Actual content lives in markdown files, with the database tracking:
   - Location (file paths)
   - Status and metadata
   - Relationships to other entities

4. **Pragmatic Relationships**: Only create join tables for true many-to-many relationships (like character connections) or when tracking history is important.

5. **Status Tracking Focus**: The database excels at tracking what exists, when it was last updated, and what needs attention.

## Core Queries the System Should Support

```sql
-- What do I need to prep for next session?
SELECT 
    s.session_number,
    s.session_date,
    m.name as module_name,
    COUNT(DISTINCT ct.id) as pending_tasks
FROM sessions s
LEFT JOIN modules m ON m.id = (
    SELECT id FROM modules 
    WHERE campaign_id = ? 
    AND status = 'active' 
    LIMIT 1
)
LEFT JOIN campaign_tasks ct ON ct.related_session_id = s.id AND ct.completed = FALSE
WHERE s.campaign_id = ? 
AND s.session_date >= CURRENT_DATE
ORDER BY s.session_date
LIMIT 1;

-- Which character needs spotlight time?
SELECT 
    c.character_name,
    p.name as player_name,
    c.spotlight_count,
    COUNT(sa.session_id) as sessions_attended
FROM characters c
JOIN players p ON p.id = c.player_id
LEFT JOIN session_attendance sa ON sa.character_id = c.id AND sa.attended = TRUE
WHERE c.campaign_id = ? AND c.active = TRUE
GROUP BY c.id, c.character_name, p.name, c.spotlight_count
ORDER BY c.spotlight_count ASC, sessions_attended DESC;

-- What plot threads are active?
SELECT 
    thread_name,
    description,
    s.session_number as introduced_session,
    (? - s.session_number) as sessions_active,
    deadline_session_number,
    CASE 
        WHEN deadline_session_number IS NOT NULL 
        THEN deadline_session_number - ? 
        ELSE NULL 
    END as sessions_until_deadline
FROM plot_threads pt
JOIN sessions s ON s.id = pt.introduced_session_id
WHERE pt.campaign_id = ? 
AND pt.status = 'active'
ORDER BY is_ticking_clock DESC, sessions_active DESC;

-- Get NPC summary by location
SELECT 
    location_name,
    COUNT(*) as total_npcs,
    COUNT(CASE WHEN is_recurring = TRUE THEN 1 END) as recurring_npcs,
    COUNT(CASE WHEN disposition_to_party = 'allied' THEN 1 END) as allies,
    COUNT(CASE WHEN alive = FALSE THEN 1 END) as deceased
FROM npcs
WHERE campaign_id = ?
GROUP BY location_name
ORDER BY total_npcs DESC;

-- Which NPCs should be promoted to recurring?
SELECT 
    n.id,
    n.name,
    n.role,
    n.appearance_count,
    n.first_appearance_session_id,
    s1.session_number as first_session,
    s2.session_number as last_session
FROM npcs n
JOIN sessions s1 ON s1.id = n.first_appearance_session_id
JOIN sessions s2 ON s2.id = n.last_appearance_session_id
WHERE n.campaign_id = ?
    AND n.is_recurring = FALSE
    AND (n.appearance_count >= 2 
         OR n.role IN ('villain', 'patron', 'rival'))
ORDER BY n.appearance_count DESC;
```

## Implementation Notes

1. **NPC Lifecycle**:
   - First appearance: Create NPC record with basic info
   - After session: Update appearance_count
   - At 2+ appearances: Suggest promotion to recurring
   - If promoted: Create detailed notes file, add stat block if needed
   - Recurring NPCs get: Full tracking, relationship mapping, detailed notes

2. **Indexes Needed**:
   - campaign_id on all tables
   - session_date on sessions
   - status fields where used for filtering
   - file paths for quick document lookup

3. **Triggers**:
   - Update campaign.updated_at on related changes
   - Update character.spotlight_count when tagged in session notes
   - Update location.visited_count when referenced in session
   - Create task when NPC reaches 2 appearances (suggest promotion)
   - Update NPC appearance_count when referenced in session recap

4. **File System Integration**:
   - Store relative paths in database
   - Use campaign name as root folder
   - Implement file watcher for external edits

5. **Backup Strategy**:
   - Database: Regular SQL dumps
   - Files: Version control (git) or sync service
   - Export feature for full campaign archive

This design prioritizes practical use over theoretical purity, making it easy to track what matters while keeping the schema manageable.