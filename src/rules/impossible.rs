use super::*;

pub fn impossible(known: &mut Knowledge, _: &Board) {
    if known.possibilities().iter().any(|s| s.is_empty()) {
        known.contradict();
    }
}
