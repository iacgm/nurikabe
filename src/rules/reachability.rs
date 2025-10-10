use super::*;

pub fn reachability(known: &mut Knowledge, board: &Board) {
    use Possibility::*;

    for island in known.island_set().clone() {
        let mut possible_squares = board
            .iter()
            .map(|p| p.0)
            .filter(|&c| known.get(c).contains(&Isle(island)))
            .collect::<Set<_>>();

        let paths = known.island_paths(island);

        // Find nodes not reachable by this path
        for path in paths {
            for square in path {
                possible_squares.remove(square);
            }
        }

        for square in possible_squares {
            known.elim_island(Reason::Unreachable, square, island);
        }
    }
}
