use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

pub fn expand_tilde(path: &Path) -> Result<PathBuf> {
    if let Ok(stripped) = path.strip_prefix("~") {
        let home = get_home_dir()?;
        Ok(home.join(stripped))
    } else {
        Ok(path.to_path_buf())
    }
}

pub fn unexpand_tilde(path: &Path) -> Result<PathBuf> {
    let home = get_home_dir()?;
    if let Ok(stripped) = path.strip_prefix(home) {
        Ok(PathBuf::from("~").join(stripped))
    } else {
        Ok(path.to_path_buf())
    }
}

pub fn get_home_dir() -> Result<PathBuf> {
    dirs::home_dir().context("I couldn't find your home directory — is $HOME set?")
}

pub fn absolutize(path: &Path) -> Result<PathBuf> {
    let stripped = expand_tilde(path)?;

    if stripped.is_absolute() {
        Ok(stripped)
    } else {
        Ok(std::env::current_dir()
            .context("I couldn't find the current directory — has it been deleted?")?
            .join(stripped))
    }
}

pub fn walk_files(dir: &Path) -> impl Iterator<Item = Result<PathBuf>> {
    walkdir::WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| match e {
            Ok(e) if e.file_type().is_dir() => None,
            Ok(e) => Some(Ok(e.into_path())),
            Err(e) => Some(Err(e.into())),
        })
}
