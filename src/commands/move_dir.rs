use anyhow::{Context, Result, bail};
use std::{
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
};

use crate::{
    context,
    utils::{self, normalize, paths_equal},
};

pub fn run(context: &mut context::Context, path: &Path) -> Result<()> {
    let old = context.dotfiles_dir.clone();
    let new = utils::absolutize(path)?;

    if paths_equal(&old, &new) {
        eprintln!("nothing to move. dotfiles already at {}", new.display());
        return Ok(());
    }
    if !old.is_dir() {
        bail!("there's no dotfiles directory at {} to move", old.display());
    }
    let (old_norm, new_norm) = (normalize(&old), normalize(&new));
    if new_norm.starts_with(&old_norm) || old_norm.starts_with(&new_norm) {
        bail!(
            "i can't move {} to {} — one is inside the other",
            old.display(),
            new.display()
        );
    }
    if new.symlink_metadata().is_ok() {
        let is_empty_dir = new.is_dir()
            && !new.is_symlink()
            && fs::read_dir(&new)
                .with_context(|| format!("i couldn't read {}", new.display()))?
                .next()
                .is_none();
        if !is_empty_dir {
            bail!(
                "{} already exists — pick a path that doesn't. nothing was changed",
                new.display()
            );
        }
        fs::remove_dir(&new).with_context(|| format!("i couldn't prepare {}", new.display()))?;
    }
    utils::create_parent_dirs(&new)?;

    let copied = match fs::rename(&old, &new) {
        Ok(()) => false,
        Err(e) if e.kind() == ErrorKind::CrossesDevices => {
            context.log(1, "different filesystem — copying instead of renaming");
            if let Err(e) = copy_tree(&old, &new) {
                // `new` didn't exist (or was empty) before: it's ours to clean up
                let _ = fs::remove_dir_all(&new);
                return Err(e)
                    .context("something went wrong during the move - nothing was changed");
            }
            true
        }
        Err(e) => {
            return Err(e).with_context(|| {
                format!(
                    "i couldn't move {} to {} - nothing was changed",
                    old.display(),
                    new.display()
                )
            });
        }
    };

    let relinked = relink(context, &old, &new)?;

    let deletion_note = if copied {
        fs::remove_dir_all(&old)
            .err()
            .map(|e| format!(", but i couldn't delete {}: {e}", old.display()))
    } else {
        None
    };

    context.dotfiles_dir = new.clone();

    eprintln!(
        "everything is now settled in {} ({} links updated){}",
        new.display(),
        relinked,
        deletion_note.as_deref().unwrap_or("")
    );
    Ok(())
}

/// Copy a directory tree, keeping symlinks as symlinks and empty dirs as dirs.
fn copy_tree(from: &Path, to: &Path) -> Result<()> {
    for entry in walkdir::WalkDir::new(from) {
        let entry = entry?;
        let relative = entry.path().strip_prefix(from)?;
        let target = to.join(relative);
        let file_type = entry.file_type();

        if file_type.is_dir() {
            fs::create_dir_all(&target)
                .with_context(|| format!("i couldn't create {}", target.display()))?;
        } else if file_type.is_symlink() {
            let link = fs::read_link(entry.path())?;
            utils::symlink(&link, &target)?;
        } else {
            utils::copy(entry.path(), &target)?;
        }
    }
    Ok(())
}

/// Re-point the links in `$HOME` that pointed into `old` at the same file in
/// `new`. Links that point anywhere else are not ours and stay untouched.
fn relink(context: &context::Context, old: &Path, new: &Path) -> Result<usize> {
    let home = utils::get_home_dir()?;
    let mut count = 0;

    for module in &context.all_modules {
        let module_dir = new.join(module);
        if !module_dir.is_dir() {
            continue;
        }

        for file in utils::walk_files(&module_dir) {
            let file = file?;
            let relative: PathBuf = file.strip_prefix(&module_dir)?.to_path_buf();
            let link = home.join(&relative);
            let previous = old.join(module).join(&relative);

            match fs::read_link(&link) {
                Ok(target) if paths_equal(&target, &previous) => {
                    fs::remove_file(&link)
                        .with_context(|| format!("i couldn't remove {}", link.display()))?;
                    utils::symlink(&file, &link)?;
                    count += 1;
                    context.log(
                        2,
                        &format!("{} now points to {}", link.display(), file.display()),
                    );
                }
                _ => {}
            }
        }
    }

    Ok(count)
}
