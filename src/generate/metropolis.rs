use super::*;
use rand::seq::IndexedRandom;

// Labels board using MH algorithm
pub fn metropolis_label(board: &Board, settings: BoardGenSettings) -> Board {
    let mut curr = label_randomly(board);
    let mut forced = solve_with_limits(&curr, 0).forced_board().clone();
    let mut score = score_board(&forced);

    let mut best = forced.clone();
    let mut best_score = score;

    for _ in 0..settings.label_attempts {
        let mut prop = mutate(&forced, board, settings);

        // Avoid resolving, if possible. We could even cache old results...
        prop.islands.sort();
        curr.islands.sort();
        if prop.islands == curr.islands {
            continue;
        }

        let prop_soln = solve_with_limits(&prop, 0);
        let prop_forced = prop_soln.forced_board().clone();
        let prop_score = score_board(&prop_forced);

        if prop_soln.solved && prop_soln.unique {
            return prop_forced;
        }

        let g1 = g(&prop, &forced, board, settings);
        let g2 = g(&curr, &prop_forced, board, settings);
        let ratio = (prop_score / score) * (g2 / g1);

        let acceptance_prob = ratio.min(1f64);

        if with_prob(acceptance_prob) {
            curr = prop;
            forced = prop_forced;
            score = prop_score;

            if score > best_score {
                best = forced.clone();
                best_score = score;
            }
        }
    }

    best
}

pub const MUTATION_RATE: f64 = 0.05;
pub const FIX_RATE: f64 = 0.25;
pub const TWEAK_RATE: f64 = 0.25;

fn mutate(forced: &Board, intended_soln: &Board, _settings: BoardGenSettings) -> Board {
    let (h, w) = intended_soln.dims();

    let mut out = Board::empty(h, w);

    // Relabel
    for &is in &forced.islands {
        if is.n == 1 {
            out.add_island(is);
            continue;
        }

        let coord = (is.r, is.c);
        let forced_area = area(forced, coord);
        let mut area = area(intended_soln, coord);
        let complete = is.n == forced_area.len();

        // If complete, low chance of mutation
        if complete {
            if with_prob(MUTATION_RATE) {
                area.retain(|&v| v != coord);
                let &(r, c) = area.choose(&mut rand::rng()).unwrap();
                let is = Island::from((r, c, is.n));
                out.add_island(is);
            } else {
                out.add_island(is);
            }
        } else if with_prob(FIX_RATE) {
            area.retain(|&v| forced[v] != Land);
            let &(r, c) = area.choose(&mut rand::rng()).unwrap();
            let is = Island::from((r, c, is.n));
            out.add_island(is);
        } else {
            out.add_island(is);
        }
    }

    out
}

fn g(new: &Board, forced: &Board, _intended_soln: &Board, _settings: BoardGenSettings) -> f64 {
    let mut g = 1.0;

    for &is in &forced.islands {
        if is.n == 1 {
            continue;
        }

        let changed = !new.islands.contains(&is);

        let coord = (is.r, is.c);
        let island = area(forced, coord);
        let complete = is.n == island.len();

        let rate = match (changed, complete) {
            (true, true) => MUTATION_RATE / (is.n - 1) as f64,
            (true, false) => {
                let options = is.n - island.len();
                FIX_RATE / options as f64
            }
            (false, true) => 1. - MUTATION_RATE,
            (false, false) => 1. - FIX_RATE,
        };

        g *= rate;
    }

    g
}

fn score_board(forced: &Board) -> f64 {
    let empty_frac = empty_frac(forced);

    const TUNING_PARAM: f64 = -12.;

    (empty_frac * TUNING_PARAM).exp2()
}

fn label_randomly(board: &Board) -> Board {
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

    trial
}
