use super::*;

pub fn cornered(board: &Board) -> Option<Update> {
    let mut update = Update::new(Justification::Cornered);

    let (h, w) = board.dims();
    for r in 0..h {
        for c in 0..w {
            let coord = (r, c);
            let tile = board[coord];

            if tile != Empty {
                continue;
            }

            let mut neighboring_islands = neighbors(board, coord)
                .into_iter()
                .filter_map(|(r, c)| board.island_map[r][c])
                .collect::<Vec<_>>();

            neighboring_islands.sort();
            neighboring_islands.dedup();

            let num_island_neighbors = neighboring_islands.len();

            if num_island_neighbors > 1 {
                update.set_sea(coord);
            }
        }
    }

    update.check(board)
}
