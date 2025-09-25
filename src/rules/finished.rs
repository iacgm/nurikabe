use super::*;

pub fn finished(knowledge: &mut Knowledge, board: &Board) {
    use Possibility::*;
    for (coord, t) in board.iter() {
        if t != Land {
            continue;
        }

        let Some(Isle(island)) = knowledge.if_known(coord) else {
            continue;
        };

        let area = area(board, coord);

        if area.len() == island.n {
            for n in surrounding(board, &area) {
                knowledge.set_sea(Reason::Finished, n);
            }

            for (c, _) in board.iter().filter(|(c, _)| !area.contains(c)) {
                knowledge.elim_island(Reason::Finished, c, island);
            }
        }
    }
}
