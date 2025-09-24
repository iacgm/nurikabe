use super::*;

// Checks that Sea+Empty tiles form a single connected component
pub fn noncontiguous(known: &mut Knowledge) {
    let board = known.board();
    let count = board.iter().filter(|&(_, t)| t != Land).count();

    let Some(start) = board
        .iter()
        .find_map(|(c, t)| Some(c).filter(|_| t != Land))
    else {
        return;
    };

    if count != flood_count(&board, start) {
        known.contradict();
    }
}

// Get size of a contiguous non-land segment
pub fn flood_count(board: &Board, start: Coord) -> usize {
    let mut stack = vec![start];
    let mut visited = vec![];
    let mut count = 0;

    while let Some(coord) = stack.pop() {
        if visited.contains(&coord) {
            continue;
        }

        if board[coord] != Land {
            count += 1;
        }

        visited.push(coord);

        let neighbors = neighbors(board, coord);
        stack.extend(neighbors);
    }

    count
}
