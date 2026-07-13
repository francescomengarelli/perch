use anyhow::{Context, Result};
use std::{path::PathBuf, process::Command};

use crate::{
    commands,
    config::{self},
    context,
};

pub fn run(context: &mut context::Context) -> Result<()> {
    let status = Command::new("git")
        .arg("-C")
        .arg(&context.dotfiles_dir)
        .arg("pull")
        .status()
        .context("failed to run git pull")?;

    if !status.success() {
        anyhow::bail!("git pull failed");
    }

    let new_config_dotfiles_dir = (|| -> anyhow::Result<Option<config::Config>> {
        for module_name in &context.filtered_modules {
            let module_path = context.dotfiles_dir.join(module_name);
            let config_path = module_path.join(".config/perch/config.toml");
            let config_path = config_path.exists().then_some(config_path);

            if let Some(config_path) = config_path {
                let loaded = config::load(&config_path)
                    .context("Invalid config in the dotfiles directory")?;
                return Ok(Some(loaded));
            }
        }
        Ok(None)
    })()?;

    // sync happens before eventually moving the dotfiles dir because context.dotfiles_dir is up to
    // date as long as we dont manually move the dotfiles dir.
    crate::commands::sync::run(context)?;

    let new_dotfiles_dir = new_config_dotfiles_dir.and_then(|c| c.dotfiles_dir);

    if let Some(new_dir) = new_dotfiles_dir {
        let target_path = PathBuf::from(new_dir);
        if target_path != context.dotfiles_dir {
            println!(
                "I noticed your dotfiles configuration changed the target directory. I'm moving everything over to {} now...",
                target_path.display()
            );
            commands::move_dir::run(context, &target_path)?;
        }
    }

    Ok(())
}
