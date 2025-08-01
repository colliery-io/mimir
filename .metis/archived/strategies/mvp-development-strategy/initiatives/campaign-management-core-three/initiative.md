---
id: campaign-management-core-three
level: initiative
title: "Campaign Management Core - Three-Board System"
created_at: 2025-07-31T18:26:30.468997+00:00
updated_at: 2025-07-31T18:26:30.468997+00:00
parent: mvp-development-strategy
blocked_by: []
archived: true

tags:
  - "#initiative"
  - "#phase/discovery"


exit_criteria_met: false
estimated_complexity: M
---

# Campaign Management Core - Three-Board System Initiative

## Context

The campaign management system requires a core data model and programmatic API that implements the three-board Flight Levels architecture described in the campaign design documents:

- **Campaign Board (FL3 - Strategic)**: Overall campaign vision, Big Three (Big Bad, Stakes, Personal Connection), Six Truths, and phase tracking
- **Module Board (FL2 - Tactical)**: Story arc management with 3-6 session modules, central questions, and module-specific stakes  
- **Session Board (FL1 - Operational)**: Individual session tracking with goals, designed endings, and prep artifacts

The system follows a denormalized database design for fixed campaign elements, with document content stored on the filesystem and metadata tracked in SQLite. This initiative implements the core data structures, database schema, and business logic API without any UI components.

## Goals & Non-Goals

**Goals:**
- Implement the core Campaign, Module, and Session data structures with proper Flight Levels hierarchy
- Create database schema with denormalized campaign fields (Big Three, Six Truths, world info)
- Build programmatic API for campaign lifecycle management (creation, phase transitions, status tracking)
- Support filesystem storage for documents with database metadata tracking
- Implement phase transitions with exit criteria validation
- Provide type-safe ENUMs in Rust code while storing as strings in database
- Enable just-in-time document generation based on campaign workflow

**Non-Goals:**
- User interface implementation (future initiative)
- Authentication or multi-user support
- Cloud storage or synchronization
- Pre-built campaign templates or content
- LLM integration for content generation (handled by agents framework)
- Import/export functionality beyond basic database operations

## Detailed Design

### Data Model Architecture

#### Campaign (FL3 - Strategic)
```rust
pub struct Campaign {
    pub id: CampaignId,
    pub name: String,
    pub pitch_file_path: Option<PathBuf>,
    
    // Session Configuration
    pub typical_session_length: SessionLength, // ENUM: OneHour, TwoHour, ThreeHour, FourPlusHour
    pub target_prep_time_minutes: Option<u32>,
    
    // The Big Three (denormalized)
    pub big_bad: String,
    pub stakes: String,
    pub personal_connection: String,
    
    // Six Truths (denormalized)
    pub truths: [String; 6],
    
    // Campaign Metadata
    pub genre: Genre, // ENUM
    pub tone: Tone, // ENUM
    pub expected_duration: CampaignDuration, // ENUM: Short, Medium, Long, Mega
    pub status: CampaignStatus, // ENUM: Planning, Active, Hiatus, Completed
    pub phase: CampaignPhase, // ENUM: Inception, SessionZero, Preparation, Active, Concluding
    
    // Pillar ratings (1-5)
    pub combat_rating: u8,
    pub exploration_rating: u8,
    pub social_rating: u8,
    
    // World info (denormalized)
    pub world_name: Option<String>,
    pub world_concept: Option<String>,
    pub starting_region: Option<String>,
    pub starting_settlement: Option<String>,
    
    // Key dates
    pub session_zero_date: Option<NaiveDate>,
    pub first_session_date: Option<NaiveDate>,
    pub last_session_date: Option<NaiveDate>,
}
```

#### Module (FL2 - Tactical)
```rust
pub struct Module {
    pub id: ModuleId,
    pub campaign_id: CampaignId,
    pub module_number: u32,
    pub name: String,
    
    // Module concept
    pub central_question: String,
    pub stakes_success: String,
    pub stakes_failure: String,
    pub tone_shift: Option<String>,
    
    // Overview
    pub hook_summary: String,
    pub expected_sessions: u32,
    pub actual_sessions: Option<u32>,
    pub level_range: Option<String>,
    
    // Pillar focus (1-5)
    pub combat_focus: u8,
    pub exploration_focus: u8,
    pub social_focus: u8,
    
    // Status tracking
    pub status: ModuleStatus, // ENUM: Planning, Development, Ready, Active, Completed
    pub phase_started_at: BTreeMap<ModuleStatus, DateTime<Utc>>,
    
    // File references
    pub overview_file_path: Option<PathBuf>,
}
```

