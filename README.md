# Mimir - D&D Campaign Assistant

A local-first desktop application for managing D&D campaigns, built with Tauri, Vue 3, and Rust.

## Installation

### For Users

Download the latest release for your platform from [GitHub Releases](https://github.com/colliery-io/mimir/releases):

#### macOS
1. Download `Mimir_[version]_[arch].dmg` for your Mac (Intel or Apple Silicon)
2. Open the DMG file and drag Mimir to your Applications folder
3. **Important**: Remove the quarantine attribute to run the app:
   ```bash
   xattr -cr /Applications/Mimir.app
   ```
4. Launch Mimir from your Applications folder

#### Windows
1. Download `Mimir_[version]_x64_en-US.msi`
2. Run the installer and follow the installation wizard
3. Launch Mimir from the Start menu

#### Linux
1. Download the appropriate package for your distribution:
   - **Debian/Ubuntu**: `mimir_[version]_amd64.deb`
   - **Universal**: `mimir_[version]_amd64.AppImage`
2. Install:
   - **DEB**: `sudo dpkg -i mimir_[version]_amd64.deb`
   - **AppImage**: `chmod +x mimir_[version]_amd64.AppImage && ./mimir_[version]_amd64.AppImage`

### For Developers

See the [Development Setup](#development-setup) section below for instructions on building from source.

## Development Setup

### Prerequisites

- **Rust** (1.70 or higher) - [Install Rust](https://rustup.rs/)
- **Node.js** (v18 or higher) - [Install Node.js](https://nodejs.org/)
- **npm** (comes with Node.js) or **pnpm**
- **Tauri Prerequisites** - Platform-specific dependencies:
  - **Linux**: `webkit2gtk-4.0-dev`, `libappindicator3-dev`, `librsvg2-dev`, `patchelf`
  - **macOS**: Xcode Command Line Tools
  - **Windows**: Microsoft Visual Studio C++ Build Tools

## Development Setup

### 1. Clone the repository

```bash
git clone <repository-url>
cd mimir
```

### 2. Install dependencies

Install Rust dependencies:
```bash
cargo build
```

Install frontend dependencies:
```bash
cd crates/mimir-dm/frontend
npm install
cd -
```

### 3. Start development mode

From the project root, run:
```bash
cd crates/mimir-dm/frontend && npm run build && cd ../../.. && cargo tauri dev
```

**Or use the development script (recommended):**
```bash
cd crates/mimir-dm && cargo tauri dev
```

This will:
- Automatically build the Vue frontend
- Launch the Tauri application in development mode with hot reload
- Open the application window
- Watch for frontend changes and rebuild as needed

## Project Structure

```
mimir/
├── crates/                      # Rust workspace crates
│   ├── mimir-dm/               # Main Tauri desktop application
│   │   ├── frontend/           # Vue 3 frontend with TypeScript
│   │   └── src/                # Rust backend with Tauri commands
│   ├── mimir-dm-core/          # Core business logic and database layer
│   ├── mimir-dm-llm/           # LLM provider abstraction (Ollama support)
│   └── mimir-5etools-splitter/ # 5etools data processing utilities
├── data/                        # D&D rules data (5etools JSON)
├── docs/                        # User and developer documentation
└── .metis/                     # Project management (Flight Levels)
```

## Available Scripts

### Development
- `cargo tauri dev` - Start the application in development mode with hot reload
- `cd crates/mimir-dm/frontend && npm run dev` - Frontend development server (standalone)
- `cargo test` - Run all Rust tests
- `cd crates/mimir-dm/frontend && npm test` - Run frontend tests

### Production
- `cargo tauri build` - Build optimized application for production
- `cd crates/mimir-dm/frontend && npm run build` - Build frontend for production

### Utilities
- `cargo run --bin mimir-5esplit` - Process 5etools data archives
- `cargo run --bin mimir-5etest` - Test 5etools data loading

## Features

- **Campaign Management** - Create and manage D&D campaigns with structured phases and workflows
- **Module System** - Organize adventures into reusable modules with templates
- **Session Tracking** - Track session progress, notes, and outcomes
- **D&D 5e Catalog** - Comprehensive rules database with full-text search
  - Classes, backgrounds, feats, spells, items, monsters
  - Multiple sourcebooks supported (PHB, DMG, MM, Xanathar's, Tasha's, etc.)
- **Document Templates** - Pre-built Markdown templates with YAML frontmatter
- **Local-First** - All data stored locally in SQLite, no cloud dependencies
- **LLM Integration** - Local LLM support via Ollama for content generation with configurable todo system
- **Cross-Platform** - Native desktop app for Windows, macOS, and Linux
- **5etools Integration** - Import and process official D&D data from 5etools

## Tech Stack

- **Frontend**: Vue 3, TypeScript, Vite, TailwindCSS
- **Backend**: Rust, Tauri 2.0
- **Database**: SQLite with Diesel ORM, vector search via sqlite-vec
- **LLM**: Ollama integration with rate limiting
- **UI Framework**: TipTap editor, custom theming (dark/light/hyper)
- **Build**: Cargo workspace, npm/Vite frontend build

## Crate Overview

### Core Crates
- **[mimir-dm](crates/mimir-dm/)** - Main Tauri desktop application with Vue frontend
- **[mimir-dm-core](crates/mimir-dm-core/)** - Business logic, database models, and services  
- **[mimir-dm-llm](crates/mimir-dm-llm/)** - LLM provider abstraction layer with todo system (Ollama support)

### Utility Crates
- **[mimir-5etools-splitter](crates/mimir-5etools-splitter/)** - Data processing for 5etools repositories

## Troubleshooting

### Common Issues

**Build fails with "failed to run custom build command for tauri"**
- Ensure all Tauri prerequisites are installed for your platform
- On Linux: `sudo apt install webkit2gtk-4.0-dev libappindicator3-dev librsvg2-dev patchelf`
- On macOS: Install Xcode Command Line Tools: `xcode-select --install`

**Frontend build fails**
- Clear node_modules: `cd crates/mimir-dm/frontend && rm -rf node_modules && npm install`
- Ensure Node.js version is 18+: `node --version`

**Database migration errors**
- Delete development database: `rm -rf ~/Library/Application\ Support/com.mimir.mimir-test/`
- Restart the application to recreate fresh database

**LLM integration not working**
- Install and run Ollama: [ollama.ai](https://ollama.ai/)
- Pull a model: `ollama pull llama3`
- Ensure Ollama is running on `http://localhost:11434`
- Todo storage automatically configured to app data directory on startup

### Development Tips
- Use `MIMIR_DEV=1` environment variable to force development mode
- Frontend changes require rebuild: `cd crates/mimir-dm/frontend && npm run build`
- Database schema changes require new migrations in `crates/mimir-dm-core/migrations/`

## Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on how to get started, code style, testing requirements, and the pull request process.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.