use crate::{
    commands,
    config::{self},
    context,
    utils::expand_tilde,
};
use anyhow::{Context, Result};
use std::{path::PathBuf, process::Command};

pub fn run(context: &mut context::Context) -> Result<()> {
    let status = Command::new("git")
        .arg("-C")
        .arg(&context.dotfiles_dir)
        .arg("pull")
        .status()
        .context("I tried to run git pull but something went wrong")?;

    if !status.success() {
        anyhow::bail!("git pull didn't go well — check the output above");
    }

    let new_cfg = (|| -> anyhow::Result<Option<config::Config>> {
        for module_name in &context.filtered_modules {
            let config_path = context
                .dotfiles_dir
                .join(module_name)
                .join(".config/perch/config.toml");

            if let Some(config_path) = config_path.exists().then_some(config_path) {
                let loaded = config::load(&config_path)
                    .context("I found a config in your dotfiles directory but couldn't read it")?;
                return Ok(Some(loaded));
            }
        }
        Ok(None)
    })()?;

    crate::commands::sync::run(context)?;

    let new_dotfiles_dir = new_cfg.and_then(|cfg| cfg.dotfiles_dir);
    let move_note: String = if let Some(new_dir) = new_dotfiles_dir {
        let target_path = expand_tilde(&PathBuf::from(new_dir))?;
        if target_path != context.dotfiles_dir {
            eprintln!(
                "your config points to a new dotfiles directory — moving everything over from {} to {} now",
                context.dotfiles_dir.display(),
                target_path.display()
            );
            commands::move_dir::run(context, &target_path)?;
        }
        String::from(format!(
            " and moved your dotfiles to {}",
            target_path.display()
        ))
    } else {
        String::from("")
    };

    eprintln!(
        "i completed the update. synced {} modules{}",
        context.filtered_modules.len(),
        move_note
    );

    Ok(())
}
