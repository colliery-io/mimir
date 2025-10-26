---
id: standardize-database-connection
level: task
title: "Standardize database connection pattern across codebase"
short_code: "MIMIR-T-0020"
created_at: 2025-10-24T11:54:03.988551+00:00
updated_at: 2025-10-25T14:10:00.421389+00:00
parent: MIMIR-I-0001
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/completed"


exit_criteria_met: false
strategy_id: NULL
initiative_id: MIMIR-I-0001
---

# Standardize database connection pattern across codebase

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0001]]

## Objective **[REQUIRED]**

Move database connection management to the core layer where it belongs, establishing proper architectural layering. Currently, the UI layer (mimir-dm) owns connection pooling while core layer (mimir-dm-core) uses direct connections - this is backwards. Core should own the DAL and provide DatabaseService, with the UI layer simply consuming it.

### Technical Debt Impact

- **Current Problems**:
  - **Inverted architecture**: UI layer owns DB_POOL instead of core layer
  - Two disconnected database connection patterns:
    - UI layer (mimir-dm/src/db_connection.rs): r2d2 pooled connections
    - Core layer (mimir-dm-core/src/connection.rs): Direct connections with async wrappers
  - Core layer cannot use its own connection pooling
  - UI layer has database management responsibilities it shouldn't have
  - Violates separation of concerns and layered architecture principles
  - 9 files in core using workaround patterns due to lack of proper pooling

- **Benefits of Fixing**:
  - **Proper layering**: Core owns DAL, UI consumes it
  - Single connection pool managed in core layer
  - Core services can directly use pooled connections
  - UI layer becomes thinner and focused on presentation/commands
  - Better testability - core can be tested independently with its own pool
  - Clearer architectural boundaries
  - Easier to reason about connection lifecycle

- **Risk Assessment**:
  - Low-medium risk - requires coordination between layers
  - Breaking change to internal API structure
  - Must ensure UI layer properly imports from core
  - Tests will verify no regressions
  - Can be done incrementally with feature flags if needed

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria **[REQUIRED]**

- [x] r2d2 connection pooling moved from mimir-dm to mimir-dm-core
- [x] mimir-dm-core exports DatabaseService with pool management
- [x] mimir-dm imports and uses DatabaseService from core
- [x] mimir-dm/src/db_connection.rs removed
- [x] All core services use pooled connections directly
- [x] Direct connection async wrappers (`with_connection`, `with_transaction`) removed
- [x] All affected core files refactored to use pool-based connections
- [x] All tests passing with no regressions (63 core tests + 18 LLM tests passed)
- [x] Build succeeds with no compilation errors (cargo check passes)
- [x] Proper architectural layering: Core owns connections, UI consumes DAL

## Implementation Notes

### Current Architecture (Incorrect Layering)

**UI Layer** (mimir-dm/src/db_connection.rs) - OWNS connection pool:
```rust
// Global pool with OnceLock - should NOT be in UI layer
pub static DB_POOL: OnceLock<DbPool> = OnceLock::new();

pub fn get_connection() -> Result<PooledConnection<...>> {
    DB_POOL.get()?.get()
}
```

**UI Layer** (mimir-dm/src/services/database.rs) - Thin wrapper:
```rust
// Delegates to ui layer's db_connection
pub struct DatabaseService;
impl DatabaseService {
    pub fn get_connection(&self) -> Result<DbConnection> {
        crate::db_connection::get_connection()
    }
}
```

**Core Layer** (mimir-dm-core/src/connection.rs) - Direct connections:
```rust
// Can't use pooling because it's in UI layer!
pub fn establish_connection(url: &str) -> Result<DbConnection> {
    DbConnection::establish(url)?
}

// Workaround with async wrappers
pub async fn with_connection<F, R>(url: String, f: F) -> Result<R> {
    tokio::task::spawn_blocking(move || {
        let mut conn = establish_connection(&url)?;
        f(&mut conn)
    }).await?
}
```

### Target Architecture (Proper Layering)

**Core Layer** (mimir-dm-core) - OWNS connection pool and provides DAL:
```rust
// mimir-dm-core/src/db.rs (new file)
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::SqliteConnection;

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<SqliteConnection>>;

pub struct DatabaseService {
    pool: DbPool,
}

impl DatabaseService {
    pub fn new(database_url: &str, is_memory_db: bool) -> Result<Self> {
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        let pool = Pool::builder()
            .min_idle(if is_memory_db { Some(1) } else { None })
            .max_size(if is_memory_db { 1 } else { 10 })
            .build(manager)?;
        Ok(Self { pool })
    }

    pub fn get_connection(&self) -> Result<DbConnection> {
        self.pool.get().map_err(|e| ...)
    }
}

// Core services use DatabaseService directly
impl CampaignService {
    pub fn list(conn: &mut DbConnection) -> Result<Vec<Campaign>> {
        // Direct database access
    }
}
```

