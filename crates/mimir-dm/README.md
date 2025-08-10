# mimir-dm

## Purpose & Boundaries

The `mimir-dm` crate is the desktop application layer of Mimir D&D Campaign Assistant. It provides a Tauri-based desktop application with a Vue.js frontend, bridging the gap between the core business logic and the user interface. This crate orchestrates all the other components to deliver a complete D&D campaign management experience.

### Responsibilities

- **Desktop Application Shell**: Tauri framework integration for native desktop experience
- **Command Bridge**: IPC commands connecting frontend to backend services
- **Application Lifecycle**: Initialization, database setup, and resource management
- **Frontend Hosting**: Vue.js single-page application with routing and state management
- **File System Integration**: Campaign directory management and document persistence
- **Database Management**: Connection pooling and migration orchestration
- **Template Seeding**: Initial data population for new installations

### What This Crate Does NOT Do

- No business logic implementation (delegates to mimir-dm-core)
- No direct database operations (uses services from core)
- No LLM operations (future integration point)
- No web server hosting (desktop-only)
- No cloud synchronization

## Architecture

### Backend Structure (`src/`)

```
src/
├── main.rs                 # Application entry point and Tauri setup
├── app_init.rs            # Application initialization and paths management
├── db_connection.rs       # Database connection pool management
├── seed_templates.rs      # Template seeding for new databases
├── types.rs               # Shared types (ApiResponse, ApiError)
├── commands/              # Tauri command handlers (IPC bridge)
│   ├── mod.rs            # Command module exports
│   ├── app_info.rs      # Application information commands
│   ├── campaigns.rs      # Campaign CRUD operations
│   ├── documents.rs      # Document management
│   ├── boards.rs         # Board configuration and workflow
│   └── stage_transitions.rs  # Campaign stage transitions
└── services/              # Service layer
    ├── mod.rs
    └── database.rs       # Database service wrapper
```

### Frontend Structure (`frontend/src/`)

```
frontend/src/
├── main.ts                # Vue app initialization
├── App.vue               # Root component
├── router/               # Vue Router configuration
│   └── index.ts         # Route definitions
├── stores/               # Pinia state management
│   ├── campaigns.ts     # Campaign state and actions
│   └── theme.ts         # Theme preferences
├── views/                # Page-level components
│   ├── HomeView.vue     # Landing page
│   ├── SettingsView.vue # Application settings
│   ├── campaigns/       # Campaign management views
│   │   ├── CampaignListView.vue    # Campaign list
│   │   ├── CampaignCreateView.vue  # New campaign form
│   │   ├── CampaignDetailView.vue  # Campaign details
│   │   └── CampaignBoardView.vue   # Campaign workflow board
│   ├── modules/         # Module management views
│   ├── sessions/        # Session management views
│   └── templates/       # Template management views
├── components/           # Reusable components
│   ├── campaign/        # Campaign-specific components
│   ├── campaigns/       # Multi-campaign components
│   │   ├── DocumentEditor.vue      # Markdown editor
│   │   ├── DocumentSidebar.vue     # Document navigation
│   │   └── StageLandingView.vue    # Stage overview
│   ├── common/          # Shared components
│   └── layout/          # Layout components
│       ├── AppHeader.vue
│       └── MainLayout.vue
├── types/                # TypeScript type definitions
│   ├── api.ts           # API response types
│   └── campaign.ts      # Campaign domain types
├── utils/                # Utility functions
│   ├── debounce.ts      # Input debouncing
│   └── debug.ts         # Debug helpers
├── assets/               # Static assets
│   ├── images/          # UI images and icons
│   └── styles/          # CSS and theme files
│       └── themes/      # Theme variations (dark, light, hyper)
└── test/                 # Test configuration
    └── setup.ts         # Test environment setup
```

## Key Features

### Application Initialization
1. **Directory Setup**: Creates application directories in platform-specific locations
2. **Database Initialization**: SQLite setup with migration support
3. **Development Mode**: In-memory database option for development
4. **Template Seeding**: Populates initial templates on first run

### Command System (IPC)
The Tauri command system provides type-safe IPC between frontend and backend:

