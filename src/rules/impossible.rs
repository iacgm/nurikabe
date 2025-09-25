use super::*;

pub fn impossible(known: &mut Knowledge, _: &Board) {
    if known.grid().iter().flatten().any(|s| s.is_empty()) {
        known.contradict();
    }
}
