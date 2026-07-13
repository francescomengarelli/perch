use crate::{context, utils};
use anyhow::{Context, Result};
use std::process::Command;

pub fn run(context: &context::Context) -> Result<()> {
    let home = utils::get_home_dir()?;
    let dotfiles = home.join(&context.dotfiles_dir);
    let status = Command::new("git")
        .args(["-C", dotfiles.to_str().unwrap(), "status"])
        .status()
        .context("I tried to run git status but something went wrong")?;
    if !status.success() {
        anyhow::bail!("git status didn't go well — check the output above");
    }
    Ok(())
}
