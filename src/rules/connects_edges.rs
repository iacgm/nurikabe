use rustc_hash::FxHashSet as HashSet;

use super::*;

pub fn connects_edges(note: &Annotation) -> Option<Update> {
    let mut update = Update::new(Justification::ConnectsEdges);

    let board = note.board;
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

            let Some(i) = note.island(coord) else {
                continue;
            };

            if !grounded.contains(&i) {
                grounded.insert(i);
            }

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

    for r in 0..h {
        for c in 0..w {
            let coord = (r, c);
            let tile = board[coord];

            if tile != Empty {
                continue;
            }

            let mut grounded_is = neighbors(board, coord)
                .into_iter()
                .chain(corners(board, coord).into_iter())
                .filter(|&c| board[c] == Land)
                .filter_map(|c| note.island(c))
                .filter(|i| grounded.contains(i))
                .collect::<Vec<_>>();

            grounded_is.sort();
            grounded_is.dedup();

            if grounded_is.len() > 1 {
                update.set(coord, Sea);
                return update.check(board);
            }
        }
    }

    update.check(board)
}
