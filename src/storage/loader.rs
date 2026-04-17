// Deserialize characters/monsters from JSON files
use std::path::Path;
use std::fs;
use anyhow::Result;
use crate::model::character::dnd::DndCharacter;

pub fn load_character(path: &Path) -> Result<DndCharacter> {
    let contents = fs::read_to_string(path)?;
    let character = serde_json::from_str(&contents)?;
    Ok(character)
}
