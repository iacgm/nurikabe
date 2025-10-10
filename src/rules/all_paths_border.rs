use rustc_hash::FxHashSet as HashSet;

use super::*;

pub fn all_paths_border(knowledge: &mut Knowledge, board: &Board) {
    let mut islands = knowledge.island_set().clone();
    islands.sort_by_key(|i| i.n);

    for is in islands {
        let mut paths = enumerate_island_paths(knowledge, is);

        let Some(path) = paths.next() else {
            continue;
        };

        // This will cache the results for us.
        drop(paths);
        let paths = knowledge.island_paths(is).clone();

        let border = surrounding(board, &path);

        let mut intersection: HashSet<(usize, usize)> = border.into_iter().collect();

        for path in paths {
            let border = surrounding(board, &path);

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
