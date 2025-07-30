# mimir-dm-import

Bundle import functionality for the Mimir D&D campaign assistant.

## Overview

This crate provides the ability to import standardized D&D rule bundles (.tar.gz archives) into the Mimir database. It handles:

- Bundle extraction and validation
- Entity parsing from JSON files
- Atomic database imports
- Progress reporting
- Comprehensive error handling

## Usage

```rust
use mimir_dm_import::{BundleImporter, import_bundle};

// Using the convenience function
import_bundle("sqlite://mimir.db", "dnd5e-2014-core.tar.gz").await?;

// Using the importer directly for more control
let importer = BundleImporter::new("sqlite://mimir.db".to_string());
importer.import_bundle("dnd5e-2014-core.tar.gz").await?;
```

## Bundle Format

Bundles are `.tar.gz` archives containing:

- `manifest.json` - Bundle metadata and entity counts
- `version.json` - Version information
- `sources.json` - Book/source definitions
- Entity files: `races.json`, `classes.json`, `items.json`, etc.

See the [import bundle format specification](../../docs/data-design/import-bundle-format.md) for details.

## Testing

Run tests with:

```bash
cargo test -p mimir-dm-import
```

Tests include:
- Unit tests for bundle extraction
- Integration tests for full import pipeline
- Error handling tests