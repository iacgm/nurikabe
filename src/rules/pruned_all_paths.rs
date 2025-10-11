use rustc_hash::FxHashSet as HashSet;

use super::*;

const LONG: usize = 5;

pub fn pruned_all_paths_intersect(knowledge: &mut Knowledge, board: &Board) {
    let mut islands = knowledge.island_set().clone();
    islands.sort_by_key(|i| i.n);

    for is in islands {
        let mut paths = knowledge.island_paths(is).clone();
        if paths.len() >= LONG {
            continue;
        }
        paths.retain(|p| is_valid(&board_with(board, p)));

        if paths.is_empty() {
            continue;
        }

        let mut intersection: HashSet<(usize, usize)> = paths[0].iter().cloned().collect();

        for path in &paths[1..] {
            let cells = HashSet::from_iter(path.iter().cloned());

            intersection = intersection.intersection(&cells).copied().collect();

            if intersection.is_empty() {
                break;
            }
        }

        if !intersection.is_empty() {
            for cell in intersection {
                knowledge.set_land(Reason::AllPathsIntersect, cell);
            }
            if knowledge.reason.is_set() {
                return;
            }
        }
    }
}

pub fn pruned_all_paths_border(knowledge: &mut Knowledge, board: &Board) {
    let mut islands = knowledge.island_set().clone();
    islands.sort_by_key(|i| i.n);

    for is in islands {
        let mut paths = knowledge.island_paths(is).clone();
        if paths.len() >= LONG {
            continue;
        }
        paths.retain(|p| is_valid(&board_with(board, p)));

        if paths.is_empty() {
            continue;
        }

        let border = surrounding(board, &paths[0]);

        let mut intersection: HashSet<(usize, usize)> = border.iter().cloned().collect();

        for path in &paths[1..] {
            let border = surrounding(board, path);
            let cells = HashSet::from_iter(border.into_iter());

            intersection = intersection.intersection(&cells).copied().collect();

            if intersection.is_empty() {
                break;
            }
        }

        if !intersection.is_empty() {
            for cell in intersection {
                knowledge.set_sea(Reason::AllPathsBorder, cell);
            }

            if knowledge.reason.is_set() {
                return;
            }
        }
    }
}
