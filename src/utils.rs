use anyhow::{Context, Result};
use std::{
    fs,
    path::{Path, PathBuf},
};

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
            .with_context(|| "I couldn't find the current directory — has it been deleted?")?
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

pub fn create_parent_dirs(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("I couldn't create {}", parent.display()))?;
    }

    Ok(())
}

pub fn symlink(original: &Path, link: &Path) -> Result<()> {
    std::os::unix::fs::symlink(original, link).with_context(|| {
        format!(
            "I couldn't symlink {} to {}",
            original.display(),
            link.display()
        )
    })
}

pub fn copy(from: &Path, to: &Path) -> Result<()> {
    create_parent_dirs(to)?;

    fs::copy(from, &to)
        .with_context(|| format!("Failed to copy from {} to {}", from.display(), to.display()))?;
    Ok(())
}
