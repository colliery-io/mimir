---
id: refactor-global-state-to-use-tauri
level: task
title: "Refactor global state to use Tauri state management"
short_code: "MIMIR-T-0022"
created_at: 2025-10-24T11:54:04.170129+00:00
updated_at: 2025-10-26T01:17:16.769671+00:00
parent: MIMIR-I-0001
blocked_by: []
archived: true

tags:
  - "#task"
  - "#phase/completed"


exit_criteria_met: true
strategy_id: NULL
initiative_id: MIMIR-I-0001
---

# Refactor global state to use Tauri state management

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0001]]

## Objective **[REQUIRED]**

Refactor global `APP_PATHS` (OnceLock) to use Tauri's built-in state management system. Currently, application paths are stored in a global static variable accessed throughout the codebase, which violates dependency injection principles and makes testing difficult. This refactoring will move `AppPaths` to Tauri managed state and update all ~20+ usage sites to access it via proper state injection.

### Type
- [ ] Bug - Production issue that needs fixing
- [ ] Feature - New functionality or enhancement  
- [x] Tech Debt - Code improvement or refactoring
- [ ] Chore - Maintenance or setup work

### Priority
- [ ] P0 - Critical (blocks users/revenue)
- [ ] P1 - High (important for user experience)
- [x] P2 - Medium (nice to have)
- [ ] P3 - Low (when time permits)

### Technical Debt Impact

