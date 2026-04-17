use clap::Parser;
use cli::commands::{Cli, Command};

mod cli;
mod combat;
mod model;
mod state;
mod storage;

fn main() {
    let cli = Cli::parse();

    match cli.command {
	Command::Show { file } => {
	    let character = storage::loader::load_character(&file).unwrap();
	    cli::display::show_character(&character);
	}
    }
}
