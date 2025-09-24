use super::*;

// We only apply this if there are 2 possibilities, otherwise we just us a generic guess
pub fn island_contra(known: &mut Knowledge) {
    let islands = known.island_set();
    for &is in islands {
        let paths = enumerate_island_paths(known, is).collect::<Vec<_>>();

        let [path_a, path_b] = &paths[..] else {
            continue;
        };

        // Try each, using the other as a backup
        let combos = [(path_a, path_b), (path_b, path_a)];

        for (trial, backup) in combos {
            let mut bifurcation = known.bifurcate();

            for &t in trial {
                bifurcation.set_land(Reason::Bifurcation, t);
            }

            let solution = solve_knowing(&mut bifurcation);
            let depth = solution.depth;
            let length = solution.states.len() - 1;
            if !bifurcation.solved() {
                for &t in backup {
                    known.set_land(Reason::ByContradiction(length, depth), t);
                }
                return;
            }
        }
    }
}
