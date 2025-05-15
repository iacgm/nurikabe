use super::*;

pub fn distance(note: &Annotation) -> Option<Update> {
    let board = note.board;
    let (h, w) = board.dims();

    let mut update = Update::new(Justification::TooFar);
    for r in 0..h {
        for c in 0..w {
            let tile = board.tiles[r][c];
            if tile != Empty {
                continue;
            }

            let mut too_far = true;
            for island in &board.islands {
                let Island { r: ri, c: ci, n } = *island;

                let dr = r.abs_diff(ri);
                let dc = c.abs_diff(ci);

                if dr + dc <= n - 1 {
                    too_far = false;
                    break;
                }
            }

            if too_far {
                update.sea.push((r, c));
            }
        }
    }

    update.check(board)
}
