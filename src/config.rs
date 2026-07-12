use anyhow::Result;
use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize)]
pub struct Config {
    pub dotfiles_dir: Option<String>,
    #[serde[default]]
    pub module: Vec<Module>,
}

#[derive(Clone, Deserialize)]
pub struct Module {
    pub name: String,
    pub when: Option<String>,
}

pub fn load(path: &Path) -> Result<Config> {
    let contents = std::fs::read_to_string(path)?;
    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}
