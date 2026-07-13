use crate::{
    context,
    utils::{self, create_parent_dirs, symlink, walk_files},
};
use std::{fs, path::PathBuf};

use anyhow::{Result, bail};

pub fn run(context: &context::Context, paths: &[PathBuf], module: &str) -> Result<()> {
    let home = utils::get_home_dir()?;
    let target_dir = context.dotfiles_dir.join(module);

    for path in paths {
        for file in walk_files(path) {
            let file = file?.canonicalize()?;
            // A file being added is either:
            //
            // 1. Already inside the dotfiles repo (e.g. the user passed an absolute path
            //    that happens to live under ~/dotfiles/some_module/...).
            //    In this case, strip the dotfiles_dir prefix — which leaves
            //    "some_module/.config/foo" — then skip the first component (the module
            //    name) to get the home-relative path ".config/foo".
            //
            // 2. Somewhere under $HOME (the normal case: a live config file the user
            //    wants to adopt). Strip $HOME to get the home-relative path ".config/foo".
            //
            // Either way, `from_home` ends up as the path relative to $HOME,
            // which we then re-root under the target module dir to get the
            // final destination inside the dotfiles repo.
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
                bail!(
                    "{} is already in my dotfiles directory — not overwriting it",
                    target.display()
                );
            }

            create_parent_dirs(&target)?;

            fs::rename(&file, &target)?;

            symlink(&target, &file)?;

            eprintln!(
                "{} is now managed — moved into '{}' and linked back",
                file.display(),
                module
            );
        }
    }

    Ok(())
}
