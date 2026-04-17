use crate::cli::display::show_character;
use crate::combat::session::Session;
use crate::{combat::initiative::TurnOrder, model::character::CharacterSheet, storage::loader::load_character};
use std::path::PathBuf;
use std::{io, io::Write};

enum ReplCommand {
    Load(PathBuf),
    List,
    Show(String),
    Initiative,
    Next,
    Quit,
    Unknown,
}

fn parse_command(input: &str) -> ReplCommand {
    let mut parts = input.trim().splitn(2, ' ');
    match parts.next() {
	Some("load") => match parts.next() {
	    Some(path) => ReplCommand::Load(PathBuf::from(path)),
	    None => ReplCommand::Unknown,
	},
	Some("list") => ReplCommand::List,
	Some("show") => match parts.next() {
	    Some(name) => ReplCommand::Show(name.to_string()),
	    None => ReplCommand::Unknown,
	},
	Some("initiative") => ReplCommand::Initiative,
	Some("next") => ReplCommand::Next,
	Some("quit") | Some("exit") => ReplCommand::Quit,
	_ => ReplCommand::Unknown,
    }
}

pub fn run() {
    let mut session = Session::new();
    println!("Combat Helper. Type 'quit' to exit.");

    loop {
	print!("> ");
	io::stdout().flush().unwrap();

	let mut input = String::new();
	io::stdin().read_line(&mut input).unwrap();

	match parse_command(&input) {
	    ReplCommand::Load(path) => {
		match load_character(&path) {
		    Ok(c) => {
			println!("Loaded {}", c.name());
			session.characters.insert(c.name().to_string(), c);
		    }
		    Err(e) => println!("Error: {e}"),
		}
	    }
	    ReplCommand::List => {
		if session.characters.is_empty() {
		    println!("No characters loaded.");
		} else {
		    for (name, _) in &session.characters {
			println!("{}", name);
		    }
		}
	    }
	    ReplCommand::Show(name) => {
		match session.characters.get(&name) {
		    Some(c) => show_character(c),
		    None => println!("No character named '{name}'")
		}
	    }
	    ReplCommand::Initiative => {
		let entries = session.characters
		    .iter()
		    .map(|(name, c)| (name.to_string(), c.initiative()))
		    .collect();
		session.turn_order = Some(TurnOrder::new(entries));
		session.current_turn = 0;
		if let Some(ref order) = session.turn_order {
		    for (name, roll) in &order.entries {
			println!(" {}: {}", name, roll);
		    }
		}
	    }
	    ReplCommand::Next => {
		match session.next_turn() {
		    Some(name) => println!("Turn: {}", name),
		    None => println!("No initiative order. Use 'roll' first."),
		}
	    }
	    ReplCommand::Quit => break,
	    ReplCommand::Unknown => println!("Unknown command."),
	}
    }
}
