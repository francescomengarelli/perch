mod cli;
mod commands;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Sync => commands::sync::run()?,
        Commands::Bootstrap { repo } => println!("bootstrap {repo}"),
        Commands::Update => println!("update"),
        Commands::Edit => println!("edit"),
        Commands::Status => println!("status"),
    }

    Ok(())
}
