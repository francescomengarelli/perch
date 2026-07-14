use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "perch", about = "your dotfiles, settled into place", version = env!("CARGO_PKG_VERSION"))]
pub struct Cli {
    /// Path to a custom config file (default: ~/.config/perch/config.toml)
    #[arg(long, global = true)]
    pub config: Option<PathBuf>,
    /// Increase output verbosity (-v, -vv, -vvv)
    #[arg(short, global = true, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Symlink all enabled modules into your home directory.
    Sync,
    /// Pull the latest changes from remote and re-sync.
    ///
    /// Runs `git pull` in your dotfiles repo, then re-applies all symlinks. If the
    /// updated config points to a new `dotfiles_dir`, I'll migrate everything over automatically.
    Update,
    /// Show the git status of your dotfiles repo.
    Status,
    /// Move files into your dotfiles directory and symlink them back.
    ///
    /// Files at the given paths are moved into `[dotfiles_dir]/[module]/` and
    /// symlinked back to their original locations. Directories are moved recursively.
    ///
    /// Example: `perch add linux ~/.config/zathura`
    Add {
        /// Module to add to (e.g. "macos", "work", "common")
        module: String,
        /// Paths to move into the module. Directories are moved recursively.
        path: Vec<PathBuf>,
    },
    /// Move the dotfiles directory to a new location and re-sync.
    ///
    /// Physically moves the dotfiles directory to `path`, updates all symlinks,
    /// and saves the new location to ~/.config/perch/config.toml if it exists.
    ///
    /// Example: `perch move-dir ~/new/dotfiles`
    MoveDir {
        /// The new path for the dotfiles directory.
        path: PathBuf,
    },

    /// Check for latest version and upgrade if needed
    SelfUpgrade,
}
