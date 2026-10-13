use crate::{
    commands,
    config::{self, Config},
    context::{self, Context as PerchContext},
};
use anyhow::{Context, Result};
use std::{path::Path, process::Command};

pub fn run(context: &mut PerchContext, config_path: Option<&Path>) -> Result<()> {
    let status = Command::new("git")
        .arg("-C")
        .arg(&context.dotfiles_dir)
        .arg("pull")
        .status()
        .context("i tried to run git pull but something went wrong")?;

    if !status.success() {
        anyhow::bail!("git pull didn't go well — check the output above");
    }

    // The pull may have changed the config (modules, dotfiles_dir), so read
    // it again instead of trusting what was loaded before.
    let config = match config_path {
        Some(path) => Some(
            config::load(path)
                .with_context(|| format!("i couldn't read {} after pulling", path.display()))?,
        ),
        None => find_repo_config(context)?,
    };

    let mut fresh = context::Context::new(config.as_ref(), context.verbose)?;
    let target_dir = fresh.dotfiles_dir.clone();
    fresh.dotfiles_dir = context.dotfiles_dir.clone();

    let mut move_note = "";
    if target_dir != fresh.dotfiles_dir {
        eprintln!(
            "your config points to a new dotfiles directory — moving everything over from {} to {} now",
            fresh.dotfiles_dir.display(),
            target_dir.display()
        );
        commands::move_dir::run(&mut fresh, &target_dir)?;
        move_note = ", dotfiles dir moved";
    }

    commands::sync::run(&fresh, false)?;
    *context = fresh;

    eprintln!("update completed. dotfiles synced{}", move_note);
    Ok(())
}

/// Without a config file in place, look for one shipped inside an active module.
fn find_repo_config(context: &PerchContext) -> Result<Option<Config>> {
    for module_name in &context.filtered_modules {
        let config_path = context
            .dotfiles_dir
            .join(module_name)
            .join(".config/perch/config.toml");

        if config_path.exists() {
            let loaded = config::load(&config_path)
                .context("i found a config in your dotfiles directory but couldn't read it")?;
            return Ok(Some(loaded));
        }
    }
    Ok(None)
}
