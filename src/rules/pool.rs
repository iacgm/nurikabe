use super::*;

pub fn pool(note: &Annotation) -> Option<Update> {
    let board = note.board;
    let (h, w) = board.dims();

    let mut update = Update::new(Justification::Pool);
    for r in 0..h - 1 {
        for c in 0..w - 1 {
            let coords = [(r, c), (r + 1, c), (r, c + 1), (r + 1, c + 1)];

            let mut count = 0;
            for (r, c) in coords {
                if board.tiles[r][c] == Sea {
                    count += 1;
                }
            }

            if count != 3 {
                continue;
            }

            for (r, c) in coords {
                if board.tiles[r][c] == Empty {
                    update.land.push((r, c));
                }
            }
        }
    }

    update.check(board)
}
