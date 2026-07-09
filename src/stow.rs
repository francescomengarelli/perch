use anyhow::{Result, bail};
use std::fs;
use std::os::unix::fs::symlink;
use std::path::Path;

pub fn stow(path: &Path, target: &Path) -> Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        let name = entry.file_name();

        // FIXME: this should not be hardcoded
        if name == ".DS_Store" {
            continue;
        }

        let target_path = target.join(&name);

        if path.is_dir() {
            fs::create_dir_all(&target_path)?;
            stow(&path, &target_path)?;
        } else {
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
    }

    Ok(())
}
