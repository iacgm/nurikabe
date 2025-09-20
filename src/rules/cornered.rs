use super::*;

pub fn cornered(knowledge: &mut Knowledge) {
    let board = knowledge.board();
    for (coord, tile) in board.iter() {
        if tile != Empty {
            continue;
        }

        let mut neighboring_islands = neighbors(&board, coord)
            .into_iter()
            .filter(|&c| board[c] == Land)
            .filter_map(|c| knowledge.if_known(c))
            .collect::<Vec<_>>();

        neighboring_islands.sort();
        neighboring_islands.dedup();

        let num_island_neighbors = neighboring_islands.len();

        if num_island_neighbors > 1 {
            knowledge.set_sea(Reason::Cornered, coord);
        }
    }
}
