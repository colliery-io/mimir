---
id: phase-2-update-command-layer-to
level: task
title: "Phase 2: Update command layer to use ApiError instead of String"
short_code: "MIMIR-T-0035"
created_at: 2025-10-26T11:21:04.167237+00:00
updated_at: 2025-10-26T11:21:04.167237+00:00
parent: MIMIR-I-0001
blocked_by: [MIMIR-T-0034]
archived: false

tags:
  - "#task"
  - "#phase/todo"


exit_criteria_met: false
strategy_id: NULL
initiative_id: MIMIR-I-0001
---

# Phase 2: Update command layer to use ApiError instead of String

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0001]]

## Objective **[REQUIRED]**

Update all command handlers to use ApiError instead of String for error returns, enabling structured error responses to the frontend. This preserves error type information across the Rust/TypeScript boundary and enables better error handling on the frontend.

Part of error handling standardization effort (see MIMIR-T-0025 for full design). Requires MIMIR-T-0034 to be completed first.

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
- 40+ command files return Result<ApiResponse<T>, String>
- Converting DbError to String loses error type information
- Frontend cannot distinguish between NotFound, Validation, Database errors
- Inconsistent error message formatting across commands
- Debugging is difficult without structured error context

**Benefits of Fixing**:
- Structured errors sent to frontend (type + message)
- Frontend can handle different error types appropriately
- Better error messages for users (type-specific handling)
- Easier debugging with preserved error context
- Foundation for centralized error logging/monitoring
- Type safety in error handling

**Risk Assessment**:
- Medium impact: Changes 40+ command files
- Low risk with proper testing: Frontend already handles error messages
- Incremental approach: Update in batches, test each batch
- Easy rollback: Changes are localized to command files
- No user-facing changes to success paths

## Acceptance Criteria **[REQUIRED]**

- [ ] All command signatures changed from Result<ApiResponse<T>, String> to Result<ApiResponse<T>, ApiError>
- [ ] All .map_err(|e| e.to_string()) replaced with proper ApiError conversion using ?
- [ ] Campaign commands updated (8 files)
- [ ] Module commands updated (6 files)
- [ ] Session commands updated (4 files)
- [ ] Document commands updated (5 files)
- [ ] Book/catalog commands updated (10 files)
- [ ] Remaining commands updated (7 files)
- [ ] cargo check passes with no errors
- [ ] cargo test passes all tests
- [ ] Manual testing of error scenarios (NotFound, Validation, Database errors)
- [ ] Error messages remain clear and actionable to users
- [ ] No breaking changes to successful response paths

## Implementation Notes

### Technical Approach

**Current Pattern** (40+ command files):
```rust
#[tauri::command]
pub fn get_campaign(id: i32) -> Result<ApiResponse<Campaign>, String> {
    let campaign = campaign_service.get(id)
        .map_err(|e| e.to_string())?;  // Loses error type info
    Ok(ApiResponse::success(campaign))
}
```

**New Pattern**:
```rust
#[tauri::command]
pub fn get_campaign(id: i32) -> Result<ApiResponse<Campaign>, ApiError> {
    let campaign = campaign_service.get(id)?;  // Auto-converts DbError -> ApiError
    Ok(ApiResponse::success(campaign))
}
```

**Implementation Strategy**:

1. **Batch 1: Campaign Commands** (8 files in crates/mimir-dm/src/commands/campaigns.rs area)
   - Update signatures
   - Remove .map_err(|e| e.to_string())
   - Test campaign CRUD operations

2. **Batch 2: Module Commands** (6 files)
   - Similar pattern to campaigns
   - Test module lifecycle operations

3. **Batch 3: Session Commands** (4 files)
   - Update session management commands
   - Test session operations

4. **Batch 4: Document Commands** (5 files)
   - Update document operations
   - Test document management

5. **Batch 5: Book/Catalog Commands** (10 files)
   - Update search and catalog commands
   - Test catalog operations

6. **Batch 6: Remaining Commands** (7 files)
   - Context, settings, utility commands
   - Final verification

**Testing Strategy for Each Batch**:
1. Update command signatures
2. Run cargo check
3. Run cargo test
4. Manual testing of error scenarios:
   - Trigger NotFound error (invalid ID)
   - Trigger Validation error (empty name)
   - Trigger Database error (if possible)
5. Verify error messages are clear
6. Commit batch before moving to next

### Dependencies

- **Blocked by**: MIMIR-T-0034 (Phase 1 - ApiError serialization) - MUST BE COMPLETE
- **Blocks**: MIMIR-T-0036 (Phase 3 - frontend patterns)
- **No external dependencies**: All work in command files

### Risk Considerations

**Medium Impact, Low Risk**:
- Changes many files but pattern is consistent
- DbError -> ApiError conversion already exists in From impl
- Success paths unchanged
- Frontend already handles error.message

**Mitigation**:
- Incremental batched approach with testing between batches
- Each batch is independently committable
- Easy to identify which batch has issues
- Can pause/rollback individual batches if problems arise

**Manual Testing Required**:
- Test each error type (NotFound, Validation, Database)
- Verify frontend displays errors correctly
- Ensure no user-facing regressions



## Status Updates **[REQUIRED]**

*To be added during implementation*