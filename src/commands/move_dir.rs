use anyhow::{Context, Result, bail};
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{
    context,
    utils::{self},
};

pub fn run(context: &mut context::Context, path: &Path) -> Result<()> {
    let path = utils::absolutize(path)?;

    if path == context.dotfiles_dir {
        eprintln!("Nothing to move. dotfiles already at {}", path.display());
        return Ok(());
    }

    let result = (|| -> Result<()> {
        for file in utils::walk_files(&context.dotfiles_dir) {
            let file = file?.canonicalize()?;

            let relative_to_dir: PathBuf = file.strip_prefix(&context.dotfiles_dir)?.to_path_buf();
            let target = path.join(&relative_to_dir);

            let mut components = relative_to_dir.components();
            let first_str = components
                .next()
                .expect("Path must have at least one component. (should be at LEAST ~/home/user/)")
                .as_os_str();

            let rest: PathBuf = components.collect();

            utils::copy(&file, &target)?;
            if context.all_modules.iter().any(|s| first_str == s.as_str())
                && rest.components().next().is_some()
            {
                let symlinked = utils::get_home_dir()?.join(rest);

                let handle_symlink = if let Err(e) = fs::read_link(&symlinked) {
                    if !matches!(
                        e.kind(),
                        std::io::ErrorKind::NotFound | std::io::ErrorKind::InvalidInput
                    ) {
                        bail!(
                            "I tried to read the symlink at {} — {}",
                            symlinked.display(),
                            e
                        )
                    }
                    false
                } else {
                    true
                };

                if handle_symlink {
                    fs::remove_file(&symlinked)
                        .with_context(|| format!("I couldn't to remove {}", symlinked.display()))?;
                    std::os::unix::fs::symlink(&target, &symlinked).with_context(|| {
                        format!(
                            "I couldn't symlink from {} to {}",
                            target.display(),
                            symlinked.display()
                        )
                    })?;
                }
            }
            eprintln!("moved {} to new path.", file.display());
        }
        Ok(())
    })();

    if let Err(e) = result {
        let deletion_note = fs::remove_dir_all(&path)
            .err()
            .map(|e| format!(", but i couldn't delete {}: {e}", path.display()));
        return Err(e).context(format!(
            "something went wrong during the move - nothing was changed{}",
            deletion_note.as_deref().unwrap_or(""),
        ));
    }

    let deletion_note = fs::remove_dir_all(&context.dotfiles_dir).err().map(|e| {
        format!(
            ", but i couldn't delete {}: {e}",
            context.dotfiles_dir.display()
        )
    });

    eprintln!(
        "everything is now settled in {}{}",
        path.display(),
        deletion_note.as_deref().unwrap_or("")
    );
    context.dotfiles_dir = path.clone();
    Ok(())
}
