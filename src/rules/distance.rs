use super::*;

pub fn distance(board: &mut Board) -> Option<Justification> {
    use Tile::*;

    let (h, w) = board.dims();

    let mut justification = None;
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
                justification = Some(Justification::Unreachable);
                board.tiles[r][c] = Sea;
            }
        }
    }

    justification
}
