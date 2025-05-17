use std::iter::once;

use super::*;

pub type Area = Vec<Coord>;

pub fn neighbors(board: &Board, (r, c): Coord) -> Area {
    let mut neighbors = vec![];

    let (h, w) = board.dims();

    if r > 0 {
        neighbors.push((r - 1, c));
    }
    if r < h - 1 {
        neighbors.push((r + 1, c));
    }

    if c > 0 {
        neighbors.push((r, c - 1));
    }
    if c < w - 1 {
        neighbors.push((r, c + 1));
    }
    neighbors
}

pub fn corners(board: &Board, (r, c): Coord) -> Area {
    let mut neighbors = vec![];

    let (h, w) = board.dims();

    let rs = r > 0;
    let re = r < h - 1;
    let cs = c > 0;
    let ce = c < w - 1;

    if rs && cs {
        neighbors.push((r - 1, c - 1));
    }
    if re && cs {
        neighbors.push((r + 1, c - 1));
    }

    if rs && ce {
        neighbors.push((r - 1, c + 1));
    }
    if re && ce {
        neighbors.push((r + 1, c + 1));
    }
    neighbors
}

pub fn area(board: &Board, (r, c): (usize, usize)) -> Area {
    let (h, w) = board.dims();

    let kind = board[(r, c)];
    let mut area = vec![];

    let mut visited = vec![];
    let mut stack = vec![(r, c)];

    while let Some((r, c)) = stack.pop() {
        if r >= h || c >= w || visited.contains(&(r, c)) {
            continue;
        }

        if board.tiles[r][c] == kind {
            area.push((r, c));
            stack.extend(neighbors(board, (r, c)));
        }

        visited.push((r, c));
    }

    area
}

pub fn surrounding(board: &Board, area: &Area) -> Area {
    let mut all_neighbors = vec![];
    for &(r, c) in area {
        for n in neighbors(board, (r, c)) {
            if !area.contains(&n) {
                all_neighbors.push(n);
            }
        }
    }

    all_neighbors
}

pub fn island_reaches(note: &Annotation, island: Island, coord: Coord) -> bool {
    fn dfs(
        note: &Annotation,
        curr: Coord,
        goal: Coord,
        visited: &mut Vec<Coord>,
        island: Island,
        limit: usize,
    ) -> bool {
        let tile = note.board[curr];

        let (r, c) = curr;
        if limit == 0 || tile == Tile::Sea || !note.possible_islands[r][c].contains(&island) {
            return false;
        }

        if curr == goal {
            return true;
        }

        for nt in neighbors(note.board, curr) {
            if dfs(note, nt, goal, visited, island, limit - 1) {
                return true;
            }
        }

        false
    }

    let Island { r, c, n } = island;
    let ic = (r, c);

    let area = area(&note.board, ic);

    area.iter()
        .any(|&ic| dfs(note, ic, coord, &mut vec![], island, n + 1 - area.len()))
}

pub fn enumerate_island_paths(
    note: &Annotation,
    island: Island,
) -> impl Iterator<Item = Vec<Coord>> {
    fn dfs<'a>(
        note: &'a Annotation,
        island: Island,
        current: Vec<Coord>,
    ) -> Box<dyn Iterator<Item = Vec<Coord>> + 'a> {
        if current.len() == island.n {
            return Box::new(once(current.clone()));
        }

        let mut reachable = surrounding(note.board, &current);

        // At first step, no lexicographic ordering required
        if current.len() != 1 {
            let prev = current.last().unwrap();
            let news = neighbors(note.board, *prev);
            reachable.retain(|t| t > prev || news.contains(t));
        }

        Box::new(
            reachable
                .clone()
                .into_iter()
                .filter(move |&(r, c)| note.possible_islands[r][c].contains(&island))
                .flat_map(move |n| {
                    let mut new = current.clone();
                    new.push(n);

                    dfs(note, island, new)
                }),
        )
    }

    let ic = (island.r, island.c);
    dfs(note, island, vec![ic]).filter(move |path| {
        !note.board.iter().any(|(c, t)| {
            t == Land && note.island(c) == Some(island) && !path.contains(&c)
        })
    })
}
