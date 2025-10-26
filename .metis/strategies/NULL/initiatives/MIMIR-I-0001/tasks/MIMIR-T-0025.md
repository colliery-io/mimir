---
id: standardize-error-handling
level: task
title: "Standardize error handling patterns across codebase"
short_code: "MIMIR-T-0025"
created_at: 2025-10-24T11:54:16.072497+00:00
updated_at: 2025-10-24T11:54:16.072497+00:00
parent: MIMIR-I-0001
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/todo"


exit_criteria_met: false
strategy_id: NULL
initiative_id: MIMIR-I-0001
---

# Standardize error handling patterns across codebase

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0001]]

## Objective **[REQUIRED]**

Standardize error handling patterns across the Rust backend and TypeScript frontend to ensure consistent error propagation, user-facing error messages, and debugging experience. Currently, the codebase has multiple inconsistent approaches to error handling, making it difficult to maintain and debug.

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

**Current Problems**:
- **Mixed error types in Rust**: Commands use `Result<ApiResponse<T>, String>`, core uses `Result<T, DbError>`, some services use `anyhow::Result`
- **Inconsistent TypeScript patterns**:
  - Some services wrap and re-throw with custom messages: `throw new Error(`Failed to ${op}: ${err}`)`
  - Some re-throw original error: `throw error`
  - Some log and swallow: `console.error(...)` without re-throwing
  - Some have no error handling at all (rely on natural propagation)
  - Special cases use `console.warn` for non-critical errors
- **Lost error context**: Converting DbError to String in commands loses structured error information
- **Inconsistent user-facing messages**: No standard format for error messages shown to users
- **Difficult debugging**: Mixed patterns make it hard to trace errors through the stack
- **No error tracking**: No centralized error logging or monitoring

**Files Affected**:
- Rust: 40+ command files, 39+ core files with Result/Error types, 8+ service files using anyhow
- TypeScript: 68+ files with try/catch blocks, multiple service files with different patterns

**Benefits of Fixing**:
- **Consistent error experience**: Users see helpful, consistent error messages
- **Better debugging**: Structured errors with context make it easier to diagnose issues
- **Easier maintenance**: Single pattern to follow when writing new code
- **Potential for error tracking**: Standardized errors enable centralized logging/monitoring
- **Type safety**: Proper error types instead of String provide compile-time checking
- **Error recovery**: Structured errors enable smarter error handling and recovery

**Risk Assessment**:
- **Low risk**: Mostly internal refactoring, minimal user-facing changes
- **High value**: Significantly improves developer experience and code quality
- **Incremental approach**: Can be done layer-by-layer (core → services → commands → frontend)

## Acceptance Criteria **[REQUIRED]**

- [ ] All Rust error patterns documented and analyzed
- [ ] All TypeScript error patterns documented and analyzed
- [ ] Standardized error handling approach designed and documented
- [ ] Rust error handling guidelines created (when to use which error type)
- [ ] TypeScript error handling guidelines created (service vs composable vs component)
- [ ] Example implementations provided for each layer
- [ ] Migration strategy documented (incremental approach)
- [ ] No breaking changes to existing functionality
- [ ] Error messages remain clear and actionable for users

## Implementation Notes

### Current Error Patterns - Rust Backend

**1. Core Layer (mimir-dm-core)**
```rust
// mimir-dm-core/src/error.rs (86 lines)
pub type Result<T> = std::result::Result<T, DbError>;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Database connection error: {0}")]
    Connection(#[from] diesel::ConnectionError),

    #[error("Database query error: {0}")]
    Query(#[from] diesel::result::Error),

    #[error("Entity not found: {entity_type} with id '{id}'")]
    NotFound { entity_type: String, id: String },

    #[error("Constraint violation: {field} - {message}")]
    ConstraintViolation { field: String, message: String },

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Invalid data: {0}")]
    InvalidData(String),
    // ... more variants
}

// Helper methods for error inspection
impl DbError {
    pub fn is_unique_violation(&self) -> bool { /* ... */ }
    pub fn is_foreign_key_violation(&self) -> bool { /* ... */ }
    pub fn is_not_found(&self) -> bool { /* ... */ }
}
```

**Status**: Well-structured, uses thiserror, provides helper methods. **Recommendation: Keep and expand.**

**2. Service Layer (mimir-dm-core/src/services)**
```rust
// Example: campaign_service.rs
pub fn create_campaign(&self, request: &CreateCampaignRequest) -> Result<Campaign> {
    // Uses DbError::InvalidData for validation
    if request.name.trim().is_empty() {
        return Err(DbError::InvalidData("Campaign name cannot be empty".to_string()))
    }
    // Returns Result<Campaign, DbError>
}
```

**Status**: Consistent use of DbError. **Recommendation: Keep pattern.**

