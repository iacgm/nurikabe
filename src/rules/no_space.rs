use super::*;

pub fn no_space(known: &mut Knowledge, board: &Board) {
    for &island in known.island_set() {
        if !enumerate_island_paths(known, island)
            .any(|path| !noncontiguous_board(&board_with(board, &path)))
        {
            known.contradict();
            return;
        }
    }
}
