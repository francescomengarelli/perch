mod cli;
mod commands;
mod config;
mod context;
mod manifest;
mod utils;

use crate::{
    context::Context,
    utils::{absolutize, expand_tilde},
};
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    let config_path = if let Some(path) = cli.config.map(|f| absolutize(&f)) {
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

    let config = config_path.as_ref().map(|p| config::load(p)).transpose()?;
    let mut context = Context::new(config.as_ref(), cli.verbose)?;
    let original_dotfiles_dir = context.dotfiles_dir.clone();

    match cli.command {
        Commands::Sync { dry_run } => commands::sync::run(&context, dry_run)?,
        Commands::Update => commands::update::run(&mut context, config_path.as_deref())?,
        Commands::Status => commands::status::run(&context)?,
        Commands::Add { path, module } => commands::add::run(&context, &path, &module)?,
        Commands::Remove { path } => commands::remove::run(&context, &path)?,
        Commands::Prune { dry_run } => commands::prune::run(&context, dry_run)?,
        Commands::MoveDir { path } => commands::move_dir::run(&mut context, &path)?,
        Commands::SelfUpgrade => commands::self_upgrade::run()?,
    }

    // Commands like move-dir relocate the dotfiles; remember where they went.
    // A config passed by path from inside the repo moved along with it.
    if let Some(config_path) = &config_path {
        let config_path = match config_path.strip_prefix(&original_dotfiles_dir) {
            Ok(relative) => context.dotfiles_dir.join(relative),
            Err(_) => config_path.clone(),
        };
        config::persist_dotfiles_dir(&config_path, &context.dotfiles_dir)?;
    }

    Ok(())
}