**3. API Layer (mimir-dm/src/types.rs)**
```rust
#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Database error: {0}")]
    Database(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Validation error: {0}")]
    Validation(String),
    // ... more variants
}

// Conversion from DbError to ApiError
impl From<DbError> for ApiError {
    fn from(err: DbError) -> Self {
        match err {
            DbError::NotFound { entity_type, id } => {
                ApiError::NotFound(format!("{} with id '{}' not found", entity_type, id))
            }
            DbError::ConstraintViolation { field, message } => {
                ApiError::Validation(format!("{}: {}", field, message))
            }
            _ => ApiError::Database(err.to_string()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}
```

**Status**: Good conversion layer, but ApiResponse structure is not consistently used. **Recommendation: Standardize ApiResponse usage.**

**4. Command Layer (mimir-dm/src/commands)**
```rust
// Current pattern (40+ files):
#[tauri::command]
pub fn some_command(request: SomeRequest) -> Result<ApiResponse<SomeData>, String> {
    let result = service.do_something(&request)
        .map_err(|e| e.to_string())?;  // Converts DbError to String

    Ok(ApiResponse::success(result))
}
```

**Problems**:
- Converts structured errors to String, losing context
- No consistent error message format
- Can't distinguish error types on frontend
- No structured error logging

**Recommendation**: Use `Result<ApiResponse<T>, ApiError>` and implement Serialize for ApiError

**5. Some Services Use anyhow (8+ files)**
```rust
use anyhow::Result;

pub fn some_function() -> Result<Data> {
    // Uses anyhow for flexible error handling
}
```

**Status**: Useful for flexibility but inconsistent with rest of codebase. **Recommendation: Use only for internal utilities, not public APIs.**

### Current Error Patterns - TypeScript Frontend

**1. Service Layer - Pattern A: Wrap and Re-throw**
```typescript
// ModuleService.ts (most common pattern)
async get(id: number): Promise<Module> {
  try {
    const response = await invoke<{ data: Module }>('get_module', { id })
    return response.data
  } catch (error) {
    throw new Error(`Failed to get module: ${error}`)
  }
}
```

**2. Service Layer - Pattern B: No Error Handling**
```typescript
// SearchService.ts (let errors propagate naturally)
async search(params: SearchParams): Promise<any[]> {
  // No try/catch - errors propagate to caller
  return await this.searchSpells(query, sources, filters)
}
```

**3. Composable Layer - Pattern C: Re-throw Original**
```typescript
// useModuleStage.ts
async function transitionToNextStage() {
  try {
    await ModuleService.updateStatus(module.value.id, nextStageKey)
    window.location.reload()
  } catch (error) {
    throw error  // Re-throw without wrapping
  }
}
```

**4. Store Layer - Pattern D: Log and Swallow**
```typescript
// sharedContext.ts
const syncToBackend = async (contextType: string, data: any) => {
  try {
    await invoke('update_context', { /* ... */ })
  } catch (error) {
    console.error('Failed to sync context to backend:', error)
    // Does not re-throw - error is swallowed
  }
}
```

**5. Special Case - Pattern E: Warn for Non-Critical**
```typescript
// ModuleService.ts
async incrementSessionCount(id: number): Promise<void> {
  try {
    await invoke('increment_module_sessions', { module_id: id })
  } catch (error) {
    // Non-critical error - log warning but don't throw
    console.warn(`Failed to increment session count: ${error}`)
  }
}
```

**Files Affected**: 68+ files with try/catch blocks across stores, services, composables, and components

### Proposed Standardized Approach

#### Rust Backend

**1. Keep DbError for Core Layer**
- Continue using `Result<T, DbError>` in mimir-dm-core
- Expand DbError variants as needed
- Add more helper methods for error inspection

**2. Enhance ApiError for API Layer**
```rust
#[derive(Error, Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "message")]
pub enum ApiError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

// Implement Serialize/Deserialize for structured error responses
```

**3. Update Commands to Use ApiError**
```rust
#[tauri::command]
pub fn some_command(request: SomeRequest) -> Result<ApiResponse<SomeData>, ApiError> {
    let result = service.do_something(&request)?;  // ApiError::from(DbError)
    Ok(ApiResponse::success(result))
}
```

**4. Standardize ApiResponse**
```rust
impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self { success: true, data: Some(data), error: None }
    }

    pub fn error(error: ApiError) -> Self {
        Self { success: false, data: None, error: Some(error) }
    }
}
```

#### TypeScript Frontend

**1. Service Layer: Always Wrap with Context**
```typescript
// Services should add context to errors
async get(id: number): Promise<Module> {
  try {
    const response = await invoke<{ data: Module }>('get_module', { id })
    return response.data
  } catch (error) {
    throw new Error(`Failed to get module ${id}: ${error}`)
  }
}
```

**2. Composable Layer: Let Errors Propagate or Add Minimal Context**
```typescript
// Composables generally let errors bubble up
// Only catch if adding valuable context or handling specially
async function transitionToNextStage() {
  // No try/catch - let error propagate to component
  await ModuleService.updateStatus(module.value.id, nextStageKey)
  window.location.reload()
}
```

