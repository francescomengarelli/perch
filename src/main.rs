mod cli;
mod commands;
mod stow;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Sync => commands::sync::run()?,
        Commands::Update => commands::update::run()?,
        Commands::Status => commands::status::run()?,
    }

    Ok(())
}
