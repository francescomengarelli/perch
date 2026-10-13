use std::{
    path::{Path, PathBuf},
    process::Stdio,
};

use crate::{
    config::{Config, Module},
    utils::resolve_dotfiles_dir,
};

pub struct Context {
    pub dotfiles_dir: PathBuf,
    pub filtered_modules: Vec<String>,
    pub all_modules: Vec<String>,
    pub verbose: u8,
}

impl Context {
    pub fn new(config: Option<&Config>, verbose: u8) -> anyhow::Result<Self> {
        if !config.is_some() && verbose > 0 {
            eprintln!("config not found. using sensible defaults")
        }

        let dotfiles_dir = resolve_dotfiles_dir(
            config
                .and_then(|c| c.dotfiles_dir.as_deref())
                .unwrap_or(Path::new("~/dotfiles")),
        )?;

        let modules = if let Some(config) = config {
            config.module.clone()
        } else {
            vec![
                Module {
                    name: "common".to_string(),
                    when: None,
                },
                Module {
                    name: "linux".to_string(),
                    when: Some("uname -s | grep -q Linux".to_string()),
                },
                Module {
                    name: "macos".to_string(),
                    when: Some("uname -s | grep -q Darwin".to_string()),
                },
            ]
        };

        Ok(Context {
            verbose,
            dotfiles_dir,
            filtered_modules: modules
                .iter()
                .filter(|m| m.when.as_deref().map(check_condition).unwrap_or(true))
                .map(|m| m.name.clone())
                .collect(),
            all_modules: modules.into_iter().map(|m| m.name).collect(),
        })
    }

    pub fn log(&self, level: u8, msg: &str) {
        if self.verbose >= level {
            eprintln!("{}", msg);
        }
    }
}

fn check_condition(cmd: &str) -> bool {
    std::process::Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}