**Current Problems:**
- Global static variable (`APP_PATHS`) accessed via OnceLock violates dependency injection
- ~20+ direct accesses to global state throughout codebase
- Difficult to test code that depends on global state (can't easily mock or override)
- No compile-time guarantee that APP_PATHS is initialized when accessed
- Implicit dependencies hidden from function signatures
- Services (llm_service, tools) directly access global instead of receiving via parameters

**Benefits of Fixing:**
- Explicit dependencies via Tauri State injection
- Easier to test: can inject test AppPaths without affecting global state
- Type-safe access: Tauri ensures state is initialized
- Consistent with Tauri best practices (already using for DatabaseService, ContextState, etc.)
- Better IDE support: dependencies visible in function signatures
- Enables future refactoring: services become more portable and reusable

**Risk Assessment:**
- Low risk: purely internal refactoring, no functional changes
- Well-established pattern already in use for DatabaseService
- Changes are mechanical: replace global access with State parameter
- Can be tested incrementally file-by-file

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria **[REQUIRED]**

- [x] `APP_PATHS` global static removed from main.rs
- [x] `AppPaths` registered as Tauri managed state in app setup
- [x] All command handlers updated to use `State<'_, AppPaths>` injection
- [x] All service methods updated to accept `AppPaths` as parameter (no global access)
- [x] All ~20+ usage sites updated to use state injection
- [x] No compilation errors or warnings related to AppPaths
- [x] All existing tests pass
- [x] Application starts and runs correctly with new state management

## Implementation Notes

### Current Architecture

**Global State Pattern (Current - Anti-pattern):**
```rust
// main.rs
pub static APP_PATHS: OnceLock<AppPaths> = OnceLock::new();

fn main() {
    let app_paths = initialize_app().unwrap();
    APP_PATHS.set(app_paths).unwrap();
    // ...
}

// commands/logs.rs
use crate::APP_PATHS;

#[tauri::command]
pub async fn get_log_files() -> Result<Vec<String>, String> {
    let app_paths = APP_PATHS.get().ok_or("App not initialized")?;
    // use app_paths...
}
```

**Problems:**
- Global mutable state (OnceLock)
- No dependency injection
- Hidden dependencies
- Hard to test
- Runtime panics possible if accessed before initialization

### Target Architecture

**Tauri State Management Pattern (Target - Best Practice):**
```rust
// main.rs
fn main() {
    let app_paths = initialize_app().unwrap();
    
    tauri::Builder::default()
        .setup(|app| {
            app.manage(app_paths); // Register as Tauri state
            // ...
        })
        // ...
}

// commands/logs.rs  
#[tauri::command]
pub async fn get_log_files(
    app_paths: State<'_, AppPaths>
) -> Result<Vec<String>, String> {
    // Use app_paths directly - no global access!
    // Tauri ensures it's initialized
}
```

**Benefits:**
- Explicit dependencies in function signatures
- Type-safe: Tauri ensures state exists
- Easy to test: can inject test AppPaths
- No runtime panics
- Consistent with existing DatabaseService pattern

### Files Requiring Updates

**Files Accessing APP_PATHS (20+ locations):**

1. **main.rs** (2 uses)
   - Remove global static declaration
   - Add `app.manage(app_paths)` in setup

2. **commands/logs.rs** (4 uses)
   - `get_log_files()` - add State parameter
   - `get_log_file_content()` - add State parameter
   - `tail_log_file()` - add State parameter
   - `search_logs()` - add State parameter

3. **commands/app_info.rs** (1 use)
   - `get_app_info()` - add State parameter

4. **commands/dev_tools.rs** (1 use)
   - `clear_app_data()` - add State parameter

5. **commands/books/book_reference.rs** (1 use)
   - `get_book_reference_file()` - add State parameter

6. **commands/books/book_upload.rs** (1 use)
   - `extract_uploaded_book()` - add State parameter

7. **commands/books/book_content.rs** (2 uses)
   - `get_book_content()` - add State parameter
   - `search_book_content()` - add State parameter

8. **services/llm/llm_service.rs** (1 use)
   - `save_conversation_to_file()` method - add AppPaths parameter

9. **services/tools/mod.rs** (3 uses)
   - `generate_system_rules()` - add AppPaths parameter
   - Update callers to pass AppPaths

### Refactoring Steps

**Step 1: Add State Management in main.rs**
1. Remove `pub static APP_PATHS: OnceLock<AppPaths> = OnceLock::new();`
2. Remove `APP_PATHS.set(app_paths).unwrap();`
3. Change `app.manage(db_service)` section to also include:
   ```rust
   let app_paths_state = Arc::new(app_paths);
   app.manage(app_paths_state);
   ```

**Step 2: Update Command Handlers**
For each command handler:
1. Remove `use crate::APP_PATHS;`
2. Add parameter: `app_paths: State<'_, Arc<AppPaths>>`
3. Replace `APP_PATHS.get().ok_or(...)?` with direct `app_paths` usage

**Step 3: Update Service Methods**
For llm_service and tools:
1. Add `app_paths: &AppPaths` parameter to methods
2. Update callers to pass app_paths from State
3. Remove `use crate::APP_PATHS;`

**Step 4: Verify and Test**
1. Run `cargo check` to verify no compilation errors
2. Run test suite
3. Manual smoke test: start app, verify logging works, verify book operations work

### Pattern to Follow

**Before (Command Handler):**
```rust
use crate::APP_PATHS;

#[tauri::command]
pub async fn some_command() -> Result<String, String> {
    let app_paths = APP_PATHS.get()
        .ok_or("App not initialized")?;
    
    let data_dir = &app_paths.data_dir;
    // use data_dir...
}
```

**After (Command Handler):**
```rust
use std::sync::Arc;

#[tauri::command]
pub async fn some_command(
    app_paths: State<'_, Arc<AppPaths>>
) -> Result<String, String> {
    let data_dir = &app_paths.data_dir;
    // use data_dir...
}
```

**Before (Service Method):**
```rust
impl SomeService {
    fn method(&self) {
        let app_paths = crate::APP_PATHS.get().unwrap();
        // use app_paths...
    }
}
```

**After (Service Method):**
```rust
impl SomeService {
    fn method(&self, app_paths: &AppPaths) {
        // use app_paths...
    }
}
```

### Dependencies

- No blocking dependencies
- Builds on patterns established in MIMIR-T-0020 (database connection standardization)

### Risk Considerations

- **Compilation errors**: Easy to fix by following compiler errors for missing State parameters
- **Runtime errors**: Tauri guarantees state exists, no runtime panics
- **Testing**: Existing tests should continue to pass; may need to update test setup if tests call refactored functions
- **Rollback**: Easy - can revert single commit if issues arise

## Status Updates **[REQUIRED]**

### Completion - 2025-10-26

Successfully refactored all global `APP_PATHS` usage to Tauri state management. All acceptance criteria met:

**Files Modified:**
1. `main.rs` - Removed global APP_PATHS, registered as Tauri state, passed to initialize_llm
2. `commands/app_info.rs` - Updated get_app_info to use State injection
3. `commands/dev_tools.rs` - Updated remove_dev_test_book to use State injection
4. `commands/logs.rs` - Updated 4 functions (list_log_files, read_log_file, tail_log_file, open_logs_folder)
5. `commands/books/book_content.rs` - Updated 2 functions (get_book_content, serve_book_image)
6. `commands/books/book_reference.rs` - Updated lookup_reference
7. `commands/books/book_upload.rs` - Updated upload_book_archive
8. `services/llm/llm_service.rs` - Added app_paths field to LlmService struct, updated constructor and initialize_llm, refactored get_chat_logger
9. `services/tools/mod.rs` - Removed diagnostic APP_PATHS logging from generate_system_rules_with_directory

**Pattern Applied:**
- Commands: Added `app_paths: State<'_, Arc<AppPaths>>` parameter
- Services: Added `app_paths: Arc<AppPaths>` field to struct, updated constructor
- Replaced all `APP_PATHS.get().ok_or(...)?` with direct state access

**Test Results:**
- `cargo check --workspace`: No errors, only 1 unrelated warning in mimir-5etools-splitter
- `cargo test -p mimir-dm`: All 8 tests pass (2 chat_logger tests, 6 LLM integration tests)
- Fixed unused import warning in main.rs

**Verification:**
- No remaining APP_PATHS references in code (only in this task document)
- All Tauri commands use explicit State injection
- Services receive AppPaths via dependency injection
- Pattern consistent with existing DatabaseService state management