use std::{path::PathBuf, process::Stdio};

use crate::{
    config::{Config, Module},
    utils::{absolutize, expand_tilde},
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

        let mut dotfiles_dir = if let Some(config) = &config {
            config
                .dotfiles_dir
                .as_ref()
                .map(|d| expand_tilde(&PathBuf::from(d)))
                .unwrap_or_else(|| expand_tilde(&PathBuf::from("~/dotfiles")))?
        } else {
            expand_tilde(&PathBuf::from("~/dotfiles"))?
        };

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

        dotfiles_dir = absolutize(&dotfiles_dir)?;

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
