use super::*;

#[derive(Clone, Copy)]
pub struct GenSettings {
    pub dims: (usize, usize),
    pub max_island_size: usize,
    pub branch_factor: usize,
    pub max_island_count: usize,
}

pub fn try_generate(settings: GenSettings) -> Option<Board> {
    let (r, c) = settings.dims;

    let board = Board::empty(r, c);

    let mut options = vec![gen_options(&board, &settings)];
    let mut boards = vec![board];

    while let Some((i, opts)) = options.iter_mut().enumerate().last() {
        if i > settings.max_island_count {
            boards.pop();
            options.pop();
            continue;
        }

        let Some(next) = opts.pop() else {
            boards.pop();
            options.pop();
            continue;
        };

        let mut newb = boards.last().unwrap().clone();
        newb.add_island(next);

        if !advance(&mut newb) {
            continue;
        }

        if newb.solved() {
            return Some(newb);
        }

        options.push(gen_options(&newb, &settings));
        boards.push(newb);
    }

    None
}

pub fn gen_options(board: &Board, settings: &GenSettings) -> Vec<Island> {
    let n = settings.max_island_size;

    let mut v = board
        .iter()
        .filter_map(|(c, t)| if t == Empty { Some(c) } else { None })
        .filter(|&c| !neighbors(board, c).iter().any(|&n| board[n] == Land))
        .flat_map(|(r, c)| (1..n).map(move |n| Island { r, c, n }))
        .collect::<Vec<Island>>();

    shuffle(&mut v);
    v.truncate(settings.branch_factor);

    v
}

// true if board is valid
fn advance(board: &mut Board) -> bool {
    let mut known = Knowledge::new(board);

    'solve: loop {
        let board = known.board();

        for rule in MONOTONIC {
            use ReasonKind::*;

            rule(&mut known, &board);
            let reason = known.take_reason();

            match reason {
                Contradiction => return false,
                Loud(_) | Quiet(_) => {
                    continue 'solve;
                }
                Nil => (),
                _ => unreachable!(),
            }
        }

        break;
    }

    *board = known.board();

    true
}

fn shuffle<T>(v: &mut [T]) {
    use rand::seq::SliceRandom;
    let mut rng = rand::rng();
    v.shuffle(&mut rng);
}
