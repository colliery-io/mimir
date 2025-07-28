# Mimir - D&D Campaign Assistant Design Document

## Project Overview

**Mimir** is a local-first, Rust-based TUI application designed to help Dungeon Masters manage the cognitive complexity of running D&D campaigns. Named after the wise floating skull of Planescape lore, Mimir focuses on narrative consistency, NPC management, plot tracking, and session preparation rather than gameplay automation.

## Core Technology Stack

- **Language**: Rust
- **UI**: Ratatui (TUI framework)
- **Database**: SQLite with sqlite-vss (vector similarity search) and FTS5 (full-text search)
- **LLM**: Ollama (local) with llama3/mistral models
- **Embeddings**: nomic-embed-text via Ollama
- **Rule Data**: 5e-bits JSON files (embedded at build time)

## System Architecture

### Data Layer
- **SQLite with sqlite-vss and FTS5**: Provides structured data, vector search, and full-text search
- **Triple-Index Strategy**:
  - **FTS5**: For exact keyword matches (spell names, NPC names, rule terms)
  - **Vector Search**: For semantic similarity and conceptual queries
  - **Structured Indices**: For filtered queries (by date, level, type)
- **Hybrid Storage**: Campaign data, NPCs, plots, sessions with both structured fields and embedded narratives
- **Rule Storage**: Pre-processed SRD content with both FTS5 indices and vector embeddings

### Agent Framework
- **Lightweight Actor Pattern**: Message-passing between async Rust tasks
- **Agent Types**:
  - NPC Manager: Personality consistency and relationship tracking
  - Plot Weaver: Story thread management and connection suggestions
  - Session Orchestrator: Prep tools and session summaries
  - Rule Advisor: Quick mechanical lookups without flow interruption

### RAG Implementation
- **Triple-Index Approach**:
  - **FTS5 Index**: Full-text search for exact matches and phrase queries
  - **Rules Vector Index**: Pre-computed SRD embeddings for semantic search
  - **Campaign Vector Index**: Dynamic campaign content embeddings
- **Hybrid Search Algorithm**:
  ```
  final_score = 0.5 * fts_score + 0.35 * vector_similarity + 0.15 * recency_weight
  ```
- **Query Router**: 
  - Exact terms (spell names, NPC names) → FTS5 first
  - Conceptual queries → Vector search first
  - Combined results with de-duplication
- **Smart Context Assembly**: Preserves complete rule sections

## Implementation Plan

### Phase 1: Foundation (Weeks 1-2)
1. SQLite setup with vector extensions and FTS5
   - Create FTS5 virtual tables for searchable content
   - Configure tokenizers for D&D-specific terms
2. Ollama integration and streaming
3. Rule data import with dual indexing (FTS5 + embeddings)

### Phase 2: Core Agents (Weeks 3-4)
1. Agent framework with message passing
2. NPC Manager implementation
3. RAG pipeline development

### Phase 3: Enhanced Capabilities (Weeks 5-6)
1. Additional agents (Plot, Session, Rules)
2. TUI development with Ratatui

### Phase 4: Intelligence Layer (Weeks 7-8)
1. Smart features (consistency checking, suggestions)
2. Performance optimization

## TUI Design Specifications

### Main Campaign View (Default)

```
┌─ Mimir ──────────────────────────────────────────────────────────────────────────┐
│ Campaign: Curse of the Crimson Crown | Session #12 | Next: Sun 7pm               │
├───────────────────────────────────────┬──────────────────────────────────────────┤
│ [F1] Query  [F2] NPC  [F3] Plot       │ Context Panel                            │
│ [F4] Session  [F5] Search  [ESC] Menu │ ──────────────                           │
├───────────────────────────────────────┤                                          │
│ > What happened with the stolen crown? │ Active Plots (3)                         │
│                                       │ • The Stolen Crown ████████░░ 80%        │
│ Assistant:                            │ • Thieves' Guild War ██████░░░░ 60%      │
│ ─────────                             │ • Missing Heir ███░░░░░░░░░ 25%          │
│                                       │                                          │
│ Based on your session notes, the      │ Recent NPCs                              │
│ crown was last seen with Marcus the   │ • Marcus the Fence (2 sessions ago)      │
│ Fence in Session #10. He mentioned    │ • Lady Valdris (last session)            │
│ selling it to a "northern buyer."     │ • Captain Thorne (last session)          │
│                                       │                                          │
│ Key developments:                     │ Location: Waterdeep Markets              │
│ • Session #10: Party tracked crown    │                                          │
│   to Marcus's shop                    │ Next Session Prep                        │
│ • Marcus revealed buyer from Neverwi- │ • Resolve crown buyer identity           │
│   nter approached him                 │ • Thieves' guild meeting scheduled       │
│ • Session #11: Lady Valdris hinted    │ • Check in on Valdris suspicions         │
│   at court involvement                │                                          │
│                                       │ [Tab] Toggle Panel  [↑↓] Scroll          │
└───────────────────────────────────────┴──────────────────────────────────────────┘
│ Ready | Ollama: Connected | Mode: Query | 15.2ms                                 │
└───────────────────────────────────────────────────────────────────────────────────┘
```

