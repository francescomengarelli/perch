use anyhow::{Context, Result};
use std::path::PathBuf;

pub fn expand_tilde(path: PathBuf) -> Result<PathBuf> {
    if let Ok(stripped) = path.strip_prefix("~") {
        let home = get_home_dir()?;
        Ok(home.join(stripped))
    } else {
        Ok(path)
    }
}

pub fn get_home_dir() -> Result<PathBuf> {
    dirs::home_dir().context("Could not determine home directory")
}