#### Campaign Commands
- `list_campaigns` - Retrieve all campaigns
- `create_campaign` - Create new campaign with directory structure
- `get_campaign` - Get campaign details
- `generate_campaign_document` - Generate document from template

#### Document Commands
- `get_campaign_documents` - List campaign documents
- `get_documents_by_level` - Filter documents by level
- `create_document` - Create new document
- `update_document` - Update existing document
- `complete_document` - Mark document as complete
- `delete_document` - Remove document
- `read_document_file` - Read document from filesystem
- `save_document_file` - Persist document to filesystem

#### Board Commands
- `get_board_configuration` - Get board workflow definition
- `check_campaign_stage_completion` - Check stage requirements
- `transition_campaign_stage` - Move to next stage
- `initialize_stage_documents` - Create required documents for stage
- `get_next_stage` - Determine next valid stage

### Frontend Architecture

#### Technology Stack
- **Vue 3**: Composition API with TypeScript
- **Pinia**: State management
- **Vue Router**: Client-side routing
- **TipTap**: Rich text editor with Markdown support
- **Tailwind CSS**: Utility-first styling
- **Vitest**: Unit testing framework

#### State Management
Pinia stores manage application state:
- **Campaign Store**: Campaign CRUD and selection
- **Theme Store**: User theme preferences (dark/light/hyper)

#### Component Organization
- **Views**: Page-level components mapped to routes
- **Components**: Reusable UI components
- **Layouts**: Application structure components
- **Common**: Shared utilities and helpers

## Configuration

### Tauri Configuration (`tauri.conf.json`)
```json
{
  "productName": "Mimir",
  "identifier": "com.mimir.app",
  "build": {
    "frontendDist": "./frontend/dist"
  },
  "app": {
    "windows": [{
      "title": "Mimir - D&D Campaign Assistant",
      "width": 1400,
      "height": 900,
      "minWidth": 1200,
      "minHeight": 700
    }]
  }
}
```

### Development Environment
- **Memory Database**: Set `MIMIR_DEV=1` for in-memory database
- **File Database**: Set `MIMIR_USE_FILE_DB=1` to force file database in dev

## Dependencies

### Backend
- `tauri` - Desktop application framework
- `tauri-plugin-shell` - Shell command execution
- `tauri-plugin-dialog` - Native dialogs
- `mimir-dm-core` - Core business logic
- `tokio` - Async runtime
- `tracing` - Structured logging
- `serde` - Serialization
- `directories` - Platform-specific paths

### Frontend
- `@tauri-apps/api` - Tauri frontend API
- `vue` - UI framework
- `vue-router` - Routing
- `pinia` - State management
- `@tiptap` - Rich text editor
- `tailwindcss` - CSS framework

## Building & Running

### Development
```bash
# Install frontend dependencies
cd frontend && npm install

# Run in development mode
cargo tauri dev
```

### Production Build
```bash
# Build frontend
cd frontend && npm run build

# Build Tauri app
cargo tauri build
```

### Testing
```bash
# Frontend tests
cd frontend && npm test

# Backend tests
cargo test -p mimir-dm

# E2E tests (requires built app)
cargo test --features e2e
```

## Design Principles

1. **Separation of Concerns**: Clear boundary between UI and business logic
2. **Type Safety**: TypeScript frontend with Rust backend
3. **Command Pattern**: All IPC through defined commands
4. **State Management**: Centralized state in Pinia stores
5. **Component Reusability**: Modular Vue components
6. **Theme Support**: Multiple theme variations
7. **Offline First**: Full functionality without internet
8. **Progressive Enhancement**: Features added incrementally

## API Response Pattern

All commands return a standardized response:
```rust
ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<String>
}
```

This ensures consistent error handling across the application.

## Future Enhancements

- [ ] LLM integration for content generation
- [ ] Plugin system for custom rules/content
- [ ] Export/import campaign archives
- [ ] Multiplayer collaboration support
- [ ] Mobile companion app
- [ ] Cloud backup integration
- [ ] Advanced search with filters
- [ ] Campaign timeline visualization
- [ ] Character sheet integration
- [ ] Battle map support