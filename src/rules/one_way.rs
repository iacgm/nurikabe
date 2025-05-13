use super::*;

pub fn one_way(board: &Board) -> Option<Update> {
    let mut update = Update::new(Justification::OneWayOut);

    let (h, w) = board.dims();
    for r in 0..h {
        for c in 0..w {
            let coord = (r, c);
            let tile = board[coord];

            if tile != Land {
                continue;
            }

            let (area, _) = island(board, coord);

            let surrounding = surrounding(board, area);

            let mut empties = surrounding.iter().filter(|&&c| board[c] == Empty);

            let Some(&empty) = empties.next() else {
                continue;
            };

            if empties.next().is_some() {
                continue;
            }

            update.set_land(empty);
        }
    }

    update.check(board)
}
