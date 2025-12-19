# Development Guide

This guide covers setting up a development environment for Mimir and common development workflows.

## Prerequisites

### Required Software

- **Rust** (1.70 or higher) - [Install via rustup](https://rustup.rs/)
- **Node.js** (v18 or higher) - [Download from nodejs.org](https://nodejs.org/)
- **npm** (comes with Node.js)

### Platform-Specific Dependencies

#### macOS
```bash
# Install Xcode Command Line Tools
xcode-select --install
```

#### Linux (Ubuntu/Debian)
```bash
sudo apt-get update
sudo apt-get install -y \
  libwebkit2gtk-4.1-dev \
  libappindicator3-dev \
  librsvg2-dev \
  patchelf \
  build-essential \
  curl \
  wget \
  file
```

#### Windows
- Install [Microsoft Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- Or install Visual Studio with "Desktop development with C++" workload

## Initial Setup

### 1. Clone the Repository

```bash
git clone https://github.com/colliery-io/mimir.git
cd mimir
```

### 2. Install Rust Dependencies

```bash
cargo build
```

This will download and compile all Rust dependencies. First build may take several minutes.

### 3. Install Frontend Dependencies

```bash
cd crates/mimir-dm/frontend
npm install
cd ../../..
```

### 4. Install Development Tools (Optional)

```bash
# Install angreal for test management
pip install 'angreal>=2'

# Install Tauri CLI globally (optional, faster startup)
cargo install tauri-cli
```

## Running the Application

### Development Mode

From the project root:

```bash
cd crates/mimir-dm
cargo tauri dev
```

This will:
- Build the Vue frontend with hot reload
- Start the Rust backend in debug mode
- Launch the application window
- Watch for changes and rebuild automatically

### Frontend-Only Development

If you only need to work on the UI:

```bash
cd crates/mimir-dm/frontend
npm run dev
```

This starts the Vite development server on http://localhost:5173, but Tauri commands won't work.

## Testing

### Run All Tests

```bash
# All Rust tests
cargo test --workspace

# Frontend tests
cd crates/mimir-dm/frontend && npm test

# Run tests with coverage
npm run test:coverage
```

### Run Specific Test Suites

```bash
# Unit tests only (via angreal)
angreal test unit

# Specific crate tests
cargo test -p mimir-dm-core
cargo test -p mimir-dm-llm

# Specific test file
cargo test --test integration_test
```

### Frontend Testing

```bash
cd crates/mimir-dm/frontend

# Run tests
npm test

# Run tests in watch mode
npm test -- --watch

# Run tests with UI
npm run test:ui

# Generate coverage report
npm run test:coverage
```

## Building for Production

### Full Build

```bash
cd crates/mimir-dm
cargo tauri build
```

This creates platform-specific installers in `target/release/bundle/`.

### Frontend Build Only

```bash
cd crates/mimir-dm/frontend
npm run build
```

Output goes to `crates/mimir-dm/frontend/dist/`.

## Project Structure

```
mimir/
├── crates/                          # Rust workspace
│   ├── mimir-dm/                   # Main Tauri application
│   │   ├── src/                    # Rust backend
│   │   │   ├── main.rs            # Application entry point
│   │   │   ├── commands/          # Tauri command handlers
│   │   │   └── services/          # Business logic services
│   │   ├── frontend/               # Vue 3 frontend
│   │   │   ├── src/
│   │   │   │   ├── app/           # App setup and routing
│   │   │   │   ├── components/    # Reusable components
│   │   │   │   ├── features/      # Feature modules
│   │   │   │   ├── stores/        # Pinia stores
│   │   │   │   └── services/      # API services
│   │   │   └── package.json
│   │   ├── icons/                  # App icons
│   │   ├── tauri.conf.json        # Tauri configuration
│   │   └── Cargo.toml
│   │
│   ├── mimir-dm-core/              # Core business logic
│   │   ├── src/
│   │   │   ├── models/            # Domain models
│   │   │   ├── services/          # Business services
│   │   │   ├── dal/               # Data access layer
│   │   │   └── migrations/        # Database migrations
│   │   └── Cargo.toml
│   │
│   ├── mimir-dm-llm/               # LLM provider abstraction
│   │   ├── src/
│   │   │   ├── providers/         # LLM providers (Ollama, etc.)
│   │   │   └── traits/            # Provider traits
│   │   └── Cargo.toml
│   │
│   └── mimir-5etools-splitter/     # Data processing utility
│       ├── src/
│       └── Cargo.toml
│
├── docs/                            # Documentation
│   └── src/                        # mdBook source
│
├── data/                            # D&D reference data
├── .metis/                         # Project management
└── Cargo.toml                      # Workspace configuration
```

## Common Development Tasks

### Adding a New Tauri Command

1. Create command handler in `crates/mimir-dm/src/commands/`:
```rust
#[tauri::command]
pub async fn my_command(param: String) -> Result<String, String> {
    // Implementation
    Ok(format!("Result: {}", param))
}
```

2. Register in `crates/mimir-dm/src/main.rs`:
```rust
.invoke_handler(tauri::generate_handler![
    commands::my_command,
    // ... other commands
])
```

3. Call from frontend:
```typescript
import { invoke } from '@tauri-apps/api/core';

const result = await invoke<string>('my_command', { param: 'value' });
```

### Adding a Database Migration

```bash
cd crates/mimir-dm-core

# Create new migration
diesel migration generate migration_name

# Edit up.sql and down.sql in migrations/

# Run migration
diesel migration run

# Test rollback
diesel migration redo
```

### Adding a Frontend Component

```bash
cd crates/mimir-dm/frontend/src/components
# Create MyComponent.vue
```

```vue
<template>
  <div class="my-component">
    {{ message }}
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';

const message = ref('Hello');
</script>

<style scoped>
.my-component {
  /* styles */
}
</style>
```

### Working with LLM Integration

```bash
# Install and start Ollama
brew install ollama  # macOS
# or download from ollama.ai

# Start Ollama server
ollama serve

# Pull a model
ollama pull llama3

# Test in Mimir
# LLM features will now be available in the UI
```

## Troubleshooting

### Build Errors

**"failed to run custom build command for tauri"**
- Ensure all platform-specific dependencies are installed
- On Linux: verify webkit2gtk-4.1-dev is installed (not 4.0)
- On macOS: run `xcode-select --install`

**Frontend build fails**
```bash
cd crates/mimir-dm/frontend
rm -rf node_modules package-lock.json
npm install
```

**Rust compilation errors**
```bash
cargo clean
cargo build
```

### Runtime Errors

**Database migration errors**
```bash
# Delete development database (macOS)
rm -rf ~/Library/Application\ Support/com.mimir.mimir-test/

# Delete development database (Linux)
rm -rf ~/.local/share/com.mimir.mimir-test/

# Restart the app to recreate
```

**Frontend hot reload not working**
- Stop the app and restart with `cargo tauri dev`
- Check console for build errors
- Ensure Vite dev server is running

### Development Mode

Force development mode with environment variable:
```bash
MIMIR_DEV=1 cargo tauri dev
```

This uses a separate test database and enables debug logging.

## Code Style and Linting

### Rust

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check

# Run linter
cargo clippy --all-targets --all-features

# Fix auto-fixable clippy warnings
cargo clippy --fix
```

### TypeScript/Vue

```bash
cd crates/mimir-dm/frontend

# Lint
npm run lint

# Type check
npm run type-check
```

## Debugging

### Rust Backend

Use `tracing` for logging:
```rust
use tracing::{info, warn, error};

info!("Starting operation");
warn!("Something might be wrong");
error!("Operation failed: {}", err);
```

### Frontend

Use browser DevTools:
- Right-click in the app and select "Inspect Element"
- Use Console, Network, and Vue DevTools
- Check Application tab for local storage/database

### Database

```bash
# Connect to development database (macOS)
sqlite3 ~/Library/Application\ Support/com.mimir.mimir-test/mimir.db

# Run SQL queries
.tables
SELECT * FROM campaigns;
.quit
```

## Additional Resources

- [Tauri Documentation](https://tauri.app/)
- [Vue 3 Documentation](https://vuejs.org/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Diesel ORM Guide](https://diesel.rs/guides/)

## Getting Help

- Check existing [GitHub Issues](https://github.com/colliery-io/mimir/issues)
- Read the [CONTRIBUTING.md](CONTRIBUTING.md) guide
- Review crate-specific READMEs in `crates/*/README.md`
