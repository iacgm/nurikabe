use super::*;

pub fn no_space(known: &mut Knowledge, _: &Board) {
    for &island in known.island_set() {
        if enumerate_island_paths(known, island).next().is_none() {
            known.contradict();
            return;
        }
    }
}
