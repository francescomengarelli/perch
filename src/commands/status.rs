use anyhow::{Context, Result};
use std::path::PathBuf;
use std::process::Command;

use crate::context;

pub fn run(context: &context::Context) -> Result<()> {
    let home = std::env::var("HOME").context("HOME not set")?;
    let dotfiles = PathBuf::from(&home).join(&context.dotfiles_dir);

    let status = Command::new("git")
        .args(["-C", dotfiles.to_str().unwrap(), "status"])
        .status()
        .context("failed to run git status")?;

    if !status.success() {
        anyhow::bail!("git status failed");
    }

    Ok(())
}
