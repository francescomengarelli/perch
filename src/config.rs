use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::{Path, PathBuf};

use crate::utils::{expand_tilde, unexpand_tilde};

#[derive(Deserialize)]
pub struct Config {
    pub dotfiles_dir: Option<PathBuf>,
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

/// Record `dotfiles_dir` in the config at `path`, touching nothing else.
///
/// The file is only written when the stored value actually differs, and only
/// that key is edited — comments and formatting survive.
pub fn persist_dotfiles_dir(path: &Path, dotfiles_dir: &Path) -> Result<()> {
    let contents = std::fs::read_to_string(path)
        .with_context(|| format!("i couldn't read {}", path.display()))?;
    let mut doc: toml_edit::DocumentMut = contents
        .parse()
        .with_context(|| format!("i couldn't parse {}", path.display()))?;

    let stored = doc
        .get("dotfiles_dir")
        .and_then(|v| v.as_str())
        .unwrap_or("~/dotfiles");
    if expand_tilde(Path::new(stored))? == dotfiles_dir {
        return Ok(());
    }

    let value = unexpand_tilde(dotfiles_dir)?;
    doc["dotfiles_dir"] = toml_edit::value(value.to_string_lossy().as_ref());

    std::fs::write(path, doc.to_string())
        .with_context(|| format!("i couldn't write {}", path.display()))?;
    Ok(())
}
