use anyhow::{Result, bail};
use std::fs;
use std::os::unix::fs::symlink;
use std::path::Path;

use crate::utils;

pub fn stow(source: &Path, target: &Path) -> Result<()> {
    for entry in utils::walk_files(source) {
        let path = entry?;
        let relative = path.strip_prefix(source)?;
        let target_path = target.join(&relative);

        if relative.to_string_lossy() == ".DS_Store" {
            continue;
        }

        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent)?;
        }

        if target_path.is_symlink() {
            if fs::read_link(&target_path)? == path {
                continue;
            }

            fs::remove_file(&target_path)?;
        } else if target_path.exists() {
            bail!("conflict: real file exists at {}", target_path.display());
        }

        println!("linking {}", target_path.display());
        symlink(&path, &target_path)?;
    }

    Ok(())
}
