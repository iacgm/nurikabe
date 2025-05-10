use super::*;

pub fn finished(board: &mut Board) -> Option<Justification> {
    let (h, w) = board.dims();

    let mut changed_board = false;
    for r in 0..h {
        for c in 0..w {
            let Tile::Number(n) = board.tiles[r][c] else {
                continue;
            };

            let island = island(board, r, c);

            if island.len() == n {
                changed_board |= surround(board, island);
            }
        }
    }

    if changed_board {
        Some(Justification::Finished)
    } else {
        None
    }
}

fn island(board: &Board, r: usize, c: usize) -> Vec<(usize, usize)> {
    let (h, w) = board.dims();

    let mut island = vec![];
    let mut visited = vec![];

    let mut stack = vec![(r, c)];

    while let Some((r, c)) = stack.pop() {
        if r >= h || c >= w || visited.contains(&(r, c)) {
            continue;
        }

        if board.tiles[r][c].land() {
            island.push((r, c));
            stack.extend([
                (r, c + 1),
                (r + 1, c),
                (r, c.saturating_sub(1)),
                (r.saturating_sub(1), c),
            ]);
        }
        visited.push((r, c));

    }

    island
}

fn surround(board: &mut Board, island: Vec<(usize, usize)>) -> bool {
    let mut changed_board = false;
    for &(r, c) in &island {
        let neighbors = [
            (r, c + 1),
            (r + 1, c),
            (r, c.saturating_sub(1)),
            (r.saturating_sub(1), c),
        ];

        for (nr, nc) in neighbors {
            if !island.contains(&(nr, nc)) {
                if board.tiles[nr][nc] == Tile::Empty {
                    board.tiles[nr][nc] = Tile::Sea;
                    changed_board = true;
                }
            }
        }
    }

    changed_board
}
