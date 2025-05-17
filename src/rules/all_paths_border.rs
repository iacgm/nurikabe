use std::collections::HashSet;

use super::*;

pub fn all_paths_border(note: &Annotation) -> Option<Update> {
    let mut islands = note.board.islands.clone();
    islands.sort_by_key(|i| i.n);

    for is in islands {
        let mut paths = enumerate_island_paths(note, is);

        let Some(path) = paths.next() else {
            continue;
        };

        let border = surrounding(note.board, &path);

        let mut intersection: HashSet<(usize, usize)> = border.into_iter().collect();

        for path in paths {
            let border = surrounding(note.board, &path);

            let cells = HashSet::from_iter(border.into_iter());

            intersection = intersection.intersection(&cells).copied().collect();

            if intersection.is_empty() {
                break;
            }
        }

        if !intersection.is_empty() {
            let mut update = Update::new(Justification::AllPathsBorder);
            for cell in intersection {
                update.set(cell, Sea);
            }
            let try_update = update.check(note.board);
            if try_update.is_some() {
                return try_update;
            }
        }
    }

    None
}
