use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// A dotfiles manager!
///
/// Manages symlinks from a central dotfiles directory to their target
/// locations on your system. Run `dot <subcommand> --help` for details.
#[derive(Parser)]
#[command(name = "dot", about = "Dotfiles manager", version = env!("CARGO_PKG_VERSION"))]
pub struct Cli {
    /// Path to a custom config file (default: ~/.config/dot/config.toml)
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

    /// Pull the latest changes from the remote and re-sync all symlinks.
    ///
    /// Runs `git pull` in the dotfiles repo, then re-applies symlinks
    /// via `sync`.
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
    /// Example: `dot add linux ~/.config/zathura` moves `~/.config/zathura` to
    /// `[dotfiles_dir]/linux/.config/zathura`, then symlinks the latter back to
    /// the former.
    Add {
        /// Name of the module to create or add to (e.g. "macos", "work", "common")
        module: String,

        /// Paths to move into the module. Directories are moved recursively.
        path: Vec<PathBuf>,
    },
}
