---
id: add-coverage-threshold-enforcement
level: task
title: "Add coverage threshold enforcement to CI"
short_code: "MIMIR-T-0093"
created_at: 2025-11-24T20:29:00.568461+00:00
updated_at: 2025-11-25T01:12:28.565265+00:00
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

# Add coverage threshold enforcement to CI

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0008]]

## Objective

Configure CI to enforce a minimum code coverage threshold, failing the build if coverage drops below the target.

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [ ] CI fails if coverage drops below 60% for `mimir-dm-core`
- [ ] CI fails if coverage drops below 50% for `mimir-dm-llm`
- [ ] Coverage check runs on all PRs
- [ ] Clear error message when coverage threshold not met
- [ ] Coverage percentage visible in PR checks

## Implementation Notes

### Files to Modify
- `.github/workflows/ci.yml` - Add coverage threshold check
- `tarpaulin.toml` - Configure threshold settings

### Tarpaulin Threshold Configuration
```toml
# tarpaulin.toml
[coverage]
fail-under = 60
```

Or via command line:
```bash
cargo tarpaulin --fail-under 60
```

### CI Workflow Addition
```yaml
coverage-check:
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - name: Install Rust
      uses: dtolnay/rust-action@stable
    - name: Install tarpaulin
      run: cargo install cargo-tarpaulin
    - name: Check coverage threshold
      run: |
        cargo tarpaulin --packages mimir-dm-core --fail-under 60
        cargo tarpaulin --packages mimir-dm-llm --fail-under 50
```

### Threshold Rationale
- **60% for mimir-dm-core**: Core business logic should have good coverage
- **50% for mimir-dm-llm**: Lower threshold due to external API dependencies
- Initial thresholds are conservative; can be raised as coverage improves

### Future Improvements
- Add coverage badge to README
- Track coverage trends over time
- Set up coverage diff reporting on PRs

### Dependencies
- Requires MIMIR-T-0091 (tarpaulin integration) to be completed first

### Risk Considerations
- May cause initial CI failures if current coverage is below threshold
- Need to complete test tasks before enforcing threshold
- Consider starting with warning-only mode before enforcement

## Status Updates **[REQUIRED]**

*To be added during implementation*