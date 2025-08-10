# mimir-dm-core

## Purpose & Boundaries

The `mimir-dm-core` crate is the heart of the Mimir D&D Campaign Assistant. It provides the core business logic, domain models, and data persistence layer. This crate implements domain-driven design patterns with a clear separation between D&D rules reference data and campaign management systems.

### Responsibilities

- **Domain Models**: Core entities for both rules (D&D 5e) and campaigns
- **Business Services**: High-level operations for campaign and template management
- **Data Persistence**: SQLite database with Diesel ORM, including migrations
- **Workflow Management**: Board definitions and state transitions for campaigns/modules/sessions
- **Document System**: Template engine and document generation with frontmatter support
- **Search Infrastructure**: Full-text search (FTS5) and vector similarity search
- **Data Seeding**: Initial template and reference data population

### What This Crate Does NOT Do

- No UI or presentation logic
- No LLM inference (depends on mimir-dm-llm for embeddings only)
- No agent orchestration
- No network operations beyond database connections
- No direct user interaction

## Architecture

The crate is organized into two distinct domains:

### 1. Rules Domain (`models::rules`, `dal::rules`)
Static D&D reference data that doesn't change during gameplay:
- Rule systems (D&D 5e, etc.)
- Source books and supplements
- Races, classes, backgrounds
- Items, spells, feats
- Creatures and monsters

### 2. Campaign Domain (`models::campaign`, `dal::campaign`)
Dynamic campaign management and story organization:
- Campaigns and their lifecycle
- Modules (story arcs)
- Sessions (individual game sessions)
- Documents and templates
- Workflow cards and board states

## Layout

```
src/
├── lib.rs                 # Crate root with public exports
├── connection.rs          # Database connection management
├── schema.rs              # Diesel schema definitions (generated)
├── error.rs               # Error types and Result type alias
│
├── models/                # Domain models (split by domain)
│   ├── rules/            # D&D reference data models
│   │   ├── races.rs      # Player races with traits
│   │   ├── classes.rs    # Character classes and features
│   │   ├── items.rs      # Equipment and magic items
│   │   ├── creatures.rs  # Monsters and NPCs
│   │   ├── spells.rs     # Spell definitions
│   │   └── ...
│   └── campaign/         # Campaign management models
│       ├── campaigns.rs  # Campaign lifecycle
│       ├── modules.rs    # Story arc management
│       ├── sessions.rs   # Game session tracking
│       ├── documents.rs  # Campaign documents
│       └── ...
│
├── dal/                   # Data Access Layer (Repository pattern)
│   ├── traits.rs         # Repository trait definitions
│   ├── rules/           # Repositories for rules data
│   └── campaign/        # Repositories for campaign data
│
├── domain/               # Domain logic and business rules
│   └── boards/          # Workflow board definitions
│       ├── campaign_board.rs
│       ├── module_board.rs
│       └── session_board.rs
│
├── services/             # Business service layer
│   ├── campaign_service.rs  # Campaign operations
│   └── template_service.rs  # Document generation
│
├── seed/                 # Data initialization
│   ├── template_loader.rs   # Load templates from filesystem
│   └── template_seeder.rs   # Seed initial templates
│
└── migrations/           # Diesel SQL migrations
```

## Key Features

### Domain-Driven Design
- Clear separation between rules reference and campaign management
- Rich domain models with business logic
- Repository pattern for data access
- Service layer for complex operations

### Database Infrastructure
- **SQLite** with **Diesel ORM** for type-safe queries
- Automatic schema migrations
- Connection pooling
- Transaction support

### (Planned) Search Capabilities
- **Full-Text Search (FTS5)** for fast text queries
- **Vector Similarity Search** via sqlite-vec for semantic search
- Dual search strategy for optimal results
- Embedding storage integrated with LLM providers

### Workflow System
- Board definitions for campaigns, modules, and sessions
- Stage transitions with validation
- Required document tracking
- Progress monitoring

### Template Engine
- Markdown templates with YAML frontmatter
- Variable substitution using Tera
- Template versioning and validation
- Automatic document generation



## Usage

```rust
use mimir_dm_core::{
    establish_connection, 
    run_migrations,
    services::CampaignService,
};

// Initialize database
let mut conn = establish_connection("path/to/db.sqlite")?;
run_migrations(&mut conn)?;

// Use services for business operations
let campaign_service = CampaignService::new();
let campaign = campaign_service.create_campaign(
    &mut conn,
    "The Lost Mines",
    "/path/to/campaigns"
)?;

// Access domain models directly
use mimir_dm_core::models::campaign::Campaign;
use mimir_dm_core::models::rules::Race;

// Use DAL for data access
use mimir_dm_core::dal::campaign::campaigns::CampaignRepository;
let repo = CampaignRepository::new(&mut conn);
let all_campaigns = repo.list()?;
```

## Testing

The crate includes comprehensive tests:
- Unit tests for domain models
- Integration tests for repositories
- Service layer tests with test databases
- Template validation tests

Run tests with:
```bash
cargo test -p mimir-dm-core
```

