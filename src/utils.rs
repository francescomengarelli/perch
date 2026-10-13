use anyhow::{Context, Result};
use ignore::WalkBuilder;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn expand_tilde(path: &Path) -> Result<PathBuf> {
    if let Ok(stripped) = path.strip_prefix("~") {
        let home = get_home_dir()?;
        Ok(home.join(stripped))
    } else {
        Ok(path.to_path_buf())
    }
}

pub fn unexpand_tilde(path: &Path) -> Result<PathBuf> {
    let home = get_home_dir()?;
    if let Ok(stripped) = path.strip_prefix(home) {
        Ok(PathBuf::from("~").join(stripped))
    } else {
        Ok(path.to_path_buf())
    }
}

pub fn get_home_dir() -> Result<PathBuf> {
    dirs::home_dir().context("i couldn't find your home directory — is $HOME set?")
}

pub fn absolutize(path: &Path) -> Result<PathBuf> {
    let stripped = expand_tilde(path)?;

    if stripped.is_absolute() {
        Ok(stripped)
    } else {
        Ok(std::env::current_dir()
            .with_context(|| "i couldn't find the current directory — has it been deleted?")?
            .join(stripped))
    }
}

pub fn paths_equal(a: &Path, b: &Path) -> bool {
    // Canonicalize only the stored paths, not the symlink targets themselves
    // (canonicalize follows symlinks, so we normalize lexically instead)
    let a = normalize(a);
    let b = normalize(b);
    a == b
}

pub fn normalize(p: &Path) -> std::path::PathBuf {
    // Collapse . and .. lexically without hitting the filesystem
    let mut out = std::path::PathBuf::new();
    for component in p.components() {
        match component {
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                out.pop();
            }
            c => out.push(c),
        }
    }
    out
}

pub fn walk_files(dir: &Path) -> impl Iterator<Item = Result<PathBuf>> {
    walkdir::WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| match e {
            Ok(e) if e.file_type().is_dir() => None,
            Ok(e) => Some(Ok(e.into_path())),
            Err(e) => Some(Err(e.into())),
        })
}

/// Walk a source tree the way `add` does: honor `.perchignore` files found
/// inside it, keep hidden files, ignore git's own ignore files.
pub fn walk_source_files(dir: &Path) -> impl Iterator<Item = Result<PathBuf>> {
    let mut builder = WalkBuilder::new(dir);

    builder
        .hidden(false)
        .add_custom_ignore_filename(".perchignore")
        .git_ignore(false)
        .git_global(false)
        .git_exclude(false);

    builder.build().filter_map(|e| match e {
        Ok(e) if e.file_type().map(|t| t.is_dir()).unwrap_or(false) => None,
        Ok(e) => Some(Ok(e.into_path())),
        Err(e) => Some(Err(e.into())),
    })
}

pub fn walk_dotfiles(root: &Path, module: &Path) -> impl Iterator<Item = Result<PathBuf>> {
    let mut builder = WalkBuilder::new(module);

    builder
        .hidden(false)
        .add_custom_ignore_filename(".perchignore")
        .git_ignore(false)
        .git_global(false)
        .git_exclude(false);

    let root_ignore = root.join(".perchignore");
    if root_ignore.exists() {
        builder.add_ignore(&root_ignore);
    }

    builder.build().filter_map(|e| match e {
        Ok(e) if e.file_type().map(|t| t.is_dir()).unwrap_or(false) => None,
        // ignore rules configure perch; they don't belong in $HOME
        Ok(e) if e.file_name() == ".perchignore" => None,
        Ok(e) => Some(Ok(e.into_path())),
        Err(e) => Some(Err(e.into())),
    })
}

pub fn create_parent_dirs(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("i couldn't create {}", parent.display()))?;
    }

    Ok(())
}

pub fn symlink(original: &Path, link: &Path) -> Result<()> {
    std::os::unix::fs::symlink(original, link).with_context(|| {
        format!(
            "i couldn't symlink {} to {}",
            original.display(),
            link.display()
        )
    })
}

pub fn copy(from: &Path, to: &Path) -> Result<()> {
    create_parent_dirs(to)?;

    fs::copy(from, &to).with_context(|| {
        format!(
            "i couldn't copy from {} to {}",
            from.display(),
            to.display()
        )
    })?;
    Ok(())
}

/// Matcher for the repo-root `.perchignore`, whose patterns are relative to
/// the dotfiles directory (e.g. `common/.config/nvim/lazy-lock.json`).
pub fn root_ignore(dotfiles_dir: &Path) -> Result<ignore::gitignore::Gitignore> {
    let mut builder = ignore::gitignore::GitignoreBuilder::new(dotfiles_dir);
    let root_ignore = dotfiles_dir.join(".perchignore");
    if root_ignore.exists() {
        if let Some(err) = builder.add(&root_ignore) {
            return Err(err).with_context(|| format!("i couldn't read {}", root_ignore.display()));
        }
    }
    builder
        .build()
        .with_context(|| format!("i couldn't read {}", root_ignore.display()))
}

/// Move a file, falling back to copy + delete when `from` and `to` live on
/// different filesystems.
pub fn move_file(from: &Path, to: &Path) -> Result<()> {
    match fs::rename(from, to) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::CrossesDevices => {
            fs::copy(from, to).with_context(|| {
                format!("i couldn't copy {} to {}", from.display(), to.display())
            })?;
            fs::remove_file(from).with_context(|| format!("i couldn't remove {}", from.display()))
        }
        Err(e) => Err(e)
            .with_context(|| format!("i couldn't move {} to {}", from.display(), to.display())),
    }
}

/// Resolve a configured `dotfiles_dir`: `~` is expanded and relative paths are
/// taken relative to `$HOME`, so the result never depends on the cwd.
pub fn resolve_dotfiles_dir(path: &Path) -> Result<PathBuf> {
    let expanded = expand_tilde(path)?;
    if expanded.is_absolute() {
        Ok(expanded)
    } else {
        Ok(get_home_dir()?.join(expanded))
    }
}
