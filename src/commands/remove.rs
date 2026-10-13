use anyhow::{Context, Result};
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{
    context,
    manifest::Manifest,
    utils::{self, normalize},
};

/// Stop managing files: replace each perch link with the real file, moved
/// back out of the dotfiles repo.
pub fn run(context: &context::Context, paths: &[PathBuf]) -> Result<()> {
    let dotfiles_dir = normalize(&context.dotfiles_dir);

    // (link in $HOME, file in the repo)
    let mut to_restore: Vec<(PathBuf, PathBuf)> = vec![];
    for path in paths {
        let path = utils::absolutize(path)?;
        let meta = fs::symlink_metadata(&path)
            .with_context(|| format!("i couldn't find {}", path.display()))?;

        let candidates: Vec<PathBuf> = if meta.is_dir() {
            walkdir::WalkDir::new(&path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.path_is_symlink())
                .map(|e| e.into_path())
                .collect()
        } else {
            vec![path.clone()]
        };

        for link in candidates {
            match fs::read_link(&link) {
                Ok(source) if normalize(&source).starts_with(&dotfiles_dir) => {
                    if fs::symlink_metadata(&source).is_err() {
                        eprintln!(
                            "{} points to {}, which is gone — skipping (perch prune cleans these up)",
                            link.display(),
                            source.display()
                        );
                        continue;
                    }
                    to_restore.push((link, source));
                }
                // inside a directory, foreign links and plain files are just not ours
                _ if meta.is_dir() => {}
                _ => eprintln!("{} isn't managed by me — skipping", link.display()),
            }
        }
    }

    let mut manifest = Manifest::load();
    let mut count = 0;
    let result = (|| -> Result<()> {
        for (link, source) in &to_restore {
            fs::remove_file(link)
                .with_context(|| format!("i couldn't remove {}", link.display()))?;
            if let Err(e) = utils::move_file(source, link) {
                // put the link back so nothing is lost
                let _ = utils::symlink(source, link);
                return Err(e);
            }
            manifest.remove(link);
            remove_empty_parents(source, &dotfiles_dir);
            count += 1;
            context.log(2, &format!("{} is back in place", link.display()));
        }
        Ok(())
    })();
    manifest.save();
    result?;

    eprintln!(
        "{} files are no longer managed. remember to commit the removal in {}",
        count,
        utils::unexpand_tilde(&context.dotfiles_dir)?.display()
    );
    Ok(())
}

/// Tidy up directories in the repo left empty by the move, stopping at the
/// module directory.
fn remove_empty_parents(source: &Path, dotfiles_dir: &Path) {
    let mut dir = source.parent();
    while let Some(current) = dir {
        let Ok(relative) = normalize(current)
            .strip_prefix(dotfiles_dir)
            .map(Path::to_path_buf)
        else {
            break;
        };
        // keep the module directory itself
        if relative.components().count() <= 1 || fs::remove_dir(current).is_err() {
            break;
        }
        dir = current.parent();
    }
}
