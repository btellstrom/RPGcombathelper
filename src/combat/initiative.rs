// Auto-roll or manual initiative assignment

pub struct InitiativeEntry {
    pub name: String,
    pub initiative: i32,
}

pub struct TurnOrder {
    pub entries: Vec<InitiativeEntry>,
}

impl TurnOrder {
    pub fn new(mut entries: Vec<InitiativeEntry>) -> Self {
        entries.sort_by(|a, b| b.initiative.cmp(&a.initiative));
        TurnOrder { entries }
    }
}
