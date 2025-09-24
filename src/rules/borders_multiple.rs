use super::*;
pub fn borders_multiple(known: &mut Knowledge) {
    use Possibility::*;

    let board = known.board();
    for (coord, tile) in board.iter() {
        if tile != Land {
            continue;
        }

        let island = area(&board, coord);
        let surrounding = surrounding(&board, &island);

        let mut possibilities = known.get(coord).clone();
        for &n in &island {
            possibilities = possibilities.intersection(known.get(n)).copied().collect();
        }

        let island_set = known.island_set().clone();
        let to_update = island.iter().chain(surrounding.iter());

        for &n in to_update {
            for &i in island_set
                .iter()
                .filter(|&&i| !possibilities.contains(&Isle(i)))
            {
                known.elim_island(Reason::TouchesIslands, n, i);
            }
        }
    }
}
