---
id: move-business-logic-from-command
level: task
title: "Move business logic from command handlers to core services"
short_code: "MIMIR-T-0021"
created_at: 2025-10-24T11:54:04.077833+00:00
updated_at: 2025-10-25T16:05:45.491503+00:00
parent: MIMIR-I-0001
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/active"


exit_criteria_met: false
strategy_id: NULL
initiative_id: MIMIR-I-0001
---

# Move business logic from command handlers to core services

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0001]]

## Objective **[REQUIRED]**

Refactor command handlers to be thin facades that delegate business logic to core services. Currently, many command handlers in `mimir-dm/src/commands` contain business logic, file I/O, and data manipulation that should reside in `mimir-dm-core/src/services`. This violates proper architectural layering and makes code harder to test and maintain.

### Technical Debt Impact

- **Current Problems**:
  - Command handlers contain business logic instead of being thin facades
  - Core services can't be tested independently from Tauri framework
  - Duplicate logic across similar command handlers
  - Difficult to reuse business logic in different contexts (CLI, tests, etc.)
  - Mixing of concerns: Tauri-specific code with business logic

- **Benefits of Fixing**:
  - Clean separation of concerns: Commands handle I/O, services handle logic
  - Core services become framework-agnostic and easily testable
  - Reusable business logic across different interfaces
  - Easier to mock and unit test
  - Clearer code organization following established patterns

- **Risk Assessment**:
  - Low risk - mostly moving code between layers
  - Well-established pattern already exists for database-backed catalogs
  - Tests will verify no functional regressions
  - Can be done incrementally file by file

## Acceptance Criteria

## Acceptance Criteria **[REQUIRED]**

- [ ] All campaign management logic moved from commands/campaigns.rs to core CampaignService
- [ ] All module management logic moved from commands/modules.rs to core ModuleService  
- [ ] All session management logic moved from commands/sessions.rs to core SessionService
- [ ] All document management logic moved from commands/documents.rs to core DocumentService
- [ ] Command handlers are thin facades (<50 lines per command)
- [ ] Core services contain all business logic and validations
- [ ] All existing tests still pass
- [ ] Core services can be tested without Tauri framework

## Implementation Notes

### Current Architecture (Problem)

**Command Handler with Business Logic (Anti-pattern):**
```rust
// commands/campaigns.rs - BAD: Business logic in command handler
#[tauri::command]
pub async fn create_campaign(
    request: CreateCampaignRequest,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<ApiResponse<Campaign>, String> {
    let mut conn = db_service.get_connection()?;
    
    // Business logic shouldn't be here!
    let campaign_dir = PathBuf::from(&request.directory);
    if !campaign_dir.exists() {
        fs::create_dir_all(&campaign_dir)?;
    }
    
    // Validation logic shouldn't be here!
    if request.name.trim().is_empty() {
        return Err("Campaign name cannot be empty".to_string());
    }
    
    // Data transformation shouldn't be here!
    let new_campaign = NewCampaign {
        name: request.name.trim().to_string(),
        directory_path: campaign_dir.to_string_lossy().to_string(),
        status: "concept".to_string(),
    };
    
    // Direct DB access in command handler!
    let mut repo = CampaignRepository::new(&mut conn);
    let campaign = repo.create(new_campaign)?;
    
    Ok(ApiResponse::success(campaign))
}
```

### Target Architecture (Solution)

**Thin Command Handler:**
```rust
// commands/campaigns.rs - GOOD: Thin facade
#[tauri::command]
pub async fn create_campaign(
    request: CreateCampaignRequest,
    db_service: State<'_, Arc<DatabaseService>>,
) -> Result<ApiResponse<Campaign>, String> {
    let mut conn = db_service.get_connection()
        .map_err(|e| format!("Database connection failed: {}", e))?;
    
    // Just delegate to service
    let campaign = CampaignService::create(
        &mut conn,
        &request.name,
        &request.directory,
    ).map_err(|e| format!("Failed to create campaign: {}", e))?;
    
    Ok(ApiResponse::success(campaign))
}
```

