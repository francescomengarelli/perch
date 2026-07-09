use anyhow::{Context, Result};
use std::path::PathBuf;
use std::process::Command;

pub fn run() -> Result<()> {
    let home = std::env::var("HOME").context("HOME not set")?;
    let dotfiles = PathBuf::from(&home).join("dotfiles");

    let status = Command::new("git")
        .args(["-C", dotfiles.to_str().unwrap(), "status"])
        .status()
        .context("failed to run git status")?;

    if !status.success() {
        anyhow::bail!("git status failed");
    }

    Ok(())
}
