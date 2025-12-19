---
id: fix-panic-points-in-main-rs
level: task
title: "Fix panic points in main.rs"
short_code: "MIMIR-T-0071"
created_at: 2025-11-24T20:28:56.854687+00:00
updated_at: 2025-11-24T21:12:02.088566+00:00
parent: MIMIR-I-0008
blocked_by: []
archived: true

tags:
  - "#task"
  - "#phase/completed"


exit_criteria_met: false
strategy_id: NULL
initiative_id: MIMIR-I-0008
---

# Fix panic points in main.rs

## Parent Initiative

[[MIMIR-I-0008]] - Foundation Hardening

## Objective

Replace all `expect()` and `unwrap()` calls in the application initialization path with graceful error handling that logs errors and exits cleanly rather than panicking.

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [ ] No `expect()` calls in `main.rs` initialization path
- [ ] No `unwrap()` calls in `main.rs` initialization path  
- [ ] Database initialization failure logs error and exits with code 1
- [ ] Migration failure logs error and exits with code 1
- [ ] All error messages include context about what failed
- [ ] Application exits gracefully (no panic stack traces)

## Implementation Notes

### Files to Modify
- `crates/mimir-dm/src/main.rs`
- `crates/mimir-dm/src/app_init.rs` (if applicable)

### Current Problem Areas
```rust
// These need to be replaced:
let db_service = DatabaseService::new(...)
    .expect("Failed to initialize database service");

let mut conn = db_service.get_connection()
    .expect("Failed to get connection for migrations");
```

### Technical Approach
Replace `expect()` with `match` statements that:
1. Log the error using `tracing::error!()`
2. Provide context about what operation failed
3. Call `std::process::exit(1)` for fatal errors
4. For non-fatal errors, continue with degraded functionality where appropriate

### Example Pattern
```rust
let db_service = match DatabaseService::new(...) {
    Ok(service) => {
        info!("Database service initialized successfully");
        service
    }
    Err(e) => {
        error!("Failed to initialize database service: {}", e);
        std::process::exit(1);
    }
};
```

### Dependencies
None - this is a foundational task

### Risk Considerations
- Low risk: Changes are isolated to startup code
- Test by simulating database failures (e.g., invalid path, corrupted DB)

## Status Updates

*To be added during implementation*