use reedline::{
    default_emacs_keybindings, ColumnarMenu, DefaultPrompt, Emacs, KeyCode,
    KeyModifiers, MenuBuilder, Reedline, ReedlineEvent, Signal,
};

use crate::cli::completer::CommandCompleter;
use crate::cli::display::show_character;
use crate::combat::session::{InCombat, Session, Setup};
use crate::model::character::dnd::DndCharacter;
use crate::{model::character::CharacterSheet, storage::loader::load_character};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

enum ReplCommand {
    Load(PathBuf),
    List,
    Show(String),
    Start,
    Order,
    Next,
    Quit,
    Unknown,
    SetInitiative(String, i32),
    Damage(String, i32),
    Heal(String, i32)
}

enum ActiveSession {
    Setup(Session<DndCharacter, Setup>),
    InCombat(Session<DndCharacter, InCombat>)
}

fn parse_command(input: &str) -> ReplCommand {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    match parts.as_slice() {
        ["show", name] => ReplCommand::Show(name.to_string()),
        ["load", path] => ReplCommand::Load(PathBuf::from(path)),
        ["initiative", name, value] => match value.parse::<i32>() {
            Ok(n) => ReplCommand::SetInitiative(name.to_string(), n),
            Err(_) => ReplCommand::Unknown,
        },
	["start"] => ReplCommand::Start,
        ["list"] => ReplCommand::List,
        ["order"] => ReplCommand::Order,
        ["next"] => ReplCommand::Next,
	["heal", name, amount] => match amount.parse::<i32>() {
	    Ok(n) => ReplCommand::Heal(name.to_string(), n),
	    Err(_) => ReplCommand::Unknown,
	}
	["damage", name, amount] => match amount.parse::<i32>() {
	    Ok(n) => ReplCommand::Damage(name.to_string(), n),
	    Err(_) => ReplCommand::Unknown,
	}
        ["quit"] | ["exit"] => ReplCommand::Quit,
        _ => ReplCommand::Unknown,
    }
}

pub fn run() {
    let mut active = ActiveSession::Setup(Session::new());
    println!("Combat Helper. Type 'quit' to exit.");

    let mut keybindings = default_emacs_keybindings();
    keybindings.add_binding(
        KeyModifiers::NONE,
        KeyCode::Tab,
        ReedlineEvent::UntilFound(vec![
            ReedlineEvent::Menu("completion_menu".to_string()),
            ReedlineEvent::MenuNext,
        ]),
    );

    let names: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
    let completer = Box::new(CommandCompleter {
        names: Arc::clone(&names),
    });
    let completion_menu = Box::new(ColumnarMenu::default().with_name("completion_menu"));

    let mut line_editor = Reedline::create()
        .with_completer(completer)
        .with_menu(reedline::ReedlineMenu::EngineCompleter(completion_menu))
        .with_edit_mode(Box::new(Emacs::new(keybindings)));

    let prompt = DefaultPrompt::default();

    loop {
        match line_editor.read_line(&prompt) {
            Ok(Signal::Success(input)) => match parse_command(&input) {
		// Load a character
                ReplCommand::Load(path) => match &mut active {
		    // If we are in combat, don't load the character
		    ActiveSession::InCombat(_) => println!("Cannot load characters during combat."),
		    ActiveSession::Setup(session) => match load_character(&path) {
			Ok(c) => {
                            let name = session.unique_name(c.name());
                            println!("Loaded {}", name);
                            names.lock().unwrap().push(name.clone());
                            session.characters.insert(name, c);
			}
			Err(e) => println!("Error: {e}"),
                    },
		},
		// List all characters
                ReplCommand::List => {
		    let characters = match &active {
			ActiveSession::InCombat(s) => &s.characters,
			ActiveSession::Setup(s) => &s.characters,
		    };
                    if characters.is_empty() {
                        println!("No characters loaded.");
                    } else {
                        for name in characters.keys() {
                            println!("{}", name);
                        }
                    }
                }
                ReplCommand::Show(name) => {
		    let characters = match &active {
			ActiveSession::InCombat(s) => &s.characters,
			ActiveSession::Setup(s) => &s.characters,
		    };
		    match characters.get(&name) {
			Some(c) => show_character(c),
			None => println!("No character named '{name}'"),
		    }
                },
                ReplCommand::Order => match &active {
		    ActiveSession::Setup(_) => println!("Start combat first with 'start'."),
		    ActiveSession::InCombat(session) => match &session.turn_order {
			None => println!("No initiative order."),
			Some(order) => {
			    for (i, entry) in order.entries.iter().enumerate() {
				if i == session.current_turn {
				    println!("  *  {} ({})", entry.name, entry.initiative);
				} else {
				    println!("     {} ({})", entry.name, entry.initiative);
				}
			    }
			}
		    }
                },
                ReplCommand::Start => {
		    active = match active {
			ActiveSession::InCombat(session) => {
			    println!("Combat already started.");
			    ActiveSession::InCombat(session)
			},
			ActiveSession::Setup(session) if session.characters.is_empty() => {
			    println!("No characters have been loaded.");
			    ActiveSession::Setup(session)
			},
			ActiveSession::Setup(session) => {
			    let combat = session.start();
			    if let Some(ref order) = combat.turn_order {
				for entry in &order.entries {
				    // We can just take the initiative since start calculates
				    // the initiative for each entry
				    println!("     {} ({})", entry.name, entry.initiative)
				}
			    }
			    println!("Combat started!");
			    ActiveSession::InCombat(combat)
			}
		    }
                },
                ReplCommand::SetInitiative(name, value) => match &mut active {
		    ActiveSession::InCombat(session) => session.set_initiative(name, value),
		    ActiveSession::Setup(session) => session.set_initiative(name, value),
                }
                ReplCommand::Next => match &mut active {
		    ActiveSession::Setup(_) => println!("Start combat first."),
		    ActiveSession::InCombat(session) => match session.next_turn() {
			Some(name) => println!("Turn: {}", name),
			None => println!("No initiative order."),
		    }
                },
		ReplCommand::Heal(name, amount) => match &mut active {
		    ActiveSession::Setup(_) => println!("Start combat first."),
		    ActiveSession::InCombat(session) => match session.characters.get_mut(&name) {
			Some(c) => {
			    c.apply_heal(amount);
			    println!("{} heals for {} damage. {}", name, amount, c.health_to_string())
			},
			None => println!("No character with name {}", name)
		    }
		},
		ReplCommand::Damage(name, amount) => match &mut active {
		    ActiveSession::Setup(_) => println!("Start combat first."),
		    ActiveSession::InCombat(session) => match session.characters.get_mut(&name) {
			Some(c) => {
			    c.apply_damage(amount);
			    println!("{} takes {} damage. {}", name, amount, c.health_to_string())
			},
			None => println!("No character with name {}", name)
		    }
		},
                ReplCommand::Quit => break,
                ReplCommand::Unknown => println!("Unknown command."),
            },
            Ok(Signal::CtrlD) | Ok(Signal::CtrlC) => break,
            Ok(_) => {}
            Err(e) => {
                println!("Error: {e}");
                break;
            }
        }
    }
}
