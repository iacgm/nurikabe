use super::*;

pub fn reachability(note: &Annotation) -> Option<Update> {
    let mut update = Update::new(Justification::Unreachable);

    let board = note.board;
    let (h, w) = board.dims();

    for r in 0..h {
        for c in 0..w {
            let coord = (r, c);
            let tile = board[coord];

            if tile != Empty {
                continue;
            }

            if note.possible_islands[r][c].is_empty() {
                update.set((r, c), Sea);
            }
        }
    }

    update.check(board)
}