#### Session (FL1 - Operational)
```rust
pub struct Session {
    pub id: SessionId,
    pub campaign_id: CampaignId,
    pub module_id: Option<ModuleId>,
    pub session_number: u32,
    pub session_date: Option<NaiveDate>,
    
    // Session plan
    pub planned_goal: String,
    pub designed_ending: String,
    pub pillar_focus: SessionPillar, // ENUM: Combat, Exploration, Social, Mixed
    
    // Prep tracking
    pub prep_started_at: Option<DateTime<Utc>>,
    pub prep_completed_at: Option<DateTime<Utc>>,
    pub prep_file_path: Option<PathBuf>,
    
    // Execution
    pub actual_duration_minutes: Option<u32>,
    pub recap_file_path: Option<PathBuf>,
    
    // Design tracking
    pub designed_ending_achieved: Option<bool>,
    pub actual_ending_type: Option<EndingType>, // ENUM: Cliffhanger, Natural, Energy, Time
    pub ending_satisfaction: Option<u8>, // 1-5
    
    // Quick summary
    pub strong_start_summary: Option<String>,
    pub major_events_summary: Option<String>,
    pub cliffhanger_summary: Option<String>,
}
```

### Database Schema

Using Diesel with SQLite, storing ENUMs as strings:

```sql
-- campaigns table with denormalized fields
CREATE TABLE campaigns (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    pitch_file_path TEXT,
    
    -- Session config
    typical_session_length TEXT NOT NULL CHECK (typical_session_length IN ('one_hour', 'two_hour', 'three_hour', 'four_plus_hour')),
    target_prep_time_minutes INTEGER,
    
    -- Big Three
    big_bad TEXT NOT NULL,
    stakes TEXT NOT NULL,
    personal_connection TEXT NOT NULL,
    
    -- Six Truths
    truth_1 TEXT NOT NULL,
    truth_2 TEXT NOT NULL,
    truth_3 TEXT NOT NULL,
    truth_4 TEXT NOT NULL,
    truth_5 TEXT NOT NULL,
    truth_6 TEXT NOT NULL,
    
    -- Metadata
    genre TEXT NOT NULL,
    tone TEXT NOT NULL,
    expected_duration TEXT NOT NULL CHECK (expected_duration IN ('short', 'medium', 'long', 'mega')),
    status TEXT NOT NULL CHECK (status IN ('planning', 'active', 'hiatus', 'completed')),
    phase TEXT NOT NULL CHECK (phase IN ('inception', 'session_zero', 'preparation', 'active', 'concluding')),
    
    -- Pillars
    combat_rating INTEGER NOT NULL CHECK (combat_rating BETWEEN 1 AND 5),
    exploration_rating INTEGER NOT NULL CHECK (exploration_rating BETWEEN 1 AND 5),
    social_rating INTEGER NOT NULL CHECK (social_rating BETWEEN 1 AND 5),
    
    -- World info
    world_name TEXT,
    world_concept TEXT,
    starting_region TEXT,
    starting_settlement TEXT,
    
    -- Dates
    session_zero_date DATE,
    first_session_date DATE,
    last_session_date DATE,
    
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

### API Design

The core API provides lifecycle management:

```rust
// Campaign lifecycle
impl CampaignManager {
    pub fn create_campaign(params: CreateCampaignParams) -> Result<Campaign>;
    pub fn transition_phase(id: CampaignId, to_phase: CampaignPhase) -> Result<Campaign>;
    pub fn validate_phase_exit_criteria(campaign: &Campaign) -> Result<Vec<ExitCriterion>>;
}

// Module management
impl ModuleManager {
    pub fn create_module(campaign_id: CampaignId, params: CreateModuleParams) -> Result<Module>;
    pub fn transition_status(id: ModuleId, to_status: ModuleStatus) -> Result<Module>;
    pub fn should_create_next_module(current: &Module) -> bool; // 60% rule
}

