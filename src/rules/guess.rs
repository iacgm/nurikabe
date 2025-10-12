use super::*;

pub fn guess(known: &mut Knowledge, board: &Board) {
    use ReasonKind::*;
    let mut cells: Vec<_> = board
        .iter()
        .filter_map(|(c, t)| if t == Empty { Some(c) } else { None })
        .collect();

    // Long expression. It just counts the number of possibilities for this tile, with multiplicity for islands.
    cells.sort_by_key(|&c| {
        known
            .get(c)
            .clone()
            .iter()
            .filter_map(|p| {
                if let Possibility::Isle(i) = p {
                    Some(i)
                } else {
                    None
                }
            })
            .map(|&i| {
                known
                    .island_paths(i)
                    .iter()
                    .filter(|&p| p.contains(&c))
                    .count()
            })
            .sum::<usize>()
    });

    for c in cells {
        let mut sol_found = false;

        let Some(mut bif) = known.bifurcate() else {
            return;
        };

        bif.set_land(Reason::Bifurcation, c);

        let solution = solve_knowing(&mut bif);

        if bif.reason == Contradiction {
            let len = solution.steps();
            known.set_sea(Reason::ByContradiction(len), c);
            if known.reason.is_set() {
                return;
            }
        } else if bif.solved() {
            if !bif.unique {
                known.unique = false;
                known.set_sea(Reason::Bifurcation, c);
                return;
            }
            sol_found = true;
        }

        let Some(mut bif) = known.bifurcate() else {
            return;
        };

        bif.set_sea(Reason::Bifurcation, c);

        let solution = solve_knowing(&mut bif);
        if bif.reason == Contradiction {
            let len = solution.steps();
            known.set_land(Reason::ByContradiction(len), c);
            if known.reason.is_set() {
                return;
            }
        } else if bif.solved() && (sol_found || !bif.unique) {
            known.unique = false;
            known.set_land(Reason::Bifurcation, c);
            return;
        }
    }

    if !known.solved() {
        known.reason = MaxDepthReached;
    }
}
