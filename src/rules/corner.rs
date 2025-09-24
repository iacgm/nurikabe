use super::*;

pub fn cornered(knowledge: &mut Knowledge) {
    let board = knowledge.board();
    let (h, w) = board.dims();

    for r in 0..h - 1 {
        for c in 0..w - 1 {
            let coords = [(r, c), (r + 1, c), (r, c + 1), (r + 1, c + 1)];

            let mut count = 0;
            for (r, c) in coords {
                if board.tiles[r][c] == Water {
                    count += 1;
                }
            }

            if count != 3 {
                continue;
            }

            for c in coords {
                if knowledge.tile_known(c).is_none() {
                    knowledge.set_land(Reason::Pool, c);
                }
            }
        }
    }
}
