use std::iter::once;

use super::*;

pub type Area = Vec<Coord>;

pub fn all_neighbors(board: &Board, c: Coord) -> Area {
    let mut all = neighbors(board, c);
    all.extend(corners(board, c));
    all
}

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

pub fn enumerate_island_paths(
    note: &Knowledge,
    island: Island,
) -> impl Iterator<Item = Vec<Coord>> {
    use Possibility::*;

    fn dfs<'a>(
        note: &'a Knowledge,
        island: Island,
        mut current: Vec<Coord>,
        board: Board,
    ) -> Box<dyn Iterator<Item = Vec<Coord>> + 'a> {
        if current.len() == island.n {
            return Box::new(once(current.clone()));
        }

        let mut reachable = surrounding(&board, &current);

        // At first step, no lexicographic ordering required
        if current.len() != 1 {
            let prev = current.last().unwrap();
            let news = neighbors(&board, *prev);
            reachable.retain(|t| t > prev || news.contains(t));
        }

        reachable.retain(|&c| note.get(c).contains(&Isle(island)));
        reachable.sort();

        if reachable.is_empty() {
            return Box::new([].into_iter());
        }

        let f = reachable[0];

        if board[f] == Land {
            current.push(f);
            return dfs(note, island, current, board);
        }

        Box::new(reachable.clone().into_iter().flat_map(move |n| {
            let mut new = current.clone();
            new.push(n);

            dfs(note, island, new, board.clone())
        }))
    }

    let board = note.board();

    let ic = (island.r, island.c);
    dfs(note, island, vec![ic], board.clone()).filter(move |path| {
        let surrounding = surrounding(&board, path);

        !board
            .iter()
            .any(|(c, t)| t == Land && note.if_known(c) == Some(Isle(island)) && !path.contains(&c))
            && !surrounding.iter().any(|&c| board[c] == Land)
    })
}
