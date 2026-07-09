use std::path::PathBuf;

use anyhow::{Context, Result};

fn is_hyprland() -> bool {
    std::process::Command::new("which")
        .arg("hyprctl")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn stow_modules(modules: &[&str]) -> Result<()> {
    let home = std::env::var("HOME").context("HOME not set")?;
    let dotfiles = PathBuf::from(&home).join("dotfiles");

    for module in modules {
        println!("stowing {module}...");
        let module_path = dotfiles.join(module);
        crate::stow::stow(&module_path, &PathBuf::from(&home))?;
    }

    Ok(())
}

pub fn run() -> Result<()> {
    let os = std::env::consts::OS;
    let hyprland = is_hyprland();

    let mut modules = vec!["common"];

    match os {
        "macos" => modules.push("macos"),
        "linux" => {
            modules.push("linux");
            if hyprland {
                modules.push("hyprland");
            }
        }
        _ => {}
    }

    println!("modules to stow: {:?}", modules);
    stow_modules(&modules)?;

    Ok(())
}
