use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// A dotfiles manager!
///
/// Manages symlinks from a central dotfiles directory to their target
/// locations on your system.
#[derive(Parser)]
#[command(name = "perch", about = "Dotfiles manager", version = env!("CARGO_PKG_VERSION"))]
pub struct Cli {
    /// Path to a custom config file (default: ~/.config/perch/config.toml)
    #[arg(long, global = true)]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

/// Available subcommands
#[derive(Subcommand)]
pub enum Commands {
    /// Create symlinks for all enabled modules from the dotfiles directory
    /// to their target locations.
    Sync,

    /// Pulls the latest changes from the remote repository and re-syncs all symlinks.
    ///
    /// Runs `git pull` in the dotfiles repository, then re-applies all symlinks. If the newly
    /// loaded `config.toml` specifies a `dotfiles_dir` that differs from the current location,
    /// it will automatically migrate the dotfiles directory to the new path.
    Update,

    /// Show the git status of the dotfiles repository.
    ///
    /// Runs `git status` scoped to the dotfiles directory
    Status,

    /// Move files into the dotfiles directory and symlink them back.
    ///
    /// Files at the given paths are MOVED (not copied) into
    /// `[dotfiles_dir]/[module]/`, then symlinked back to their
    /// original locations. Directories are moved recursively.
    ///
    /// Example: `perch add linux ~/.config/zathura` moves `~/.config/zathura` to
    /// `[dotfiles_dir]/linux/.config/zathura`, then symlinks the latter back to
    /// the former.
    Add {
        /// Name of the module to create or add to (e.g. "macos", "work", "common")
        module: String,

        /// Paths to move into the module. Directories are moved recursively.
        path: Vec<PathBuf>,
    },

    /// Move the dotfiles directory to a new location and re-sync all symlinks.
    ///
    /// The dotfiles directory is physically moved to `path`, then all symlinks
    /// are updated to point to the new location. ~/.config/perch/config.toml is updated automatically
    /// (if it exists)
    ///
    /// Example: `perch move-dir ~/new/dotfiles`
    MoveDir {
        /// The new path for the dotfiles directory.
        path: PathBuf,
    },
}
