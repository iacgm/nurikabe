use rustc_hash::FxHashSet as HashSet;

use super::*;

pub fn all_paths_intersect(knowledge: &mut Knowledge, _: &Board) {
    let mut islands = knowledge.island_set().clone();
    islands.sort_by_key(|i| i.n);

    for &is in &islands {
        assert!(!knowledge.island_paths(is).is_empty());
    }

    'outer: for &is in &islands {
        let paths = knowledge.island_paths(is).clone();

        let mut intersection: HashSet<(usize, usize)> = paths[0].iter().copied().collect();

        for path in paths {
            let cells = HashSet::from_iter(path.into_iter());

            intersection = intersection.intersection(&cells).copied().collect();

            if intersection.is_empty() {
                continue 'outer;
            }
        }

        for &is in &islands {
            assert!(!knowledge.island_paths(is).is_empty());
        }
        for cell in intersection {
            assert!(knowledge.get(cell).contains(&Possibility::Isle(is)));
            knowledge.set_island(Reason::AllPathsIntersect, cell, is);
        }
        if knowledge.reason.is_set() {
            return;
        }
        for &is in &islands {
            assert!(!knowledge.island_paths(is).is_empty());
        }

    }
}
