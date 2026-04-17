// Rendering character stat blocks, turn tracker, inventory tables
use comfy_table::{Table, presets::UTF8_FULL};

use crate::model::character::{CharacterSheet, dnd::DndCharacter};

pub fn show_character(character: &DndCharacter) {
    println!("=== {} ===\n", character.name());

    //Stats table
    let mut stats_table = Table::new();
    let s = character.stats();
    stats_table.load_preset(UTF8_FULL);
    stats_table
        .set_header(vec!["Stat", "Value"])
        .add_row(vec!["HP", &format!("{}/{}", s.hp, s.max_hp)])
        .add_row(vec!["Armour Class", &s.armour_class.to_string()])
        .add_row(vec!["Strength", &s.strength.to_string()])
        .add_row(vec!["Dexterity", &s.dexterity.to_string()])
	.add_row(vec!["Constitution", &s.constitution.to_string()])
	.add_row(vec!["Intelligence", &s.intelligence.to_string()])
	.add_row(vec!["Wisdom", &s.wisdom.to_string()])
	.add_row(vec!["Charisma", &s.charisma.to_string()]);
    println!("{stats_table}\n");

    // Inventory table
    let mut inv_table = Table::new();
    inv_table.load_preset(UTF8_FULL);
    inv_table.set_header(vec!["Item", "Quantity", "Description"]);
    for (_, item) in character.inventory() {
	inv_table.add_row(vec!{
	    &item.name,
	    &item.quantity.to_string(),
	    item.description.as_deref().unwrap_or("-"),
	});
    }
    println!("{inv_table}\n");

    let mut actions_table = Table::new();
    actions_table.load_preset(UTF8_FULL);
    actions_table.set_header(vec!["Action", "Description", "Damage", "Recharge"]);
    for (_, action) in character.actions() {
	actions_table.add_row(vec![
	    &action.name,
	    &action.description,
	    action.damage.as_deref().unwrap_or("-"),
	    action.recharge.as_deref().unwrap_or("-"),
	]);
    }
    println!("{actions_table}");
}

