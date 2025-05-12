use super::*;

pub fn reachability(board: &Board) -> Option<Update> {
    let mut update = Update::new(Justification::Unreachable);

    let (h, w) = board.dims();

    for r in 0..h {
        for c in 0..w {
            let coord = (r,c);
            let tile = board.tiles[r][c];

            if tile != Empty {
                continue;
            }

            if !board.islands.iter().any(|&i| reachable_by(board, coord, i)) {
                update.set_sea(coord)
            }
        }
    }

    update.check(board)
}

fn reachable_by(board: &Board, coord: Coord, island: Island) -> bool {
    fn dfs(board: &Board, curr: Coord, goal: Coord, visited: &mut Vec<Coord>, limit: usize) -> bool {
        let tile = board[curr];

        if limit == 0 || tile == Tile::Sea || visited.contains(&curr) {
            return false;
        }

        if curr == goal {
            return true;
        }

        visited.push(curr);

        for nt in neighbors(board, curr) {
            if dfs(board, nt, goal, visited, limit - 1) {
                return true;
            }
        }

        false
    }

    let Island { r, c, n } = island;
    let ic = (r, c);

    dfs(board, ic, coord, &mut vec![], n)
}
