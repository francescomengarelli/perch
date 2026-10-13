use crate::{
    context,
    utils::{self, create_parent_dirs, move_file, symlink, unexpand_tilde, walk_source_files},
};
use std::{collections::HashSet, fs, path::PathBuf};

use anyhow::{Context as _, Result, bail};

pub fn run(context: &context::Context, paths: &[PathBuf], module: &str) -> Result<()> {
    let home = utils::get_home_dir()?;
    // Canonicalized files live under the real (symlink-resolved) home — e.g.
    // /private/var on macOS — so resolve $HOME the same way before stripping.
    let home = fs::canonicalize(&home).unwrap_or(home);
    let target_dir = context.dotfiles_dir.join(module);
    let dotfiles_dir =
        fs::canonicalize(&context.dotfiles_dir).unwrap_or_else(|_| context.dotfiles_dir.clone());

    if !context.all_modules.iter().any(|m| m == module) {
        eprintln!(
            "module '{}' isn't in your config — other machines won't link these files",
            module
        );
    } else if !context.filtered_modules.iter().any(|m| m == module) {
        eprintln!(
            "module '{}' isn't active on this machine — `perch sync` won't link these files here",
            module
        );
    }

    let ignore = utils::root_ignore(&context.dotfiles_dir)?;

    let mut count = 0;
    let mut conflict_count = 0;
    let mut seen: HashSet<PathBuf> = HashSet::new();
    let mut to_add: Vec<(PathBuf, PathBuf)> = vec![];
    for path in paths {
        context.log(1, &format!("adding {}...", path.display()));
        for entry in walk_source_files(path) {
            let entry = entry.with_context(|| format!("i couldn't walk {}", path.display()))?;
            let entry_meta = fs::symlink_metadata(&entry)
                .with_context(|| format!("i couldn't read {}", entry.display()))?;

            // Resolve each entry to the real file it points at. A broken
            // symlink has nothing to move — skip it instead of failing the
            // whole run.
            let file = match entry.canonicalize() {
                Ok(file) => file,
                Err(err) if entry_meta.file_type().is_symlink() => {
                    eprintln!("skipping broken symlink {} ({err})", entry.display());
                    continue;
                }
                Err(err) => {
                    return Err(err)
                        .with_context(|| format!("i couldn't resolve {}", entry.display()));
                }
            };

            // A file already inside the dotfiles repo is managed — never move
            // it between modules from here, just leave it alone.
            if file.starts_with(&dotfiles_dir) {
                context.log(
                    2,
                    &format!("{} is already managed — skipping", file.display()),
                );
                continue;
            }

            // The same real file can be reached more than once through
            // symlinks — only handle it the first time.
            if !seen.insert(file.clone()) {
                continue;
            }

            // The normal case: a live config file under $HOME the user wants
            // to adopt. Strip $HOME to get the home-relative path ".config/foo",
            // then re-root it under the target module dir.
            let Ok(from_home) = file.strip_prefix(&home) else {
                eprintln!(
                    "{} is outside your home directory — skipping",
                    file.display()
                );
                continue;
            };
            let target = target_dir.join(from_home);

            // Respect the repo-root .perchignore: sync would never link this
            // file back, so don't adopt it in the first place.
            let repo_relative = target.strip_prefix(&context.dotfiles_dir)?;
            if ignore
                .matched_path_or_any_parents(repo_relative, false)
                .is_ignore()
            {
                context.log(
                    1,
                    &format!("{} is ignored by .perchignore — skipping", file.display()),
                );
                continue;
            }

            if target.symlink_metadata().is_ok() {
                eprintln!(
                    "{} is already in my dotfiles directory — not overwriting it",
                    target.display()
                );
                conflict_count += 1;
            } else {
                to_add.push((file, target));
                count += 1;
            }
        }
    }

    if conflict_count > 0 {
        bail!(
            "{} conflict found. resolve before adding them",
            conflict_count
        );
    }

    for (file, target) in to_add {
        create_parent_dirs(&target)?;

        move_file(&file, &target)?;

        symlink(&target, &file)?;

        context.log(
            2,
            &format!(
                "{} is now managed — moved into '{}' and linked back",
                file.display(),
                module
            ),
        );
    }

    eprintln!(
        "{} files added to {} and symlinked them back",
        count,
        unexpand_tilde(&context.dotfiles_dir)?.display()
    );

    Ok(())
}
