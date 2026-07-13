use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub dotfiles_dir: Option<String>,
    #[serde[default]]
    pub module: Vec<Module>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Module {
    pub name: String,
    pub when: Option<String>,
}

pub fn load(path: &Path) -> Result<Config> {
    let contents = std::fs::read_to_string(path)?;
    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}

pub fn save(config: &Config, path: &Path) -> Result<()> {
    let contents = toml::to_string(config)?;
    std::fs::write(path, contents)?;
    Ok(())
}
