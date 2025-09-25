use super::*;

// Checks that Sea+Empty tiles form a single connected component
pub fn noncontiguous(known: &mut Knowledge, board: &Board) {
    if noncontiguous_board(board) {
        known.contradict();
    }
}
