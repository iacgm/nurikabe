use super::*;

pub fn impossible(known: &mut Knowledge) {
    if known.grid().iter().flatten().any(|s| s.is_empty()) {
        known.contradict();
    }
}
