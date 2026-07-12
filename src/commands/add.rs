use crate::{
    context,
    utils::{self, walk_files},
};
use std::{fs, path::PathBuf};

use anyhow::{Result, bail};

pub fn run(context: &context::Context, paths: &[PathBuf], module: &str) -> Result<()> {
    let home = utils::get_home_dir()?;
    let target_dir = context.dotfiles_dir.join(module);

    for path in paths {
        for file in walk_files(path) {
            let file = file?.canonicalize()?;
            let from_home: PathBuf = if file.starts_with(&context.dotfiles_dir) {
                file.strip_prefix(&context.dotfiles_dir)?
                    .components()
                    .skip(1)
                    .collect()
            } else {
                file.strip_prefix(&home)?.to_path_buf()
            };
            let target = target_dir.join(&from_home);

            if target == file {
                continue;
            }

            if target.exists() {
                bail!("conflict: file already exists at {}", target.display());
            }

            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }

            fs::rename(&file, &target)?;
            std::os::unix::fs::symlink(&target, &file)?;

            println!("symlinked {} to {}", file.display(), target.display());
        }
    }

    Ok(())
}
