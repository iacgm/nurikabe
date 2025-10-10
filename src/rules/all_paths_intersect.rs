use rustc_hash::FxHashSet as HashSet;

use super::*;

pub fn all_paths_intersect(knowledge: &mut Knowledge, _: &Board) {
    let mut islands = knowledge.island_set().clone();
    islands.sort_by_key(|i| i.n);

    for is in islands {
        let mut paths = enumerate_island_paths(knowledge, is);

        let Some(intersection) = paths.next() else {
            continue;
        };

        // This will cache the results for us.
        drop(paths);
        let paths = knowledge.island_paths(is).clone();

        let mut intersection: HashSet<(usize, usize)> = intersection.into_iter().collect();

        for path in paths {
            let cells = HashSet::from_iter(path.into_iter());

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