**UI Layer** (mimir-dm) - Imports and uses core's DatabaseService:
```rust
// mimir-dm/src/main.rs
use mimir_dm_core::db::DatabaseService;

fn main() {
    let db_service = DatabaseService::new(&database_url, false)?;

    tauri::Builder::default()
        .manage(Arc::new(db_service))  // Pass core's service to Tauri
        .invoke_handler(tauri::generate_handler![...])
        .run()?;
}

// Commands use core's DatabaseService
#[tauri::command]
async fn list_campaigns(
    db: State<'_, Arc<DatabaseService>>
) -> Result<Vec<Campaign>> {
    let mut conn = db.get_connection()?;
    CampaignService::list(&mut conn)  // Use core service
}
```

### Files Requiring Changes

**Core Layer - New/Modified:**
1. `/crates/mimir-dm-core/src/db.rs` - NEW: Connection pool and DatabaseService
2. `/crates/mimir-dm-core/src/lib.rs` - Export new db module
3. `/crates/mimir-dm-core/src/connection.rs` - Remove async wrappers, keep establish_connection for migrations
4. `/crates/mimir-dm-core/src/services/*.rs` - Update to use pooled connections
5. `/crates/mimir-dm-core/src/seed/template_seeder.rs` - Use pooled connections
6. `/crates/mimir-dm-core/tests/**/*.rs` - Use core's DatabaseService for tests

**UI Layer - Modified/Removed:**
1. `/crates/mimir-dm/src/db_connection.rs` - REMOVE or reduce to thin re-export
2. `/crates/mimir-dm/src/services/database.rs` - REMOVE (use core's version)
3. `/crates/mimir-dm/src/main.rs` - Import DatabaseService from core
4. `/crates/mimir-dm/src/app_init.rs` - Use core's DatabaseService
5. `/crates/mimir-dm/src/commands/**/*.rs` - Import from core, not local

**Total:** 6 core files + 5 UI files = 11 files

### Implementation Steps

**Phase 1: Create core's DatabaseService**
1. Create `/crates/mimir-dm-core/src/db.rs` with connection pool
2. Move r2d2 pool initialization logic from UI to core
3. Implement DatabaseService in core with pool ownership
4. Export from mimir-dm-core/src/lib.rs

**Phase 2: Update core services**
1. Modify core services to accept `&mut DbConnection` from pool
2. Remove `database_url: String` parameters from service methods
3. Remove usage of `with_connection` / `with_transaction` wrappers
4. Update core tests to use DatabaseService

**Phase 3: Update UI layer**
1. Import DatabaseService from mimir-dm-core in main.rs
2. Initialize core's DatabaseService and pass to Tauri State
3. Update all command handlers to use core's DatabaseService
4. Remove mimir-dm/src/db_connection.rs
5. Remove mimir-dm/src/services/database.rs

**Phase 4: Cleanup**
1. Remove async wrapper functions from core/src/connection.rs
2. Keep only `establish_connection()` for migration runner
3. Update all imports across both crates
4. Run full test suite to verify

### Dependencies

- Requires diesel and r2d2 in mimir-dm-core's Cargo.toml
- Core must export DatabaseService publicly
- UI layer must import from core, not define its own

### Risk Considerations

- **Import ordering**: UI layer must not create circular dependencies
- **Test setup**: Both crates need access to test database setup
- **Migration runner**: Keep establish_connection for running migrations
- **API surface**: Core's DatabaseService becomes public API
- **Feature flags**: May need conditional compilation for test utilities

## Status Updates **[REQUIRED]**

### 2025-10-25: Implementation Complete

Successfully completed all phases of the database connection standardization:

**Phase 1: Core DatabaseService Creation**
- Created `/crates/mimir-dm-core/src/db.rs` with r2d2 pool management
- Added r2d2 dependency to workspace and core Cargo.toml
- Exported DatabaseService from mimir-dm-core/src/lib.rs
- Proper handling of in-memory vs file-based databases

**Phase 2: Core Services Update**
- All core services now use pooled connections directly
- Removed database_url parameters from service methods
- Core services maintain clean signatures accepting `&mut DbConnection`

**Phase 3: UI Layer Update**
- Updated 40+ command files to import DatabaseService from core
- Fixed all imports: `use mimir_dm_core::DatabaseService;`
- Updated main.rs to initialize core's DatabaseService
- Removed `/crates/mimir-dm/src/db_connection.rs`
- Removed `/crates/mimir-dm/src/services/database.rs`
- Removed module declarations from main.rs and services/mod.rs

**Phase 4: Cleanup**
- Removed async wrappers `with_connection` and `with_transaction` from core/src/connection.rs
- Kept `establish_connection()` for migration runner
- Fixed type mismatches in sessions.rs, stage_transitions.rs, and seed_templates.rs
- All error handling properly converts anyhow::Error to String where needed

**Verification:**
- cargo check: ✓ Passes with only 1 unrelated warning
- cargo test --lib: ✓ All 81 tests pass (63 core + 18 LLM)
- No compilation errors
- Proper architectural layering achieved

**Architecture After Changes:**
- Core layer (mimir-dm-core) owns DatabaseService and connection pool
- UI layer (mimir-dm) imports and consumes DatabaseService from core
- Clean separation of concerns with proper layering
- Simplified connection lifecycle management