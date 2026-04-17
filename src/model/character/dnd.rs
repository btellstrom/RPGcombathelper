use super::{CharacterSheet, Item, Action};
use clap::Args;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Args, Serialize, Deserialize)]
pub struct DndStats {
    pub hp: i32,
    pub max_hp: i32,
    pub armour_class: i32,
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub intelligence: i32,
    pub wisdom: i32,
    pub charisma: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DndCharacter {
    pub name: String,
    pub stats: DndStats,
    pub inventory: HashMap<String, Item>,
    pub actions: HashMap<String, Action>,
}

impl CharacterSheet for DndCharacter {
    type Stats = DndStats;

    fn name(&self) -> &str {
	&self.name
    }

    fn stats(&self) -> &Self::Stats {
	&self.stats
    }

    fn stats_mut(&mut self) -> &mut Self::Stats {
	&mut self.stats
    }

    fn inventory(&self) -> &HashMap<String, Item> {
	&self.inventory
    }

    fn actions(&self) -> &HashMap<String, Action> {
	&self.actions
    }
}
