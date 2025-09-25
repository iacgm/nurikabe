use rustc_hash::FxHashMap as HashMap;

use super::*;

pub fn connects_edges(knowledge: &mut Knowledge, board: &Board) {
    use Possibility::*;
    let (h, w) = board.dims();

    // Map island to island grounding it
    let mut grounded: HashMap<Island, Island> = Default::default();

    let mut ground = |coord: Coord| {
        let Some(Isle(edge_is)) = knowledge.if_known(coord) else {
            return;
        };

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

            grounded.insert(i, edge_is);

            let area = area(board, coord);
            stack.extend(area.iter().flat_map(|&t| corners(board, t)));
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

        let mut grounded_is = all_neighbors(board, coord)
            .into_iter()
            .filter_map(|c| {
                if let Some(Isle(i)) = knowledge.if_known(c) {
                    grounded.get(&i)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        grounded_is.sort();
        grounded_is.dedup();

        if grounded_is.len() > 1 {
            knowledge.set_sea(Reason::ConnectsEdges, coord);
            return;
        }

        // Edge connections are (literally) an edge case
        let (r, c) = coord;
        let is_edge = r == 0 || c == 0 || r == h - 1 || c == w - 1;
        if let [island] = &grounded_is[..]
            && is_edge
            && !knowledge.get(coord).contains(&Isle(**island))
        {
            knowledge.set_sea(Reason::ConnectsEdges, coord);
        }
    }
}
