mod cli;
mod commands;
mod config;
mod context;
mod stow;
mod utils;

use crate::{context::Context, utils::expand_tilde};
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    let config_path = if let Some(path) = cli.config.map(expand_tilde) {
        let path = path?;
        if !path.exists() {
            anyhow::bail!("config file not found: {}", path.display());
        }
        Some(path)
    } else {
        let fallback = expand_tilde(PathBuf::from("~/.config/dot/config.toml"))?;
        fallback.exists().then_some(fallback)
    };

    let config = config_path.map(|p| config::load(&p)).transpose()?;
    let context = Context::new(config)?;

    match cli.command {
        Commands::Sync => commands::sync::run(&context)?,
        Commands::Update => commands::update::run(&context)?,
        Commands::Status => commands::status::run(&context)?,
        Commands::Add { path, module } => commands::add::run(&context, &path, &module)?,
    }

    Ok(())
}
