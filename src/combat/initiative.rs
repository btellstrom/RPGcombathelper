// Auto-roll or manual initiative assignment

pub struct TurnOrder {
    pub entries : Vec<(String, i32)>,
}

impl TurnOrder {
    pub fn new(mut entries: Vec<(String, i32)>) -> Self {
	entries.sort_by(|a, b| b.1.cmp(&a.1));
	TurnOrder {entries}
    }
}
