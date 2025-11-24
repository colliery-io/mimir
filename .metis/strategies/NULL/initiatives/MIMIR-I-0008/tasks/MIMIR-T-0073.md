---
id: standardize-logging-in-services
level: task
title: "Standardize logging in services"
short_code: "MIMIR-T-0073"
created_at: 2025-11-24T20:28:56.963042+00:00
updated_at: 2025-11-24T21:17:22.855979+00:00
parent: MIMIR-I-0008
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/completed"


exit_criteria_met: false
strategy_id: NULL
initiative_id: MIMIR-I-0008
---

# Standardize logging in services

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0008]]

## Objective

Replace all `println!()` and `eprintln!()` calls in service layer code with appropriate `tracing` macros for consistent, structured logging.

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [ ] No `println!()` calls in `mimir-dm-core/src/services/`
- [ ] No `eprintln!()` calls in `mimir-dm-core/src/services/`
- [ ] No `println!()` calls in `mimir-dm/src/services/`
- [ ] No `eprintln!()` calls in `mimir-dm/src/services/`
- [ ] All replacements use appropriate log levels (error, warn, info, debug)
- [ ] Log messages include relevant context (entity IDs, operation names)

## Implementation Notes

### Files to Modify
Known locations with `println!`/`eprintln!`:
- `crates/mimir-dm-core/src/services/campaign_service.rs` (lines 61, 69, 108, 266, 291, 303)
- `crates/mimir-dm-core/src/services/template_service.rs`
- Scan all service files for additional occurrences

### Technical Approach

1. Search for all `println!` and `eprintln!` in service directories
2. Evaluate each occurrence for appropriate log level:
   - `eprintln!` for errors -> `tracing::error!`
   - `println!` for success messages -> `tracing::info!` or `tracing::debug!`
   - Debug output -> `tracing::debug!`
3. Add context to log messages where beneficial

### Log Level Guidelines
- `error!` - Operation failures, unrecoverable errors
- `warn!` - Recoverable issues, deprecation warnings
- `info!` - Significant operations (campaign created, migration complete)
- `debug!` - Detailed operation flow, useful for debugging

### Example Transformation
```rust
// Before
eprintln!("Failed to cleanup campaign directory: {}", remove_err);

// After
tracing::error!(
    campaign_id = %campaign.id,
    path = %campaign_path.display(),
    error = %remove_err,
    "Failed to cleanup campaign directory after database error"
);
```

### Dependencies
None

### Risk Considerations
- Low risk: Logging changes don't affect functionality
- Verify log output appears correctly in both console and file appenders

## Status Updates **[REQUIRED]**

*To be added during implementation*