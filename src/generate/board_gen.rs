use rand::{
    distr::{Bernoulli, Distribution},
    seq::IndexedRandom,
};

use super::*;

#[derive(Clone, Copy)]
pub struct BoardGenSettings {
    pub dims: (usize, usize),
    pub power: f32,
    pub max_island_size: usize, // Not technically a hard limit, but creating larger than this is an edge case
    pub max_attempts: usize,
    pub branch_factor: usize,
    pub label_attempts: usize,
}

pub fn gen_board(settings: BoardGenSettings) -> Option<Board> {
    let board = gen_unlabelled(settings)?;

    dbg!("Labelling");
    try_label(board, settings)
}

pub fn try_label(board: Board, settings: BoardGenSettings) -> Option<Board> {
    let mut island_opts = vec![];

    let mut visited = vec![];
    for (c, t) in board.iter() {
        if t != Land {
            continue;
        }

        if visited.contains(&c) {
            continue;
        }

        let area = area(&board, c);
        let n = area.len();
        visited.extend_from_slice(&area);

        let candidates = area
            .into_iter()
            .map(|(r, c)| (r, c, n).into())
            .collect::<Vec<Island>>();

        island_opts.push(candidates);
    }
    island_opts.sort_by_key(|is| is.len());
    dbg!(&island_opts);

    let (h, w) = board.dims();
    let mut init = Board::empty(h, w);

    for i in island_opts.iter().filter(|i| i.len() == 1) {
        init.add_island(i[0]);
    }
    island_opts.retain(|i| i.len() > 1);

    for i in 0..settings.label_attempts {
        dbg!(i);
        let mut trial = init.clone();

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

        dbg!("??");
        dbg!(&trial.islands);
        let solution = solve(trial.clone());
        dbg!("!!");

        if solution.solved && solution.unique {
            return Some(trial);
        }
    }

    None
}

pub fn gen_unlabelled(settings: BoardGenSettings) -> Option<Board> {
    let (r, c) = settings.dims;

    let board = Board::empty(r, c);

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

        if newb.solved() {
            return Some(newb);
        }

        options.push(gen_options(&newb, settings));
        boards.push(newb.clone());
    }

    None
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

    *candidates.choose(&mut rand::rng()).unwrap()
}

fn grow_stem(board: &Board, stem: Coord, settings: BoardGenSettings) -> Vec<Coord> {
    let pow = settings.power as f64;

    let d = Bernoulli::new(pow).unwrap();

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

    let mut vec = vec![];
    while d.sample(&mut rand::rng()) && size < settings.max_island_size {
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
