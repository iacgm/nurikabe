use super::*;

pub type Area = Vec<Coord>;

pub fn island(board: &Board, (r, c): (usize, usize)) -> (Area, Option<Island>) {
    let (h, w) = board.dims();

    let mut area = vec![];
    let mut island = None;

    let mut visited = vec![];
    let mut stack = vec![(r, c)];

    while let Some((r, c)) = stack.pop() {
        if r >= h || c >= w || visited.contains(&(r, c)) {
            continue;
        }

        island = board.check_island((r, c));

        if board.tiles[r][c] == Land {
            area.push((r, c));
            stack.extend(neighbors(board, (r, c)));
        }
        visited.push((r, c));
    }

    (area, island)
}

pub fn surrounding(board: &Board, area: Area) -> Area {
    let mut all_neighbors = vec![];
    for &(r, c) in &area {
        for n in neighbors(board, (r, c)) {
            if !area.contains(&n) {
                all_neighbors.push(n);
            }
        }
    }

    all_neighbors
}
