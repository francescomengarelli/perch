use anyhow::{Result, bail};

use crate::{context, stow, utils};

pub fn run(context: &context::Context) -> Result<()> {
    let home = utils::get_home_dir()?;

    let mut conflict_count = 0;
    for module in &context.filtered_modules {
        let module_path = context.dotfiles_dir.join(module);
        for entry in utils::walk_files(&module_path) {
            let path = entry?;
            let relative = path.strip_prefix(&module_path)?;
            let target_path = home.join(&relative);

            if target_path.is_symlink() || target_path.exists() {
                if !stow::is_our_symlink(&target_path, &path) {
                    eprintln!(
                        "{} is not mine. i am not touching it",
                        target_path.display()
                    );
                    conflict_count += 1;
                }
            }
        }
    }

    if conflict_count > 0 {
        bail!(
            "{} conflicts found. resolve before syncing. no changes applied",
            conflict_count
        );
    }

    for module in &context.filtered_modules {
        context.log(1, &format!("settling '{}' into place...", module));
        let module_path = context.dotfiles_dir.join(module);
        stow::stow(&module_path, &home, context.verbose > 1)?;
    }

    eprintln!("{} modules synced", context.filtered_modules.len());

    Ok(())
}
