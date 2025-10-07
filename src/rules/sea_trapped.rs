use super::*;

pub fn trapped(knowledge: &mut Knowledge, board: &Board) {
    for (coord, tile) in board.iter() {
        if tile != Empty {
            continue;
        }

        let area = area(board, coord);
        let surrounding = surrounding(board, &area);

        let trapped = board.iter().any(|(_, t)| t == Water)
            && surrounding.iter().all(|coord| board[*coord] == Land);
        let mut islands = surrounding
            .iter()
            .filter_map(|&(r, c)| knowledge.if_known((r, c)))
            .collect::<Vec<_>>();

        islands.sort();
        islands.dedup();

        if trapped && islands.len() == 1 {
            for &t in &area {
                knowledge.set_land(Reason::Trapped, t);
            }
        }
    }
}
