use std::collections::HashSet;

use super::*;

pub fn connects_edges(board: &Board) -> Option<Update> {
    let mut update = Update::new(Justification::ConnectsEdges);

    let (h, w) = board.dims();

    let mut grounded: HashSet<Island> = Default::default(); // Islands which touch the edge of the board

    let mut ground = |(r, c)| {
        let mut stack = vec![(r, c)];
        let mut visited = vec![];
        while let Some((r, c)) = stack.pop() {
            if visited.contains(&(r, c)) {
                continue;
            };
            visited.push((r, c));

            // Unsure why type inference breaks here and nowere else?
            let row: &Vec<Option<Island>> = &board.island_map[r];
            let Some(i) = row[c] else {
                continue;
            };

            grounded.insert(i);

            let area = area(board, (r, c));
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
                .filter_map(|(r, c)| board.island_map[r][c])
                .filter(|i| grounded.contains(i))
                .collect::<Vec<_>>();

            grounded_is.sort();
            grounded_is.dedup();

            if grounded_is.len() > 1 {
                update.set(coord, Sea);
            }
        }
    }

    update.check(board)
}
