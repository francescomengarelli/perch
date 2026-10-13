use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "perch", about = "your dotfiles, settled into place", version = env!("CARGO_PKG_VERSION"))]
pub struct Cli {
    /// Path to a custom config file (default: ~/.config/perch/config.toml)
    #[arg(long, global = true)]
    pub config: Option<PathBuf>,
    /// Increase output verbosity (-v, -vv)
    #[arg(short, global = true, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Symlink all enabled modules into your home directory.
    Sync {
        /// Show what would be linked and what conflicts, without changing anything.
        #[arg(short = 'n', long)]
        dry_run: bool,
    },
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
        #[arg(required = true)]
        path: Vec<PathBuf>,
    },
    /// Stop managing files: move them back out of the dotfiles directory.
    ///
    /// Each given perch symlink (or every perch symlink inside a given
    /// directory) is replaced by the real file, moved back from the repo.
    ///
    /// Example: `perch remove ~/.config/zathura`
    Remove {
        /// Symlinks, or directories containing symlinks, to stop managing.
        #[arg(required = true)]
        path: Vec<PathBuf>,
    },
    /// Remove stale symlinks: ones whose file left the repo, or whose module is no longer active.
    Prune {
        /// List what would be removed without changing anything.
        #[arg(short = 'n', long)]
        dry_run: bool,
    },
    /// Move the dotfiles directory to a new location and re-sync.
    ///
    /// Moves the dotfiles directory to `path` (which must not exist yet), re-points
    /// the symlinks that pointed into it, and saves the new location to the
    /// config file in use, if there is one.
    ///
    /// Example: `perch move-dir ~/new/dotfiles`
    MoveDir {
        /// The new path for the dotfiles directory.
        path: PathBuf,
    },

    /// Check for latest version and upgrade if needed
    SelfUpgrade,
}
