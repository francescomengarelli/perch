use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::context;

pub fn run(context: &context::Context) -> Result<()> {
    let home = std::env::var("HOME").context("HOME not set")?;
    for module in &context.modules {
        println!("stowing {module}...");
        let module_path = context.dotfiles_dir.join(module);
        crate::stow::stow(&module_path, &PathBuf::from(&home))?;
    }

    Ok(())
}
