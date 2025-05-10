use super::*;

pub fn pool(board: &mut Board) -> Option<Justification> {
    let (h, w) = board.dims();

    let mut used = None;
    for r in 0..h-1 {
        for c in 0..w-1 {
            let coords = [(r, c), (r+1,c), (r,c+1), (r+1,c+1)];

            let mut count = 0;
            for (r, c) in coords {
                if board.tiles[r][c] == Tile::Sea {
                    count += 1;
                }
            }

            if count != 3 {
                continue;
            }
            
            for (r,c) in coords {
                if board.tiles[r][c] == Tile::Empty {
                    board.tiles[r][c] = Tile::Land;
                    used = Some(Justification::Pool);
                }
            }

        }
    }

    used
}
