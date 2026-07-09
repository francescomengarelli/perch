use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "dot", about = "Dotfiles manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Sync,
    Bootstrap { repo: String },
    Update,
    Edit,
    Status,
}
