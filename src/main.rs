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

    let config_path = if let Some(path) = cli.config.map(|f| expand_tilde(&f)) {
        let path = path?;
        if !path.exists() {
            anyhow::bail!(
                "i couldn't find a config file at {} - does it exist?",
                path.display()
            );
        }
        Some(path)
    } else {
        let fallback = expand_tilde(&PathBuf::from("~/.config/perch/config.toml"))?;
        fallback.exists().then_some(fallback)
    };

    let mut config = config_path.as_ref().map(|p| config::load(&p)).transpose()?;
    let mut context = Context::new(config.as_ref(), cli.verbose)?;

    match cli.command {
        Commands::Sync => commands::sync::run(&context)?,
        Commands::Update => commands::update::run(&mut context)?,
        Commands::Status => commands::status::run(&context)?,
        Commands::Add { path, module } => commands::add::run(&context, &path, &module)?,
        Commands::MoveDir { path } => commands::move_dir::run(&mut context, &path)?,
    }

    // FIXME: clean this up
    if let (Some(config), Some(config_path)) = (&mut config, &config_path) {
        config.dotfiles_dir = Some(utils::unexpand_tilde(&context.dotfiles_dir)?);
        config::save(&config, &config_path)?;
    }

    Ok(())
}
