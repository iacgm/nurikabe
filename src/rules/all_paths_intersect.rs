use rustc_hash::FxHashSet as HashSet;

use super::*;

pub fn all_paths_intersect(note: &Annotation) -> Option<Update> {
    let mut islands = note.board.islands.clone();
    islands.sort_by_key(|i| i.n);

    for is in islands {
        let mut paths = enumerate_island_paths(note, is);

        let Some(intersection) = paths.next() else {
            continue;
        };

        let mut intersection: HashSet<(usize, usize)> = intersection.into_iter().collect();

        for path in paths {
            let cells = HashSet::from_iter(path.into_iter());

            intersection = intersection.intersection(&cells).copied().collect();

            if intersection.is_empty() {
                break;
            }
        }

        if !intersection.is_empty() {
            let mut update = Update::new(Justification::AllPathsIntersect);
            for cell in intersection {
                update.set(cell, Land);
            }
            let try_update = update.check(note.board);
            if try_update.is_some() {
                return try_update;
            }
        }
    }

    None
}
