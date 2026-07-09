use anyhow::{Context, Result};
use std::path::PathBuf;
use std::process::Command;

pub fn run() -> Result<()> {
    let home = std::env::var("HOME").context("HOME not set")?;
    let dotfiles = PathBuf::from(&home).join("dotfiles");

    let status = Command::new("git")
        .args(["-C", dotfiles.to_str().unwrap(), "pull"])
        .status()
        .context("failed to run git pull")?;

    if !status.success() {
        anyhow::bail!("git pull failed");
    }

    crate::commands::sync::run()
}
