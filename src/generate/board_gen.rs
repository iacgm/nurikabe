use rand::{
    distr::{Bernoulli, Distribution},
    seq::IndexedRandom,
};

use super::*;

#[derive(Clone, Copy)]
pub struct BoardGenSettings {
    pub dims: (usize, usize),
    pub mean_island_size: usize,
    pub max_island_size: usize,
    pub max_attempts: usize,
    pub branch_factor: usize,
    pub label_attempts: usize,
    pub max_depth: usize,
    pub max_amends: usize,
}

const FIXABLE_FRAC: f64 = 0.05;

pub fn gen_board(settings: BoardGenSettings) -> Option<Board> {
    let board = gen_unlabelled(settings)?;

    dbg!("Labelling");
    let labelled = metropolis_label(&board, settings);

    if empty_frac(&labelled) < FIXABLE_FRAC {
        return None;
    }

    dbg!("Amending");
    amend(&labelled, settings)
}

pub fn amend(board: &Board, settings: BoardGenSettings) -> Option<Board> {
    let (h, w) = board.dims();

    let mut out = Board::from_islands(h, w, board.islands.iter().copied());
    let mut soln = solve_with_limits(board, settings.max_depth);

    for _ in 0..settings.max_amends {
        if soln.contradiction {
            return None;
        }

        if soln.solved && soln.unique {
            return Some(out);
        }

        let forced = soln.forced_board();

        out.islands.sort_by_key(|&i| empty_reachable_by(forced, i));
        for is in &mut out.islands {
            let coord = (is.r, is.c);
            let area = area(forced, coord);

            if area.len() < is.n {
                is.n += 1;
                dbg!("!!");
                out = Board::from_islands(h, w, out.islands.into_iter());
                soln = solve_with_limits(&out, settings.max_depth);
                break;
            }
        }
    }

    if soln.solved && soln.unique {
        Some(out)
    } else {
        None
    }
}

fn empty_reachable_by(board: &Board, island: Island) -> usize {
    let coord = (island.r, island.c);
    let (h, w) = board.dims();

    let mut visited = vec![false; h * w];
    let mut stack = vec![coord];
    let mut count = 0;
    while let Some(next) = stack.pop() {
        let (r, c) = next;
        let id = r * w + c;
        if visited[id] {
            continue;
        }
        visited[id] = true;

        if board[next] != Water {
            stack.extend(neighbors(board, next));
        }

        if board[next] == Empty {
            count += 1;
        }
    }

    count
}

pub fn try_label(board: &Board, settings: BoardGenSettings) -> Option<Board> {
    let mut island_opts = vec![];

    let mut visited = vec![];
    for (c, t) in board.iter() {
        if t != Land {
            continue;
        }

        if visited.contains(&c) {
            continue;
        }

        let area = area(board, c);
        let n = area.len();
        visited.extend_from_slice(&area);

        let candidates = area
            .into_iter()
            .map(|(r, c)| (r, c, n).into())
            .collect::<Vec<Island>>();

        island_opts.push(candidates);
    }
    island_opts.sort_by_key(|is| is.len());

    let (h, w) = board.dims();
    let mut trial = Board::empty(h, w);
    for opts in &island_opts {
        // advance monotonically, board should never be invalid
        debug_assert!(monotonic(&mut trial));

        // try to select a clue that isn't already implied, if possible
        let first_choices = opts
            .iter()
            .cloned()
            .filter(|i| trial[(i.r, i.c)] != Land)
            .collect::<Vec<_>>();

        let opts = if first_choices.is_empty() {
            opts
        } else {
            &first_choices
        };

        let &opt = opts.choose(&mut rand::rng()).unwrap();
        trial.add_island(opt);
    }

    for _ in 0..settings.label_attempts {
        let solution = solve_with_limits(&trial, settings.max_depth);

        if solution.solved && solution.unique {
            return Some(trial);
        }

        let forced = solution.forced_board();
        trial = mutate(board, forced);
    }

    // TODO
    dbg!(&trial.islands);
    Some(trial)
}

fn mutate(board: &Board, trial: &Board) -> Board {
    use Tile::*;
    let (h, w) = board.dims();

    let mut changed = 0;
    let mut out = Board::empty(h, w);

    for &is in &trial.islands {
        let c = (is.r, is.c);
        let trial_area = area(trial, c);

        if trial_area.len() == is.n {
            out.add_island(is);
            continue;
        }

        debug_assert!(advance(&mut out));
        debug_assert!(monotonic(&mut out));

        changed += 1;

        let area = area(board, c);
        let mut candidates = area.clone();
        candidates.retain(|&c| out[c] != Land);

        let (r, c) = candidates
            .choose(&mut rand::rng())
            .cloned()
            .unwrap_or(area[0]);

        let is = Island { r, c, n: is.n };
        out.add_island(is);
    }
    dbg!(changed);

    out
}

