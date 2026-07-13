use crate::utils::{self, create_parent_dirs, symlink};
use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn stow(source: &Path, target: &Path, verbose: bool) -> Result<()> {
    for entry in utils::walk_files(source) {
        let path = entry?;
        let relative = path.strip_prefix(source)?;
        let target_path = target.join(&relative);

        if relative.to_string_lossy() == ".DS_Store" {
            continue;
        }

        create_parent_dirs(&target_path)?;

        if target_path.is_symlink() || target_path.exists() {
            if is_our_symlink(&target_path, &path) {
                continue;
            }
            fs::remove_file(&target_path)?;
        };

        symlink(&path, &target_path)?;
        if verbose {
            eprintln!("{} is in place", target_path.display());
        }
    }
    Ok(())
}

// utils.rs or stow.rs
pub fn is_our_symlink(target: &Path, source: &Path) -> bool {
    target.is_symlink() && fs::read_link(target).ok().as_deref() == Some(source)
}
