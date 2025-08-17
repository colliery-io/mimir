---
id: bundle-core-rules-in-binary
level: task
title: "Bundle Core Rules in Binary"
created_at: 2025-08-17T02:37:00.000000+00:00
updated_at: 2025-08-17T02:37:00.000000+00:00
parent: character-creation-integration
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/todo"

exit_criteria_met: false
---

# Bundle Core Rules in Binary

## Overview
Embed the D&D 5e 2014 core rules bundle (347KB) directly in the binary to ensure offline functionality from first run. This provides immediate access to PHB, DMG, and MM content without requiring network connectivity.

## Acceptance Criteria

- [ ] Core rules bundle (`dnd5e-2014-core-v1.210.46.tar.gz`) embedded as binary resource
- [ ] Automatic import on first run if database is empty
- [ ] CLI command `--import-rules` for manual import/reimport
- [ ] Import progress shown to user during initialization
- [ ] Successful import verified with basic queries

## Technical Approach

### 1. Embed Bundle as Binary Resource
```rust
// In build.rs or using include_bytes!
const CORE_RULES_BUNDLE: &[u8] = include_bytes!("../data/rule_sets/dnd5e-2014-core-v1.210.46.tar.gz");
```

### 2. Initialize on First Run
```rust
// In main.rs startup
async fn initialize_rules(db: &Database) -> Result<()> {
    if !has_rules_imported(db).await? {
        println!("First run detected. Importing core D&D rules...");
        import_embedded_bundle(db, CORE_RULES_BUNDLE).await?;
    }
    Ok(())
}
```

### 3. CLI Command for Manual Import
```rust
// Add to clap command structure
#[derive(Parser)]
struct Cli {
    #[clap(long)]
    import_rules: bool,
    
    #[clap(long)]
    rules_bundle: Option<PathBuf>, // Optional external bundle
}
```

## Implementation Steps

1. **Add bundle to project**
   - Copy `dnd5e-2014-core-v1.210.46.tar.gz` to appropriate location
   - Update `.gitignore` if needed for other bundles

2. **Modify build process**
   - Use `include_bytes!` macro or build.rs to embed bundle
   - Ensure bundle is compressed efficiently

3. **Update BundleImporter**
   - Add method to import from embedded bytes
   - Handle in-memory extraction without temp files

4. **Add initialization check**
   - Check if `rule_systems` table has core rules
   - Run import if database is empty

5. **Create CLI command**
   - Add `--import-rules` flag
   - Support optional external bundle path
   - Show progress during import

## Testing Requirements

- [ ] Clean database import works on first run
- [ ] Existing database skips import
- [ ] Manual import command works
- [ ] Import completes in < 5 seconds
- [ ] All core entities queryable after import

## Dependencies
- Existing `mimir-dm-import` crate
- Database connection at startup

## Estimated Effort
1-2 days

## Notes
- This is the foundation for all character creation features
- Ensures consistent development environment for all developers
- Future enhancement: download additional rule sets on demand