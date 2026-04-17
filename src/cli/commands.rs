// Command parsing and dispatch
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "combathelper", about = "A CLI combat tracker for tabletop RPGs")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Clone)]
pub enum Command {
    /// Display a character's stats, inventory, and actions
    Show {
	/// Path to the character JSON file
	file: PathBuf,
    },
}
