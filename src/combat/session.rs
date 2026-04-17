use crate::{
    combat::initiative::{InitiativeEntry, TurnOrder},
    model::character::CharacterSheet,
};
use std::collections::HashMap;

pub struct Session<C: CharacterSheet> {
    pub characters: HashMap<String, C>,
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
        self.current_turn = (self.current_turn + 1) % order.entries.len();
        Some(order.entries[self.current_turn].name.as_str())
    }

    pub fn set_initiative(&mut self, name: String, value: i32) {
        let entries = match self.turn_order.take() {
            Some(mut order) => {
                if let Some(entry) = order.entries.iter_mut().find(|e| e.name == name) {
                    entry.initiative = value;
                } else {
                    order.entries.push(InitiativeEntry {
                        name,
                        initiative: value,
                    });
                }
                order.entries
            }
            None => vec![InitiativeEntry {
                name,
                initiative: value,
            }],
        };
        self.turn_order = Some(TurnOrder::new(entries));
        self.current_turn = 0;
    }

    pub fn roll_initiative(&mut self) {
        let entries = self.characters
            .iter()
            .map(|(name, c)| InitiativeEntry {
                name: name.to_string(),
                initiative: c.initiative(),
            })
            .collect();
        self.turn_order = Some(TurnOrder::new(entries));
        self.current_turn = 0;
    }

    pub fn unique_name(&self, name: &str) -> String {
        if !self.characters.contains_key(name) {
            return name.to_string();
        }
        let mut i = 2;
        loop {
            let candidate = format!("{}_{}", name, i);
            if !self.characters.contains_key(&candidate) {
                return candidate;
            }
            i += 1;
        }
    }
}
