use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing::info;

#[derive(Parser)]
#[command(name = "mimir")]
#[command(about = "A local-first D&D campaign assistant for Dungeon Masters")]
#[command(version, author)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Import a D&D rule bundle into the database
    Import {
        /// Path to the bundle file (.tar.gz)
        #[arg(value_name = "BUNDLE")]
        bundle_path: String,
        
        /// Database URL (defaults to local SQLite database)
        #[arg(short, long, default_value = "sqlite://mimir.db")]
        database: String,
    },
    
    /// Start the terminal user interface
    Tui,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Import { bundle_path, database: _ } => {
            info!("Importing bundle from: {}", bundle_path);
            // TODO: Call mimir_dm_import::import_bundle when ready
            println!("Import functionality not yet implemented");
        }
        Commands::Tui => {
            info!("Starting TUI");
            // TODO: Call mimir_dm_tui when ready
            println!("TUI not yet implemented");
        }
    }
    
    Ok(())
}