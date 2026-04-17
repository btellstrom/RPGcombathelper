// Character, a generic chacter for a roleplaying game
pub mod dnd;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub quantity: u32,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub name: String,
    pub description: String,
    pub damage: Option<String>,
    pub recharge: Option<String>,
}

pub trait CharacterSheet {
    type Stats;

    fn name(&self) -> &str;
    fn stats(&self) -> &Self::Stats;
    fn stats_mut(&mut self) -> &mut Self::Stats;
    fn inventory(&self) -> &HashMap<String, Item>;
    fn actions(&self) -> &HashMap<String, Action>;
    fn initiative(&self) -> i32;
}


