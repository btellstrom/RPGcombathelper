use crate::{combat::initiative::TurnOrder, model::character::CharacterSheet};
use std::collections::HashMap;

pub struct Session<C: CharacterSheet> {
    pub characters : HashMap<String, C>,
    pub turn_order: Option<TurnOrder>,
    pub current_turn: usize,
}

impl<C: CharacterSheet> Session<C> {
    pub fn new() -> Self {
	Session {
	    characters: HashMap::new(),
	    turn_order: None,
	    current_turn: 0,
	}
    }
    
    pub fn next_turn(&mut self) -> Option<&str> {
	let order = self.turn_order.as_ref()?;
	let entry = &order.entries[self.current_turn];
	let name = entry.0.as_str();
	self.current_turn = (self.current_turn + 1) % order.entries.len();
	Some(name)
    }
}
