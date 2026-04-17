use combathelper::model::character::dnd::{DndCharacter, DndStats};
use combathelper::model::character::{Action, CharacterSheet, Item};
use std::collections::HashMap;

fn make_character() -> DndCharacter {
    let mut inventory = HashMap::new();
    inventory.insert("Longsword".to_string(), Item {
        name: "Longsword".to_string(),
        quantity: 1,
        description: Some("A finely crafted blade.".to_string()),
    });

    let mut actions = HashMap::new();
    actions.insert("Attack".to_string(), Action {
        name: "Attack".to_string(),
        description: "Melee weapon attack.".to_string(),
        damage: Some("1d8+4".to_string()),
        recharge: None,
    });

    DndCharacter {
        name: "Aragorn".to_string(),
        stats: DndStats {
            hp: 40,
            max_hp: 40,
            armour_class: 16,
            strength: 18,
            dexterity: 14,
            constitution: 16,
            intelligence: 12,
            wisdom: 14,
            charisma: 16,
        },
        inventory,
        actions,
    }
}

#[test]
fn character_serializes_and_deserializes() {
    let original = make_character();
    let json = serde_json::to_string(&original).unwrap();
    let restored: DndCharacter = serde_json::from_str(&json).unwrap();
    assert_eq!(original.name(), restored.name());
    assert_eq!(original.stats().hp, restored.stats().hp);
    assert_eq!(original.stats().max_hp, restored.stats().max_hp);
    assert_eq!(original.stats().armour_class, restored.stats().armour_class);

    let orig_sword = original.inventory().get("Longsword").unwrap();
    let rest_sword = restored.inventory().get("Longsword").unwrap();
    assert_eq!(orig_sword.name, rest_sword.name);
    assert_eq!(orig_sword.quantity, rest_sword.quantity);
    assert_eq!(orig_sword.description, rest_sword.description);

    let orig_attack = original.actions().get("Attack").unwrap();
    let rest_attack = restored.actions().get("Attack").unwrap();
    assert_eq!(orig_attack.name, rest_attack.name);
    assert_eq!(orig_attack.damage, rest_attack.damage);
}
