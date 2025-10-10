use super::*;

// We only apply this if there are 2 possibilities, otherwise we just us a generic guess
pub fn island_contra(known: &mut Knowledge, _: &Board) {
    use ReasonKind::*;
    let islands = known.island_set().clone();
    for is in islands {
        let paths = known.island_paths(is);

        let [path_a, path_b] = &paths[..] else {
            continue;
        };

        let path_a = &path_a.clone();
        let path_b = &path_b.clone();

        // Try each, using the other as a backup
        let combos = [(path_a, path_b), (path_b, path_a)];

        for (trial, backup) in combos {
            let Some(mut bifurcation) = known.bifurcate() else {
                return;
            };

            for &t in trial {
                bifurcation.set_land(Reason::Bifurcation, t);
            }

            let solution = solve_knowing(&mut bifurcation);
            let len = solution.reasons.len();
            if bifurcation.reason == Contradiction {
                for &t in backup {
                    known.set_land(Reason::ByContradiction(len), t);
                }
                return;
            }
        }
    }
}
