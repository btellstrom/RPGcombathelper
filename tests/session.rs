use combathelper::combat::initiative::{InitiativeEntry, TurnOrder};
use combathelper::combat::session::Session;
use combathelper::model::character::dnd::{DndCharacter, DndStats};
use std::collections::HashMap;

fn make_character(name: &str, dexterity: i32) -> DndCharacter {
    DndCharacter {
        name: name.to_string(),
        stats: DndStats {
            hp: 20,
            max_hp: 20,
            armour_class: 12,
            strength: 10,
            dexterity,
            constitution: 10,
            intelligence: 10,
            wisdom: 10,
            charisma: 10,
        },
        inventory: HashMap::new(),
        actions: HashMap::new(),
    }
}

fn make_entry(name: &str, initiative: i32) -> InitiativeEntry {
    InitiativeEntry { name: name.to_string(), initiative }
}

#[test]
fn loaded_character_is_retrievable() {
    let mut session: Session<DndCharacter> = Session::new();
    session.characters.insert("Aragorn".to_string(), make_character("Aragorn", 14));
    assert!(session.characters.contains_key("Aragorn"));
}

#[test]
fn turn_order_is_sorted_descending() {
    let mut session: Session<DndCharacter> = Session::new();
    session.turn_order = Some(TurnOrder::new(vec![
        make_entry("Goblin", 12),
        make_entry("Aragorn", 18),
    ]));
    let order = session.turn_order.unwrap();
    assert_eq!(order.entries.len(), 2);
    assert!(order.entries[0].initiative >= order.entries[1].initiative);
}

#[test]
fn next_turn_returns_none_without_roll() {
    let mut session: Session<DndCharacter> = Session::new();
    assert!(session.next_turn().is_none());
}

#[test]
fn next_turn_wraps_around() {
    let mut session: Session<DndCharacter> = Session::new();
    session.turn_order = Some(TurnOrder::new(vec![
        make_entry("Aragorn", 18),
        make_entry("Goblin", 12),
    ]));
    session.next_turn();
    session.next_turn();
    assert_eq!(session.current_turn, 0);
}
