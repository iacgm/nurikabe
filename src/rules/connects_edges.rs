use rustc_hash::FxHashSet as HashSet;

use super::*;

pub fn connects_edges(knowledge: &mut Knowledge) {
    use Possibility::*;
    let board = knowledge.board();
    let (h, w) = board.dims();

    let mut grounded: HashSet<Island> = Default::default(); // Islands which touch the edge of the board

    let mut ground = |coord: Coord| {
        let mut stack = vec![coord];
        let mut visited = vec![];
        while let Some(coord) = stack.pop() {
            if visited.contains(&coord) {
                continue;
            };
            visited.push(coord);

            if board[coord] != Land {
                continue;
            }

            let Some(Isle(i)) = knowledge.if_known(coord) else {
                continue;
            };

            if !grounded.contains(&i) {
                grounded.insert(i);
            }

            let area = area(&board, coord);
            stack.extend(area.iter().flat_map(|&t| corners(&board, t)));
        }
    };

    for r in 0..h {
        ground((r, 0));
        ground((r, w - 1));
    }

    for c in 0..w {
        ground((0, c));
        ground((h - 1, c));
    }

    for (coord, tile) in board.iter() {
        if tile != Empty {
            continue;
        }

        let mut grounded_is = neighbors(&board, coord)
            .into_iter()
            .chain(corners(&board, coord).into_iter())
            .filter(|&c| board[c] == Land)
            .filter_map(|c| {
                if let Some(Isle(i)) = knowledge.if_known(c) {
                    Some(i)
                } else {
                    None
                }
            })
            .filter(|i| grounded.contains(i))
            .collect::<Vec<_>>();

        grounded_is.sort();
        grounded_is.dedup();

        if grounded_is.len() > 1 {
            knowledge.set_sea(Reason::ConnectsEdges, coord);
            return;
        }
    }
}
