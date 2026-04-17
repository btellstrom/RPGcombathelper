use reedline::{
    default_emacs_keybindings, ColumnarMenu, DefaultPrompt, Emacs, KeyCode,
    KeyModifiers, MenuBuilder, Reedline, ReedlineEvent, Signal,
};

use crate::cli::completer::CommandCompleter;
use crate::cli::display::show_character;
use crate::combat::session::Session;
use crate::{model::character::CharacterSheet, storage::loader::load_character};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

enum ReplCommand {
    Load(PathBuf),
    List,
    Show(String),
    Initiative,
    Order,
    Next,
    Quit,
    Unknown,
    SetInitiative(String, i32),
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
        ["initiative"] => ReplCommand::Initiative,
        ["list"] => ReplCommand::List,
        ["order"] => ReplCommand::Order,
        ["next"] => ReplCommand::Next,
        ["quit"] | ["exit"] => ReplCommand::Quit,
        _ => ReplCommand::Unknown,
    }
}

pub fn run() {
    let mut session = Session::new();
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
                ReplCommand::Load(path) => match load_character(&path) {
                    Ok(c) => {
                        let name = session.unique_name(c.name());
                        println!("Loaded {}", name);
                        names.lock().unwrap().push(name.clone());
                        session.characters.insert(name, c);
                    }
                    Err(e) => println!("Error: {e}"),
                },
                ReplCommand::List => {
                    if session.characters.is_empty() {
                        println!("No characters loaded.");
                    } else {
                        for (name, _) in &session.characters {
                            println!("{}", name);
                        }
                    }
                }
                ReplCommand::Show(name) => match session.characters.get(&name) {
                    Some(c) => show_character(c),
                    None => println!("No character named '{name}'"),
                },
                ReplCommand::Order => match &session.turn_order {
                    None => println!("No initiative order set."),
                    Some(order) => {
                        for (i, entry) in order.entries.iter().enumerate() {
                            if i == session.current_turn {
                                println!(" * {} ({})", entry.name, entry.initiative);
                            } else {
                                println!("   {} ({})", entry.name, entry.initiative);
                            }
                        }
                    }
                },
                ReplCommand::Initiative => {
                    session.roll_initiative();
                    if let Some(ref order) = session.turn_order {
                        for entry in &order.entries {
                            println!(" {}: {}", entry.name, entry.initiative);
                        }
                    }
                }
                ReplCommand::SetInitiative(name, value) => {
                    session.set_initiative(name, value);
                }
                ReplCommand::Next => match session.next_turn() {
                    Some(name) => println!("Turn: {}", name),
                    None => println!("No initiative order. Use 'initiative' first."),
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