**3. Component Layer: Catch and Display to User**
```typescript
// Components catch errors and show user-friendly messages
async function handleAction() {
  try {
    await moduleComposable.transitionToNextStage()
  } catch (error) {
    // Show user-friendly error message
    errorMessage.value = error.message || 'An error occurred'
  }
}
```

**4. Store Layer: Decide Based on Criticality**
```typescript
// Critical operations: throw errors
async updateCampaign(data: CampaignContext) {
  try {
    await syncToBackend('campaign', data)
  } catch (error) {
    throw new Error(`Failed to update campaign: ${error}`)
  }
}

// Non-critical operations: log and continue
async syncMetrics() {
  try {
    await invoke('sync_metrics')
  } catch (error) {
    console.warn('Failed to sync metrics:', error)
    // Don't throw - metrics sync is non-critical
  }
}
```

### Migration Strategy

**Phase 1: Enhance Error Types (Low Risk)**
1. Make ApiError Serializable/Deserializable
2. Add new ApiError variants as needed
3. Improve error messages in DbError variants
4. No changes to function signatures yet

**Phase 2: Update Commands Layer (Medium Impact)**
1. Change command signatures from `Result<T, String>` to `Result<T, ApiError>`
2. Update ~40 command files incrementally
3. Test each batch of changes
4. Frontend should handle both String and structured errors during transition

**Phase 3: Standardize Frontend Patterns (Low Risk)**
1. Document error handling guidelines for each layer
2. Update service layer files to follow wrap-and-throw pattern
3. Update composables to minimize error handling
4. Update components to catch and display user-friendly messages
5. Can be done incrementally per feature area

**Phase 4: Add Error Tracking (Optional Enhancement)**
1. Add centralized error logging on backend
2. Add error reporting on frontend
3. Consider error monitoring service integration

### Dependencies

- No blocking dependencies
- Can start immediately with documentation and planning
- Implementation can be done incrementally

### Risk Considerations

**Low Risk Items**:
- Adding Serialize/Deserialize to ApiError
- Documenting error handling guidelines
- Updating frontend services to follow consistent patterns

**Medium Risk Items**:
- Changing command return types from String to ApiError
  - Mitigation: Test thoroughly, deploy incrementally
  - Rollback: Easy to revert individual commands

**Testing Strategy**:
- Manual testing of error scenarios in each updated command
- Verify error messages are clear and actionable
- Check that frontend displays errors appropriately
- No breaking changes to successful response paths

## Status Updates **[REQUIRED]**

### Investigation Complete - 2025-10-26

Completed comprehensive analysis of error handling patterns across the codebase.

**Rust Backend Findings**:
- Core layer (mimir-dm-core): Well-structured DbError with thiserror - KEEP
- Service layer: Consistent DbError usage - KEEP
- API layer: ApiError exists but needs Serialize/Deserialize - ENHANCE
- Command layer (40+ files): Uses Result<T, String>, loses context - NEEDS UPDATE
- 8+ files use anyhow::Result - USE SPARINGLY

**TypeScript Frontend Findings** (68+ files with try/catch):
- Pattern A: Wrap and re-throw with context (ModuleService) - RECOMMENDED for services
- Pattern B: No error handling, natural propagation (SearchService) - DISCOURAGED
- Pattern C: Re-throw original error (useModuleStage) - OK for composables
- Pattern D: Log and swallow (sharedContext) - OK for non-critical operations
- Pattern E: Warn for non-critical (incrementSessionCount) - OK for background tasks

**Proposed Approach**:
- Phase 1: Make ApiError Serializable (low risk)
- Phase 2: Update command signatures to use ApiError instead of String (medium impact)
- Phase 3: Standardize frontend patterns by layer (low risk)
- Phase 4: Optional - add centralized error tracking

**Key Recommendations**:
- Rust: Keep DbError for core, enhance ApiError for API layer, update commands to use ApiError
- TypeScript: Services wrap errors with context, composables propagate, components display to users
- Migration: Incremental, layer-by-layer approach minimizes risk

All acceptance criteria for investigation and documentation phase met:
- [x] All Rust error patterns documented and analyzed
- [x] All TypeScript error patterns documented and analyzed
- [x] Standardized error handling approach designed and documented
- [x] Rust error handling guidelines created (when to use which error type)
- [x] TypeScript error handling guidelines created (service vs composable vs component)
- [x] Example implementations provided for each layer
- [x] Migration strategy documented (incremental approach)

**Next Steps**: This task is complete for planning/analysis phase. Implementation of standardized patterns should be tracked in separate tasks for each phase:
- Create T-0025-P1: Make ApiError Serializable
- Create T-0025-P2: Update command layer to use ApiError
- Create T-0025-P3: Standardize frontend error handling patterns