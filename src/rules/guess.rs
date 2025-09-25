use super::*;

pub fn guess(known: &mut Knowledge) {
    use ReasonKind::*;

    let board = known.board();
    let mut cells: Vec<_> = board
        .iter()
        .filter_map(|(c, t)| if t == Empty { Some(c) } else { None })
        .collect();

    cells.sort_by_key(|&c| known.get(c).len());

    for c in cells {
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
        }
    }

    if !known.solved() {
        known.reason = MaxDepthReached;
    }
}
