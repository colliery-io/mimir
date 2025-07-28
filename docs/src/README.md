# Mimir - D&D Campaign Assistant

> *"Remember, I know everything that can be known in the multiverse."* - Mimir, Planescape

Welcome to Mimir, a local-first D&D campaign assistant designed to help Dungeon Masters manage the cognitive complexity of running campaigns. Named after the wise floating skull of Planescape lore, Mimir focuses on narrative consistency, NPC management, plot tracking, and session preparation.

## What is Mimir?

Mimir is a **terminal-based application** built in Rust that serves as your intelligent campaign companion. Unlike online tools that require internet connectivity, Mimir runs entirely on your local machine, keeping your campaign data private and always accessible.

### Key Features

- 🧠 **Local AI Integration** - Uses Ollama for privacy-focused AI assistance
- 👥 **Intelligent NPC Management** - Track personalities, relationships, and consistency
- 🕸️ **Plot Thread Visualization** - Connect story elements and detect narrative patterns  
- 📝 **Session Preparation Tools** - Streamlined prep and execution support
- 🔍 **Hybrid Search** - Combines exact keyword matching with semantic understanding
- 💻 **Beautiful Terminal UI** - Clean, responsive interface built with Ratatui
- 🔒 **Privacy-First** - Your campaign data never leaves your machine

## Quick Start

If you're new to Mimir, start with our **[Getting Started Tutorial](./tutorials/getting-started.md)** to install and configure the application.

For your first campaign, follow the **[Your First Campaign](./tutorials/first-campaign.md)** guide.

## Documentation Structure

This documentation follows the [Diátaxis](https://diataxis.fr/) framework:

- **[Tutorials](./tutorials/getting-started.md)** - Learning-oriented guides to get you started
- **[How-To Guides](./how-to/campaign-management.md)** - Goal-oriented solutions for specific tasks  
- **[Reference](./reference/cli-commands.md)** - Information-oriented technical details
- **[Explanation](./explanation/architecture.md)** - Understanding-oriented background and context

## Technology Stack

- **Language**: Rust 🦀
- **Database**: SQLite with [sqlite-vec](https://github.com/asg017/sqlite-vec) for vector search
- **AI**: [Ollama](https://ollama.ai/) with llama3/mistral models
- **UI**: [Ratatui](https://ratatui.rs/) terminal user interface
- **Search**: Hybrid FTS5 + vector similarity search

## Project Status

🚧 **Under Active Development** - Mimir is currently in early development (v0.0.0).

See our [project roadmap](./explanation/architecture.md#development-roadmap) for current progress and upcoming features.

## Getting Help

- **Issues**: Report bugs or request features on [GitHub Issues](https://github.com/yourusername/mimir/issues)
- **Discussions**: Join community discussions on [GitHub Discussions](https://github.com/yourusername/mimir/discussions)
- **Troubleshooting**: Check our [troubleshooting guide](./how-to/troubleshooting.md)

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.