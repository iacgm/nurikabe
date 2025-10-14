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
    let mut stack = vec![(r, c)];

    let mut visited = vec![false; h * w];

    let id = |(r, c)| r * w + c;

    while let Some(c) = stack.pop() {
        let i = id(c);

        if visited[i] {
            continue;
        }

        if board[c] == kind {
            area.push(c);
            stack.extend(neighbors(board, c));
        }

        visited[i] = true;
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

pub fn noncontiguous_board(board: &Board) -> bool {
    let count = board.iter().filter(|&(_, t)| t == Water).count();

    let Some(start) = board
        .iter()
        .find_map(|(c, t)| Some(c).filter(|_| t == Water))
    else {
        return false;
    };

    count != flood_count(board, start)
}

// Get size of a contiguous non-land segment
pub fn flood_count(board: &Board, start: Coord) -> usize {
    let mut stack = vec![start];
    let mut count = 0;

    let (h, w) = board.dims();
    let mut visited = vec![false; h * w];

    let id = |(r, c)| r * w + c;

    while let Some(coord) = stack.pop() {
        let i = id(coord);
        if visited[i] {
            continue;
        }
        visited[i] = true;

        let tile = board[coord];
        if tile != Land {
            if tile == Water {
                count += 1;
            }
            let neighbors = neighbors(board, coord);
            stack.extend(neighbors);
        }
    }

    count
}

pub fn board_with(board: &Board, path: &Area) -> Board {
    let mut board = board.clone();

    for &c in path {
        board[c] = Land;
    }

    board
}

pub fn is_valid(board: &Board) -> bool {
    make_valid(&mut board.clone())
}

pub fn make_valid(board: &mut Board) -> bool {
    let mut done = false;
    while !done {
        let Some(c) = fill_corners(board) else {
            return false;
        };
        done = c == 0;

        let Some(c) = extend_sea(board) else {
            return false;
        };

        done = done && (c == 0);
    }

    !noncontiguous_board(board)
}

// number of changes made if board is valid
pub fn fill_corners(board: &mut Board) -> Option<usize> {
    let (h, w) = board.dims();

    let mut change_count = 0;

    for r in 0..h - 1 {
        for c in 0..w - 1 {
            let coords = [(r, c), (r + 1, c), (r, c + 1), (r + 1, c + 1)];

            let mut count = 0;
            for c in coords {
                if board[c] == Water {
                    count += 1;
                }
            }

            if count == 4 {
                return None;
            }

            if count != 3 {
                continue;
            }

            for c in coords {
                if board[c] == Empty {
                    board[c] = Land;
                    change_count += 1;
                }
            }
        }
    }

    Some(change_count)
}

pub fn extend_sea(board: &mut Board) -> Option<usize> {
    let (h, w) = board.dims();

    let mut count = 0;
    let mut visited = vec![false; h * w];

    for r in 0..h {
        for c in 0..w {
            let coord = (r, c);
            if board[coord] != Water {
                continue;
            }

            let i = r * w + c;
            if visited[i] {
                continue;
            }
            visited[i] = true;

            let sea = area(board, coord);
            let mut border = surrounding(board, &sea);

            for (r, c) in sea {
                let i = r * w + c;
                visited[i] = true;
            }

            border.retain(|&c| board[c] == Empty);

            while border.len() == 1 {
                let c = border[0];
                board[c] = Water;
                visited[i] = true;
                count += 1;

                let sea = area(board, c);
                border = surrounding(board, &sea);
                border.retain(|&c| board[c] == Empty);
            }
        }
    }

    if !noncontiguous_board(board) {
        Some(count)
    } else {
        None
    }
}
