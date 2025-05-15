use super::*;

pub fn cornered(note: &Annotation) -> Option<Update> {
    let mut update = Update::new(Justification::Cornered);

    let board = note.board;
    for (coord, tile) in board.iter() {
        if tile != Empty {
            continue;
        }

        let mut neighboring_islands = neighbors(board, coord)
            .into_iter()
            .filter(|&c| board[c] == Land)
            .filter_map(|c| note.island(c))
            .collect::<Vec<_>>();

        neighboring_islands.sort();
        neighboring_islands.dedup();

        let num_island_neighbors = neighboring_islands.len();

        if num_island_neighbors > 1 {
            update.set(coord, Sea);
        }
    }

    update.check(board)
}
