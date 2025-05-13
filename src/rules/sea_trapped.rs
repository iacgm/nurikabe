use super::*;

pub fn sea_trapped(board: &Board) -> Option<Update> {
    let mut update = Update::new(Justification::SeaTrapped);

    let (h, w) = board.dims();
    for r in 0..h {
        for c in 0..w {
            let coord = (r, c);
            let tile = board[coord];

            if tile != Sea {
                continue;
            }

            let area = area(board, coord);
            let surrounding = surrounding(board, area);

            let mut empties = surrounding.iter().filter(|&&c| board[c] == Empty);

            let Some(&empty) = empties.next() else {
                continue;
            };

            if empties.next().is_some() {
                continue;
            }

            update.set_sea(empty);
        }
    }

    update.check(board)
}
