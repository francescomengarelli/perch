//! A record of the links perch has put in place, so `perch prune` can find
//! stale ones without scanning all of `$HOME`.
//!
//! Lives at `${XDG_STATE_HOME:-~/.local/state}/perch/links`, one absolute path
//! per line. It is a convenience: failing to read or write it only warns.

use anyhow::{Context, Result};
use std::{
    collections::BTreeSet,
    ffi::OsStr,
    fs,
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
};

use crate::utils;

pub struct Manifest {
    path: Option<PathBuf>,
    links: BTreeSet<PathBuf>,
}

impl Manifest {
    pub fn load() -> Self {
        let path = match state_path() {
            Ok(path) => path,
            Err(e) => {
                eprintln!("i can't keep track of my links: {e:#}");
                return Manifest {
                    path: None,
                    links: BTreeSet::new(),
                };
            }
        };

        let links = match fs::read(&path) {
            Ok(bytes) => bytes
                .split(|b| *b == b'\n')
                .filter(|line| !line.is_empty())
                .map(|line| PathBuf::from(OsStr::from_bytes(line)))
                .collect(),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => BTreeSet::new(),
            Err(e) => {
                eprintln!("i couldn't read my link record at {}: {e}", path.display());
                BTreeSet::new()
            }
        };

        Manifest {
            path: Some(path),
            links,
        }
    }

    pub fn insert(&mut self, link: &Path) {
        self.links.insert(link.to_path_buf());
    }

    pub fn remove(&mut self, link: &Path) {
        self.links.remove(link);
    }

    pub fn is_empty(&self) -> bool {
        self.links.is_empty()
    }

    pub fn links(&self) -> Vec<PathBuf> {
        self.links.iter().cloned().collect()
    }

    pub fn save(&self) {
        let Some(path) = &self.path else { return };
        if let Err(e) = self.write(path) {
            eprintln!("i couldn't update my link record: {e:#}");
        }
    }

    fn write(&self, path: &Path) -> Result<()> {
        utils::create_parent_dirs(path)?;
        let mut contents = Vec::new();
        for link in &self.links {
            contents.extend_from_slice(link.as_os_str().as_bytes());
            contents.push(b'\n');
        }
        fs::write(path, contents).with_context(|| format!("i couldn't write {}", path.display()))
    }
}

fn state_path() -> Result<PathBuf> {
    let state_home = match std::env::var_os("XDG_STATE_HOME") {
        Some(dir) if Path::new(&dir).is_absolute() => PathBuf::from(dir),
        _ => utils::get_home_dir()?.join(".local/state"),
    };
    Ok(state_home.join("perch/links"))
}
