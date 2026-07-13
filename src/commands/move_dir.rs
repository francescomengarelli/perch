use anyhow::{Result, bail};
use std::{fs, path::Path, path::PathBuf};

use crate::{context, utils};

pub fn run(context: &mut context::Context, path: &Path) -> Result<()> {
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

                let symlink_target = fs::read_link(&symlinked);

                match symlink_target {
                    Ok(symlink_target) => {
                        if symlink_target != file {
                            bail!(
                                "{} is pointing somewhere unexpected — {} instead of {}",
                                symlinked.display(),
                                symlink_target.display(),
                                file.display()
                            );
                        }
                    }
                    Err(e)
                        if matches!(
                            e.kind(),
                            std::io::ErrorKind::NotFound | std::io::ErrorKind::InvalidInput
                        ) =>
                    {
                        copy_file(&file, &target)?;
                        continue;
                    }
                    Err(e) => bail!(
                        "I tried to read the symlink at {} — {}",
                        symlinked.display(),
                        e
                    ),
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
        eprintln!("something went wrong during the move — I've cleaned up, nothing was changed");
        return Err(e);
    }

    let _ = fs::remove_dir_all(&context.dotfiles_dir);
    context.dotfiles_dir = path.clone();
    eprintln!("everything is now settled in {}", path.display());
    Ok(())
}

fn copy_file(from: &Path, to: &Path) -> Result<()> {
    if let Some(parent) = to.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::copy(from, &to)?;
    Ok(())
}
