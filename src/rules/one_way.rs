use super::*;

pub fn one_way(known: &mut Knowledge, board: &Board) {
    for (coord, tile) in board.iter() {
        if tile == Empty {
            continue;
        }

        let area = area(board, coord);
        let surrounding = surrounding(board, &area);

        let mut empties = surrounding.iter().filter(|&&c| board[c] == Empty);

        let Some(&empty) = empties.next() else {
            continue;
        };

        if empties.next().is_some() {
            continue;
        }

        if tile == Land {
            known.set_land(Reason::OneWayOut, empty);
        } else {
            known.set_sea(Reason::OneWayOut, empty);
        }
    }
}
