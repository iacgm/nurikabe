use super::*;

pub fn reachability(known: &mut Knowledge) {
    use Possibility::*;

    let board = known.board();

    for island in known.island_set().clone() {
        let mut possible_squares = board
            .iter()
            .map(|p| p.0)
            .filter(|&c| known.get(c).contains(&Isle(island)))
            .collect::<Set<_>>();

        let paths = enumerate_island_paths(known, island);

        // Find nodes not reachable by this path
        for path in paths {
            for square in path {
                possible_squares.remove(&square);
            }
        }

        for square in possible_squares {
            known.elim_island(Reason::Unreachable, square, island);
        }
    }
}