// Session workflow
impl SessionManager {
    pub fn create_session(module_id: ModuleId, params: CreateSessionParams) -> Result<Session>;
    pub fn start_prep(id: SessionId) -> Result<Session>;
    pub fn complete_prep(id: SessionId, prep_path: PathBuf) -> Result<Session>;
    pub fn record_outcome(id: SessionId, outcome: SessionOutcome) -> Result<Session>;
}
```

### Phase Transitions and Exit Criteria

Campaign phases have specific exit criteria:

```rust
pub enum CampaignPhase {
    Inception,      // Exit: Campaign pitch complete, Big Three defined
    SessionZero,    // Exit: Characters created, schedule confirmed, integration complete
    Preparation,    // Exit: First module ready, starting location detailed
    Active,         // Exit: Big Bad confronted or campaign goals achieved
    Concluding,     // Exit: Epilogue complete
}

impl CampaignPhase {
    pub fn exit_criteria(&self) -> Vec<ExitCriterion> {
        match self {
            Self::Inception => vec![
                ExitCriterion::new("Campaign pitch document created"),
                ExitCriterion::new("Big Three (Big Bad, Stakes, Personal Connection) defined"),
                ExitCriterion::new("Six Truths documented"),
                ExitCriterion::new("Session Zero date scheduled"),
            ],
            Self::SessionZero => vec![
                ExitCriterion::new("All player characters created"),
                ExitCriterion::new("Character relationships established"),
                ExitCriterion::new("Regular play schedule confirmed"),
                ExitCriterion::new("Safety tools discussed and agreed"),
                ExitCriterion::new("Character integration notes completed"),
            ],
            Self::Preparation => vec![
                ExitCriterion::new("First module overview complete"),
                ExitCriterion::new("Starting location detailed"),
                ExitCriterion::new("Initial NPCs created"),
                ExitCriterion::new("First session prep ready"),
            ],
            // ... etc
        }
    }
}
```

### Filesystem Integration

Documents are stored in a structured filesystem layout:

```rust
pub struct DocumentManager {
    root_path: PathBuf, // e.g., /campaigns
}

impl DocumentManager {
    pub fn campaign_path(&self, campaign: &Campaign) -> PathBuf {
        self.root_path.join(&campaign.name)
    }
    
    pub fn module_path(&self, campaign: &Campaign, module: &Module) -> PathBuf {
        self.campaign_path(campaign)
            .join("modules")
            .join(format!("{:02}_{}", module.module_number, module.name))
    }
    
    pub fn session_path(&self, campaign: &Campaign, session: &Session) -> PathBuf {
        self.campaign_path(campaign)
            .join("sessions")
            .join(format!("session_{:03}", session.session_number))
    }
}
```

## Alternatives Considered

### 1. Fully Normalized Database Design
**Approach**: Normalize all campaign attributes into separate tables (e.g., truths table, pillars table)
**Rejected Because**: 
- Adds unnecessary complexity for fixed-size data (always 6 truths, always 3 pillars)
- Requires multiple joins for basic campaign queries
- The denormalized approach better matches the domain model

### 2. Document Database (MongoDB/PostgreSQL JSONB)
**Approach**: Store campaigns as JSON documents with flexible schema
**Rejected Because**:
- SQLite with structured tables provides better query performance
- Type safety is harder to maintain with JSON
- Vector search integration (future) works better with SQLite
- Simpler deployment with embedded SQLite

### 3. All Content in Database
**Approach**: Store markdown content as TEXT/BLOB fields in database
**Rejected Because**:
- Makes version control difficult
- Harder to edit content with external tools
- Database size grows significantly
- Filesystem + metadata approach provides better flexibility

### 4. Generic Workflow Engine
**Approach**: Build a general-purpose state machine for all transitions
**Rejected Because**:
- Over-engineering for the specific domain needs
- Campaign/Module/Session have distinct transition rules
- Domain-specific phase transitions are clearer and more maintainable

### 5. Rigid 8-Week Timeline Implementation
**Approach**: Enforce the 8-week preparation timeline from the source documents
**Rejected Because**:
- Real campaigns have variable preparation times
- Focus should be on phases and exit criteria, not calendar time
- Flexibility allows for different group schedules

## Implementation Plan

{Phases and timeline for execution}

## Testing Strategy

{How the initiative will be validated}