### NPC Management Mode (F2)

```
┌─ NPC Manager ────────────────────────────────────────────────────────────────────┐
│ Search: [Marcus        ] [X] Active Only  [X] This Arc  Sort: Last Seen ▼        │
├─────────────────────────┬────────────────────────────────────────────────────────┤
│ NPCs (47 total)         │ Marcus the Fence                                       │
│ ─────────────           │ ════════════════                                       │
│                         │ First Seen: Session #8 | Last: Session #10            │
│ ▶ Marcus the Fence      │                                                        │
│   Lady Valdris          │ Description:                                           │
│   Captain Thorne        │ A nervous halfling fence operating from the Dock       │
│   Grimjaw (deceased)    │ Ward. Always sweating, speaks in whispers.             │
│   Elara Moonwhisper     │                                                        │
│   Lord Blackstone       │ Personality: Cowardly, Greedy, Surprisingly Loyal      │
│   Sister Meredith       │ Voice: High-pitched, frequent clearing of throat       │
│   Razok the Red         │                                                        │
│                         │ Relationships:                                         │
│ Groups:                 │ • Fears: The Zhentarim                                 │
│ [+] Thieves Guild (12)  │ • Owes Favor: Party (warned about ambush)              │
│ [+] Noble Court (8)     │ • Business Partner: Razok the Red                      │
│ [+] City Watch (5)      │                                                        │
│                         │ Key Information:                                       │
│ [N] New NPC             │ • Knows crown buyer's identity (not yet revealed)      │
│ [G] Generate Random     │ • Has connection to Northern merchants                 │
│ [I] Import              │ • Shop has hidden basement (party discovered)          │
│                         ├────────────────────────────────────────────────────────┤
│                         │ [E] Edit  [D] Delete  [R] Relationships  [H] History  │
└─────────────────────────┴────────────────────────────────────────────────────────┘
```

### Plot Thread Tracker (F3)

```
┌─ Plot Thread Manager ────────────────────────────────────────────────────────────┐
│ Active: 3 | Dormant: 2 | Resolved: 14 | Filter: [All Arcs ▼]                     │
├───────────────────────────────────────────────────────────────────────────────────┤
│ The Stolen Crown                                            ████████░░ 80%        │
│ ════════════════                                           Updated: Session #11   │
│                                                                                   │
│ Thread Summary:                                                                   │
│ Royal crown stolen from palace vault. Party tracking through criminal underworld. │
│                                                                                   │
│ Key Events:                                          ┌─ Connections ─────────┐   │
│ • S#7:  Crown discovered missing                     │ → Thieves Guild War   │   │
│ • S#8:  Palace guard Grimjaw found murdered          │ → Missing Heir        │   │
│ • S#9:  Thieves guild involvement confirmed          │ ← Lady Valdris        │   │
│ • S#10: Marcus reveals northern buyer                │ ← Northern Merchants  │   │
│ • S#11: Court faction possibly involved              └───────────────────────┘   │
│                                                                                   │
│ Open Questions:                     Suggested Next Steps:                         │
│ • Who is the northern buyer?       • Investigate Neverwinter connections         │
│ • Why does Lady Valdris know?      • Question Lady Valdris directly             │
│ • Is crown still in the city?      • Check with harbor master for ships         │
│                                                                                   │
│ [Enter] Expand  [C] Connect Plots  [U] Update  [R] Resolve  [A] Archive          │
├───────────────────────────────────────────────────────────────────────────────────┤
│ Thieves' Guild War                                          ██████░░░░ 60%        │
│ Missing Heir                                                ███░░░░░░░ 25%        │
└───────────────────────────────────────────────────────────────────────────────────┘
```

### Session Planning Mode (F4)

