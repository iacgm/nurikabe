use super::*;

pub fn one_way(note: &Annotation) -> Option<Update> {
    let mut update = Update::new(Justification::OneWayOut);

    let board = note.board;
    let (h, w) = board.dims();
    for r in 0..h {
        for c in 0..w {
            let coord = (r, c);
            let tile = board[coord];

            if tile == Empty {
                continue;
            }

            let area = area(board, coord);
            let surrounding = surrounding(board, &area);

            let mut empties = surrounding.iter().filter(|&&c| board[c] == Empty);

            let Some(&empty) = empties.next() else {
                continue;
            };

            if empties.next().is_some() {
                continue;
            }

            update.set(empty, tile);
        }
    }

    update.check(board)
}
