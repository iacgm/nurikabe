use super::*;

const FEW: usize = 4;

// We only apply this if there are few possibilities, otherwise we just us a generic guess
pub fn island_contra(known: &mut Knowledge, _: &Board) {
    use ReasonKind::*;
    let mut islands = known.island_set().clone();
    islands.sort_by_key(|&is| known.island_paths(is).len());

    for is in islands {
        let paths = known.island_paths(is);

        let n = paths.len();
        if n == 1 || n > FEW {
            continue;
        }

        let paths = paths.clone();

        let mut contradictory = vec![None; n];

        for (i, path) in paths.iter().enumerate() {
            let Some(mut bifurcation) = known.bifurcate() else {
                return;
            };

            for &t in path {
                bifurcation.set_land(Reason::Bifurcation, t);
            }

            let solution = solve_knowing(&mut bifurcation);

            let prev_sol_found = contradictory[0..i].iter().any(|o| o.is_none());

            if bifurcation.reason == Contradiction {
                let len = solution.steps();
                contradictory[i] = Some(len);
                continue;
            } else if bifurcation.solved() && (!bifurcation.unique || prev_sol_found) {
                known.unique = false;
                for &t in path {
                    known.set_land(Reason::Bifurcation, t);
                }
                return;
            }
        }

        // Find correct solution
        let Some(sol) =
            contradictory
                .iter()
                .enumerate()
                .find_map(|(i, l)| if l.is_none() { Some(i) } else { None })
        else {
            // All result in a contradiction
            known.contradict();
            return;
        };

        if contradictory
            .iter()
            .enumerate()
            .all(|(i, p)| p.is_some() || i == sol)
        {
            let len = contradictory.iter().filter_map(|&o| o).max().unwrap();
            for &t in &paths[sol] {
                known.set_land(Reason::ByContradiction(len), t);
            }
            return;
        }
    }
}