```
┌─ Session Planner ─────────────────────────────────────────────────────────────────┐
│ Next Session: #13 | Date: Sunday 7pm | Location: The Yawning Portal              │
├─────────────────────────┬─────────────────────────────────────────────────────────┤
│ Prep Checklist          │ Session Outline                                         │
│ ══════════════          │ ═══════════════                                         │
│                         │                                                         │
│ [X] Review last session │ Opening Scene:                                          │
│ [X] Update NPC notes    │ Party reconvenes at Yawning Portal after investigating  │
│ [X] Prepare encounters  │ leads. Durnan mentions strange northerners asking       │
│ [ ] Print handouts      │ about "special merchandise."                            │
│ [ ] Set up playlist     │                                                         │
│                         │ Expected Developments:                                  │
│ Must Address:           │ • Party will likely pursue northern buyer lead         │
│ • Crown buyer identity  │ • Thieves guild meeting at midnight (Razok attending)  │
│ • Guild meeting         │ • Lady Valdris may approach party privately            │
│ • Valdris's knowledge   │                                                         │
│                         │ Prepared Scenes:                                        │
│ Prepared Elements:      │ 1. Harbor Investigation (skill challenge)              │
│ • 3 Combat encounters   │ 2. Thieves Guild Meeting (roleplay/combat)             │
│ • 2 Skill challenges    │ 3. Lady Valdris's Revelation (plot twist)              │
│ • 4 NPC interactions    │                                                         │
│                         │ Contingencies:                                          │
│ [S] Start Session       │ • If party splits: Focus on crown investigation        │
│ [E] Export Notes        │ • If combat avoided: Zhentarim agents appear later     │
│ [T] Timer Mode          │ • If Valdris killed: Letter found with same info       │
└─────────────────────────┴─────────────────────────────────────────────────────────┘
```

### Quick Search Overlay (F5)

```
┌─ Quick Search ────────────────────────────────────────────────────────┐
│ Search: [fireball spell                                            ]  │
├───────────────────────────────────────────────────────────────────────┤
│ Results:                                                              │
│                                                                       │
│ Rules (SRD) - Exact Matches                                          │
│ ═══════════════════════════                                          │
│ • **Fireball** - 3rd Level Evocation                          [Enter]│
│   Range: 150 feet, Damage: 8d6 fire, DEX save                       │
│   "A bright streak flashes...explodes with a low roar"              │
│                                                                       │
│ • Delayed Blast **Fireball** - 7th Level Evocation           [2]    │
│   Range: 150 feet, Damage: 12d6 fire, delayed trigger               │
│                                                                       │
│ Campaign References - Context Matches                                 │
│ ═══════════════════════════════════                                  │
│ • Session #6: Razok used **fireball** in warehouse fight      [3]    │
│ • NPC: Elara Moonwhisper knows **fireball**                  [4]    │
│ • Item: Wand of **Fireballs** (held by party)                [5]    │
│                                                                       │
│ [↑↓] Navigate  [Enter] Select  [Tab] Change Category  [ESC] Close    │
└───────────────────────────────────────────────────────────────────────┘
```

## Campaign Management

### Main Menu (ESC from main view)

```
┌─ Main Menu ──────────────────────────────────────────────────────────────────────┐
│                                                                                   │
│   Mimir - The Campaign Chronicle                                                  │
│   ═════════════════════════════                                                  │
│                                                                                   │
│   [C] Campaigns          Manage your campaigns                                    │
│   [S] Settings           Configure Ollama, models, and preferences                │
│   [H] Help              Documentation and tutorials                               │
│   [A] About             Version and system info                                   │
│   [Q] Quit              Exit application                                          │
│                                                                                   │
│   Current Campaign: Curse of the Crimson Crown                                    │
│   Last Session: #12 (3 days ago)                                                  │
│                                                                                   │
└───────────────────────────────────────────────────────────────────────────────────┘
```

### Campaign Manager

```
┌─ Campaign Manager ────────────────────────────────────────────────────────────────┐
│ Campaigns: 4 Active | 2 Archived | Sort: Last Played ▼                           │
├───────────────────────────────────────────────────────────────────────────────────┤
│                                                                                   │
│ ▶ Curse of the Crimson Crown                          Last played: 3 days ago    │
│   ═══════════════════════════                         Sessions: 12 | NPCs: 47    │
│   A political intrigue campaign in Waterdeep                                      │
│   Players: Sarah (Elf Wizard), Mike (Human Fighter), Jane (Halfling Rogue)       │
│                                                                                   │
│   Lost Mines of Phandelver                            Last played: 2 weeks ago   │
│   ════════════════════════                            Sessions: 8 | NPCs: 23     │
│   Classic starter adventure with custom twists                                    │
│   Players: Tom (Dwarf Cleric), Amy (Elf Ranger), Bob (Human Wizard)             │
│                                                                                   │
│   Homebrew: Shattered Realms                          Last played: 1 month ago   │
│   ═════════════════════════                           Sessions: 24 | NPCs: 89    │
│   Original setting with planar travel mechanics                                   │
│   Players: Core group + rotating guests                                           │
│                                                                                   │
│ [Enter] Open  [N] New Campaign  [I] Import  [E] Export  [A] Archive  [D] Delete  │
└───────────────────────────────────────────────────────────────────────────────────┘
```

