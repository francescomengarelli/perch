use anyhow::{Context, Result};
use std::path::PathBuf;
use std::process::Command;

use crate::{context, utils};

pub fn run(context: &context::Context) -> Result<()> {
    let home = utils::get_home_dir()?;
    let dotfiles = PathBuf::from(&home).join(&context.dotfiles_dir);

    let status = Command::new("git")
        .args(["-C", dotfiles.to_str().unwrap(), "pull"])
        .status()
        .context("failed to run git pull")?;

    if !status.success() {
        anyhow::bail!("git pull failed");
    }

    crate::commands::sync::run(context)
}
