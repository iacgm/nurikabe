use super::*;

pub fn pools(known: &mut Knowledge, board: &Board) {
    let (h, w) = board.dims();

    for r in 0..h - 1 {
        for c in 0..w - 1 {
            let square = [(r, c), (r + 1, c), (r, c + 1), (r + 1, c + 1)];

            let pool = square.iter().all(|&c| board[c] == Water);

            if pool {
                known.contradict();
                return;
            }
        }
    }
}