pub fn gen_unlabelled(settings: BoardGenSettings) -> Option<Board> {
    let (h, w) = settings.dims;

    let board = Board::empty(h, w);

    let mut options = vec![gen_options(&board, settings)];
    let mut boards = vec![board];
    let mut attempts = 0;

    while let Some(opts) = options.last_mut() {
        let Some(next) = opts.pop() else {
            boards.pop();
            options.pop();
            continue;
        };

        let mut newb = boards.last().unwrap().clone();

        attempts += 1;
        if attempts > settings.max_attempts {
            return None;
        }

        for &c in &next {
            newb[c] = Land;
        }

        if !advance(&mut newb) {
            continue;
        }

        if large_islands(&newb, settings.max_island_size) {
            continue;
        }

        if newb.solved() {
            // Verify we have no large islands

            return Some(newb);
        }

        options.push(gen_options(&newb, settings));
        boards.push(newb.clone());
    }

    None
}

fn large_islands(board: &Board, max_size: usize) -> bool {
    let (h, w) = board.dims();
    let mut visited = vec![false; h * w];
    for ((r, c), t) in board.iter() {
        let i = r * w + c;
        if visited[i] {
            continue;
        }
        visited[i] = true;

        if t != Land {
            continue;
        }

        let area = area(board, (r, c));
        if area.len() > max_size {
            return true;
        }

        for (r, c) in area {
            let i = r * w + c;
            visited[i] = true;
        }
    }

    false
}

fn gen_options(board: &Board, settings: BoardGenSettings) -> Vec<Vec<Coord>> {
    let stem = find_stem(board);

    let mut v = (0..settings.branch_factor)
        .map(|_| grow_stem(board, stem, settings))
        .collect::<Vec<_>>();

    for p in &mut v {
        p.sort();
    }

    v.sort();
    v.dedup();

    v
}

// if false, this board has a contradiction
fn advance(board: &mut Board) -> bool {
    finish_all(board);

    make_valid(board)
}

fn finish_all(board: &mut Board) {
    let (h, w) = board.dims();

    for r in 0..h - 1 {
        for c in 0..w - 1 {
            let coord = (r, c);
            let tile = board[coord];

            if tile != Land {
                continue;
            }

            let area = area(board, coord);
            let border = surrounding(board, &area);

            for c in border {
                board[c] = Water
            }
        }
    }
}

fn find_stem(board: &Board) -> Coord {
    for (c, t) in board.iter() {
        if t != Land {
            continue;
        }

        if let Some(&c) = neighbors(board, c).iter().find(|&&c| board[c] == Empty) {
            return c;
        }
    }

    let candidates = board
        .iter()
        .filter_map(|(c, t)| if t == Empty { Some(c) } else { None })
        .collect::<Vec<_>>();

    let mut preferred = candidates.clone();
    preferred.retain(|&c| area(board, c).len() > 1);

    if let Some(s) = preferred.choose(&mut rand::rng()) {
        *s
    } else {
        *candidates.choose(&mut rand::rng()).unwrap()
    }
}

fn grow_stem(board: &Board, stem: Coord, settings: BoardGenSettings) -> Vec<Coord> {
    let area = area(board, stem);
    let mut size;
    let mut candidates = if board[stem] != Land {
        size = 0;
        vec![stem]
    } else {
        size = area.len();
        let mut v = surrounding(board, &area);
        v.retain(|&c| board[c] == Empty);
        v
    };

    let target_count = sample(size, settings);

    let mut vec = vec![];
    while size < target_count {
        let Some(&next) = candidates.choose(&mut rand::rng()) else {
            break;
        };

        vec.push(next);
        size += 1;
        candidates.extend(neighbors(board, next));
        candidates.retain(|&c| c != next && board[c] == Empty);
        candidates.sort();
        candidates.dedup();
    }
    vec
}

// Who likes using libraries?
pub fn sample(curr_size: usize, settings: BoardGenSettings) -> usize {
    if curr_size > settings.max_island_size {
        return 0;
    }

    let max = settings.max_island_size - curr_size;
    let mean = settings.mean_island_size - curr_size;
    let p = mean as f64 / max as f64;

    let mut count = 0;
    for _ in 0..max {
        if with_prob(p) {
            count += 1;
        }
    }

    count
}

pub fn with_prob(p: f64) -> bool {
    Bernoulli::new(p).unwrap().sample(&mut rand::rng())
}

pub fn empty_frac(board: &Board) -> f64 {
    use Tile::*;
    let (h, w) = board.dims();
    let empty_count = board.iter().filter(|(_, t)| *t == Empty).count();
    empty_count as f64 / (h * w) as f64
}
