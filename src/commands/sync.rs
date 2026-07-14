use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Result, bail};

use crate::{context, utils};

pub fn run(context: &context::Context) -> Result<()> {
    let home = utils::get_home_dir()?;

    let mut conflict_count = 0;
    let mut to_symlink: Vec<(PathBuf, PathBuf)> = vec![];
    for module in &context.filtered_modules {
        let module_path = context.dotfiles_dir.join(module);
        for entry in utils::walk_dotfiles(&context.dotfiles_dir, &module_path) {
            let path = entry?;
            let relative = path.strip_prefix(&module_path)?;
            let target_path = home.join(&relative);

            // symlink from path to target_path

            if target_path.is_symlink() || target_path.exists() {
                if !is_our_symlink(&target_path, &path) {
                    eprintln!(
                        "{} is not mine. i am not touching it",
                        target_path.display()
                    );
                    conflict_count += 1;
                }
                continue;
            }

            to_symlink.push((path, target_path));
        }
    }

    if conflict_count > 0 {
        bail!(
            "{} conflicts found. resolve before syncing. no changes applied",
            conflict_count
        );
    }

    for (from, to) in to_symlink {
        utils::create_parent_dirs(&to)?;

        utils::symlink(&from, &to)?;
        context.log(2, &format!("{} is in place", to.display()));
    }

    eprintln!("{} modules synced", context.filtered_modules.len());

    Ok(())
}

fn is_our_symlink(target: &Path, source: &Path) -> bool {
    match fs::read_link(target) {
        Ok(actual) => utils::paths_equal(&actual, source),
        Err(_) => false,
    }
}
