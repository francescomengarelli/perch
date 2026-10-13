#![allow(dead_code)]

use assert_cmd::Command;
use std::{
    fs,
    path::{Path, PathBuf},
};
use tempfile::TempDir;

/// A throwaway `$HOME` with a `~/dotfiles` repo inside it.
pub struct Sandbox {
    _dir: TempDir,
    pub home: PathBuf,
}

impl Sandbox {
    pub fn new() -> Self {
        let dir = tempfile::tempdir().unwrap();
        // Canonicalize so paths match what perch computes (e.g. /private/var on macOS).
        let home = fs::canonicalize(dir.path()).unwrap();
        fs::create_dir_all(home.join("dotfiles")).unwrap();
        Sandbox { _dir: dir, home }
    }

    pub fn path(&self, rel: &str) -> PathBuf {
        self.home.join(rel)
    }

    pub fn dotfiles(&self) -> PathBuf {
        self.home.join("dotfiles")
    }

    /// Write a file relative to `$HOME`, creating parent dirs.
    pub fn write(&self, rel: &str, content: &str) -> PathBuf {
        let path = self.path(rel);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, content).unwrap();
        path
    }

    pub fn read(&self, rel: &str) -> String {
        fs::read_to_string(self.path(rel)).unwrap()
    }

    pub fn mkdir(&self, rel: &str) {
        fs::create_dir_all(self.path(rel)).unwrap();
    }

    /// Write `~/.config/perch/config.toml`.
    pub fn config(&self, toml: &str) -> PathBuf {
        self.write(".config/perch/config.toml", toml)
    }

    pub fn state_dir(&self) -> PathBuf {
        self.home.join(".local/state")
    }

    pub fn perch(&self, args: &[&str]) -> Command {
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_perch"));
        cmd.env("HOME", &self.home)
            .env("XDG_STATE_HOME", self.state_dir())
            .current_dir(&self.home)
            .args(args);
        cmd
    }

    pub fn git(&self, dir: &Path, args: &[&str]) {
        let status = std::process::Command::new("git")
            .env("HOME", &self.home)
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_AUTHOR_NAME", "t")
            .env("GIT_AUTHOR_EMAIL", "t@t")
            .env("GIT_COMMITTER_NAME", "t")
            .env("GIT_COMMITTER_EMAIL", "t@t")
            .arg("-C")
            .arg(dir)
            .args(args)
            .output()
            .unwrap();
        assert!(
            status.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&status.stderr)
        );
    }

    pub fn is_link(&self, rel: &str) -> bool {
        self.path(rel).is_symlink()
    }

    /// Assert `~/<home_rel>` is a symlink pointing at `~/<repo_rel>`.
    pub fn assert_link(&self, home_rel: &str, repo_rel: &str) {
        let link = self.path(home_rel);
        let target = fs::read_link(&link)
            .unwrap_or_else(|e| panic!("{} is not a symlink: {e}", link.display()));
        assert_eq!(
            target,
            self.path(repo_rel),
            "{} points elsewhere",
            link.display()
        );
    }
}
