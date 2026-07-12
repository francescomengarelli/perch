use std::{path::PathBuf, process::Stdio};

use crate::{
    config::{Config, Module},
    utils::{absolutize, expand_tilde},
};

pub struct Context {
    pub dotfiles_dir: PathBuf,
    pub modules: Vec<String>,
}

impl Context {
    pub fn new(config: Option<Config>) -> anyhow::Result<Self> {
        let mut dotfiles_dir = if let Some(config) = &config {
            config
                .dotfiles_dir
                .as_ref()
                .map(|d| expand_tilde(PathBuf::from(d)))
                .unwrap_or_else(|| expand_tilde(PathBuf::from("~/dotfiles")))?
        } else {
            expand_tilde(PathBuf::from("~/dotfiles"))?
        };

        let modules = if let Some(config) = config {
            config.module
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

        dotfiles_dir = absolutize(dotfiles_dir)?;

        Ok(Context {
            dotfiles_dir,
            modules: modules
                .into_iter()
                .filter(|m| m.when.as_deref().map(check_condition).unwrap_or(true))
                .map(|m| m.name)
                .collect(),
        })
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
