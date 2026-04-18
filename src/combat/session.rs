use std::collections::HashMap;
use std::collections::HashSet;
use std::marker::PhantomData;

use crate::{
    combat::initiative::{InitiativeEntry, TurnOrder},
    model::character::CharacterSheet,
};

pub struct Setup;
pub struct InCombat;

pub struct Session<C: CharacterSheet, Phase> {
    pub characters: HashMap<String, C>,
    pub turn_order: Option<TurnOrder>,
    pub current_turn: usize,
    _phase: PhantomData<Phase>,
}

impl<C: CharacterSheet, Phase> Session<C, Phase> {
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
    pub fn set_initiative(&mut self, name: String, value: i32) {
        // turn_order may or may not be a vector here, and we try to get it
        let entries = match self.turn_order.take() {
            Some(mut order) => {
                // Look through all entries in the vector to see if any matches the name
                if let Some(entry) = order.entries.iter_mut().find(|e| e.name == name) {
                    // If it matches, set the initiative to a specific value
                    entry.initiative = value;
                } else {
                    // Else, push a new one to the turn order
                    order.entries.push(InitiativeEntry {
                        name,
                        initiative: value,
                    });
                }
                // Resort at the end to match the current turn order
                order.entries
            }
            // If no entries in the turnorder yet, create a new vector which we will put
            // into a turnorder later
            None => vec![InitiativeEntry {
                name,
                initiative: value,
            }],
        };
        self.turn_order = Some(TurnOrder::new(entries));
    }
}

impl<C: CharacterSheet> Session<C, Setup> {
    pub fn new() -> Self {
        Session {
            characters: HashMap::new(),
            turn_order: None,
            current_turn: 0,
            _phase: PhantomData,
        }
    }

    pub fn start(mut self) -> Session<C, InCombat> {
        // Create a hashset to ensure that we only put people into the turnorder once
        let existing: HashSet<String> = self
            .turn_order
            .as_ref()
            .map(|o| o.entries.iter().map(|e| e.name.clone()).collect())
            .unwrap_or_default();

        // Take entries out of the TurnOrder
        let mut entries: Vec<InitiativeEntry> = self
            .turn_order
            .take()
            .map(|o| o.entries)
            .unwrap_or_default();

        for (name, c) in &self.characters {
            if !existing.contains(name) {
                entries.push(InitiativeEntry {
                    name: name.clone(),
                    initiative: c.initiative(),
                });
            }
        }

        Session {
            characters: self.characters,
            turn_order: Some(TurnOrder::new(entries)),
            current_turn: 0,
            _phase: PhantomData,
        }
    }
}

impl<C: CharacterSheet> Session<C, InCombat> {
    pub fn next_turn(&mut self) -> Option<&str> {
        let order = self.turn_order.as_ref()?;
        if order.entries.is_empty() {
            None
        } else {
            self.current_turn = (self.current_turn + 1) % order.entries.len();
            Some(order.entries[self.current_turn].name.as_str())
        }
    }
}
