use super::*;

pub fn guess(known: &mut Knowledge, board: &Board) {
    use ReasonKind::*;
    let mut cells: Vec<_> = board
        .iter()
        .filter_map(|(c, t)| if t == Empty { Some(c) } else { None })
        .collect();

    cells.sort_by_key(|&c| known.get(c).len());

    for c in cells {
        let mut sol_found = false;

        let Some(mut bif) = known.bifurcate() else {
            return;
        };

        bif.set_land(Reason::Bifurcation, c);

        let solution = solve_knowing(&mut bif);

        if bif.reason == Contradiction {
            let len = solution.reasons.len();
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
            let len = solution.reasons.len();
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