**Core Service with Business Logic:**
```rust
// mimir-dm-core/src/services/campaign_service.rs - GOOD: All logic here
impl CampaignService {
    pub fn create(
        conn: &mut DbConnection,
        name: &str,
        directory: &str,
    ) -> Result<Campaign> {
        // Validation
        if name.trim().is_empty() {
            return Err(DbError::Validation("Campaign name cannot be empty".into()));
        }
        
        // Business logic
        let campaign_dir = PathBuf::from(directory);
        if !campaign_dir.exists() {
            fs::create_dir_all(&campaign_dir)
                .context("Failed to create campaign directory")?;
        }
        
        // Data transformation
        let new_campaign = NewCampaign {
            name: name.trim().to_string(),
            directory_path: campaign_dir.to_string_lossy().to_string(),
            status: "concept".to_string(),
        };
        
        // Data access
        let mut repo = CampaignRepository::new(conn);
        let campaign = repo.create(new_campaign)?;
        
        Ok(campaign)
    }
}
```

### Files Requiring Changes

**Command Handlers (UI Layer) - Make Thin:**
1. `/crates/mimir-dm/src/commands/campaigns.rs` - Delegate to CampaignService
2. `/crates/mimir-dm/src/commands/modules.rs` - Delegate to ModuleService
3. `/crates/mimir-dm/src/commands/sessions.rs` - Delegate to SessionService
4. `/crates/mimir-dm/src/commands/documents.rs` - Delegate to DocumentService

**Core Services (Core Layer) - Add Business Logic:**
1. `/crates/mimir-dm-core/src/services/campaign_service.rs` - Move campaign logic here
2. `/crates/mimir-dm-core/src/services/module_service.rs` - Move module logic here
3. `/crates/mimir-dm-core/src/services/session_service.rs` - Move session logic here
4. `/crates/mimir-dm-core/src/services/document_service.rs` - Move document logic here (may need to create)

### Implementation Steps

**Step 1: Analyze Current Command Handlers**
1. Read each command file to identify business logic
2. List all validations, transformations, and business rules
3. Identify what should move to core services

**Step 2: Enhance Core Services**
1. Add methods to core services for each business operation
2. Move validation logic to core services
3. Move file I/O operations to core services
4. Move data transformation logic to core services
5. Ensure services return proper Result types with context

**Step 3: Refactor Command Handlers**
1. Update command handlers to call core service methods
2. Reduce command handlers to <50 lines (just I/O + delegation)
3. Keep only: connection management, error mapping, response wrapping
4. Remove all business logic from command handlers

**Step 4: Verify**
1. Run full test suite
2. Verify all commands still work
3. Check that core services can be tested independently

### Pattern to Follow

**What Stays in Command Handler:**
- Tauri State extraction (`State<'_, Arc<DatabaseService>>`)
- Connection management (`db_service.get_connection()`)
- Error format conversion (`map_err(|e| format!(...))`)
- Response wrapping (`ApiResponse::success(...)`)

**What Moves to Core Service:**
- Business validations
- File I/O operations
- Data transformations
- Business rules and logic
- Repository interactions
- Transaction management

### Dependencies

- Depends on MIMIR-T-0020 (database connection standardization) - ✓ Complete
- Core services must be enhanced before refactoring commands
- May need to create DocumentService if it doesn't exist

### Risk Considerations

- **Testing Gap**: Ensure existing tests cover the business logic being moved
- **Error Handling**: Maintain same error messages for backward compatibility
- **Transaction Boundaries**: Ensure transactions are preserved when moving logic
- **File Operations**: Ensure file I/O operations maintain same behavior (paths, permissions)
- **Validation**: Ensure all validations are moved, none left behind in commands

## Status Updates **[REQUIRED]**

### 2025-10-25: Task Prepared and Ready

Task document has been fully prepared with:
- Clear objectives and acceptance criteria
- Detailed current vs target architecture examples
- Step-by-step implementation plan
- Pattern guidelines for what stays vs what moves
- Risk considerations and dependencies

**Prerequisite Completed:**
- MIMIR-T-0020 (Database connection standardization) ✓ Complete
- All command handlers now use `mimir_dm_core::DatabaseService`
- Proper architectural foundation established

**Ready to Begin Implementation**