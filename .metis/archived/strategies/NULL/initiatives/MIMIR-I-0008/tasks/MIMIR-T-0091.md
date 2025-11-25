---
id: integrate-tarpaulin-coverage
level: task
title: "Integrate tarpaulin coverage reporting"
short_code: "MIMIR-T-0091"
created_at: 2025-11-24T20:29:00.121192+00:00
updated_at: 2025-11-25T01:11:40.913448+00:00
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

# Integrate tarpaulin coverage reporting

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0008]]

## Objective

Integrate cargo-tarpaulin for code coverage reporting and add coverage generation to the CI pipeline.

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [ ] `cargo-tarpaulin` is configured for the workspace
- [ ] Coverage can be generated locally with `cargo tarpaulin`
- [ ] Coverage report is generated in CI on every PR
- [ ] Coverage report is uploaded as CI artifact
- [ ] HTML coverage report available for viewing
- [ ] Coverage excludes test code and generated code

## Implementation Notes

### Files to Create/Modify
- `.github/workflows/ci.yml` - Add coverage job
- `tarpaulin.toml` or inline config - Configure coverage settings

### Tarpaulin Configuration
```toml
# tarpaulin.toml
[coverage]
# Exclude test files from coverage
exclude-files = ["tests/*", "**/tests/*"]
# Exclude generated schema
exclude-files = ["**/schema.rs"]
# Output formats
output-dir = "coverage"
# Workspace members to cover
packages = ["mimir-dm-core", "mimir-dm-llm"]
```

### CI Workflow Addition
```yaml
coverage:
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - name: Install Rust
      uses: dtolnay/rust-action@stable
    - name: Install tarpaulin
      run: cargo install cargo-tarpaulin
    - name: Generate coverage
      run: cargo tarpaulin --out Html --out Xml --output-dir coverage
    - name: Upload coverage report
      uses: actions/upload-artifact@v4
      with:
        name: coverage-report
        path: coverage/
```

### Local Usage
```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html --output-dir coverage

# Open report
open coverage/tarpaulin-report.html
```

### Dependencies
- Should run after test tasks to measure coverage of new tests

### Risk Considerations
- Low risk: CI-only changes
- Tarpaulin can be slow on large codebases - may need to limit scope
- May need to handle SQLite test database setup in CI environment

## Status Updates **[REQUIRED]**

*To be added during implementation*