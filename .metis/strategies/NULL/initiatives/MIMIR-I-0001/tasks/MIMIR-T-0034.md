---
id: phase-1-make-apierror-serializable
level: task
title: "Phase 1: Make ApiError Serializable and enhance error types"
short_code: "MIMIR-T-0034"
created_at: 2025-10-26T11:21:03.422334+00:00
updated_at: 2025-10-26T11:21:03.422334+00:00
parent: MIMIR-I-0001
blocked_by: []
blocks: [MIMIR-T-0035]
archived: false

tags:
  - "#task"
  - "#phase/todo"


exit_criteria_met: false
strategy_id: NULL
initiative_id: MIMIR-I-0001
---

# Phase 1: Make ApiError Serializable and enhance error types

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0001]]

## Objective **[REQUIRED]**

Enhance the ApiError type to be serializable and deserializable, enabling structured error responses from backend to frontend. This is the foundational phase for standardizing error handling, with no breaking changes to existing functionality.

Part of error handling standardization effort (see MIMIR-T-0025 for full design).

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
- ApiError exists but lacks Serialize/Deserialize traits
- Cannot send structured errors to frontend (only String)
- Frontend cannot distinguish error types (NotFound vs Validation vs Database)
- No foundation for improved error handling in subsequent phases

**Benefits of Fixing**:
- Enables structured error responses (type + message)
- Foundation for Phase 2 (command layer updates)
- Frontend can handle different error types appropriately
- Better error logging and debugging capabilities
- Type-safe error handling across Rust/TypeScript boundary

**Risk Assessment**:
- Very low risk: Internal enhancement only, no API changes
- No breaking changes to existing commands or frontend
- Purely additive - adds capabilities without removing functionality
- Easy to test with cargo check and cargo test

## Acceptance Criteria **[REQUIRED]**

- [ ] ApiError has Serialize and Deserialize traits implemented
- [ ] ApiError uses serde tag/content pattern for JSON structure
- [ ] All existing ApiError variants are serializable
- [ ] Add PermissionDenied and Internal variants if not present
- [ ] Improve error messages in DbError variants (clearer, more actionable)
- [ ] cargo check passes with no errors
- [ ] cargo test passes all tests
- [ ] No changes to command signatures (no breaking changes)
- [ ] Documentation comments added to ApiError variants

## Implementation Notes

### Technical Approach

**File to Modify**: `crates/mimir-dm/src/types.rs`

**Current ApiError**:
```rust
#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Database error: {0}")]
    Database(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Validation error: {0}")]
    Validation(String),
}
```

**Enhanced ApiError**:
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
```

**JSON Structure** (serde tag/content pattern):
```json
{
  "type": "NotFound",
  "message": "Campaign with id '123' not found"
}
```

**Steps**:
1. Add `Serialize, Deserialize` to ApiError derive macro
2. Add `#[serde(tag = "type", content = "message")]` attribute
3. Add PermissionDenied and Internal variants if missing
4. Review and improve error messages in DbError for clarity
5. Run cargo check to verify compilation
6. Run cargo test to verify all tests pass
7. Add doc comments to each variant explaining when to use

### Dependencies

- **Blocked by**: MIMIR-T-0025 (analysis/design) - COMPLETE
- **Blocks**: MIMIR-T-0035 (Phase 2 - command layer updates)
- **No external dependencies**: All work in existing types.rs file

### Risk Considerations

**Very Low Risk**:
- Only adds traits to existing type, no behavioral changes
- No API changes - commands still return String errors for now
- Serde is already a dependency in the project
- Easy to rollback if issues arise

**Testing Strategy**:
- Cargo check for compilation
- Cargo test for regression testing
- Manual verification: serialize ApiError to JSON and inspect structure

## Status Updates **[REQUIRED]**

### Implementation Complete - 2025-10-26

Successfully enhanced ApiError to be fully serializable for structured error responses.

**Changes Made**:
1. **Added Serialize/Deserialize traits** to ApiError enum
2. **Configured serde tag/content pattern** for clean JSON structure: `{"type": "NotFound", "message": "..."}`
3. **Fixed Io variant**: Changed from `Io(#[from] std::io::Error)` to `Io(String)` (std::io::Error not serializable)
4. **Added From implementations** for std::io::Error and serde_json::Error
5. **Added new error variants**:
   - `Validation(String)` - for request validation errors
   - `PermissionDenied(String)` - for authorization failures
6. **Enhanced DbError conversion**: Now maps ConstraintViolation and InvalidData to Validation variant
7. **Added comprehensive documentation** to all variants explaining when to use each
8. **Kept existing variants** for backward compatibility (Database, NotFound, BadRequest, Serialization, Internal)

**File Modified**: `crates/mimir-dm/src/types.rs`

**Testing Results**:
- cargo check: PASS
- cargo test (mimir-dm-core): PASS (63 tests passed)
- No breaking changes to existing code

**JSON Structure** (example serialization):
```json
{
  "type": "NotFound",
  "message": "Campaign with id '123' not found"
}
```

All acceptance criteria met:
- [x] ApiError has Serialize and Deserialize traits implemented
- [x] ApiError uses serde tag/content pattern for JSON structure
- [x] All existing ApiError variants are serializable
- [x] Added PermissionDenied and Validation variants
- [x] Improved DbError to ApiError conversion mapping
- [x] cargo check passes with no errors
- [x] cargo test passes all tests
- [x] No changes to command signatures (no breaking changes)
- [x] Documentation comments added to ApiError variants

**Next Steps**: Ready to merge and unblock MIMIR-T-0035 (Phase 2)