### Quick Campaign Switcher (Ctrl+K)

```
┌─ Quick Switch ─────────────────────────────────────┐
│ Type to filter campaigns...                         │
│ [                                              ]    │
├─────────────────────────────────────────────────────┤
│ Recent Campaigns:                                   │
│                                                     │
│ [1] Curse of the Crimson Crown (current)           │
│ [2] Lost Mines of Phandelver                       │
│ [3] Homebrew: Shattered Realms                     │
│ [4] Tales from the Yawning Portal                  │
│                                                     │
│ [Tab] Browse All  [ESC] Cancel                      │
└─────────────────────────────────────────────────────┘
```

## Dialog System

### Consistency Check Example

```
┌─ Consistency Check ─────────────────────────────┐
│ ⚠ Potential Inconsistency Detected              │
│                                                 │
│ You mentioned Marcus was killed in the current  │
│ session, but records show he was last seen      │
│ alive in Session #10.                           │
│                                                 │
│ Options:                                        │
│ [C] Correct (Mark as deceased)                  │
│ [I] Ignore (I misspoke)                         │
│ [E] Explain (Add clarification)                 │
└─────────────────────────────────────────────────┘
```

## Navigation and Input Modes

### Global Shortcuts
- **F1-F5**: Mode switching
- **Tab**: Cycle panels
- **Ctrl+S**: Quick save
- **Ctrl+K**: Campaign switcher
- **Ctrl+/**: Command palette
- **ESC**: Back/Menu
- **?**: Context help

### Input Modes
- **Natural Language**: `Tell me about Marcus and the guild`
- **Command Mode** (`:` prefix): `:npc create`, `:plot update`
- **Search Mode** (`/` prefix): `/fireball`, `/npc:marcus`

### Status Bar
```
│ Asking Mimir... | Ollama: Connected | Campaign: Crimson Crown | Session #12 | 15.2ms │
```

## Technical Implementation Details

### Search Architecture

#### FTS5 Configuration
```sql
-- Create FTS5 tables for different content types
CREATE VIRTUAL TABLE rules_fts USING fts5(
    title, 
    content, 
    category,
    tokenize = 'porter unicode61 remove_diacritics 2'
);

CREATE VIRTUAL TABLE npcs_fts USING fts5(
    name, 
    description, 
    notes,
    tokenize = 'porter unicode61'
);

CREATE VIRTUAL TABLE sessions_fts USING fts5(
    summary,
    notes,
    events,
    tokenize = 'porter unicode61'
);
```

#### Hybrid Search Implementation
```rust
pub struct HybridSearcher {
    fts_weight: f32,      // 0.5 default
    vector_weight: f32,   // 0.35 default  
    recency_weight: f32,  // 0.15 default
}

impl HybridSearcher {
    async fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
        // 1. Parse query for exact terms vs semantic intent
        let query_analysis = self.analyze_query(query);
        
        // 2. FTS5 search for keyword matches
        let fts_results = if query_analysis.has_exact_terms {
            self.fts_search(&query_analysis.keywords).await?
        } else {
            vec![]
        };
        
        // 3. Vector search for semantic similarity
        let embedding = self.embed(query).await?;
        let vector_results = self.vector_search(&embedding).await?;
        
        // 4. Merge and rank results
        self.merge_results(fts_results, vector_results, query_analysis)
    }
}
```

#### Query Analysis
- **Exact Match Triggers**: Quoted strings, spell names, NPC names
- **Semantic Triggers**: Questions, descriptions, conceptual queries
- **Mixed Queries**: "What spells can Marcus cast?" → FTS5 for "Marcus", vector for spell capabilities

#### D&D-Specific Search Optimizations
- **Spell Name Normalization**: "Bigby's Hand" → matches "Bigby's Hand", "bigbys hand", "bigby hand"
- **Dice Notation Recognition**: "3d6" routes to dice mechanics, not general search
- **Abbreviation Expansion**: "AC" → "Armor Class", "HP" → "Hit Points"
- **Context-Aware Ranking**: Recent NPCs/plots ranked higher in ambiguous searches
- **Rule Version Filtering**: 5e vs 5.5e content based on campaign settings

### Core Trait Definitions

```rust
// Agent trait
pub trait Agent: Send + Sync {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    async fn process(&mut self, request: Request, context: &Context) -> Result<Response>;
    fn capabilities(&self) -> Vec<Capability>;
    fn required_context(&self) -> Vec<ContextRequirement>;
}

// Storage trait
pub trait Storage: Send + Sync {
    // Key-value operations
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>>;
    async fn set(&self, key: &str, value: Vec<u8>) -> Result<()>;
    
    // Structured queries
    async fn query_npcs(&self, filter: NpcFilter) -> Result<Vec<Npc>>;
    async fn query_plots(&self, filter: PlotFilter) -> Result<Vec<Plot>>;
    
    // Full-text search
    async fn fts_search(&self, query: &str, table: FtsTable) -> Result<Vec<FtsResult>>;
    async fn fts_search_all(&self, query: &str) -> Result<Vec<FtsResult>>;
    
    // Vector search
    async fn vector_search(&self, embedding: &[f32], limit: usize) -> Result<Vec<SearchResult>>;
    
    // Hybrid search combining FTS and vector
    async fn hybrid_search(&self, query: &str, options: SearchOptions) -> Result<Vec<SearchResult>>;
}

// LLM Provider trait
pub trait LlmProvider: Send + Sync {
    async fn generate(&self, prompt: &Prompt) -> Result<String>;
    async fn embed(&self, text: &str) -> Result<Vec<f32>>;
    fn stream(&self, prompt: &Prompt) -> BoxStream<Result<String>>;
}
```

### Key Data Structures

```rust
pub struct Request {
    pub id: Uuid,
    pub from: RequestSource,
    pub intent: Intent,
    pub content: String,
    pub metadata: HashMap<String, Value>,
}

pub enum Intent {
    GenerateNpc { criteria: NpcCriteria },
    DevelopPlot { plot_id: Option<Uuid> },
    CreateEncounter { difficulty: EncounterDifficulty },
    LevelCharacter { character_id: Uuid },
    QueryCampaign { query: String },
    Custom(String),
}

pub struct Context {
    pub campaign_id: Uuid,
    pub session: SessionContext,
    pub storage: Arc<dyn Storage>,
    pub llm: Arc<dyn LlmProvider>,
    pub rules: Arc<dyn RuleSystem>,
}
```

### Configuration File

```toml
[campaign]
default_rules = "dnd5e"
storage_path = "~/.mimir/campaigns"

[llm]
provider = "ollama"
model = "llama3"
embedding_model = "nomic-embed-text"
endpoint = "http://localhost:11434"

[search]
# Hybrid search weights (must sum to 1.0)
fts_weight = 0.5
vector_weight = 0.35
recency_weight = 0.15

# FTS5 configuration
fts_tokenizer = "porter unicode61 remove_diacritics 2"
fts_snippet_length = 64
fts_highlight_start = "**"
fts_highlight_end = "**"

# Search behavior
spell_name_fuzzy_match = true
expand_abbreviations = true
boost_recent_content = true
recent_session_count = 3

[agents]
enabled = ["npc_generator", "plot_manager", "encounter_builder", "level_assistant"]

[behaviors]
enabled = ["consistency_check", "narrative_enhancement", "rule_validation"]

[ui]
theme = "dark"
animation_speed = "normal"
context_panel_width = 40
```

## Performance Targets

- **Response Time**: <2 seconds for queries on consumer hardware
- **FTS5 Search**: <50ms for keyword searches across all content
- **Vector Search**: <100ms for semantic queries
- **Hybrid Search**: <150ms for combined FTS + vector queries
- **Memory Usage**: <4GB RAM with full campaign loaded
- **Startup Time**: <500ms to main interface
- **Context Window**: 2000 tokens rules + 1000 tokens campaign context
- **Index Size**: FTS5 indices <10% of content size, vector indices <50MB per campaign

## Development Priorities

1. **Core Infrastructure** (Critical)
   - SQLite setup with vector search
   - Ollama integration
   - Basic TUI framework

2. **Essential Features** (High)
   - NPC management
   - Plot tracking
   - Session notes
   - Rule queries

3. **Enhanced Features** (Medium)
   - Consistency checking
   - Smart suggestions
   - Campaign templates
   - Import/Export

4. **Polish** (Low)
   - Themes
   - Advanced search
   - Statistics
   - Tutorials