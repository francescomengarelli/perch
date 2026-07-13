use anyhow::{Result, bail};
use std::{fs, path::Path, path::PathBuf};

use crate::{context, utils};

pub fn run(context: &mut context::Context, path: &PathBuf) -> Result<()> {
    let path = utils::absolutize(path)?;

    let result = (|| -> Result<()> {
        for file in utils::walk_files(&context.dotfiles_dir) {
            let file = file?.canonicalize()?;

            let relative_to_dir: PathBuf = file.strip_prefix(&context.dotfiles_dir)?.to_path_buf();
            let target = path.join(&relative_to_dir);

            let mut components = relative_to_dir.components();
            let first: PathBuf = PathBuf::from(components.next().unwrap().as_os_str());
            let first_str = first.to_str().unwrap();
            let rest: PathBuf = components.collect();

            if context.all_modules.iter().any(|s| s == first_str)
                && rest.components().next().is_some()
            {
                let symlinked = utils::get_home_dir()?.join(rest);

                let symlink_target = fs::read_link(&symlinked)?;

                if symlink_target != file {
                    bail!(
                        "symlink points somewhere else! {} -> {}",
                        symlinked.display(),
                        symlink_target.display()
                    );
                }

                copy_file(&file, &target)?;
                fs::remove_file(&symlinked)?;
                std::os::unix::fs::symlink(&target, &symlinked)?;
            } else {
                copy_file(&file, &target)?;
            }
        }
        Ok(())
    })();

    if let Err(e) = result {
        let _ = fs::remove_dir_all(&path);
        return Err(e);
    }

    let _ = fs::remove_dir_all(&context.dotfiles_dir);
    context.dotfiles_dir = path;

    Ok(())
}

fn copy_file(from: &Path, to: &Path) -> Result<()> {
    if let Some(parent) = to.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::copy(from, &to)?;
    Ok(())
}
