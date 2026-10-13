use crate::context;
use anyhow::{Context, Result};
use std::process::Command;

pub fn run(context: &context::Context) -> Result<()> {
    let status = Command::new("git")
        .arg("-C")
        .arg(&context.dotfiles_dir)
        .arg("status")
        .status()
        .context("i tried to run git status but something went wrong")?;
    if !status.success() {
        anyhow::bail!("git status didn't go well — check the output above");
    }
    Ok(())
}
